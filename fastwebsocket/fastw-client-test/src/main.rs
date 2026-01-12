
use crate::{arb_engine::arb_engine, connect_binance::run_binance, connect_hyperliquid::run_hyperliquid, types_enums::Prices};



mod connect_hyperliquid;
mod connect_binance;
mod arb_engine;
mod types_enums;



use std::sync::Arc;
use tokio::sync::Mutex;

pub type SharedPrices = Arc<Mutex<Prices>>;



#[derive(Debug)]
struct Inventory {
    binance_usdt: f64,
    binance_btc: f64,
    hyperliquid_margin: f64,
}

pub type SharedInventory = Arc<Mutex<Inventory>>;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // set coins here ..... / 
    let coins = vec!["BTC", "SOL", "ETH"]; 
    let pairs: Vec<String> = coins.iter().map(|c|format!("{}usdt", c.to_lowercase())).collect();
    // ...... 

    let prices = Arc::new(Mutex::new(Prices::default()));
    let inventory = Arc::new(Mutex::new(Inventory {
        binance_usdt: 1000.0,
        binance_btc: 0.0,
        hyperliquid_margin: 1000.0,
    }));

    tokio::spawn(run_hyperliquid(prices.clone() , coins));
    tokio::spawn(run_binance(prices.clone() , pairs));
    tokio::spawn(arb_engine(prices.clone(), inventory.clone()));

    tokio::signal::ctrl_c().await?;
    Ok(())
}
