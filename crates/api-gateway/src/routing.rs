//! Routing configuration for API Gateway

use std::sync::Arc;

pub mod auth;
pub mod cicd;
pub mod git;
pub mod mfe;
pub mod msr;
pub mod orchestrator;
pub mod registry;
pub mod webhook;

use axum::{
    Router,
    extract::FromRef,
    routing::{get, post},
};
use codeza_git_service::RepositoryService;
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub config: Arc<codeza_shared::Config>,
    pub metrics: codeza_shared::MetricsRegistry,
    pub git_service: Arc<RepositoryService>,
    pub registry: Arc<dyn codeza_registry::push_pull::ImageStorage>,
}

impl FromRef<AppState> for codeza_shared::Config {
    fn from_ref(state: &AppState) -> Self {
        state.config.as_ref().clone()
    }
}

/// Health check handler
async fn health_check() -> (axum::http::StatusCode, String) {
    (axum::http::StatusCode::OK, "OK".to_string())
}

/// Root handler
async fn root() -> (axum::http::StatusCode, String) {
    (
        axum::http::StatusCode::OK,
        "Codeza API Gateway v0.1.0".to_string(),
    )
}

/// Metrics handler
async fn get_metrics(
    axum::extract::State(state): axum::extract::State<AppState>,
) -> axum::Json<Vec<codeza_shared::metrics::MetricValue>> {
    axum::Json(state.metrics.collect())
}

/// Build API routes
pub fn build_routes(state: AppState) -> Router<AppState> {
    let public_routes = Router::new()
        // Health check
        .route("/health", get(health_check))
        .route("/", get(root))
        .route("/metrics", get(get_metrics))
        // Public auth routes
        .route("/auth/register", post(auth::auth_register))
        .route("/auth/login", post(auth::auth_login))
        // Git webhook route (usually public but protected by secret signature)
        .route("/api/v1/git/webhook", post(webhook::git_webhook));

    let protected_routes = Router::new()
        .route("/auth/user", get(auth::auth_user))
        // Git repository routes
        .route("/api/v1/repos", post(git::create_repository))
        .route("/api/v1/repos/{owner}", get(git::list_repositories))
        .route(
            "/api/v1/repos/{owner}/{repo}",
            get(git::get_repository).delete(git::delete_repository),
        )
        // Pipeline execution routes
        .route("/api/v1/pipelines", get(cicd::list_pipelines))
        .route("/api/v1/pipelines/{id}", get(cicd::get_pipeline_execution))
        .route("/api/v1/pipelines/{id}/jobs", get(cicd::list_pipeline_jobs))
        // MFE routes
        .route("/api/v1/mfe", get(mfe::list_mfes).post(mfe::register_mfe))
        // Registry routes
        .route("/api/v1/registry/images", get(registry::list_images))
        .route(
            "/api/v1/registry/images/{name}/{tag}",
            get(registry::get_image),
        )
        // MSR routes
        .route(
            "/api/v1/msr/services",
            get(msr::list_services).post(msr::register_service),
        )
        // Orchestrator routes
        .route(
            "/api/v1/orchestrator/apps",
            get(orchestrator::list_superapps).post(orchestrator::create_superapp),
        )
        .route(
            "/api/v1/orchestrator/apps/{id}",
            get(orchestrator::get_superapp),
        )
        .route(
            "/api/v1/orchestrator/apps/{id}/modules",
            post(orchestrator::add_module),
        )
        .route(
            "/api/v1/orchestrator/apps/{id}/manifest",
            get(orchestrator::get_manifest),
        )
        .route_layer(axum::middleware::from_fn_with_state(
            state,
            codeza_shared::middleware::auth_middleware,
        ));

    Router::new().merge(public_routes).merge(protected_routes)
}
