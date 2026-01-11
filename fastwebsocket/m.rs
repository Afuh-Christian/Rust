use fastwebsockets::{handshake, WebSocket};
use hyper::{
    Request,
    body::Bytes,
    header::{CONNECTION, HOST, UPGRADE}, upgrade::Upgraded,
};
use http_body_util::Empty;
use rustls::RootCertStore;
use tokio::net::TcpStream;
use std::future::Future;
use anyhow::Result;

pub async fn connect() -> Result<WebSocket<hyper_util::rt::tokio::TokioIo<Upgraded>>> {
    // let stream = TcpStream::connect("127.0.0.1:3000").await?;

      // 1️⃣ TCP
    let stream = TcpStream::connect("api.hyperliquid.xyz:443").await?;




    // 2️⃣ Root certificates
    let mut root_store = RootCertStore::empty();
    root_store.add_server_trust_anchors(webpki_roots::TLS_SERVER_ROOTS.0.iter().cloned());
    // 3️⃣ TLS config
    let config = rustls::ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(root_store)
        .with_no_client_auth();

    // 4️⃣ TLS connector
    let connector = tokio_rustls::TlsConnector::from(std::sync::Arc::new(config));

    // 5️⃣ TLS stream
    let domain = rustls::ServerName::try_from("api.hyperliquid.xyz")
        .map_err(|_| anyhow::anyhow!("Invalid DNS name"))?;

    let stream = connector.connect(domain, stream).await?;

        // 4️⃣ TLS handshake
    let tls_stream = connector.connect(domain, stream).await?;

    // 5️⃣ WebSocket handshake
    let req = Request::builder()
        .method("GET")
        .uri("wss://api.hyperliquid.xyz/ws")
        .header(HOST, "api.hyperliquid.xyz")
        .header(UPGRADE, "websocket")
        .header(CONNECTION, "upgrade")
        .header("Sec-WebSocket-Key", handshake::generate_key())
        .header("Sec-WebSocket-Version", "13")
        .body(Empty::<Bytes>::new())?;

    let (ws, _) = handshake::client(&SpawnExecutor, req, tls_stream).await?;
    Ok(ws)


//     let mut root_store = rustls::RootCertStore::empty();
// root_store.add_server_trust_anchors(
// webpki_roots::TLS_SERVER_ROOTS
// .0
// .iter()
// .map(|ta| {
// rustls::OwnedTrustAnchor::from_subject_spki_name_constraints(
// ta.subject,
// ta.spki,
// ta.name_constraints,
// )
// })
// );

    // let req = Request::builder()
    //     .method("GET")
    //     .uri("ws://localhost:3000/")
    //     .header(HOST, "localhost:3000")
    //     .header(UPGRADE, "websocket")
    //     .header(CONNECTION, "upgrade")
    //     .header(
    //         "Sec-WebSocket-Key",
    //         handshake::generate_key(),
    //     )
    //     .header("Sec-WebSocket-Version", "13")
    //     .body(Empty::<Bytes>::new())?;

    // let (ws, _) = handshake::client(&SpawnExecutor, req, stream).await?;
    // Ok(ws)
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
