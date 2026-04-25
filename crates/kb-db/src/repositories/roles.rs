//! Role repository

use sqlx::PgPool;

use kb_core::AppError;
use kb_core::models::Role;

use crate::pool::sqlx_error_to_app_error;

/// Role repository for database operations
pub struct RoleRepository {
    pool: PgPool,
}

impl RoleRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Find role by ID
    pub async fn find_by_id(&self, id: i64) -> Result<Role, AppError> {
        let role = sqlx::query_as::<_, Role>("SELECT * FROM roles WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(sqlx_error_to_app_error)?
            .ok_or(AppError::KbNotFound)?;

        Ok(role)
    }

    /// Find role by name
    pub async fn find_by_name(&self, name: &str) -> Result<Role, AppError> {
        let role = sqlx::query_as::<_, Role>("SELECT * FROM roles WHERE name = $1")
            .bind(name)
            .fetch_optional(&self.pool)
            .await
            .map_err(sqlx_error_to_app_error)?
            .ok_or(AppError::ValidationError(format!(
                "Role '{}' not found",
                name
            )))?;

        Ok(role)
    }

    /// List all roles
    pub async fn list_all(&self) -> Result<Vec<Role>, AppError> {
        let roles = sqlx::query_as::<_, Role>("SELECT * FROM roles ORDER BY name")
            .fetch_all(&self.pool)
            .await
            .map_err(sqlx_error_to_app_error)?;

        Ok(roles)
    }
}
