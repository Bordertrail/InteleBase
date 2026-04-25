//! MinIO stub (Phase 2)

use kb_core::AppError;

pub struct MinioClient;

impl MinioClient {
    pub fn new(endpoint: &str, access_key: &str, secret_key: &str) -> Result<Self, AppError> {
        Ok(Self)
    }
}
