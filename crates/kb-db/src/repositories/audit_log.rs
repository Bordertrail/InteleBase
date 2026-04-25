//! Audit log repository

use sqlx::PgPool;

use kb_core::AppError;
use kb_core::models::{AuditLog, CreateAuditLog};

use crate::pool::sqlx_error_to_app_error;

/// Audit log repository for database operations
pub struct AuditLogRepository {
    pool: PgPool,
}

impl AuditLogRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Create audit log entry
    pub async fn create(&self, input: CreateAuditLog) -> Result<AuditLog, AppError> {
        let log = sqlx::query_as::<_, AuditLog>(
            r#"
            INSERT INTO audit_log (user_id, kb_id, action, resource_type, resource_id, details, ip_address, user_agent)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING *
            "#,
        )
        .bind(&input.user_id)
        .bind(&input.kb_id)
        .bind(input.action.as_str())
        .bind(&input.resource_type)
        .bind(&input.resource_id)
        .bind(&input.details)
        .bind(&input.ip_address)
        .bind(&input.user_agent)
        .fetch_one(&self.pool)
        .await
        .map_err(sqlx_error_to_app_error)?;

        Ok(log)
    }

    /// List audit logs with filters
    pub async fn list(
        &self,
        user_id: Option<i64>,
        kb_id: Option<i64>,
        action: Option<&str>,
        from: Option<chrono::DateTime<chrono::Utc>>,
        to: Option<chrono::DateTime<chrono::Utc>>,
        page: i64,
        per_page: i64,
    ) -> Result<Vec<AuditLog>, AppError> {
        let offset = (page - 1) * per_page;

        let logs = sqlx::query_as::<_, AuditLog>(
            r#"
            SELECT * FROM audit_log
            WHERE ($1::bigint IS NULL OR user_id = $1)
            AND ($2::bigint IS NULL OR kb_id = $2)
            AND ($3::varchar IS NULL OR action = $3)
            AND ($4::timestamptz IS NULL OR created_at >= $4)
            AND ($5::timestamptz IS NULL OR created_at <= $5)
            ORDER BY created_at DESC
            LIMIT $6 OFFSET $7
            "#,
        )
        .bind(user_id)
        .bind(kb_id)
        .bind(action)
        .bind(from)
        .bind(to)
        .bind(per_page)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(sqlx_error_to_app_error)?;

        Ok(logs)
    }

    /// Count audit logs
    pub async fn count(
        &self,
        user_id: Option<i64>,
        kb_id: Option<i64>,
        action: Option<&str>,
        from: Option<chrono::DateTime<chrono::Utc>>,
        to: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<i64, AppError> {
        let count = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(*) FROM audit_log
            WHERE ($1::bigint IS NULL OR user_id = $1)
            AND ($2::bigint IS NULL OR kb_id = $2)
            AND ($3::varchar IS NULL OR action = $3)
            AND ($4::timestamptz IS NULL OR created_at >= $4)
            AND ($5::timestamptz IS NULL OR created_at <= $5)
            "#,
        )
        .bind(user_id)
        .bind(kb_id)
        .bind(action)
        .bind(from)
        .bind(to)
        .fetch_one(&self.pool)
        .await
        .map_err(sqlx_error_to_app_error)?;

        Ok(count)
    }
}
