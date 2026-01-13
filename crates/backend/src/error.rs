// filepath: crates/backend/src/error.rs
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub message: String,
    pub code: String,
}

#[derive(Debug)]
pub enum AppError {
    NotFound(String),
    Conflict(String),
    BadRequest(String),
    Unauthorized(String),
    InternalError(String),
}

impl AppError {
    pub fn message(&self) -> String {
        match self {
            Self::NotFound(msg) => msg.clone(),
            Self::Conflict(msg) => msg.clone(),
            Self::BadRequest(msg) => msg.clone(),
            Self::Unauthorized(msg) => msg.clone(),
            Self::InternalError(msg) => msg.clone(),
        }
    }

    pub fn code(&self) -> String {
        match self {
            Self::NotFound(_) => "NOT_FOUND".to_string(),
            Self::Conflict(_) => "CONFLICT".to_string(),
            Self::BadRequest(_) => "BAD_REQUEST".to_string(),
            Self::Unauthorized(_) => "UNAUTHORIZED".to_string(),
            Self::InternalError(_) => "INTERNAL_ERROR".to_string(),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message, code) = match self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg, "NOT_FOUND"),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, msg, "CONFLICT"),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg, "BAD_REQUEST"),
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg, "UNAUTHORIZED"),
            AppError::InternalError(msg) => {
                (StatusCode::INTERNAL_SERVER_ERROR, msg, "INTERNAL_ERROR")
            }
        };

        let body = Json(ErrorResponse {
            message,
            code: code.to_string(),
        });

        (status, body).into_response()
    }
}
