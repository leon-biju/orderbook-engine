mod binance;
mod snapshot;
mod stream;

use anyhow::Result;


#[tokio::main]
async fn main() -> Result<()>{
    //install default tls provider for aws-lc-rs
    let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();
    
    let symbol = "BTCUSDT";


    println!("Connecting to WebSocket...");
    let mut ws_stream = stream::connect_depth_stream(symbol).await?;
    
    println!("Fetching snapshot...");
    let snapshot = snapshot::fetch_snapshot(symbol, 1).await?;
    println!("Snapshot lastUpdateId: {}", snapshot.last_update_id);
    
    
    
    
    
    Ok(())
}
