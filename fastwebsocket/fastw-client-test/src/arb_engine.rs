use crate::{SharedInventory, SharedPrices, types_enums::Coin};


fn coin_color(coin: Coin) -> &'static str {
    match coin {
        Coin::BTC => "\x1b[33m", // Yellow
        Coin::ETH => "\x1b[35m", // Magenta / Purple
        Coin::SOL => "\x1b[36m", // Cyan
    }
}

const RESET: &str = "\x1b[0m";



pub async fn arb_engine(
    prices: SharedPrices,
    inventory: SharedInventory,
) {
    // ── CONFIG ──
    const MIN_EDGE_USD: f64 = 40.0;   // absolute floor
    const MIN_EDGE_PCT: f64 = 0.0015; // 0.15%
    const MIN_USDT: f64 = 200.0;

    let coins = [Coin::BTC, Coin::ETH, Coin::SOL];

    println!("🚀 Arbitrage engine started");

    loop {
        // ── 1️⃣ Take a snapshot of prices (short lock) ──
        let snapshot = {
            let p = prices.lock().await;
            (
                p.binance.clone(),
                p.hyperliquid.clone(),
            )
        };

        let (binance_prices, hyperliquid_prices) = snapshot;

        // ── 2️⃣ Evaluate arbitrage PER COIN ──
        for &coin in &coins {
            let (Some(bin), Some(hl)) = (
                binance_prices.get(&coin),
                hyperliquid_prices.get(&coin),
            ) else {
                continue;
            };

            // print the prices here with the respective coins and pairs .. 
//    println!(
//         "📊 {:?} | Binance {}USDT: {:.2} | Hyperliquid {}: {:.2}",
//         coin,
//         format!("{:?}", coin),
//         bin,
//         format!("{:?}", coin),
//         hl
//     );

//  println!("Coin : {:?} "  , coin );


            let spread_usd = bin - hl;
            let edge_pct = spread_usd / bin;


        //     let color = coin_color(coin);

            
        //    println!(" {} Coin : {:?} -----   Edge % : {:?} "  , color ,  coin , edge_pct * 100.0);


            if //spread_usd < MIN_EDGE_USD || 
            edge_pct < MIN_EDGE_PCT {
                continue;
            }

            

            // ── 3️⃣ Inventory check ──
            let inv = inventory.lock().await;

            if inv.binance_usdt < MIN_USDT {
                println!("⛔ {:?} | insufficient USDT", coin);
                continue;
            }

            // ── 4️⃣ SIGNAL ──
           let color = coin_color(coin);

println!(
    "{}⚡ {:?} | Buy HL @ {:.2} | Sell Binance @ {:.2} | Spread ${:.2} | Edge {:.3}%{}",
    color,
    coin,
    hl,
    bin,
    spread_usd,
    edge_pct * 100.0,
    RESET
);


            // 👉 PLACE REST ORDERS HERE (coin-aware)
        }

        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    }
}
