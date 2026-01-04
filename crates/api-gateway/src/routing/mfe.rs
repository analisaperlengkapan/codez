use super::AppState;
use codeza_mfe_manager::MicroFrontend;
use codeza_mfe_manager::MFERepository;
use axum::Json;

/// List all MFEs
#[utoipa::path(
    get,
    path = "/api/v1/mfe",
    responses(
        (status = 200, description = "List of micro frontends", body = Vec<MicroFrontend>),
        (status = 500, description = "Internal server error")
    ),
    tag = "mfe"
)]
pub async fn list_mfes(
    axum::extract::State(state): axum::extract::State<AppState>,
) -> Result<Json<Vec<MicroFrontend>>, codeza_shared::CodezaError> {
    let repo = MFERepository::new(state.pool);
    let mfes = repo.list_active().await?;
    Ok(Json(mfes))
}

/// Register a new MFE
#[utoipa::path(
    post,
    path = "/api/v1/mfe",
    request_body = MicroFrontend,
    responses(
        (status = 200, description = "Micro frontend registered", body = MicroFrontend),
        (status = 500, description = "Internal server error")
    ),
    tag = "mfe"
)]
pub async fn register_mfe(
    axum::extract::State(state): axum::extract::State<AppState>,
    Json(mfe): Json<MicroFrontend>,
) -> Result<Json<MicroFrontend>, codeza_shared::CodezaError> {
    if let Err(e) = mfe.validate() {
        return Err(codeza_shared::CodezaError::ValidationError(e));
    }
    let repo = MFERepository::new(state.pool);
    let saved_mfe = repo.register(mfe).await?;
    Ok(Json(saved_mfe))
}
