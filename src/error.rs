use axum::Json;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Database Error")]
    Database(#[from] surrealdb::Error),

    #[error("NotFound")]
    NotFound,

    #[error("InvalidPath")]
    InvalidPath(String),

    #[error("Internal Server Error")]
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
            },
            ApiError::Database(db) => {
                (StatusCode::INTERNAL_SERVER_ERROR, db.to_string())
            }
        };

        let body = Json(json!({
            "error": error_message
        }));

        (status, body).into_response()
    }
}
