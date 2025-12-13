use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;
use orderbook_engine::{book::orderbook::OrderBook, book::scaler::Scaler, binance::types::{DepthSnapshot, DepthUpdate}};
use rust_decimal::Decimal;
use std::str::FromStr;

const SNAPSHOT_LEVELS: usize = 1000;
const UPDATE_LEVELS: usize = 10;

fn bench_from_snapshot(c: &mut Criterion) {
    // simulate a snapshot with 10k bids and 10k asks
    let snapshot = DepthSnapshot::fake_snapshot(SNAPSHOT_LEVELS);
    let scaler = Scaler::new(
        Decimal::from_str("0.01").unwrap(),
        Decimal::from_str("0.01").unwrap()
    );

    c.bench_function("from_snapshot_10k", |b| {
        b.iter(|| {
            let _book = black_box(OrderBook::from_snapshot(black_box(snapshot.clone()), &scaler));
        })
    });
}

fn bench_apply_updates(c: &mut Criterion) {
    let snapshot = DepthSnapshot::fake_snapshot(SNAPSHOT_LEVELS);
    let scaler = Scaler::new(
        Decimal::from_str("0.01").unwrap(),
        Decimal::from_str("0.01").unwrap()
    );
    let mut book = OrderBook::from_snapshot(snapshot, &scaler);

    let updates: Vec<DepthUpdate> = (0..UPDATE_LEVELS)
        .map(|i| DepthUpdate::fake_update(i as u64, UPDATE_LEVELS))
        .collect();

    c.bench_function("apply_10k_updates", |b| {
        b.iter(|| {
            for up in &updates {
                book.apply_update(black_box(up), &scaler);
            }
        })
    });
}

fn bench_query_functions(c: &mut Criterion) {
    let snapshot = DepthSnapshot::fake_snapshot(SNAPSHOT_LEVELS);
    let scaler = Scaler::new(
        Decimal::from_str("0.01").unwrap(),
        Decimal::from_str("0.01").unwrap()
    );
    let book = OrderBook::from_snapshot(snapshot, &scaler);

    c.bench_function("query_best_spread_mid", |b| {
        b.iter(|| {
            let _bid = black_box(book.best_bid());
            let _ask = black_box(book.best_ask());
            let _spread = black_box(book.spread());
            let _mid = black_box(book.mid_price());
        })
    });
}

criterion_group!(benches, bench_from_snapshot, bench_apply_updates, bench_query_functions);
criterion_main!(benches);
