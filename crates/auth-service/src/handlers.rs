//! HTTP handlers for authentication endpoints

use axum::{
    extract::{Extension, State},
    http::StatusCode,
    Json,
};
use codeza_shared::{
    error::Result,
    models::{LoginRequest, LoginResponse, RegisterRequest, UserResponse},
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::UserService;

/// Register a new user
pub async fn register(
    State(pool): State<PgPool>,
    Json(req): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<UserResponse>)> {
    let service = UserService::new(pool);
    let user = service.create_user(req).await?;
    Ok((StatusCode::CREATED, Json(user)))
}

/// Login user
pub async fn login(
    State(pool): State<PgPool>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<LoginResponse>> {
    let service = UserService::new(pool);
    let user = service.verify_credentials(&req.username, &req.password).await?;
    let roles = service.get_user_roles(user.id).await?;

    let token = codeza_shared::generate_token(
        user.id,
        user.username.clone(),
        user.email.clone(),
        roles,
        "dev-secret-key", // TODO: Get from config
        24,
    )?;

    Ok(Json(LoginResponse {
        user: user.into(),
        token,
        refresh_token: None,
        expires_in: 24 * 3600,
    }))
}

/// Get current user
pub async fn get_current_user(
    Extension(user_id): Extension<Uuid>,
    State(pool): State<PgPool>,
) -> Result<Json<UserResponse>> {
    let service = UserService::new(pool);
    let user = service.get_user_by_id(user_id).await?;
    Ok(Json(user.into()))
}
