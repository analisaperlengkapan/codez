use axum::{extract::{State, Path}, Json};
use codeza_orchestrator::{SuperApp, AppModule, SuperAppRepository};
use codeza_mfe_manager::mfe::{MFEManifest, SharedConfig};
use codeza_mfe_manager::MFERepository;
use codeza_shared::error::{CodezaError, Result};
use crate::routing::AppState;
use uuid::Uuid;
use std::collections::HashMap;

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
    let repo = SuperAppRepository::new(state.pool);
    let apps = repo.list().await?;
    Ok(Json(apps))
}

/// Create SuperApp
#[utoipa::path(
    post,
    path = "/api/v1/orchestrator/apps",
    request_body = SuperApp,
    responses(
        (status = 200, description = "SuperApp created", body = SuperApp)
    ),
    tag = "orchestrator"
)]
pub async fn create_superapp(
    State(state): State<AppState>,
    Json(app): Json<SuperApp>,
) -> Result<Json<SuperApp>> {
    let repo = SuperAppRepository::new(state.pool);
    let created = repo.create(app).await?;
    Ok(Json(created))
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
    let repo = SuperAppRepository::new(state.pool);
    let app = repo.get(id).await?;
    match app {
        Some(app) => Ok(Json(app)),
        None => Err(CodezaError::NotFound(format!("SuperApp {}", id))),
    }
}

/// Add module to SuperApp
#[utoipa::path(
    post,
    path = "/api/v1/orchestrator/apps/{id}/modules",
    params(
        ("id" = Uuid, Path, description = "SuperApp ID")
    ),
    request_body = AppModule,
    responses(
        (status = 200, description = "Module added", body = AppModule),
        (status = 404, description = "SuperApp not found")
    ),
    tag = "orchestrator"
)]
pub async fn add_module(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(module): Json<AppModule>,
) -> Result<Json<AppModule>> {
    let repo = SuperAppRepository::new(state.pool);
    // Verify app exists
    if repo.get(id).await?.is_none() {
        return Err(CodezaError::NotFound(format!("SuperApp {}", id)));
    }

    let added = repo.add_module(id, module).await?;
    Ok(Json(added))
}

/// Get SuperApp Manifest (Module Federation)
#[utoipa::path(
    get,
    path = "/api/v1/orchestrator/apps/{id}/manifest",
    params(
        ("id" = Uuid, Path, description = "SuperApp ID")
    ),
    responses(
        (status = 200, description = "SuperApp Manifest", body = MFEManifest),
        (status = 404, description = "SuperApp not found")
    ),
    tag = "orchestrator"
)]
pub async fn get_manifest(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<MFEManifest>> {
    let repo = SuperAppRepository::new(state.pool.clone());
    let app = repo.get(id).await?
        .ok_or_else(|| CodezaError::NotFound(format!("SuperApp {}", id)))?;

    // In a real scenario, we might want to resolve `latest` versions from MFERegistry
    // or validate that the modules exist. For now, we construct the manifest
    // based on the SuperApp configuration which serves as the source of truth
    // for the "integrated" application.

    // Resolve module URLs from MFE Registry to ensure freshness
    let mfe_repo = MFERepository::new(state.pool.clone());

    let mut remotes = HashMap::new();
    for module in app.modules {
        // Try to fetch latest details from registry
        let remote_entry = if let Some(mfe) = mfe_repo.get_mfe_by_name(&module.name).await? {
            // Use registered URL
            mfe.remote_entry
        } else {
            // Fallback to configured URL
            module.remote_entry
        };

        // Format: "name@url" or just "name" -> "url" depending on MF implementation
        // Here we map scope -> remote_entry
        // usually: "scope": "url"
        remotes.insert(module.scope, remote_entry);
    }

    // Use shared dependencies from SuperApp config if available, otherwise use defaults
    let mut shared = app.config.shared_dependencies.clone();

    if shared.is_empty() {
        shared.insert("react".to_string(), SharedConfig {
            singleton: true,
            strict_version: true,
            eager: true,
            required_version: Some("^18.0.0".to_string()),
        });
        shared.insert("react-dom".to_string(), SharedConfig {
            singleton: true,
            strict_version: true,
            eager: true,
            required_version: Some("^18.0.0".to_string()),
        });
    }

    let manifest = MFEManifest {
        name: app.name,
        version: app.version,
        remotes,
        exposes: HashMap::new(), // SuperApp usually doesn't expose, it consumes
        shared,
    };

    Ok(Json(manifest))
}
