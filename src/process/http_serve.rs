use std::net::SocketAddr;
use std::path::Path;

use axum::{routing::get, Router};

pub async fn process_http_serve(path: &Path, port: u16) -> anyhow::Result<()> {
    tracing::info!("serving: {:?} on port: {}", path, port);

    let router = Router::new().route("/", get(index_handler));
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;
    anyhow::Ok(())
}

async fn index_handler() -> &'static str {
    "Hello, word!"
}
