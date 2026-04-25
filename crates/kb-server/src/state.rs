//! Application state shared across all handlers

use kb_db::PgPool;

use kb_core::config::{AppConfig, JwtConfig};

/// Application state
#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub db: PgPool,
    // pub redis: redis::aio::ConnectionManager,  // Phase 2
    // pub cache: MokaCache,                       // Phase 2
    // pub tantivy: TantivyIndex,                  // Phase 3
    // pub qdrant: QdrantClient,                   // Phase 3
}

impl AppState {
    pub fn new(config: AppConfig, db: PgPool) -> Self {
        Self { config, db }
    }

    pub fn jwt_config(&self) -> JwtConfig {
        self.config.jwt.clone()
    }
}
