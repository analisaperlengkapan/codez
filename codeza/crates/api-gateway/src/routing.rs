//! Routing configuration for API Gateway

mod auth;
mod git;
mod cicd;
mod webhook;

use axum::{
    routing::{get, post},
    Router,
};
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub config: codeza_shared::Config,
    pub metrics: codeza_shared::MetricsRegistry,
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

/// Build API routes
pub fn build_routes() -> Router<AppState> {
    Router::new()
        // Health check
        .route("/health", get(health_check))
        .route("/", get(root))
        .route("/metrics", get(metrics))
        
        // Authentication routes
        .route("/auth/register", post(auth::auth_register))
        .route("/auth/login", post(auth::auth_login))
        .route("/auth/user", get(auth::auth_user))
        
        // Git repository routes
        .route("/api/v1/repos", post(git::create_repository))
        .route("/api/v1/repos/:owner", get(git::list_repositories))
        .route(
            "/api/v1/repos/:owner/:repo",
            get(git::get_repository).delete(git::delete_repository),
        )
        
        // Pipeline execution routes
        .route("/api/v1/pipelines", get(cicd::list_pipelines))
        .route("/api/v1/pipelines/:id", get(cicd::get_pipeline_execution))
        .route("/api/v1/pipelines/:id/jobs", get(cicd::list_pipeline_jobs))
        
        // Git webhook route
        .route("/api/v1/git/webhook", post(webhook::git_webhook))
}
