mod config;
mod handlers;
mod models;
mod error;
mod utils;

use handlers::*;
use std::ops::Deref;
use std::sync::Arc;
use axum::handler::Handler;
use axum::Router;
use axum::routing::{get, patch, post};
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;
use tower_http::LatencyUnit;
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::Level;
pub use config::*;
pub use models::*;
pub use error::*;
pub use utils::*;

#[derive(Debug, Clone)]
pub(crate) struct AppState {
    inner: Arc<AppStateInner>,
}

#[derive(Debug)]
pub(crate) struct AppStateInner {
    pub(crate) pool: sqlx::PgPool,
    pub(crate) config: AppConfig,
}

impl Deref for AppState {
    type Target = AppStateInner;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl AppState {
    
    pub async fn new(config: AppConfig) -> Result<Self,AppError> {
        let pool = sqlx::PgPool::connect(&config.server.db_url)
            .await
            .map_err(|_| AppError::Database(sqlx::Error::Configuration("Failed to connect to database".into())))?;
        Ok(Self {
            inner: Arc::new(AppStateInner {
                pool,
                config,
            }),
        })
    }
    
}
pub async fn get_router(config: AppConfig) -> Result<Router,AppError> {
    let state: AppState = AppState::new(config).await?;

    let api = Router::new()
        .route("/sign_in", post(sign_in_handler))
        .route("/sign_up", post(sign_up_handler))
        .route("/chat", get(list_chat_handler)
            .post(create_chat_handler))
        .route("/chat/{id}", patch(update_chat_handler)
            .delete(delete_chat_handler)
            .post(send_message_handler))
        .route("/chat/{id}/message", get(list_message_handler));
    Ok(set_router_layers(Router::new()
        .route("/api", get(index_handler))
        .nest("/api", api)
        .with_state(state)))

}
