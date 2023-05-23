use std::sync::Arc;
use axum::{Json, http::StatusCode, response::{Response, IntoResponse}};
use scylla::Session;
use axum::extract::State;
use super::User;

///Endpoint for user registration
/// 
/// # Arguments
/// *state - function get established session - connection to database
/// *user - data passed by user during registration
/// 
/// Function check if email exists in database, and if not, creates new user in database. Returns suitable StatusCode, depends on situation.
pub async fn reg(State(state): State<Arc<Session>>, Json(user): Json<User>) -> Response{
    let new_user: Json<User> = Json(user);
    let query = format!("SELECT COUNT(*) FROM user_auth.users_list where email = '{}';", new_user.email);
    match state.query(query, ()).await {
        Ok(query_result) =>{
            for row in query_result.rows().unwrap(){
                let val = row.into_typed::<(i64,)>().unwrap();
                if val.0 > 0{
                    return(StatusCode::CONFLICT, "Email already exists in database").into_response()
                }
            }
        }
        Err(error) =>{
            return (StatusCode::EXPECTATION_FAILED, error.to_string()).into_response()
        }
    }
    let query = format!("INSERT INTO user_auth.users_list (user_id, email, password_hash) VALUES (uuid(), '{}', '{}')", new_user.email, new_user.password);
    match state.query(query, ()).await {
        Ok(_res) =>{
            return (StatusCode::CREATED, "New user added to database").into_response()
        }
        Err(error) =>{
            return (StatusCode::EXPECTATION_FAILED, error.to_string()).into_response()
        }
    }
}
