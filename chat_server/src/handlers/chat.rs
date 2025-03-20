use axum::Extension;
use axum::extract::State;
use axum::response::IntoResponse;
use tracing::log::{info};
use crate::{AppState, User};

pub(crate) async fn list_chat_handler(
    Extension(user): Extension<User>,
    State(state): State<AppState>
) -> impl IntoResponse {
    info!("now user registered {}", user.username);
}

pub(crate) async fn create_chat_handler() -> impl IntoResponse {
    "create_chat".to_string()
}

pub(crate) async fn delete_chat_handler() -> impl IntoResponse {
    "delete_chat".to_string()
}

pub(crate) async fn update_chat_handler() -> impl IntoResponse {
    "update_chat_handler".to_string()
}
