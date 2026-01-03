use super::AppState;

/// Auth register handler
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
pub async fn auth_login(
    axum::extract::State(state): axum::extract::State<AppState>,
    axum::Json(req): axum::Json<codeza_shared::LoginRequest>,
) -> Result<axum::Json<codeza_shared::LoginResponse>, codeza_shared::CodezaError> {
    codeza_auth_service::handlers::login(
        axum::extract::State(state.pool.clone()),
        axum::Json(req),
    )
    .await
}

/// Auth user handler
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
