use axum::{extract::{State, Path}, Json};
use codeza_registry::image::Image;
use crate::routing::AppState;

/// List images
#[utoipa::path(
    get,
    path = "/api/v1/registry/images",
    responses(
        (status = 200, description = "List of images", body = Vec<Image>)
    ),
    tag = "registry"
)]
pub async fn list_images(
    State(state): State<AppState>,
) -> Json<Vec<Image>> {
    let images = state.registry.list_images(None).await.unwrap_or_default();
    Json(images)
}

/// Get image details
#[utoipa::path(
    get,
    path = "/api/v1/registry/images/{name}/{tag}",
    params(
        ("name" = String, Path, description = "Image name"),
        ("tag" = String, Path, description = "Image tag")
    ),
    responses(
        (status = 200, description = "Image details", body = Image),
        (status = 404, description = "Image not found")
    ),
    tag = "registry"
)]
pub async fn get_image(
    State(state): State<AppState>,
    Path((name, tag)): Path<(String, String)>,
) -> Result<Json<Image>, axum::http::StatusCode> {
    match state.registry.get_image(&name, &tag).await {
        Ok(image) => Ok(Json(image)),
        Err(_) => Err(axum::http::StatusCode::NOT_FOUND),
    }
}
