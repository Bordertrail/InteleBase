//! Health check and metrics routes

use axum::{Json, extract::State};
use kb_db::PgPool;
use serde::Serialize;
use utoipa::ToSchema;

use crate::state::AppState;

/// Health status response
#[derive(Debug, Serialize, ToSchema)]
pub struct HealthResponse {
    status: String,
    database: String,
    redis: String,
}

/// Health check handler
#[utoipa::path(
    get,
    path = "/api/v1/health",
    tag = "health",
    responses(
        (status = 200, description = "Service is healthy", body = HealthResponse),
    )
)]
pub async fn health_check(State(state): State<AppState>) -> Json<HealthResponse> {
    // Check database connection
    let db_status = check_db(&state.db).await;

    Json(HealthResponse {
        status: if db_status == "ok" {
            "ok".to_string()
        } else {
            "degraded".to_string()
        },
        database: db_status,
        redis: "not_implemented".to_string(), // Phase 2
    })
}

/// Check database health
async fn check_db(pool: &PgPool) -> String {
    match kb_db::sqlx::query("SELECT 1").execute(pool).await {
        Ok(_) => "ok".to_string(),
        Err(e) => format!("error: {}", e),
    }
}

/// Metrics handler (Prometheus format)
#[utoipa::path(
    get,
    path = "/api/v1/metrics",
    tag = "health",
    responses(
        (status = 200, description = "Prometheus metrics", content_type = "text/plain"),
    )
)]
pub async fn metrics(State(state): State<AppState>) -> String {
    // Basic metrics - will be expanded in Phase 6
    let db_status = check_db(&state.db).await;

    format!(
        "# HELP kb_health_db Database health status (1=ok, 0=error)\n# TYPE kb_health_db gauge\nkb_health_db {}\n",
        if db_status == "ok" { 1 } else { 0 }
    )
}
