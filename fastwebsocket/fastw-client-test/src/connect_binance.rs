use fastwebsockets::{handshake, WebSocket};
use hyper::{
    Request,
    body::Bytes,
    header::{CONNECTION, HOST, UPGRADE},
    upgrade::Upgraded,
};
use http_body_util::Empty;
use rustls::{ClientConfig, RootCertStore, pki_types::ServerName};
use tokio::net::TcpStream;
use tokio_rustls::TlsConnector;
use hyper_util::rt::tokio::TokioIo;
use std::{future::Future, sync::Arc};
use anyhow::Result;

pub async fn connect_binance(
    symbol: &str,
) -> Result<WebSocket<TokioIo<Upgraded>>> {
    let host = "stream.binance.com";
    let port = 9443;

    // 1️⃣ TCP
    let stream = TcpStream::connect((host, port)).await?;

    // 2️⃣ TLS (rustls-only)
    let mut root_store = RootCertStore::empty();
    root_store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());

    let config = ClientConfig::builder()
        .with_root_certificates(root_store)
        .with_no_client_auth();

    let connector = TlsConnector::from(Arc::new(config));
    let domain = ServerName::try_from(host)?;
    let tls_stream = connector.connect(domain, stream).await?;

    // 3️⃣ WebSocket handshake (NO subscription frame)
    let uri = format!(
        "wss://{host}:9443/ws/{}@trade",
        symbol.to_lowercase()
    );

    let req = Request::builder()
        .method("GET")
        .uri(uri)
        .header(HOST, host)
        .header(UPGRADE, "websocket")
        .header(CONNECTION, "upgrade")
        .header("Sec-WebSocket-Key", handshake::generate_key())
        .header("Sec-WebSocket-Version", "13")
        .body(Empty::<Bytes>::new())?;

    let (ws, _) = handshake::client(&SpawnExecutor, req, tls_stream).await?;
    Ok(ws)
}

// Tokio executor
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
