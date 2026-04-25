//! ObjectStorage trait - Abstract interface for object storage

use async_trait::async_trait;

use crate::AppError;

/// ObjectStorage trait - Implemented by MinIO/S3
#[async_trait]
pub trait ObjectStorage: Send + Sync {
    /// Upload object
    async fn put(&self, key: &str, data: Vec<u8>) -> Result<(), AppError>;

    /// Download object
    async fn get(&self, key: &str) -> Result<Vec<u8>, AppError>;

    /// Delete object
    async fn delete(&self, key: &str) -> Result<(), AppError>;

    /// Generate presigned URL for download
    async fn presigned_url(&self, key: &str, expires_in_secs: u64) -> Result<String, AppError>;

    /// Check if object exists
    async fn exists(&self, key: &str) -> Result<bool, AppError>;
}
