mod config;
mod error;
mod handlers;
mod middlewares;
mod models;
mod utils;

use crate::jwt::{DecodingKey, EncodingKey};
use axum::handler::Handler;
use axum::middleware::from_fn_with_state;
use axum::routing::{get, patch, post};
use axum::Router;
pub use config::*;
pub use error::*;
use handlers::*;
pub use middlewares::*;
pub use models::*;
use std::fmt;
use std::ops::Deref;
use std::sync::Arc;
pub use utils::*;

#[derive(Debug, Clone)]
pub(crate) struct AppState {
    inner: Arc<AppStateInner>,
}

pub(crate) struct AppStateInner {
    pub(crate) pool: sqlx::PgPool,
    pub(crate) config: AppConfig,
    pub(crate) ek: EncodingKey,
    pub(crate) dk: DecodingKey,
    pub(crate) redis_client: redis::Client,
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
        let redis_client = redis::Client::open(config.server.redis_url.clone())?;
        let pool = sqlx::PgPool::connect(&config.server.db_url)
            .await
            .map_err(|_| {
                AppError::Database(sqlx::Error::Configuration(
                    "Failed to connect to database".into(),
                ))
            })?;
        Ok(Self {
            inner: Arc::new(AppStateInner {
                pool,
                config,
                ek,
                dk,
                redis_client,
            }),
        })
    }
}
pub async fn get_router(config: AppConfig) -> Result<Router, AppError> {
    let state: AppState = AppState::new(config).await?;

    let api = Router::new()
        .route("/chat", get(list_chat_handler).post(create_chat_handler))
        .route(
            "/chat/{id}",
            patch(update_chat_handler)
                .delete(delete_chat_handler)
                .post(send_message_handler),
        )
        .route("/chat/{id}/message", get(list_message_handler))
        .layer(from_fn_with_state(state.clone(), auth::verify_token))
        .route("/sign_in", post(sign_in_handler))
        .route("/sign_up", post(sign_up_handler));
    let app = Router::new()
        .route("/api", get(index_handler))
        .nest("/api", api)
        .with_state(state.clone());
    Ok(set_router_layers(app))
}

impl fmt::Debug for AppStateInner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AppState")
            .field("pool", &self.pool)
            .field("config", &self.config)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use redis::Commands;
    use super::*;

    #[test]
    fn test_redis_connection() -> Result<(), AppError> {
        let client = redis::Client::open("redis://127.0.0.1/")?;
        let mut con = client.get_connection()?;
        con.set("hello", "world")?;
        let value: String = con.get("hello")?;
        assert_eq!(value, "world");

        /* do something here */

        Ok(())
    }
}
