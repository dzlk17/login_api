mod reg;
mod log;

use std::sync::Arc;
use axum::{Router, routing::post, http::Method};
use reg::reg;
use log::log;
use scylla::Session;
use tower_http::cors::{CorsLayer, Any};

/// Function creates all needed routes.
/// 
/// Function creates all neeeded endpoints with assigned handlers.
/// Used CorsLayer to increase security level.
pub fn create_routes(session: Arc<Session>) -> Router{
    let cors = CorsLayer::new()
    .allow_methods(Method::POST)
    .allow_origin(Any);
    Router::new().route("/reg", post(reg))
    .route("/log", post(log))
    .with_state(session.clone())
    .layer(cors)
}