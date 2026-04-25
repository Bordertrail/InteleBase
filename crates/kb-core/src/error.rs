//! Error types for the application

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Application error enum
#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("User not found")]
    UserNotFound,

    #[error("Knowledge base not found")]
    KbNotFound,

    #[error("Document not found")]
    DocumentNotFound,

    #[error("Chunk not found")]
    ChunkNotFound,

    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("JWT error: {0}")]
    JwtError(String),

    #[error("Password hash error: {0}")]
    PasswordHashError(String),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Forbidden - insufficient permissions")]
    Forbidden,

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Duplicate entry: {0}")]
    DuplicateEntry(String),

    #[error("File too large")]
    FileTooLarge,

    #[error("Invalid file type")]
    InvalidFileType,

    #[error("Search error: {0}")]
    SearchError(String),

    #[error("Vector store error: {0}")]
    VectorStoreError(String),

    #[error("Storage error: {0}")]
    StorageError(String),

    #[error("LLM error: {0}")]
    LlmError(String),

    #[error("Embedding error: {0}")]
    EmbeddingError(String),

    #[error("Queue error: {0}")]
    QueueError(String),

    #[error("Internal server error: {0}")]
    InternalError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),
}

/// Error response format
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: ErrorDetail,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorDetail {
    pub code: String,
    pub message: String,
    pub details: Option<String>,
}

/// Get HTTP status code for error
impl AppError {
    pub fn status_code(&self) -> u16 {
        match self {
            AppError::UserNotFound
            | AppError::KbNotFound
            | AppError::DocumentNotFound
            | AppError::ChunkNotFound => 404,

            AppError::InvalidCredentials | AppError::Unauthorized | AppError::JwtError(_) => 401,

            AppError::Forbidden => 403,

            AppError::ValidationError(_) => 400,

            AppError::DuplicateEntry(_) => 409,

            AppError::FileTooLarge => 413,

            AppError::InvalidFileType => 400,

            AppError::DatabaseError(_) | AppError::InternalError(_) => 500,

            _ => 500,
        }
    }

    pub fn error_code(&self) -> &'static str {
        match self {
            AppError::UserNotFound
            | AppError::KbNotFound
            | AppError::DocumentNotFound
            | AppError::ChunkNotFound => "NOT_FOUND",

            AppError::InvalidCredentials | AppError::Unauthorized | AppError::JwtError(_) => {
                "UNAUTHORIZED"
            }

            AppError::Forbidden => "FORBIDDEN",

            AppError::ValidationError(_) => "VALIDATION_ERROR",

            AppError::DuplicateEntry(_) => "CONFLICT",

            AppError::FileTooLarge => "PAYLOAD_TOO_LARGE",

            AppError::InvalidFileType => "INVALID_FILE_TYPE",

            AppError::DatabaseError(_) | AppError::InternalError(_) => "INTERNAL_ERROR",

            _ => "INTERNAL_ERROR",
        }
    }
}
