//! Index stub - Tantivy index initialization (Phase 3)

use kb_core::AppError;

pub struct SearchIndex;

impl SearchIndex {
    pub fn open(path: &str) -> Result<Self, AppError> {
        Ok(Self)
    }
}
