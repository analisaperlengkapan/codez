use super::AppState;
use codeza_shared::{RegisterRequest, UserResponse, LoginRequest, LoginResponse};

/// Auth register handler
#[utoipa::path(
    post,
    path = "/auth/register",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "User registered successfully", body = UserResponse),
        (status = 400, description = "Invalid input"),
        (status = 500, description = "Internal server error")
    ),
    tag = "auth"
)]
pub async fn auth_register(
    axum::extract::State(state): axum::extract::State<AppState>,
    axum::Json(req): axum::Json<codeza_shared::RegisterRequest>,
) -> Result<
    (axum::http::StatusCode, axum::Json<codeza_shared::UserResponse>),
    codeza_shared::CodezaError,
> {
    codeza_auth_service::handlers::register(
        axum::extract::State(state.pool.clone()),
        axum::Json(req),
    )
    .await
}

/// Auth login handler
#[utoipa::path(
    post,
    path = "/auth/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = LoginResponse),
        (status = 401, description = "Invalid credentials"),
        (status = 500, description = "Internal server error")
    ),
    tag = "auth"
)]
pub async fn auth_login(
    axum::extract::State(state): axum::extract::State<AppState>,
    axum::Json(req): axum::Json<codeza_shared::LoginRequest>,
) -> Result<axum::Json<codeza_shared::LoginResponse>, codeza_shared::CodezaError> {
    codeza_auth_service::handlers::login(
        axum::extract::State(state.pool.clone()),
        axum::extract::State(state.config.clone()),
        axum::Json(req),
    )
    .await
}

/// Auth user handler
#[utoipa::path(
    get,
    path = "/auth/user",
    responses(
        (status = 200, description = "Current user details", body = UserResponse),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("jwt" = [])
    ),
    tag = "auth"
)]
pub async fn auth_user(
    axum::extract::State(state): axum::extract::State<AppState>,
    axum::extract::Extension(user_id): axum::extract::Extension<uuid::Uuid>,
) -> Result<axum::Json<codeza_shared::UserResponse>, codeza_shared::CodezaError> {
    codeza_auth_service::handlers::get_current_user(
        axum::extract::Extension(user_id),
        axum::extract::State(state.pool.clone()),
    )
    .await
}
