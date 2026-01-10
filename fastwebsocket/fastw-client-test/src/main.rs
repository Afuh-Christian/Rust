use crate::websocket_client::connect;
use fastwebsockets::{FragmentCollector, Frame, OpCode, Payload};

mod websocket_client;
use bytes::BytesMut; 

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // let mut ws = connect().await?;

    let wsd = connect().await?;
let mut ws = FragmentCollector::new(wsd);
    println!("✅ Connected to Hyperliquid");

let sub = serde_json::json!({
    "method": "subscribe",
    "subscription": {
        "type": "trades",
        "coin": "BTC"
    }
});

//  let frame = Frame::new(payload, OpCode::Text, None, true);

//  let data = "Hello, world!" ;//.as_bytes();
 let payload = Payload::Bytes(BytesMut::from(sub.to_string().as_bytes()));


    // Create a new text frame. The 'true' indicates this is the final frame of the message.
    let frame = Frame::new(true, OpCode::Binary, None, payload); // Masking is handled internally for client-to-server

    // Send the frame over the WebSocket connection
    ws.write_frame(frame).await?;
    // or use the sink method
    // ws.send(frame).await?;


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
