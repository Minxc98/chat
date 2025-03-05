use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::{AppState, CreateUser, User, AppError};

pub(crate) async fn sign_in_handler(
    State(state): State<AppState>,
    Json(payload): Json<CreateUser>,
) -> Result<impl IntoResponse, AppError> {
    let user = User::create(&state.pool, &payload).await?;
    Ok((StatusCode::CREATED, Json(user)))
}

pub(crate) async fn sign_up_handler() -> impl IntoResponse  {
    "sign_up".to_string()
}