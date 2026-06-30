use std::{net::SocketAddr, path::PathBuf, sync::Arc};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
};
use tracing::info;

#[derive(Debug)]
struct HttpServeState {
    dir: PathBuf,
}

pub async fn process_http_serve(dir: PathBuf, port: u16) -> anyhow::Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Serving {:?} on port {}", dir, addr);

    let state = HttpServeState { dir };
    // axum router
    let router = axum::Router::new()
        .route("/{*path}", get(file_handler))
        .with_state(Arc::new(state));

    let listener = tokio::net::TcpListener::bind(addr).await?;

    axum::serve(listener, router).await?;
    Ok(())
}

async fn file_handler(
    State(state): State<Arc<HttpServeState>>,
    Path(path): Path<String>,
) -> (StatusCode, String) {
    let p = std::path::Path::new(&state.dir).join(path);
    if !p.exists() {
        (StatusCode::NOT_FOUND, "File not found".to_string())
    } else {
        let content = tokio::fs::read_to_string(p).await.expect("读取文件失败");
        (StatusCode::OK, content)
    }
}
