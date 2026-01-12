use crate::websocket_client::connect;
use fastwebsockets::{FragmentCollector, Frame, OpCode, Payload};

mod websocket_client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // let mut ws = connect().await?;

    let wsd = connect("api.hyperliquid.xyz", 443 ).await?;

let mut ws = FragmentCollector::new(wsd);
    println!("✅ Connected to Hyperliquid");

let sub = serde_json::json!({
    "method": "subscribe",
    "subscription": {
        "type": "trades",
        "coin": "BTC"
    }
});

 
// let frame = Frame::text(
//     Payload::Bytes(sub.to_string().into_bytes().into())
// );


let json = sub.to_string();

let frame = Frame::text(
    Payload::Bytes(json.as_str().into())
);

// ws.write_frame(frame).await?;

ws.write_frame(frame).await?;

    loop {
        let frame = ws.read_frame().await?;

        match frame.opcode {
            OpCode::Text => {
                let text = String::from_utf8_lossy(&frame.payload);
                println!("📩 Text: {}", text);
            }

            OpCode::Binary => {
                println!("📦 Binary: {:?}", String::from_utf8_lossy(&frame.payload));
            }

            OpCode::Close => {
                println!("❌ Connection closed by server");
                break;
            }

            OpCode::Ping => {
                // optional: respond to ping
                // ws.write_frame(frame).await?;
            }

            OpCode::Pong => {
                // usually ignore
            }

            _ => {}
        }
    }

    Ok(())
}
