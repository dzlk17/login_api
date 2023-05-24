use std::sync::Arc;
use axum::{extract::State, Json, };
use dotenvy_macro::dotenv;
use jsonwebtoken::{Header, encode};
use scylla::Session;
use serde_json::{json, Value};
use crate::error::AppError;
use crate::routes::user::{verify_password, User};
use crate::auth::{Claims, get_timestamp, Keys};

///Endpoint for user login
/// 
/// ## Arguments
/// *state - function get established session - connection to database
/// *user - data passed by user during login
/// 
/// Function checks if email exists in database and if password is correct, and then returns proper StatusCode. 
/// If the user manages to log in function returns tocken in response. 
pub async fn log(State(state): State<Arc<Session>>, Json(user): Json<User>) -> Result<Json<Value>, AppError>{
    let new_user: Json<User> = Json(user);
    // secret_key should be generate by server
    let secret_key = dotenv!("SECRET_KEY");
    let key = Keys::new(secret_key.as_bytes());
    let query = format!("SELECT password_hash FROM user_auth.users_list where email = '{}';", new_user.email);
    let query_result = state.query(query, ()).await.map_err(|err| { dbg!(err); AppError::InternalServerError})?;
    for row in query_result.rows().map_err(|err| { dbg!(err); AppError::InternalServerError})?{
        let val = row.into_typed::<(String<>,)>().map_err(|err| { dbg!(err); AppError::InternalServerError})?;
        if verify_password(&new_user.password, val.0){
            let claims = Claims {
                email: new_user.email.to_owned(),
                exp: get_timestamp(),
            };
            let token = encode(&Header::default(), &claims, &key.encoding).map_err(|err| { dbg!(err); AppError::InternalServerError})?;
            return Ok(Json(json!({ "message": "User logged in", "token": token })));
        }
        else{
            return Err(AppError::WrongPassword);
        }
    }
    Err(AppError::UserNotExists)
}