//! kb-server entry point

use axum::Router;
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use kb_core::config::AppConfig;
use kb_db::create_pool;
use kb_server::{build_app, state::AppState};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting kb-server...");

    // Load configuration
    let config = AppConfig::from_env()?;
    tracing::info!("Configuration loaded");

    // Create database pool
    let db = create_pool(&config.database).await?;
    tracing::info!("Database pool created");

    // Note: Migrations should be run separately via sqlx-cli or direct SQL execution
    tracing::info!("Database is ready (migrations run manually)");

    // Create application state
    let state = AppState::new(config.clone(), db);

    // Build router
    let app: Router<()> = build_app(state);

    // Start server
    let addr = SocketAddr::new(config.server.host.parse()?, config.server.port);

    tracing::info!("Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
