//! VectorStore trait - Abstract interface for vector databases

use crate::AppError;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// Vector point for upsert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorPoint {
    pub id: String,
    pub vector: Vec<f32>,
    pub payload: serde_json::Value,
}

/// Scored point from search
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoredPoint {
    pub id: String,
    pub score: f32,
    pub payload: serde_json::Value,
}

/// Search filters
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VectorSearchFilters {
    pub kb_id: Option<i64>,
    pub document_id: Option<i64>,
    pub file_types: Option<Vec<String>>,
}

/// VectorStore trait - Implemented by Qdrant, pgvector, etc.
#[async_trait]
pub trait VectorStore: Send + Sync {
    /// Upsert vectors into a collection
    async fn upsert(&self, collection: &str, points: Vec<VectorPoint>) -> Result<(), AppError>;

    /// Search for similar vectors
    async fn search(
        &self,
        collection: &str,
        vector: Vec<f32>,
        filters: &VectorSearchFilters,
        limit: usize,
    ) -> Result<Vec<ScoredPoint>, AppError>;

    /// Delete a collection
    async fn delete_collection(&self, collection: &str) -> Result<(), AppError>;

    /// Delete specific points
    async fn delete_points(&self, collection: &str, point_ids: &[String]) -> Result<(), AppError>;

    /// Create a collection if not exists
    async fn create_collection(&self, collection: &str, vector_size: usize)
    -> Result<(), AppError>;
}
