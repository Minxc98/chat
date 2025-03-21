mod config;

use anyhow::Result;
use chat_server::{get_router, AppConfig};
use tracing::{info, level_filters::LevelFilter as Level};
use tracing_subscriber::fmt::Layer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, Layer as _};

#[tokio::main]
async fn main() -> Result<()> {
    let layer = Layer::new().with_filter(Level::INFO);
    tracing_subscriber::registry().with(layer).init();

    let config = AppConfig::load()?;
    let addr = format!("{}:{}", "0.0.0.0", config.server.port);
    let app = get_router(config).await?;
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    info!("listening on {}", addr);

    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}
