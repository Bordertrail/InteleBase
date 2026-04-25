//! Object store stub (Phase 2)

use kb_core::AppError;

pub struct ObjectStore;

impl ObjectStore {
    pub fn new(endpoint: &str, bucket: &str) -> Result<Self, AppError> {
        Ok(Self)
    }

    pub async fn put(&self, _key: &str, _data: Vec<u8>) -> Result<(), AppError> {
        Ok(())
    }

    pub async fn get(&self, _key: &str) -> Result<Vec<u8>, AppError> {
        Ok(Vec::new())
    }

    pub async fn delete(&self, _key: &str) -> Result<(), AppError> {
        Ok(())
    }
}
