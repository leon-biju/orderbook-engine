use serde::{Serialize, Deserialize};
use std::fs;
use anyhow::Result;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub orderbook_initial_snapshot_depth: u16,
    pub orderbook_imbalance_depth_levels: usize,

    pub recent_trades_starting_capacity: usize,
    pub significant_trades_retention_secs: u64,
    pub significant_trade_volume_pct: f64,
    pub min_trades_for_significance: usize,

    pub max_reconnect_attempts: u32,
    pub initial_backoff_ms: u64,
    pub max_backoff_ms: u64,
    pub message_timeout_ms: u64,

    pub orderbook_depth_display_count: usize,
    pub recent_trades_display_count: usize,
    pub significant_trades_display_count: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            orderbook_initial_snapshot_depth: 1000,
            orderbook_imbalance_depth_levels: 10,

            recent_trades_starting_capacity: 1000,
            significant_trades_retention_secs: 120,

            significant_trade_volume_pct: 0.05,
            min_trades_for_significance: 50,

            max_reconnect_attempts: 10,
            initial_backoff_ms: 100,
            max_backoff_ms: 30000,
            message_timeout_ms: 30000,

            orderbook_depth_display_count: 5,
            recent_trades_display_count: 10,
            significant_trades_display_count: 20,
        }
    }
}

pub fn load_config() -> Result<Config> {
    match load_existing_config() {
        Ok(conf) => {
            tracing::info!("Loaded config.toml");
            Ok(conf)
        }
        Err(_) => {
            tracing::info!("Config file not found, creating config.toml with default settings");
            create_default_config()
        }
        
    }
}

fn load_existing_config() -> Result<Config> {
    let content = fs::read_to_string("config.toml")?;
    Ok(toml::from_str(&content)?)
}

fn create_default_config() -> Result<Config> {

    let config = Config::default();
    let toml_string = toml::to_string_pretty(&config)?;
    fs::write("config.toml", toml_string)?;
    Ok(config)
}
