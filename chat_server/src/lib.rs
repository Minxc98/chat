mod config;
mod handlers;
mod models;
mod error;
mod utils;
mod middlewares;

use std::fmt;
use handlers::*;
use std::ops::Deref;
use std::sync::Arc;
use axum::handler::Handler;
use axum::middleware::from_fn_with_state;
use axum::Router;
use axum::routing::{get, patch, post};
pub use config::*;
pub use models::*;
pub use error::*;
pub use utils::*;
pub use middlewares::*;
use crate::jwt::{DecodingKey, EncodingKey};

#[derive(Debug, Clone)]
pub(crate) struct AppState {
    inner: Arc<AppStateInner>,
}


pub(crate) struct AppStateInner {
    pub(crate) pool: sqlx::PgPool,
    pub(crate) config: AppConfig,
    pub(crate) ek: EncodingKey,
    pub(crate) dk: DecodingKey,
}

impl Deref for AppState {
    type Target = AppStateInner;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl AppState {
    pub async fn new(config: AppConfig) -> Result<Self, AppError> {
        let ek = EncodingKey::load(&config.key_pair.private_key)?;
        let dk = DecodingKey::load(&config.key_pair.public_key)?;
        let pool = sqlx::PgPool::connect(&config.server.db_url)
            .await
            .map_err(|_| AppError::Database(sqlx::Error::Configuration("Failed to connect to database".into())))?;
        Ok(Self {
            inner: Arc::new(AppStateInner {
                pool,
                config,
                ek,
                dk,
            }),
        })
    }
}
pub async fn get_router(config: AppConfig) -> Result<Router, AppError> {
    let state: AppState = AppState::new(config).await?;

    let api = Router::new()

        .route("/chat", get(list_chat_handler)
            .post(create_chat_handler))
        .route("/chat/{id}", patch(update_chat_handler)
            .delete(delete_chat_handler)
            .post(send_message_handler))
        .route("/chat/{id}/message", get(list_message_handler))
        .layer(from_fn_with_state(state.clone(), auth::verify_token))
        .route("/sign_in", post(sign_in_handler))
        .route("/sign_up", post(sign_up_handler));
    let app = Router::new()
        .route("/api", get(index_handler))
        .nest("/api", api)
        .with_state(state.clone());
    Ok(set_router_layers(app, state))
}


impl fmt::Debug for AppStateInner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AppState")
            .field("pool", &self.pool)
            .field("config", &self.config)
            .finish()
    }
}