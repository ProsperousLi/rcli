use anyhow::Result;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Router,
};
use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use tower_http::services::ServeDir;
use tracing::{info, warn};

#[derive(Debug)]
pub struct HttpServerState {
    path: PathBuf,
}
pub async fn process_http_server(path: PathBuf, port: u16) -> Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Starting HTTP Server at {:?}:{}", path, port);
    let state = HttpServerState { path: path.clone() };
    // axum route
    let route = Router::new()
        .nest_service("/tower", ServeDir::new(path))
        .route("/*path", get(file_handler))
        .with_state(Arc::new(state));

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, route).await?;
    Ok(())
}

async fn file_handler(
    State(state): State<Arc<HttpServerState>>,
    Path(path): Path<String>,
) -> (StatusCode, String) {
    let p = std::path::Path::new(&state.path).join(path);
    info!("{:?}", p);
    if !p.exists() {
        (
            StatusCode::NOT_FOUND,
            format!("{:?} not found", p.display()),
        )
    } else {
        match tokio::fs::read_to_string(p).await {
            Ok(content) => {
                info!("read file {:?} success", content.len());
                (StatusCode::OK, content)
            }
            Err(e) => {
                warn!("eroor reading file :{:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_file_handler() {
        let state = Arc::new(HttpServerState {
            path: PathBuf::from("."),
        });

        let (status, content) = file_handler(State(state), Path("Cargo.toml".to_string())).await;

        assert_eq!(status, StatusCode::OK);
        assert!(content.trim().starts_with("[package]"));
    }
}
