use fastwebsockets::{FragmentCollector, Frame, OpCode, Payload, WebSocket, handshake};
use hyper::{
    Request,
    body::Bytes,
    header::{CONNECTION, HOST, UPGRADE}, upgrade::Upgraded,
};
use http_body_util::Empty;
use rustls::{ClientConfig, RootCertStore, pki_types::ServerName};
use tokio::net::TcpStream;
use tokio_rustls::TlsConnector;
use std::{future::Future, sync::Arc};
use anyhow::Result;

use crate::{SharedPrices, types_enums::{Coin, HlTradesMsg}};




pub async fn connect_hyperliquid() -> Result<WebSocket<hyper_util::rt::tokio::TokioIo<Upgraded>>> {
   
   let address: &'static str = "api.hyperliquid.xyz";
   let port: u16 = 443;
   
      // 1️⃣ TCP
    let stream = TcpStream::connect(format!("{}:{}", address, port)).await?;

    // 1️⃣ Root certificates (NO OpenSSL)
    let mut root_store = RootCertStore::empty();
    root_store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());

    // 2️⃣ TLS config
    let config = ClientConfig::builder()
        .with_root_certificates(root_store)
        .with_no_client_auth();

    let connector = TlsConnector::from(Arc::new(config));

    // 3️⃣ DNS name (required for TLS)
    let domain = ServerName::try_from(address)?;

    // 4️⃣ TLS handshake
    let tls_stream = connector.connect(domain, stream).await?;



    // 5️⃣ WebSocket handshake
    let req = Request::builder()
        .method("GET")
        .uri(format!("wss://{}/ws",address))
        .header(HOST, address)
        .header(UPGRADE, "websocket")
        .header(CONNECTION, "upgrade")
        .header("Sec-WebSocket-Key", handshake::generate_key())
        .header("Sec-WebSocket-Version", "13")
        .body(Empty::<Bytes>::new())?;

    let (ws, _) = handshake::client(&SpawnExecutor, req, tls_stream).await?;
    Ok(ws)

}

// Tie hyper executor to tokio runtime
struct SpawnExecutor;

impl<Fut> hyper::rt::Executor<Fut> for SpawnExecutor
where
    Fut: Future + Send + 'static,
    Fut::Output: Send + 'static,
{
    fn execute(&self, fut: Fut) {
        tokio::spawn(fut);
    }
}


pub async fn run_hyperliquid(prices: SharedPrices , coins:Vec<&str>) -> anyhow::Result<()> {
    let ws = connect_hyperliquid().await?;
    let mut ws = FragmentCollector::new(ws);

println!("🟣 Hyperliquid connected");

for coin in coins {
 
    let sub = serde_json::json!({
    "method": "subscribe",
    "subscription": {
        "type": "trades",
        "coin": coin
    }
});

let json = sub.to_string();

let frame = Frame::text(
    Payload::Bytes(json.as_str().into())
);

ws.write_frame(frame).await?;

}

    loop {
        let frame = ws.read_frame().await?;
        if frame.opcode == OpCode::Text {
    let text = String::from_utf8_lossy(&frame.payload);

    if let Ok(msg) = serde_json::from_str::<HlTradesMsg>(&text) {
        if let Some(trade) = msg.data.last() {
            if let Some(coin) = Coin::from_hyperliquid(&trade.coin) {
                if let Ok(price) = trade.px.parse::<f64>() {
                    let mut p = prices.lock().await;
                    p.hyperliquid.insert(coin, price);
                }
            }
        }
    }
}

    }
}