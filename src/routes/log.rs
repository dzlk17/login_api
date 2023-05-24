#[path = "./user.rs"]
mod user;

use std::sync::Arc;
use axum::{http::StatusCode, extract::State, Json, response::{Response, IntoResponse}};
use scylla::Session;
use user::{verify_password, User};


///Endpoint for user login
/// 
/// ## Arguments
/// *state - function get established session - connection to database
/// *user - data passed by user during login
/// 
/// Function checks if email exists in database and if password is correct, and then returns proper StatusCode. 
pub async fn log(State(state): State<Arc<Session>>, Json(user): Json<User>) -> Response{
    let new_user: Json<User> = Json(user);
    let query = format!("SELECT password_hash FROM user_auth.users_list where email = '{}';", new_user.email);
    match state.query(query, ()).await {
        Ok(query_result) =>{
            for row in query_result.rows().unwrap(){
                let val = row.into_typed::<(String<>,)>().unwrap();
                if verify_password(&new_user.password, val.0){
                    return(StatusCode::ACCEPTED, "The password is correct").into_response()
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