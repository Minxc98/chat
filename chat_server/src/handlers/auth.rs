use axum::{
    extract::State,
    http::{header, HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};

use crate::{
    models::user::{CreateUser, SignInUser},
    utils, AppError, AppState, User,
};

pub(crate) async fn sign_up_handler(
    State(state): State<AppState>,
    Json(payload): Json<CreateUser>,
) -> Result<impl IntoResponse, AppError> {
    let user = User::create(&state.pool, &payload).await?;
    Ok((StatusCode::CREATED, Json(user)))
}

pub(crate) async fn sign_in_handler(
    State(state): State<AppState>,
    Json(payload): Json<SignInUser>,
) -> Result<impl IntoResponse, AppError> {
    let is_valid = User::verify_password(&payload, &state.pool).await?;
    if !is_valid {
        return Err(AppError::InvalidCredentials);
    }
    let user = User::find_by_username( &state.pool, &payload.username).await?.ok_or(AppError::InvalidCredentials)?;
    let jwt = state.ek.sign(user)?;
    Ok(Json(jwt))
}
