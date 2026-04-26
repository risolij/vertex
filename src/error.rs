use axum::{Json, http::StatusCode, response::IntoResponse};
use serde_json::json;

#[derive(Debug)]
pub enum ApiError {
    NotFound,
    InvalidPath(String),
    InternalServerError,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            ApiError::NotFound => {
                (StatusCode::NOT_FOUND, "Record not found".to_owned())
            },
            ApiError::InvalidPath(s) => {
                (StatusCode::BAD_REQUEST, s)
            },
            ApiError::InternalServerError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong".to_owned())
            }
        };

        let body = Json(json!({
            "error": error_message
        }));

        (status, body).into_response()
    }
}
