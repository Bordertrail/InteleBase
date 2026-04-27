//! Router configuration

use axum::{Extension, Router};
use leptos::prelude::*;
use leptos_axum::{LeptosRoutes, generate_route_list};
use tower_http::{
    compression::CompressionLayer, cors::CorsLayer, trace::TraceLayer,
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::openapi::ApiDoc;
use crate::routes::{auth, health, knowledge_bases};
use crate::state::AppState;
use kb_web::App;

/// Create the application router
pub fn create_router(state: AppState) -> Router {
    // Add JWT config to extensions for auth middleware
    let jwt_config = state.jwt_config();

    // Configure Leptos options for SSR
    let leptos_options = LeptosOptions::builder()
        .site_pkg_dir("pkg")
        .output_dir("crates/kb-web/dist")
        .build();

    // Generate Leptos route list
    let routes = generate_route_list(App);

    Router::new()
        // API Routes
        .nest("/api/v1/auth", auth::routes())
        .nest("/api/v1/knowledge-bases", knowledge_bases::routes())
        .route("/api/v1/health", axum::routing::get(health::health_check))
        .route("/api/v1/metrics", axum::routing::get(health::metrics))
        // Swagger UI & OpenAPI
        .merge(SwaggerUi::new("/api/v1/docs").url("/api/v1/openapi.json", ApiDoc::openapi()))
        // Leptos SSR routes
        .leptos_routes(&leptos_options, routes, App)
        // State and extensions
        .with_state(state)
        .layer(Extension(jwt_config))
        // Middleware layers
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new().gzip(true))
}
