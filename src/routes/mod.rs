mod reg;

use axum::{Router, routing::get, body::Body};
use reg::reg;

/// Function creates all needed routes.
/// 
/// Function creates all neeeded endpoints with assigned handlers.
pub fn create_routes() -> Router<(), Body> {
    Router::new().route("/reg", get(reg))

}