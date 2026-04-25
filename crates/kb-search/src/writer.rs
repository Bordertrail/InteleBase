//! Writer stub - Index writer (Phase 3)

use kb_core::AppError;
use serde_json::Value as JsonValue;

pub struct IndexWriter;

impl IndexWriter {
    pub fn add_document(&self, _doc: JsonValue) -> Result<(), AppError> {
        Ok(())
    }

    pub fn commit(&self) -> Result<(), AppError> {
        Ok(())
    }
}
