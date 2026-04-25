//! RAG pipeline stub (Phase 4)

use kb_core::AppError;

pub struct RagPipeline;

impl RagPipeline {
    pub fn new() -> Self {
        Self
    }

    pub async fn query(&self, _question: &str, _kb_id: uuid::Uuid) -> Result<String, AppError> {
        Ok(String::new())
    }
}
