// // use hypersdk::hypercore;

// // #[tokio::main]
// // async fn main() -> anyhow::Result<()> {
// //     // Create a mainnet client
// //     let client = hypercore::mainnet();

// //     // Get perpetual markets
// //     let perps = client.perps().await?;
// //     for market in perps {
// //         println!("{}: {}x leverage", market.name, market.max_leverage);
// //     }

// //     // Get spot markets
// //     let spots = client.spot().await?;
// //     for market in spots {
// //         println!("{}", market.symbol());
// //     }

// //     Ok(())
// // }

// use hypersdk::hypercore::{self, types::*, PrivateKeySigner};
// use rust_decimal::{dec, Decimal};

// #[tokio::main]
// async fn main() -> anyhow::Result<()> {
//     let client = hypercore::mainnet();
//     // You can also use existing Foundry keystores!!
//     // let signer = LocalSigner::decrypt_keystore("/home/user/.foundry/keystores/my_user", "123")?;
//     let signer: PrivateKeySigner = "your_private_key".parse()?;

//     let order = BatchOrder {
//         orders: vec![OrderRequest {
//             asset: 0, // BTC
//             is_buy: true,
//             limit_px: dec!(50000),
//             sz: dec!(0.1),
//             reduce_only: false,
//             order_type: OrderTypePlacement::Limit {
//                 tif: TimeInForce::Gtc,
//             },
//             cloid: Default::default(),
//         }],
//         grouping: OrderGrouping::Na,
//     };

//     let nonce = chrono::Utc::now().timestamp_millis() as u64;
//     let result = client.place(&signer, order, nonce, None, None).await?;

//     println!("Order placed: {:?}", result);
//     Ok(())
// }



use hypersdk::hypercore::{self, types::*};
use futures::StreamExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut ws = hypercore::mainnet_ws();

    // Subscribe to market data
    ws.subscribe(Subscription::Trades { coin: "BTC".into() });
    ws.subscribe(Subscription::L2Book { coin: "ETH".into() });

    // Process incoming messages
    while let Some(msg) = ws.next().await {
        match msg {
            Incoming::Trades(trades) => {
                for trade in trades {
                    println!("{} @ {} size {}", trade.side, trade.px, trade.sz);
                }
            }
            Incoming::L2Book(book) => {
                println!("Order book update for {}", book.coin);
            }
            _ => {}
        }
    }

    Ok(())
}