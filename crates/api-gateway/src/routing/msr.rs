use axum::{extract::State, Json};
use codeza_msr::{Microservice, MicroserviceRepository};
use codeza_shared::error::Result;
use crate::routing::AppState;

/// List microservices
#[utoipa::path(
    get,
    path = "/api/v1/msr/services",
    responses(
        (status = 200, description = "List of microservices", body = Vec<Microservice>)
    ),
    tag = "msr"
)]
pub async fn list_services(
    State(state): State<AppState>,
) -> Result<Json<Vec<Microservice>>> {
    let repo = MicroserviceRepository::new(state.pool);
    let services = repo.list().await?;
    Ok(Json(services))
}

/// Register microservice
#[utoipa::path(
    post,
    path = "/api/v1/msr/services",
    request_body = Microservice,
    responses(
        (status = 201, description = "Service registered", body = Microservice)
    ),
    tag = "msr"
)]
pub async fn register_service(
    State(state): State<AppState>,
    Json(service): Json<Microservice>,
) -> Result<Json<Microservice>> {
    let repo = MicroserviceRepository::new(state.pool);
    let created = repo.create(service).await?;
    Ok(Json(created))
}
