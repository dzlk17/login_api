use dotenvy_macro::dotenv;
use scylla::{Session, SessionBuilder};
use crate::Result;

/// Creates new database session.
/// 
/// Function takes data from .env file and creates connection with scylladb using it. 
/// Returns Error if connection to database fails.
pub async fn create_session() -> Result<Session>{
    dotenvy::dotenv()?;
    SessionBuilder::new()
        .known_node(dotenv!("SCYLLA_URI"))
        .user(dotenv!("DB_NAME"), dotenv!("DB_PASSWORD"))
        .build()
        .await
        .map_err(From::from)
}
