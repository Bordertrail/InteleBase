//! Reader stub - Search reader (Phase 3)

use kb_core::AppError;

pub struct SearchResult {
    pub chunk_id: uuid::Uuid,
    pub score: f32,
    pub content: String,
}

pub fn search(_query: &str, _limit: usize) -> Result<Vec<SearchResult>, AppError> {
    Ok(Vec::new())
}
