//! Permission repository

use sqlx::PgPool;

use kb_core::AppError;
use kb_core::models::{KbMember, KbPermission, PermissionResult};

use crate::pool::sqlx_error_to_app_error;

/// Permission repository for database operations
pub struct PermissionRepository {
    pool: PgPool,
}

impl PermissionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Grant permission to user for a KB
    pub async fn grant(
        &self,
        user_id: i64,
        kb_id: i64,
        role_id: i64,
        granted_by: Option<i64>,
    ) -> Result<KbPermission, AppError> {
        let perm = sqlx::query_as::<_, KbPermission>(
            r#"
            INSERT INTO kb_permissions (user_id, kb_id, role_id, granted_by)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (user_id, kb_id) DO UPDATE SET role_id = $3, granted_by = $4
            RETURNING *
            "#,
        )
        .bind(user_id)
        .bind(kb_id)
        .bind(role_id)
        .bind(granted_by)
        .fetch_one(&self.pool)
        .await
        .map_err(sqlx_error_to_app_error)?;

        Ok(perm)
    }

    /// Revoke permission from user for a KB
    pub async fn revoke(&self, user_id: i64, kb_id: i64) -> Result<(), AppError> {
        let result = sqlx::query("DELETE FROM kb_permissions WHERE user_id = $1 AND kb_id = $2")
            .bind(user_id)
            .bind(kb_id)
            .execute(&self.pool)
            .await
            .map_err(sqlx_error_to_app_error)?;

        if result.rows_affected() == 0 {
            return Err(AppError::ValidationError(
                "Permission not found".to_string(),
            ));
        }

        Ok(())
    }

    /// Get user's role for a KB
    pub async fn get_role(&self, user_id: i64, kb_id: i64) -> Result<Option<String>, AppError> {
        let role_name = sqlx::query_scalar::<_, Option<String>>(
            r#"
            SELECT r.name FROM kb_permissions kp
            JOIN roles r ON kp.role_id = r.id
            WHERE kp.user_id = $1 AND kp.kb_id = $2
            "#,
        )
        .bind(user_id)
        .bind(kb_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(sqlx_error_to_app_error)?;

        Ok(role_name.flatten())
    }

    /// Check if user has permission for a KB
    pub async fn check_permission(
        &self,
        user_id: i64,
        kb_id: i64,
        required_role: &str,
    ) -> Result<PermissionResult, AppError> {
        // Check if user is system admin
        let is_admin =
            sqlx::query_scalar::<_, bool>("SELECT is_system_admin FROM users WHERE id = $1")
                .bind(user_id)
                .fetch_optional(&self.pool)
                .await
                .map_err(sqlx_error_to_app_error)?
                .unwrap_or(false);

        if is_admin {
            return Ok(PermissionResult::Allowed);
        }

        // Check if user is owner
        let is_owner = sqlx::query_scalar::<_, bool>(
            "SELECT owner_id = $2 FROM knowledge_bases WHERE id = $1",
        )
        .bind(kb_id)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(sqlx_error_to_app_error)?
        .unwrap_or(false);

        if is_owner {
            return Ok(PermissionResult::Allowed);
        }

        // Check user's role
        let role_name = self.get_role(user_id, kb_id).await?;

        match role_name {
            Some(role) => {
                // Role hierarchy: admin > editor > viewer
                let allowed = match required_role {
                    "admin" => role == "admin",
                    "editor" => role == "admin" || role == "editor",
                    "viewer" => role == "admin" || role == "editor" || role == "viewer",
                    _ => false,
                };

                if allowed {
                    Ok(PermissionResult::Allowed)
                } else {
                    Ok(PermissionResult::Denied)
                }
            }
            None => Ok(PermissionResult::Denied),
        }
    }

    /// List members of a KB
    pub async fn list_members(&self, kb_id: i64) -> Result<Vec<KbMember>, AppError> {
        let members = sqlx::query_as::<_, KbMember>(
            r#"
            SELECT u.id as user_id, u.username, u.email, r.name as role_name, kp.created_at as granted_at
            FROM kb_permissions kp
            JOIN users u ON kp.user_id = u.id
            JOIN roles r ON kp.role_id = r.id
            WHERE kp.kb_id = $1 AND u.is_active = true
            ORDER BY kp.created_at DESC
            "#,
        )
        .bind(kb_id)
        .fetch_all(&self.pool)
        .await
        .map_err(sqlx_error_to_app_error)?;

        Ok(members)
    }

    /// Get user's all KB permissions
    pub async fn list_user_permissions(&self, user_id: i64) -> Result<Vec<KbPermission>, AppError> {
        let perms =
            sqlx::query_as::<_, KbPermission>("SELECT * FROM kb_permissions WHERE user_id = $1")
                .bind(user_id)
                .fetch_all(&self.pool)
                .await
                .map_err(sqlx_error_to_app_error)?;

        Ok(perms)
    }
}
