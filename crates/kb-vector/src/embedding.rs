//! Embedding stub - TEI client (Phase 3)

use kb_core::AppError;

pub struct EmbeddingClient;

impl EmbeddingClient {
    pub fn new(url: &str) -> Result<Self, AppError> {
        Ok(Self)
    }

    pub async fn embed(&self, _texts: Vec<&str>) -> Result<Vec<Vec<f32>>, AppError> {
        Ok(Vec::new())
    }
}
