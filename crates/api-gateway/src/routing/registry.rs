use crate::routing::AppState;
use axum::{
    Json,
    extract::{Path, State},
};
use codeza_registry::image::Image;
use codeza_shared::error::{CodezaError, Result};

/// List images
#[utoipa::path(
    get,
    path = "/api/v1/registry/images",
    responses(
        (status = 200, description = "List of images", body = Vec<Image>),
        (status = 500, description = "Internal server error")
    ),
    tag = "registry"
)]
pub async fn list_images(State(state): State<AppState>) -> Result<Json<Vec<Image>>> {
    let images = state
        .registry
        .list_images(None)
        .await
        .map_err(CodezaError::InternalError)?;
    Ok(Json(images))
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
        (status = 404, description = "Image not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "registry"
)]
pub async fn get_image(
    State(state): State<AppState>,
    Path((name, tag)): Path<(String, String)>,
) -> Result<Json<Image>> {
    match state.registry.get_image(&name, &tag).await {
        Ok(image) => Ok(Json(image)),
        Err(e) => {
            if e.contains("not found") {
                Err(CodezaError::NotFound(format!("Image {}:{}", name, tag)))
            } else {
                Err(CodezaError::InternalError(e))
            }
        }
    }
}
