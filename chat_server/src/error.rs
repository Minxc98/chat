use axum::{
    http::{self, StatusCode},
    response::{IntoResponse, Response},
};
use jwt_simple::reexports::serde_json;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Password hash error")]
    PasswordHashError,
    #[error("JWT error: {0}")]
    Jwt(#[from] jwt_simple::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("HTTP error: {0}")]
    Http(#[from] http::Error),
    #[error("Axum error: {0}")]
    Axum(#[from] axum::Error),
    #[error("Redis error: {0}")]
    Redis(#[from] redis::RedisError),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            Self::Database(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            Self::InvalidCredentials => {
                (StatusCode::UNAUTHORIZED, "Invalid credentials").into_response()
            }
            Self::PasswordHashError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Password hash error").into_response()
            }
            Self::Jwt(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            Self::Json(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
            Self::Http(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            Self::Axum(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            Self::Redis(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        }
    }
}
