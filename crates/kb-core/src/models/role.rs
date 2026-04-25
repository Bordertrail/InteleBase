//! Role model

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// System roles
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RoleName {
    Admin,
    Editor,
    Viewer,
}

impl RoleName {
    pub fn as_str(&self) -> &'static str {
        match self {
            RoleName::Admin => "admin",
            RoleName::Editor => "editor",
            RoleName::Viewer => "viewer",
        }
    }
}

/// Role entity
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Role {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// Role permission hierarchy
impl RoleName {
    pub fn can_write(&self) -> bool {
        matches!(self, RoleName::Admin | RoleName::Editor)
    }

    pub fn can_delete(&self) -> bool {
        matches!(self, RoleName::Admin)
    }

    pub fn can_manage_members(&self) -> bool {
        matches!(self, RoleName::Admin)
    }
}
