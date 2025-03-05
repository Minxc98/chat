use thiserror::Error;

#[derive(Debug,Error)]
pub enum AppError {
    #[error("sql error: {0}")]
    SqlError(#[from] sqlx::Error),
    #[error("password hash error")]
    PasswordHashError,
    #[error("invalid credentials")]
    InvalidCredentials,
}

