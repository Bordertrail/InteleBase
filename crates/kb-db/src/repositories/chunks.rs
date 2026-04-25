//! Chunk repository

use sqlx::PgPool;

use kb_core::AppError;
use kb_core::models::{Chunk, ChunkMetadata};

use crate::pool::sqlx_error_to_app_error;

/// Chunk repository for database operations
pub struct ChunkRepository {
    pool: PgPool,
}

impl ChunkRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Find chunk by ID
    pub async fn find_by_id(&self, id: i64) -> Result<Chunk, AppError> {
        let chunk = sqlx::query_as::<_, Chunk>("SELECT * FROM chunks WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(sqlx_error_to_app_error)?
            .ok_or(AppError::ChunkNotFound)?;

        Ok(chunk)
    }

    /// List chunks for a document
    pub async fn list_for_document(&self, document_id: i64) -> Result<Vec<Chunk>, AppError> {
        let chunks = sqlx::query_as::<_, Chunk>(
            "SELECT * FROM chunks WHERE document_id = $1 ORDER BY chunk_index",
        )
        .bind(document_id)
        .fetch_all(&self.pool)
        .await
        .map_err(sqlx_error_to_app_error)?;

        Ok(chunks)
    }

    /// List chunks for a KB
    pub async fn list_for_kb(&self, kb_id: i64, limit: i64) -> Result<Vec<Chunk>, AppError> {
        let chunks = sqlx::query_as::<_, Chunk>(
            "SELECT * FROM chunks WHERE kb_id = $1 ORDER BY created_at DESC LIMIT $2",
        )
        .bind(kb_id)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(sqlx_error_to_app_error)?;

        Ok(chunks)
    }

    /// Count chunks for a document
    pub async fn count_for_document(&self, document_id: i64) -> Result<i64, AppError> {
        let count =
            sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM chunks WHERE document_id = $1")
                .bind(document_id)
                .fetch_one(&self.pool)
                .await
                .map_err(sqlx_error_to_app_error)?;

        Ok(count)
    }
}

/// Create chunk input
pub struct CreateChunk {
    pub document_id: i64,
    pub kb_id: i64,
    pub chunk_index: i32,
    pub content: String,
    pub content_hash: String,
    pub token_count: i32,
    pub metadata: ChunkMetadata,
}
