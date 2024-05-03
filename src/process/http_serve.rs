use std::{net::SocketAddr, path::PathBuf};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Router,
};
use std::sync::Arc;
use tower_http::services::ServeDir;

#[derive(Debug)]
struct HttpServeState {
    path: PathBuf,
}

pub async fn process_http_serve(path: PathBuf, port: u16) -> anyhow::Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("serving: {:?} on port: {}", path, addr);

    let state = HttpServeState { path: path.clone() };
    let dir_service = ServeDir::new(path);

    let router = Router::new()
        .route("/*path", get(file_handler))
        .nest_service("/tower", dir_service)
        .with_state(Arc::new(state));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;
    anyhow::Ok(())
}

async fn file_handler(
    State(state): State<Arc<HttpServeState>>,
    Path(p): Path<String>,
) -> (StatusCode, String) {
    let full_path = std::path::Path::new(&state.path).join(p.clone());

    tracing::info!("reading: {:?}", full_path);
    if !full_path.exists() {
        return (
            StatusCode::NOT_FOUND,
            format!("File: {:?} not found", full_path.display()),
        );
    }

    match tokio::fs::read_to_string(full_path).await {
        Ok(content) => {
            tracing::info!("Read {} bytes", content.len());
            (StatusCode::OK, content)
        }
        Err(e) => {
            tracing::warn!("Error reading file: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_file_handler() {
        let state = Arc::new(HttpServeState {
            path: PathBuf::from("."),
        });
        let (status, content) = file_handler(State(state), Path("Cargo.toml".to_string())).await;
        assert_eq!(status, StatusCode::OK);
        assert!(content.contains("forfd8960"));
    }
}
