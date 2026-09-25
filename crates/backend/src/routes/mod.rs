//! HTTP router construction.
//!
//! Routes are grouped into small, feature-focused sub-routers merged into a single
//! application router. The public API is versioned under `/api/v1`; all responses are
//! JSON unless noted otherwise.

mod actions;
mod admin;
mod issues;
mod misc;
mod orgs;
mod packages;
mod projects;
mod pulls;
mod releases;
mod repos;
mod users;

use axum::Router;
use tower_http::cors::CorsLayer;

use crate::state::AppState;

use self::{
    actions::action_routes, admin::admin_routes, issues::issue_routes, misc::misc_routes,
    orgs::org_routes, packages::package_routes, projects::project_routes, pulls::pull_routes,
    releases::release_routes, repos::repo_routes, users::user_routes,
};

pub fn build_router(state: AppState) -> Router {
    Router::new()
        .merge(repo_routes())
        .merge(issue_routes())
        .merge(pull_routes())
        .merge(release_routes())
        .merge(action_routes())
        .merge(package_routes())
        .merge(project_routes())
        .merge(org_routes())
        .merge(user_routes())
        .merge(admin_routes())
        .merge(misc_routes())
        .layer(CorsLayer::permissive())
        .with_state(state)
}

/// Router backed by a fresh demo state. Test-only convenience used by the
/// integration test suite.
#[cfg(test)]
pub fn api_router() -> Router {
    build_router(crate::seed::demo_state())
}
