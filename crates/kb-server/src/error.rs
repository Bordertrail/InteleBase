//! Server error handling

use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use kb_core::{AppError, ErrorDetail, ErrorResponse};

/// Error response wrapper - wraps AppError to allow IntoResponse implementation
pub struct ServerError(pub AppError);

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        let status =
            StatusCode::from_u16(self.0.status_code()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        let code = self.0.error_code();

        let body = ErrorResponse {
            error: ErrorDetail {
                code: code.to_string(),
                message: self.0.to_string(),
                details: None,
            },
        };

        (status, Json(body)).into_response()
    }
}

impl From<AppError> for ServerError {
    fn from(err: AppError) -> Self {
        ServerError(err)
    }
}
