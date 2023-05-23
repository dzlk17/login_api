use axum::{Json, http::StatusCode, response::{Response, IntoResponse}};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct User{
    email: String,
    password: String,
}
pub async fn reg(Json(user): Json<User>) -> Response{
    let a = Json(user);
    (StatusCode::CREATED, "Udane".to_owned()).into_response()
}