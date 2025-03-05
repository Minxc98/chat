mod config;
mod handlers;
use handlers::*;
use std::ops::Deref;
use std::sync::Arc;
use axum::Router;
use axum::routing::{get, patch, post};
pub use config::*;
#[derive(Debug, Clone)]
pub(crate) struct AppState {
    inner: Arc<AppStateInner>,
}

#[derive(Debug)]
pub(crate) struct AppStateInner {
    pub(crate) config: AppConfig,
}

impl Deref for AppState {
    type Target = AppStateInner;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl AppState {
    pub fn new(config: AppConfig) -> Self {
        Self {
            inner: Arc::new(AppStateInner {
                config,
            }),
        }
    }
}
pub fn get_router(config: AppConfig) -> Router {
    let state = AppState::new(config);

    let api = Router::new()
        .route("/sign_in", post(sign_in_handler))
        .route("/sign_up", post(sign_up_handler))
        .route("/chat", get(list_chat_handler)
            .post(create_chat_handler))
        .route("/chat/{id}", patch(update_chat_handler)
            .delete(delete_chat_handler)
            .post(send_message_handler))
        .route("/chat/{id}/message", get(list_message_handler));
    Router::new()
        .route("/api", get(index_handler))
        .nest("/api", api)
        .with_state(state)
}