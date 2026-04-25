//! Pipeline stub (Phase 2)

use kb_core::AppError;

pub struct DocumentPipeline;

impl DocumentPipeline {
    pub fn process(_document_id: uuid::Uuid) -> Result<(), AppError> {
        Ok(())
    }
}
