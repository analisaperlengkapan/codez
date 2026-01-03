use super::AppState;
use codeza_mfe_manager::MicroFrontend;
use codeza_mfe_manager::MFERegistry;
use axum::{routing::{get, post}, Router, Json};

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
    axum::extract::State(_state): axum::extract::State<AppState>,
) -> Json<Vec<codeza_mfe_manager::MicroFrontend>> {
    // In a real app, we would query the database via MFE manager service.
    // For now, let's just return an empty list or mock data.
    // Since MFERegistry is in memory in the library, and we don't have a shared state for it in AppState yet,
    // we would need to add MFERegistry to AppState or use a DB.
    // Assuming we want to persist them in DB eventually.
    // For this refactor, I will create a dummy registry and return active ones.

    let registry = MFERegistry::new();
    let mfes = registry.list_active();
    let mfes: Vec<codeza_mfe_manager::MicroFrontend> = mfes.into_iter().cloned().collect();

    Json(mfes)
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
    axum::extract::State(_state): axum::extract::State<AppState>,
    Json(mfe): Json<codeza_mfe_manager::MicroFrontend>,
) -> Json<codeza_mfe_manager::MicroFrontend> {
    // Again, this should persist to DB.
    // For now, we just echo it back as "registered".
    Json(mfe)
}
