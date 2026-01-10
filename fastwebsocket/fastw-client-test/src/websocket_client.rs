use fastwebsockets::{handshake, WebSocket};
use hyper::{
    Request,
    body::Bytes,
    header::{CONNECTION, HOST, UPGRADE}, upgrade::Upgraded,
};
use http_body_util::Empty;
use tokio::net::TcpStream;
use std::future::Future;
use anyhow::Result;

pub async fn connect() -> Result<WebSocket<hyper_util::rt::tokio::TokioIo<Upgraded>>> {
    let stream = TcpStream::connect("127.0.0.1:3000").await?;

    let req = Request::builder()
        .method("GET")
        .uri("ws://localhost:3000/")
        .header(HOST, "localhost:3000")
        .header(UPGRADE, "websocket")
        .header(CONNECTION, "upgrade")
        .header(
            "Sec-WebSocket-Key",
            handshake::generate_key(),
        )
        .header("Sec-WebSocket-Version", "13")
        .body(Empty::<Bytes>::new())?;

    let (ws, _) = handshake::client(&SpawnExecutor, req, stream).await?;
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
