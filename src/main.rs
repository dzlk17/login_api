mod result;
mod db;
mod routes;
mod auth;
mod error;

use std::sync::Arc;
use crate::result::Result;
use db::create_session;
use routes::create_routes;

#[tokio::main]
async fn main() -> Result<()>{
    let session = create_session().await?;
    let state = Arc::new(session);
    let app = create_routes(state);    
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
    .serve(app.into_make_service())
    .await
    .expect("Failed to start server");
    Ok(())
}
