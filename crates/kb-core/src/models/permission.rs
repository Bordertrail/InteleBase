//! Permission model

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

/// KB Permission entity
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct KbPermission {
    pub id: i64,
    #[sqlx(rename = "user_id")]
    pub user_id: i64,
    #[sqlx(rename = "kb_id")]
    pub kb_id: i64,
    #[sqlx(rename = "role_id")]
    pub role_id: i64,
    pub granted_by: Option<i64>,
    pub created_at: DateTime<Utc>,
}

/// Grant permission request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrantPermission {
    pub user_id: i64,
    pub kb_id: i64,
    pub role_name: String,
}

/// Permission check result
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PermissionResult {
    Allowed,
    Denied,
}

/// Member info for KB
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct KbMember {
    #[sqlx(rename = "user_id")]
    pub user_id: i64,
    pub username: String,
    pub email: String,
    pub role_name: String,
    #[sqlx(rename = "granted_at")]
    pub granted_at: DateTime<Utc>,
}
