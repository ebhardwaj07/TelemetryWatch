mod api;
mod config;
mod db;
mod metrics;
mod middleware;

use anyhow::Result;
use std::sync::Arc;
use tracing::info;

use api::create_router;
use config::Config;
use db::Database;
use metrics::Metrics;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "telemetrywatch=info,tower_http=info".into()),
        )
        .init();

    // Load configuration
    let config = Config::load()?;
    info!("Configuration loaded");

    // Initialize metrics
    let metrics = Metrics::new()?;
    info!("Metrics initialized");

    // Initialize database
    let database = Arc::new(
        Database::new(&config.database.url, config.database.max_connections).await?,
    );
    info!("Database initialized");

    // Create router
    let app = create_router(metrics, database);

    // Start server
    let addr = format!("{}:{}", config.server.host, config.server.port);
    info!("Starting TelemetryWatch server on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

