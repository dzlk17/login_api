#[path = "./user.rs"]
mod user;

use std::sync::Arc;
use axum::{http::StatusCode, extract::State, Json, response::{Response, IntoResponse}};
use dotenvy_macro::dotenv;
use jsonwebtoken::{Header, encode};
use scylla::Session;
use user::{verify_password, User};

use crate::{auth::{Claims, get_timestamp, Keys}};

///Endpoint for user login
/// 
/// ## Arguments
/// *state - function get established session - connection to database
/// *user - data passed by user during login
/// 
/// Function checks if email exists in database and if password is correct, and then returns proper StatusCode. 
/// If the user manages to log in function returns tocken in response. 
pub async fn log(State(state): State<Arc<Session>>, Json(user): Json<User>) -> Response{
    let new_user: Json<User> = Json(user);
    // secret_key should be generate by server
    let secret_key = dotenv!("SECRET_KEY");
    let key = Keys::new(secret_key.as_bytes());
    let query = format!("SELECT password_hash FROM user_auth.users_list where email = '{}';", new_user.email);
    match state.query(query, ()).await {
        Ok(query_result) =>{
            for row in query_result.rows().unwrap(){
                let val = row.into_typed::<(String<>,)>().unwrap();
                if verify_password(&new_user.password, val.0){
                    let claims = Claims {
                        email: new_user.email.to_owned(),
                        exp: get_timestamp(),
                    };
                    let token = encode(&Header::default(), &claims, &key.encoding).unwrap();
                    return(StatusCode::ACCEPTED, token).into_response()
                }
                else{
                    return(StatusCode::CONFLICT, "Wrong password").into_response()
                }
            }
            return (StatusCode::CONFLICT, "Email doesn't exist in database").into_response()
        }
        Err(error) =>{
            return (StatusCode::EXPECTATION_FAILED, error.to_string()).into_response()
        }
    }
}