//! Error handling for Codeza Platform

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, CodezaError>;

/// Main error type for Codeza Platform
#[derive(Debug, Error)]
pub enum CodezaError {
    #[error("Authentication failed: {0}")]
    AuthenticationError(String),

    #[error("Authorization failed: {0}")]
    AuthorizationError(String),

    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Git operation failed: {0}")]
    GitError(String),

    #[error("Pipeline execution failed: {0}")]
    PipelineError(String),

    #[error("Internal server error: {0}")]
    InternalError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Redis error: {0}")]
    RedisError(String),
}

impl IntoResponse for CodezaError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            CodezaError::AuthenticationError(msg) => (StatusCode::UNAUTHORIZED, msg),
            CodezaError::AuthorizationError(msg) => (StatusCode::FORBIDDEN, msg),
            CodezaError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            CodezaError::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg),
            CodezaError::DatabaseError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database error occurred".to_string(),
            ),
            CodezaError::GitError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Git operation failed".to_string(),
            ),
            CodezaError::PipelineError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Pipeline execution failed".to_string(),
            ),
            CodezaError::InternalError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            CodezaError::ConfigError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Configuration error".to_string(),
            ),
            CodezaError::RedisError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Cache error occurred".to_string(),
            ),
        };

        let body = Json(json!({
            "error": message,
            "status": status.as_u16(),
        }));

        (status, body).into_response()
    }
}
