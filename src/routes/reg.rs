use std::sync::Arc;
use axum::Json;
use scylla::Session;
use axum::extract::State;
use serde_json::{json, Value};
use crate::routes::user::{hash_password, User};
use crate::error::AppError;
use regex::Regex;

///Endpoint for user registration
/// 
/// ## Arguments
/// *state - function get established session - connection to database
/// *user - data passed by user during registration
/// 
/// Function check if email exists in database, and if not, creates new user in database. Returns Result with suitable message or error, depends on situation.
pub async fn reg(State(state): State<Arc<Session>>, Json(user): Json<User>) -> Result<Json<Value>, AppError> {
    if !validate_email(&user.email){
        return Err(AppError::IncorrectEmail);
    }
    let query = format!("SELECT COUNT(*) FROM user_auth.users_list where email = '{}';", user.email);
    let query_result = state.query(query, ()).await.map_err(|err| { dbg!(err); AppError::InternalServerError})?;
    for row in query_result.rows().map_err(|err| { dbg!(err); AppError::InternalServerError})?{
        let val = row.into_typed::<(i64,)>().map_err(|err| { dbg!(err); AppError::InternalServerError})?;
        if val.0 > 0 {
            return Err(AppError::UserAlreadyExists);
        }
    }
    let query = format!("INSERT INTO user_auth.users_list (user_id, email, password_hash) VALUES (uuid(), '{}', '{}')", user.email, hash_password(&user.password));
    state.query(query, ()).await.map_err(|err| { dbg!(err); AppError::InternalServerError})?;
    Ok(Json(json!({ "message": "New user added to database" })))
}

fn validate_email(email: &str) -> bool {
    let re = Regex::new(r"^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}$").unwrap();
    re.is_match(email)
}