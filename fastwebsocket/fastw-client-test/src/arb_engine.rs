use crate::{SharedInventory, SharedPrices};

pub async fn arb_engine(
    prices: SharedPrices,
    inventory: SharedInventory,
) {
    // ---- CONFIG ----
    const MIN_EDGE_USD: f64 = 40.0;      // absolute floor
    const MIN_EDGE_PCT: f64 = 0.0015;    // 0.15%
    const MIN_USDT: f64 = 200.0;

    println!("🚀 Arbitrage engine started\n");

    loop {
        // ── 1️⃣ Read latest prices (short lock) ──
        let (bin, hl) = {
            let p = prices.lock().await;

            match (p.binance, p.hyperliquid) {
                (Some(bin), Some(hl)) => (bin, hl),
                _ => {
                    tokio::time::sleep(std::time::Duration::from_millis(50)).await;
                    continue;
                }
            }
        };

        // println!(" 🟡 Binance => {}" , bin);
        // println!(" 🟣 Hyper Liquid => {}" , hl);

        // ── 2️⃣ Compute spread + percentage edge ──
        let spread_usd = bin - hl;
        let edge_pct = spread_usd / bin; // percentage edge

        // println!("------------------");
        // println!(" % Edge => {}" , edge_pct);
        //  println!("------------------");

        // ── 3️⃣ Safety checks (THIS is the key change) ──
        if spread_usd > MIN_EDGE_USD && edge_pct > MIN_EDGE_PCT {
            let inv = inventory.lock().await;

            if inv.binance_usdt < MIN_USDT {
                println!("⛔ Cannot trade: low USDT on Binance");
            } else {
                println!(" % Edge => {}" , edge_pct);
                println!(
                    "⚡ CAN TRADE | Buy HL @ {:.2} | Sell Binance @ {:.2} | Spread ${:.2} | Edge {:.3}%",
                    hl,
                    bin,
                    spread_usd,
                    edge_pct * 100.0
                );

                // 👉 PLACE REST ORDERS HERE
            }
        }

        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    }
}
