use std::sync::Arc;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};
use crate::engine::state::MarketState;

pub fn render(frame: &mut Frame, state: &Arc<MarketState>) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header
            Constraint::Min(15),    // Order book and trades
            Constraint::Length(5),  // Market metrics
            Constraint::Length(1),  // Footer
        ])
        .split(frame.area());

    render_header(frame, chunks[0], state);
    render_main(frame, chunks[1], state);
    render_metrics(frame, chunks[2], state);
    render_footer(frame, chunks[3]);
}

fn render_header(frame: &mut Frame, area: Rect, state: &Arc<MarketState>) {
    let metrics = state.metrics.load();
    
    let is_syncing = state.is_syncing.try_read()
        .map(|guard| *guard)
        .unwrap_or(true);
    
    let status = if is_syncing {
        Span::styled("SYNCING", Style::default().fg(Color::Yellow))
    } else {
        Span::styled("LIVE", Style::default().fg(Color::Green))
    };
    
    // Format depth status
    let depth_text = Span::raw("Ok");
    
    // Format lag with color coding
    let format_lag = |lag_ms: Option<u64>| -> Span {
        match lag_ms {
            None => Span::raw("N/A"),
            Some(ms) if ms < 100 => Span::styled(format!("{}ms", ms), Style::default().fg(Color::Green)),
            Some(ms) if ms < 500 => Span::styled(format!("{}ms", ms), Style::default().fg(Color::Yellow)),
            Some(ms) => Span::styled(format!("{}ms", ms), Style::default().fg(Color::Red)),
        }
    };
    
    let header_text = vec![
        Line::from(vec![
            Span::from(&state.symbol),
            Span::raw(" | "),
            status,
            Span::raw(" | "),
            Span::raw("Depth: "),
            depth_text,
            //Span::raw(format!(" ({}/{})", metrics.bid_depth, metrics.ask_depth)),
            Span::raw(" | "),
            Span::raw("Orderbook Lag: "),
            format_lag(metrics.orderbook_lag_ms),
            Span::raw(" | "),
            Span::raw("Trade Lag: "),
            format_lag(metrics.trade_lag_ms),
            Span::raw(" | "),
            Span::raw(format!("Updates/s: {:.1}", metrics.updates_per_second)),
        ]),
    ];
    
    let header = Paragraph::new(header_text)
        .block(Block::default().borders(Borders::ALL).title("Market Data Engine"));
    
    frame.render_widget(header, area);
}

fn render_main(frame: &mut Frame, area: Rect, state: &Arc<MarketState>) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(60),
            Constraint::Percentage(40),
        ])
        .split(area);
    
    // Left panel: Order Book
    render_orderbook(frame, chunks[0], state);
    
    // Right panel: Trade Flow
    render_trade_flow(frame, chunks[1], state);
}

