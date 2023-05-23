mod reg;

use std::sync::Arc;

use axum::{Router, routing::post};
use reg::reg;
use scylla::Session;

/// Function creates all needed routes.
/// 
/// Function creates all neeeded endpoints with assigned handlers.
pub fn create_routes(session: Arc<Session>) -> Router{
    Router::new().route("/reg", post(reg))
    .with_state(session.clone())
}