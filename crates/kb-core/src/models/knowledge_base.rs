//! Knowledge Base model

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use sqlx::FromRow;
use utoipa::ToSchema;

/// Knowledge Base entity
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct KnowledgeBase {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    #[sqlx(rename = "owner_id")]
    pub owner_id: i64,
    pub settings: JsonValue,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub archived_at: Option<DateTime<Utc>>,
}

/// Knowledge Base settings
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct KbSettings {
    pub chunk_size: usize,
    pub chunk_overlap: usize,
    pub embedding_model: String,
    pub llm_model: String,
    pub language: Option<String>,
}

impl Default for KbSettings {
    fn default() -> Self {
        Self {
            chunk_size: 512,
            chunk_overlap: 50,
            embedding_model: "BAAI/bge-large-en-v1.5".to_string(),
            llm_model: "llama3.1".to_string(),
            language: None,
        }
    }
}

/// Create Knowledge Base request
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateKnowledgeBase {
    pub name: String,
    pub description: Option<String>,
    pub settings: Option<KbSettings>,
}

/// Update Knowledge Base request
#[derive(Debug, Clone, Serialize, Deserialize, Default, ToSchema)]
pub struct UpdateKnowledgeBase {
    pub name: Option<String>,
    pub description: Option<String>,
    pub settings: Option<KbSettings>,
}
