//! Router configuration

use axum::{Extension, Router};
use tower_http::{
    compression::CompressionLayer, cors::CorsLayer, services::ServeDir, trace::TraceLayer,
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::openapi::ApiDoc;
use crate::routes::{auth, health, knowledge_bases};
use crate::state::AppState;

/// Create the application router
pub fn create_router(state: AppState) -> Router {
    // Add JWT config to extensions for auth middleware
    let jwt_config = state.jwt_config();

    Router::new()
        // Routes
        .nest("/api/v1/auth", auth::routes())
        .nest("/api/v1/knowledge-bases", knowledge_bases::routes())
        .route("/api/v1/health", axum::routing::get(health::health_check))
        .route("/api/v1/metrics", axum::routing::get(health::metrics))
        // Swagger UI & OpenAPI
        .merge(SwaggerUi::new("/api/v1/docs").url("/api/v1/openapi.json", ApiDoc::openapi()))
        // State and extensions
        .with_state(state)
        .layer(Extension(jwt_config))
        // Middleware layers
        .layer(CorsLayer::permissive()) // TODO: Configure properly for production
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new().gzip(true))
        // Frontend static files as fallback (serve from frontend directory)
        .fallback_service(ServeDir::new("frontend"))
}
