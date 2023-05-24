use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

///The structure stores possible errors returned by the application.
#[derive(Debug)]
pub enum AppError {
    InternalServerError,
    UserAlreadyExists,
    UserNotExists,
    WrongPassword
}

impl IntoResponse for AppError {
    /// Method allows to change error into Response
    fn into_response(self) -> axum::response::Response {
        let (status, err_msg) = match self {
            Self::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error",
            ),
            Self::UserAlreadyExists => (StatusCode::CONFLICT, "Email already exists in database"),
            Self::UserNotExists => (StatusCode::CONFLICT, "Email doesn't exist in database"),
            Self::WrongPassword => (StatusCode::CONFLICT, "Wrong password")
        };
        (status, Json(json!({ "error_message": err_msg }))).into_response()
    }
}