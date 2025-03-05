use axum::{http::StatusCode, response::{IntoResponse, Response}};


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
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            Self::Database(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            Self::InvalidCredentials => (StatusCode::UNAUTHORIZED, "Invalid credentials").into_response(),
            Self::PasswordHashError => (StatusCode::INTERNAL_SERVER_ERROR, "Password hash error").into_response(),
            Self::Jwt(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        }
    }
}

