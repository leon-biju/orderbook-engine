use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub initial_starting_capacity: usize,
    pub max_reconnect_attempts: u32,
    pub initial_backoff_ms: u64,
    pub max_backoff_ms: u64,
    pub imbalance_depth_levels: usize,
    pub initial_snapshot_depth: u16,

    pub significant_trade_volume_pct: f64,
    pub significant_trades_display_count: usize,
    pub significant_trades_retention_secs: u64,

    //pub symbols: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            initial_starting_capacity: 1000,
            max_reconnect_attempts: 10,
            initial_backoff_ms: 100,
            max_backoff_ms: 30000,
            imbalance_depth_levels: 10,
            initial_snapshot_depth: 1000,
            significant_trade_volume_pct: 0.05,
            significant_trades_display_count: 20,
            significant_trades_retention_secs: 120,
            //symbols: vec!["BTCUSDT".to_string()],
        }
    }
}

pub fn load_config() -> Config {
    fs::read_to_string("config.toml")
        .ok()
        .and_then(|content| toml::from_str(&content).ok())
        .unwrap_or_default()
}