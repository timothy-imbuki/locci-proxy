use axum::{routing::get, Router};
use std::net::SocketAddr;
use tracing::info;
mod handlers;
pub async fn run(addr: SocketAddr, api_key: String, config: crate::config::UnifiedConfig) {
    let app = Router::new()
        .route("/api/v1/status", get(handlers::status))
        .route("/api/v1/config", get(handlers::config))
        .route("/api/v1/metrics", get(handlers::metrics))
        .with_state((api_key, config));
    info!("Control API listening on {}", addr);
    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}
