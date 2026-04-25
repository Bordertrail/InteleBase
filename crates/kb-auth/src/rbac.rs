//! RBAC (Role-Based Access Control) utilities

use kb_db::permissions::PermissionRepository;

/// Permission levels
pub enum PermissionLevel {
    Read,  // viewer
    Write, // editor
    Admin, // admin
}

impl PermissionLevel {
    pub fn as_role_name(&self) -> &'static str {
        match self {
            PermissionLevel::Read => "viewer",
            PermissionLevel::Write => "editor",
            PermissionLevel::Admin => "admin",
        }
    }
}

/// Check if user has required permission for a KB
pub async fn check_kb_permission(
    user_id: i64,
    kb_id: i64,
    required_level: PermissionLevel,
    perm_repo: &PermissionRepository,
) -> Result<bool, kb_core::AppError> {
    let result = perm_repo
        .check_permission(user_id, kb_id, required_level.as_role_name())
        .await?;

    Ok(result == kb_core::models::PermissionResult::Allowed)
}

/// Require permission - returns error if not allowed
pub async fn require_kb_permission(
    user_id: i64,
    kb_id: i64,
    required_level: PermissionLevel,
    perm_repo: &PermissionRepository,
) -> Result<(), kb_core::AppError> {
    let has_permission = check_kb_permission(user_id, kb_id, required_level, perm_repo).await?;

    if !has_permission {
        return Err(kb_core::AppError::Forbidden);
    }

    Ok(())
}
