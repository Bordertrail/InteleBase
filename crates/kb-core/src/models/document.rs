//! Document model

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use sqlx::FromRow;

/// Document status enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DocumentStatus {
    Uploaded,
    Processing,
    Ready,
    Failed,
}

impl DocumentStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            DocumentStatus::Uploaded => "uploaded",
            DocumentStatus::Processing => "processing",
            DocumentStatus::Ready => "ready",
            DocumentStatus::Failed => "failed",
        }
    }
}

/// Document entity
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Document {
    pub id: i64,
    #[sqlx(rename = "kb_id")]
    pub kb_id: i64,
    pub title: String,
    pub filename: String,
    pub file_type: String,
    pub file_size: i64,
    pub storage_path: String,
    pub status: String,
    pub error_message: Option<String>,
    pub chunk_count: i32,
    pub metadata: JsonValue,
    pub created_by: Option<i64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

/// Document metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentMetadata {
    pub author: Option<String>,
    pub source_url: Option<String>,
    pub pages: Option<i32>,
    pub language: Option<String>,
}

/// Create Document request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDocument {
    pub kb_id: i64,
    pub title: String,
    pub filename: String,
    pub file_type: String,
    pub file_size: i64,
    pub storage_path: String,
    pub created_by: Option<i64>,
}
