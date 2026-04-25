//! Document repository

use sqlx::PgPool;

use kb_core::AppError;
use kb_core::models::{CreateDocument, Document, DocumentStatus};

use crate::pool::sqlx_error_to_app_error;

/// Document repository for database operations
pub struct DocumentRepository {
    pool: PgPool,
}

impl DocumentRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Create a new document
    pub async fn create(&self, input: CreateDocument) -> Result<Document, AppError> {
        let doc = sqlx::query_as::<_, Document>(
            r#"
            INSERT INTO documents (kb_id, title, filename, file_type, file_size, storage_path, created_by)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING *
            "#,
        )
        .bind(&input.kb_id)
        .bind(&input.title)
        .bind(&input.filename)
        .bind(&input.file_type)
        .bind(&input.file_size)
        .bind(&input.storage_path)
        .bind(&input.created_by)
        .fetch_one(&self.pool)
        .await
        .map_err(sqlx_error_to_app_error)?;

        Ok(doc)
    }

    /// Find document by ID
    pub async fn find_by_id(&self, id: i64) -> Result<Document, AppError> {
        let doc = sqlx::query_as::<_, Document>(
            "SELECT * FROM documents WHERE id = $1 AND deleted_at IS NULL",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(sqlx_error_to_app_error)?
        .ok_or(AppError::DocumentNotFound)?;

        Ok(doc)
    }

    /// List documents for a KB
    pub async fn list_for_kb(
        &self,
        kb_id: i64,
        page: i64,
        per_page: i64,
        status: Option<&str>,
    ) -> Result<Vec<Document>, AppError> {
        let offset = (page - 1) * per_page;

        let docs = if let Some(s) = status {
            sqlx::query_as::<_, Document>(
                r#"
                SELECT * FROM documents
                WHERE kb_id = $1 AND deleted_at IS NULL AND status = $2
                ORDER BY created_at DESC
                LIMIT $3 OFFSET $4
                "#,
            )
            .bind(kb_id)
            .bind(s)
            .bind(per_page)
            .bind(offset)
            .fetch_all(&self.pool)
            .await
            .map_err(sqlx_error_to_app_error)?
        } else {
            sqlx::query_as::<_, Document>(
                r#"
                SELECT * FROM documents
                WHERE kb_id = $1 AND deleted_at IS NULL
                ORDER BY created_at DESC
                LIMIT $2 OFFSET $3
                "#,
            )
            .bind(kb_id)
            .bind(per_page)
            .bind(offset)
            .fetch_all(&self.pool)
            .await
            .map_err(sqlx_error_to_app_error)?
        };

        Ok(docs)
    }

    /// Count documents for a KB
    pub async fn count_for_kb(&self, kb_id: i64) -> Result<i64, AppError> {
        let count = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM documents WHERE kb_id = $1 AND deleted_at IS NULL",
        )
        .bind(kb_id)
        .fetch_one(&self.pool)
        .await
        .map_err(sqlx_error_to_app_error)?;

        Ok(count)
    }

    /// Update document status
    pub async fn update_status(
        &self,
        id: i64,
        status: DocumentStatus,
        error_message: Option<&str>,
    ) -> Result<(), AppError> {
        sqlx::query(
            r#"
            UPDATE documents
            SET status = $2, error_message = $3
            WHERE id = $1
            "#,
        )
        .bind(id)
        .bind(status.as_str())
        .bind(error_message)
        .execute(&self.pool)
        .await
        .map_err(sqlx_error_to_app_error)?;

        Ok(())
    }

    /// Update chunk count
    pub async fn update_chunk_count(&self, id: i64, count: i32) -> Result<(), AppError> {
        sqlx::query("UPDATE documents SET chunk_count = $2, status = 'ready' WHERE id = $1")
            .bind(id)
            .bind(count)
            .execute(&self.pool)
            .await
            .map_err(sqlx_error_to_app_error)?;

        Ok(())
    }

    /// Delete document (soft delete)
    pub async fn delete(&self, id: i64) -> Result<(), AppError> {
        let result = sqlx::query(
            "UPDATE documents SET deleted_at = NOW() WHERE id = $1 AND deleted_at IS NULL",
        )
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(sqlx_error_to_app_error)?;

        if result.rows_affected() == 0 {
            return Err(AppError::DocumentNotFound);
        }

        Ok(())
    }
}
