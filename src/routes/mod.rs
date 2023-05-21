mod reg;

use axum::{Router, routing::get, body::Body};
use reg::reg;

pub fn create_routes() -> Router<(), Body> {
    Router::new().route("/reg", get(reg))

}