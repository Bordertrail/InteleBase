//! Chunk model

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use sqlx::FromRow;

/// Chunk entity (text segment from a document)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Chunk {
    pub id: i64,
    #[sqlx(rename = "document_id")]
    pub document_id: i64,
    #[sqlx(rename = "kb_id")]
    pub kb_id: i64,
    pub chunk_index: i32,
    pub content: String,
    pub content_hash: String,
    pub token_count: i32,
    pub metadata: JsonValue,
    pub created_at: DateTime<Utc>,
}

/// Chunk metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkMetadata {
    pub page: Option<i32>,
    pub heading: Option<String>,
    pub section: Option<String>,
}

/// Scored chunk (search result)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoredChunk {
    pub chunk: Chunk,
    pub score: f32,
    pub highlight: Option<String>,
}
