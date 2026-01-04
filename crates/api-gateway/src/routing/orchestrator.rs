use axum::{extract::{State, Path}, Json};
use codeza_orchestrator::SuperApp;
use codeza_shared::error::{CodezaError, Result};
use crate::routing::AppState;
use uuid::Uuid;

/// List SuperApps
#[utoipa::path(
    get,
    path = "/api/v1/orchestrator/apps",
    responses(
        (status = 200, description = "List of SuperApps", body = Vec<SuperApp>)
    ),
    tag = "orchestrator"
)]
pub async fn list_superapps(
    State(state): State<AppState>,
) -> Result<Json<Vec<SuperApp>>> {
    let apps = state.orchestrator.read();
    Ok(Json(apps.clone()))
}

/// Get SuperApp details
#[utoipa::path(
    get,
    path = "/api/v1/orchestrator/apps/{id}",
    params(
        ("id" = Uuid, Path, description = "SuperApp ID")
    ),
    responses(
        (status = 200, description = "SuperApp details", body = SuperApp),
        (status = 404, description = "SuperApp not found")
    ),
    tag = "orchestrator"
)]
pub async fn get_superapp(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<SuperApp>> {
    let apps = state.orchestrator.read();
    match apps.iter().find(|app| app.id == id) {
        Some(app) => Ok(Json(app.clone())),
        None => Err(CodezaError::NotFound(format!("SuperApp {}", id))),
    }
}
