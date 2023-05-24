use std::sync::Arc;
use axum::Json;
use scylla::Session;
use axum::extract::State;
use serde_json::{json, Value};
use crate::routes::user::{hash_password, User};
use crate::error::AppError;

///Endpoint for user registration
/// 
/// ## Arguments
/// *state - function get established session - connection to database
/// *user - data passed by user during registration
/// 
/// Function check if email exists in database, and if not, creates new user in database. Returns suitable StatusCode, depends on situation.
pub async fn reg(State(state): State<Arc<Session>>, Json(user): Json<User>) -> Result<Json<Value>, AppError> {
    let new_user: Json<User> = Json(user);
    let query = format!("SELECT COUNT(*) FROM user_auth.users_list where email = '{}';", new_user.email);
    let query_result = state.query(query, ()).await.map_err(|err| { dbg!(err); AppError::InternalServerError})?;
    for row in query_result.rows().map_err(|err| { dbg!(err); AppError::InternalServerError})?{
        let val = row.into_typed::<(i64,)>().map_err(|err| { dbg!(err); AppError::InternalServerError})?;
        if val.0 > 0 {
            return Err(AppError::UserAlreadyExists);
        }
    }
    let query = format!("INSERT INTO user_auth.users_list (user_id, email, password_hash) VALUES (uuid(), '{}', '{}')", new_user.email, hash_password(&new_user.password));
    let _query_result = state.query(query, ()).await.map_err(|err| { dbg!(err); AppError::InternalServerError})?;
    Ok(Json(json!({ "message": "New user added to database" })))
}
