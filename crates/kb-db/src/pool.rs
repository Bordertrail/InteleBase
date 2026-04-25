//! Database pool management

pub use sqlx::postgres::PgPool;
use sqlx::postgres::PgPoolOptions;
use tracing::info;

use kb_core::AppError;
use kb_core::config::DatabaseConfig;

/// Create a PostgreSQL connection pool
pub async fn create_pool(config: &DatabaseConfig) -> Result<PgPool, AppError> {
    info!("Connecting to database: {}", mask_url(&config.url));

    let pool = PgPoolOptions::new()
        .max_connections(config.max_connections.unwrap_or(10))
        .connect(&config.url)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    info!("Database connection pool created successfully");

    Ok(pool)
}

/// Mask sensitive parts of database URL for logging
fn mask_url(url: &str) -> String {
    if let Some(pos) = url.find("://") {
        let prefix = &url[..pos + 3];
        let rest = &url[pos + 3..];
        if let Some(at_pos) = rest.find('@') {
            let credentials = &rest[..at_pos];
            if let Some(colon_pos) = credentials.find(':') {
                let user = &credentials[..colon_pos];
                return format!("{}{}:****@{}", prefix, user, &rest[at_pos + 1..]);
            }
        }
    }
    url.to_string()
}

/// Convert sqlx::Error to AppError helper function
pub fn sqlx_error_to_app_error(err: sqlx::Error) -> AppError {
    match err {
        sqlx::Error::RowNotFound => AppError::UserNotFound,
        _ => AppError::DatabaseError(err.to_string()),
    }
}
