mod result;
mod db;
mod routes;

use crate::result::Result;
use routes::create_routes;

#[tokio::main]
async fn main() -> Result<()>{
    let app = create_routes();
    let uri = std::env::var("SCYLLA_URI").unwrap_or_else(|_| "127.0.0.1:9042".to_string());
    //Add ? to check error
    let session = db::create_session(&uri).await?;
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
    .serve(app.into_make_service())
    .await
    .unwrap();
    Ok(())
}
