mod result;
mod db;
mod routes;

use crate::result::Result;
use routes::create_routes;

#[tokio::main]
async fn main() -> Result<()>{
    let app = create_routes();
    let session = db::create_session().await?;
    
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
    .serve(app.into_make_service())
    .await
    .unwrap();
    Ok(())
}
