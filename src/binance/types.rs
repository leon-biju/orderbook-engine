use rand::Rng;
use rust_decimal::Decimal;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DepthSnapshot {
    #[serde(rename = "lastUpdateId")]
    pub last_update_id: u64,
    pub bids: Vec<[String; 2]>, // [price, qty]
    pub asks: Vec<[String; 2]>,
}

#[derive(Debug, Deserialize)]
pub struct DepthUpdate {
    #[serde(rename = "E")]
    pub event_time: u64,
    pub s: String, // symbol
    #[serde(rename = "U")]
    pub first_update_id: u64,
    #[serde(rename = "u")]
    pub final_update_id: u64,
    pub b: Vec<[String; 2]>, // bids
    pub a: Vec<[String; 2]>, // asks
}

impl DepthSnapshot {
    // generate fake snapshot for testing
    pub fn fake_snapshot(n_levels: usize) -> Self {
        let mut rng = rand::rng();

        let bids = (0..n_levels)
            .map(|i| {
                let price = Decimal::from(1000 + n_levels as u64 - i as u64).to_string(); // descending
                let qty = Decimal::from(rng.random_range(1..10)).to_string();
                [price, qty]
            })
            .collect::<Vec<_>>();

        let asks = (0..n_levels)
            .map(|i| {
                let price = Decimal::from(1000 + n_levels as u64 + i as u64).to_string(); // ascending
                let qty = Decimal::from(rng.random_range(1..10)).to_string();
                [price, qty]
            })
            .collect::<Vec<_>>();

        Self {
            last_update_id: 0,
            bids,
            asks,
        }
    }
}


impl DepthUpdate {
    // generate fake update for testing
    pub fn fake_update(last_update_id: u64, n_levels: usize) -> Self {
        let mut rng = rand::rng();
        let mut bids = Vec::new();
        let mut asks = Vec::new();

        for _ in 0..n_levels {
            let price = Decimal::from(1000 + rng.random_range(0..500)).to_string();
            let qty =  Decimal::from(rng.random_range(0..10)).to_string();
            if rng.random_bool(0.5) {
                bids.push([price, qty]);
            } else {
                asks.push([price, qty]);
            }
        }

        return Self {
            event_time: 0, // dont care about this book doesn't even use it
            s: "FAKE".to_string(),
            first_update_id: last_update_id + 1,
            final_update_id: last_update_id + n_levels as u64 - 1,
            b: bids,
            a: asks
        }
    }
}