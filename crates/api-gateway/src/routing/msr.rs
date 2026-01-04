use axum::{extract::State, Json};
use codeza_msr::Microservice;
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
) -> Json<Vec<Microservice>> {
    let services = state.msr.read();
    Json(services.clone())
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
) -> Json<Microservice> {
    let mut services = state.msr.write();
    services.push(service.clone());
    Json(service)
}
