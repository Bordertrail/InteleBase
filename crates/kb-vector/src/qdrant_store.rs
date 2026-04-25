//! Qdrant store stub (Phase 3)

use kb_core::AppError;

pub struct QdrantStore;

impl QdrantStore {
    pub fn new(url: &str) -> Result<Self, AppError> {
        Ok(Self)
    }
}
