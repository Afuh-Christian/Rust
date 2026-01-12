use crate::{SharedInventory, SharedPrices};

 pub async fn arb_engine(
    prices: SharedPrices,
    inventory: SharedInventory,
) {
    const MIN_EDGE: f64 = 40.0;
    const MIN_USDT: f64 = 200.0;

    println!("🚀 Arbitrage engine started\n");
   
    loop {
        {
           
            let p = prices.lock().await;

      
                      if let (Some(hl), Some(bin)) = (p.hyperliquid, p.binance) {
    println!("🟣 Hyperliquid BTC price: {:.2}", hl);
    println!("🟡 Binance BTCUSDT price: {:.2}", bin);
}

            let (Some(bin), Some(hl)) = (p.binance, p.hyperliquid) else {
                continue;
            };

            let spread = bin - hl;

            if spread > MIN_EDGE {
                // println!("🚀 Arbitrage engine running----");
                let inv = inventory.lock().await;

                if inv.binance_usdt < MIN_USDT {
                    println!("⛔ Cannot trade: low USDT on Binance");
                    continue;
                }

                println!(
                    "⚡ CAN TRADE | Buy HL @ {:.2} | Sell Binance @ {:.2} | Spread {:.2}",
                    hl, bin, spread
                );

                // 👉 THIS is where you place REST orders
            }
            // else {
            //     println!("❌ No arbitrage opportunity | Spread {:.2}", spread);
            // }
        }

        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    }
}
