mod models;
mod link_scrp;

use crate::link_scrp::qr_png;
use crate::link_scrp::create_link;
use anyhow::{anyhow, Result};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};

use std::{env, net::SocketAddr};
use tower_http::{cors::{Any, CorsLayer}, trace::TraceLayer};
use tracing::error;
use tokio::net::TcpListener;



#[tokio::main]
async fn main() -> Result<()> {
    // Logging
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let app = Router::new()
        .route("/healthz", get(healthz))
        .route("/api/link", post(create_link))
        .route("/qr", get(qr_png))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .layer(TraceLayer::new_for_http());

    let port: u16 = env::var("PORT").ok().and_then(|s| s.parse().ok()).unwrap_or(8080);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = TcpListener::bind(addr).await?;
    tracing::info!("listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await?;    
    Ok(())
}

async fn healthz() -> &'static str { "ok" }




// Basic error wrapper
struct AppError(anyhow::Error, StatusCode);
impl AppError {
    fn bad_request(msg: &str) -> Self { Self(anyhow!(msg.to_string()), StatusCode::BAD_REQUEST) }
    fn internal(msg: &str) -> Self { Self(anyhow!(msg.to_string()), StatusCode::INTERNAL_SERVER_ERROR) }
}
impl<E: Into<anyhow::Error>> From<E> for AppError {
    fn from(e: E) -> Self { Self(e.into(), StatusCode::INTERNAL_SERVER_ERROR) }
}
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (err, code) = (self.0, self.1);
        error!(%err, code = %code, "request failed");
        let body = serde_json::json!({ "error": err.to_string() });
        (code, Json(body)).into_response()
    }
}
