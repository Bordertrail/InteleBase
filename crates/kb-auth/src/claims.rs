//! Claims extraction for Axum middleware

use serde::{Deserialize, Serialize};

use crate::jwt::Claims;

/// Extracted claims from JWT token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedClaims {
    pub user_id: i64,
    pub username: String,
    pub email: String,
    pub is_system_admin: bool,
}

impl From<Claims> for ExtractedClaims {
    fn from(claims: Claims) -> Self {
        Self {
            user_id: claims.sub,
            username: claims.username,
            email: claims.email,
            is_system_admin: claims.is_system_admin,
        }
    }
}
