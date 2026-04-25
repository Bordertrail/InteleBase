//! Audit Log model

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use sqlx::FromRow;

/// Audit action types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuditAction {
    DocumentUpload,
    KbCreate,
    SearchQuery,
    RagAsk,
    UserLogin,
    UserRegister,
    PermissionGrant,
    PermissionRevoke,
}

impl AuditAction {
    pub fn as_str(&self) -> &'static str {
        match self {
            AuditAction::DocumentUpload => "document.upload",
            AuditAction::KbCreate => "kb.create",
            AuditAction::SearchQuery => "search.query",
            AuditAction::RagAsk => "rag.ask",
            AuditAction::UserLogin => "user.login",
            AuditAction::UserRegister => "user.register",
            AuditAction::PermissionGrant => "permission.grant",
            AuditAction::PermissionRevoke => "permission.revoke",
        }
    }
}

/// Audit Log entity
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AuditLog {
    pub id: i64,
    pub user_id: Option<i64>,
    pub kb_id: Option<i64>,
    pub action: String,
    pub resource_type: String,
    pub resource_id: Option<i64>,
    pub details: Option<JsonValue>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// Create audit log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAuditLog {
    pub user_id: Option<i64>,
    pub kb_id: Option<i64>,
    pub action: AuditAction,
    pub resource_type: String,
    pub resource_id: Option<i64>,
    pub details: Option<JsonValue>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
}
