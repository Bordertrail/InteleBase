//! Hybrid search stub - RRF fusion (Phase 3)

use kb_core::AppError;

pub struct SearchResult {
    pub chunk_id: uuid::Uuid,
    pub score: f32,
    pub content: String,
}

pub fn hybrid_search(
    _query: &str,
    _kb_id: uuid::Uuid,
    _limit: usize,
) -> Result<Vec<SearchResult>, AppError> {
    Ok(Vec::new())
}
