//! kb-server - HTTP API server using Axum
//!
//! Routes for auth, knowledge bases, documents, search, RAG, and admin.

pub mod error;
pub mod middleware;
pub mod openapi;
pub mod router;
pub mod routes;
pub mod state;

use axum::Router;
use state::AppState;

pub fn build_app(state: AppState) -> Router {
    router::create_router(state)
}
