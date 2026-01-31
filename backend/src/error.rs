use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;

#[derive(Debug, thiserror::Error)]
#[allow(dead_code)]
pub enum AppError {
    #[error("not found")]
    NotFound,
    #[error("bad request: {0}")]
    BadRequest(String),
    #[error("internal error: {0}")]
    Internal(String),
}

impl AppError {
    #[allow(dead_code)]
    pub fn bad_request(message: impl Into<String>) -> Self {
        Self::BadRequest(message.into())
    }

    #[allow(dead_code)]
    pub fn internal(message: impl Into<String>) -> Self {
        Self::Internal(message.into())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::NotFound => {
                (StatusCode::NOT_FOUND, Json(json!({"error": "not found"}))).into_response()
            }
            AppError::BadRequest(message) => {
                (StatusCode::BAD_REQUEST, Json(json!({"error": message}))).into_response()
            }
            AppError::Internal(message) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": message})),
            )
                .into_response(),
        }
    }
}
