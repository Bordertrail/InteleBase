//! Knowledge Base repository

use sqlx::PgPool;

use kb_core::AppError;
use kb_core::models::{CreateKnowledgeBase, KnowledgeBase, UpdateKnowledgeBase};

use crate::pool::sqlx_error_to_app_error;

/// Knowledge Base repository for database operations
pub struct KnowledgeBaseRepository {
    pool: PgPool,
}

impl KnowledgeBaseRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Create a new knowledge base
    pub async fn create(
        &self,
        input: CreateKnowledgeBase,
        owner_id: i64,
    ) -> Result<KnowledgeBase, AppError> {
        let settings = serde_json::to_value(input.settings.unwrap_or_default())
            .map_err(|e| AppError::InternalError(e.to_string()))?;

        let kb = sqlx::query_as::<_, KnowledgeBase>(
            r#"
            INSERT INTO knowledge_bases (name, description, owner_id, settings)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#,
        )
        .bind(&input.name)
        .bind(&input.description)
        .bind(owner_id)
        .bind(&settings)
        .fetch_one(&self.pool)
        .await
        .map_err(sqlx_error_to_app_error)?;

        Ok(kb)
    }

    /// Find knowledge base by ID
    pub async fn find_by_id(&self, id: i64) -> Result<KnowledgeBase, AppError> {
        let kb = sqlx::query_as::<_, KnowledgeBase>(
            "SELECT * FROM knowledge_bases WHERE id = $1 AND archived_at IS NULL",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(sqlx_error_to_app_error)?
        .ok_or(AppError::KbNotFound)?;

        Ok(kb)
    }

    /// List knowledge bases for a user (owned or has permissions)
    pub async fn list_for_user(
        &self,
        user_id: i64,
        page: i64,
        per_page: i64,
    ) -> Result<Vec<KnowledgeBase>, AppError> {
        let offset = (page - 1) * per_page;

        let kbs = sqlx::query_as::<_, KnowledgeBase>(
            r#"
            SELECT DISTINCT kb.* FROM knowledge_bases kb
            LEFT JOIN kb_permissions kp ON kb.id = kp.kb_id
            WHERE kb.archived_at IS NULL
            AND (kb.owner_id = $1 OR kp.user_id = $1)
            ORDER BY kb.created_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(user_id)
        .bind(per_page)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(sqlx_error_to_app_error)?;

        Ok(kbs)
    }

    /// Count knowledge bases for a user
    pub async fn count_for_user(&self, user_id: i64) -> Result<i64, AppError> {
        let count = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(DISTINCT kb.id) FROM knowledge_bases kb
            LEFT JOIN kb_permissions kp ON kb.id = kp.kb_id
            WHERE kb.archived_at IS NULL
            AND (kb.owner_id = $1 OR kp.user_id = $1)
            "#,
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await
        .map_err(sqlx_error_to_app_error)?;

        Ok(count)
    }

    /// Update knowledge base
    pub async fn update(
        &self,
        id: i64,
        input: UpdateKnowledgeBase,
    ) -> Result<KnowledgeBase, AppError> {
        let kb = sqlx::query_as::<_, KnowledgeBase>(
            r#"
            UPDATE knowledge_bases
            SET name = COALESCE($2, name),
                description = COALESCE($3, description)
            WHERE id = $1 AND archived_at IS NULL
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(&input.name)
        .bind(&input.description)
        .fetch_optional(&self.pool)
        .await
        .map_err(sqlx_error_to_app_error)?
        .ok_or(AppError::KbNotFound)?;

        Ok(kb)
    }

    /// Archive (soft delete) knowledge base
    pub async fn archive(&self, id: i64) -> Result<(), AppError> {
        let result = sqlx::query(
            "UPDATE knowledge_bases SET archived_at = NOW() WHERE id = $1 AND archived_at IS NULL",
        )
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(sqlx_error_to_app_error)?;

        if result.rows_affected() == 0 {
            return Err(AppError::KbNotFound);
        }

        Ok(())
    }

    /// Check if user is owner
    pub async fn is_owner(&self, kb_id: i64, user_id: i64) -> Result<bool, AppError> {
        let is_owner = sqlx::query_scalar::<_, bool>(
            "SELECT owner_id = $2 FROM knowledge_bases WHERE id = $1",
        )
        .bind(kb_id)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(sqlx_error_to_app_error)?
        .unwrap_or(false);

        Ok(is_owner)
    }
}
