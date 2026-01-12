
use crate::{connect_binance::run_binance, connect_hyperliquid::run_hyperliquid};


mod connect_hyperliquid;
mod connect_binance;

#[tokio::main]
async fn main() -> anyhow::Result<()> {





    let (hl_res, binance_res) = tokio::join!(
        run_hyperliquid(),
        run_binance(),
    );

    // Handle errors explicitly
    hl_res?;
    binance_res?;

    //  run_hyperliquid().await?;

    Ok(())
}