fn render_orderbook(frame: &mut Frame, area: Rect, state: &Arc<MarketState>) {
    let mut lines = vec![];
    
    // ASK header
    lines.push(Line::from(vec![
        Span::styled("  ASK", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
    ]));
    
    // Sample asks (top 5)
    for i in 0..5 {
        lines.push(Line::from(vec![
            Span::styled(format!(" {:.2}", 2821.45 - i as f64 * 0.01), Style::default().fg(Color::Red)),
            Span::raw(" | "),
            Span::raw(format!("{:>6.2}", 0.82 + i as f64 * 0.15)),
        ]));
    }
    
    // Separator
    lines.push(Line::from(vec![
        Span::raw("─".repeat(area.width as usize - 2)),
    ]));
    
    // Sample bids (top 5)
    for i in 0..5 {
        lines.push(Line::from(vec![
            Span::styled(format!(" {:.2}", 2821.41 - i as f64 * 0.01), Style::default().fg(Color::Green)),
            Span::raw(" | "),
            Span::raw(format!("{:>6.2}", 1.88 - i as f64 * 0.25)),
        ]));
    }
    
    // BID footer
    lines.push(Line::from(vec![
        Span::styled("  BID", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
    ]));
    
    let paragraph = Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title("Order Book (Top 10)"));
    
    frame.render_widget(paragraph, area);
}

fn render_trade_flow(frame: &mut Frame, area: Rect, state: &Arc<MarketState>) {
    let metrics = state.metrics.load();
    
    let mut lines = vec![];
    
    // Last trade
    lines.push(Line::from(vec![
        Span::styled("Last Trade:", Style::default().add_modifier(Modifier::BOLD)),
    ]));
    lines.push(Line::from(vec![
        Span::raw("  "),
        Span::styled(format!("{}", format_opt_decimal(metrics.last_price)), Style::default().fg(Color::Cyan)),
        Span::raw("  "),
        Span::raw(format!("{}", format_opt_decimal(metrics.last_qty))),
        Span::raw("  "),
        Span::styled("BUY", Style::default().fg(Color::Green)),
    ]));
    lines.push(Line::from(""));
    
    // Recent trades header
    lines.push(Line::from(vec![
        Span::styled("Recent Trades:", Style::default().add_modifier(Modifier::BOLD)),
    ]));
    
    // Sample recent trades
    let sample_trades = vec![
        ("2821.40", "0.0042", "BUY", Color::Green),
        ("2821.39", "0.0010", "SELL", Color::Red),
        ("2821.40", "0.0025", "BUY", Color::Green),
        ("2821.39", "0.0008", "SELL", Color::Red),
    ];
    
    for (price, qty, side, color) in sample_trades {
        lines.push(Line::from(vec![
            Span::raw(" "),
            Span::styled(price, Style::default().fg(Color::Cyan)),
            Span::raw("  "),
            Span::raw(qty),
            Span::raw("  "),
            Span::styled(side, Style::default().fg(color)),
        ]));
    }
    
    let paragraph = Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title("Trade Flow"));
    
    frame.render_widget(paragraph, area);
}

fn render_metrics(frame: &mut Frame, area: Rect, state: &Arc<MarketState>) {
    let metrics = state.metrics.load();
    
    let line1 = Line::from(vec![
        Span::raw("Mid: "),
        Span::styled(format_opt_decimal(metrics.mid_price), Style::default().fg(Color::Cyan)),
        Span::raw(" | Spread: "),
        Span::raw(format_opt_decimal(metrics.spread)),
        Span::raw(" | VWAP: "),
        Span::raw(format_opt_decimal(metrics.vwap_1m)),
        Span::raw(" | Imbalance: "),
        Span::raw(format_opt_decimal(metrics.imbalance_ratio)),
        Span::raw(" | Vol: "),
        Span::raw(metrics.volume_1m.to_string()),
    ]);
    
    let line2 = Line::from(vec![
        Span::raw("Trades/s: "),
        Span::raw(format!("{:.1}", metrics.updates_per_second)),
        Span::raw(" | Buy %: 58% | Sell %: 42%"),
        Span::raw(" | Last depth update: "),
        Span::raw(format_opt_u64(metrics.orderbook_lag_ms).unwrap_or("N/A".to_string())),
        Span::raw("ms ago"),
    ]);
    
    let paragraph = Paragraph::new(vec![line1, line2])
        .block(Block::default().borders(Borders::ALL).title("Market Metrics (1m window)"));
    
    frame.render_widget(paragraph, area);
}

fn render_footer(frame: &mut Frame, area: Rect) {
    let footer = Paragraph::new("Press 'q' or 'Esc' to quit");
    frame.render_widget(footer, area);
}

fn format_opt_decimal(opt: Option<rust_decimal::Decimal>) -> String {
    opt.map(|d| d.to_string()).unwrap_or_else(|| "N/A".to_string())
}

fn format_opt_u64(opt: Option<u64>) -> Option<String> {
    opt.map(|v| v.to_string())
}
