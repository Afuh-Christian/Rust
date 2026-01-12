
use crate::{arb_engine::arb_engine, connect_binance::run_binance, connect_hyperliquid::run_hyperliquid};


mod connect_hyperliquid;
mod connect_binance;
mod arb_engine;


#[derive(Default, Debug)]
struct Prices {
    binance: Option<f64>,
    hyperliquid: Option<f64>,
}

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



// #[tokio::main]
// async fn main() -> anyhow::Result<()> {

//     let (hl_res, binance_res) = tokio::join!(
//         run_hyperliquid(),
//         run_binance(),
//     );

//     print!("Done");

//     // Handle errors explicitly
//     hl_res?;
//     binance_res?;

//     //  run_hyperliquid().await?;

//     Ok(())
// }




#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let prices = Arc::new(Mutex::new(Prices::default()));
    let inventory = Arc::new(Mutex::new(Inventory {
        binance_usdt: 1000.0,
        binance_btc: 0.0,
        hyperliquid_margin: 1000.0,
    }));

    tokio::spawn(run_hyperliquid(prices.clone()));
    tokio::spawn(run_binance(prices.clone()));
    tokio::spawn(arb_engine(prices.clone(), inventory.clone()));

    tokio::signal::ctrl_c().await?;
    Ok(())
}
