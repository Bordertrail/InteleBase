//! SearchEngine trait - Abstract interface for full-text search

use crate::AppError;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// Document for indexing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexableDocument {
    pub chunk_id: i64,
    pub document_id: i64,
    pub kb_id: i64,
    pub content: String,
    pub title: String,
    pub file_type: String,
    pub metadata: serde_json::Value,
}

/// Scored document from search
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoredDoc {
    pub chunk_id: i64,
    pub document_id: i64,
    pub score: f32,
    pub content: String,
    pub highlight: Option<String>,
}

/// Search filters
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchFilters {
    pub kb_id: Option<i64>,
    pub file_types: Option<Vec<String>>,
    pub date_from: Option<chrono::DateTime<chrono::Utc>>,
    pub date_to: Option<chrono::DateTime<chrono::Utc>>,
}

/// SearchEngine trait - Implemented by Tantivy
#[async_trait]
pub trait SearchEngine: Send + Sync {
    /// Search for documents matching query
    fn search(
        &self,
        query: &str,
        filters: &SearchFilters,
        limit: usize,
    ) -> Result<Vec<ScoredDoc>, AppError>;

    /// Index a document
    fn index_document(&self, doc: IndexableDocument) -> Result<(), AppError>;

    /// Delete a document from index
    fn delete_document(&self, chunk_id: &i64) -> Result<(), AppError>;

    /// Commit pending index changes
    fn commit(&self) -> Result<(), AppError>;
}
