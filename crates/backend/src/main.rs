//! Codeza backend entry point.
//!
//! Serves the `/api/v1` JSON API and, when a frontend build is present, the
//! compiled WebAssembly single-page application from the configured static
//! directory (defaults to `crates/frontend/dist`).

use std::net::SocketAddr;

use axum::http::{header, StatusCode, Uri};
use axum::response::{IntoResponse, Response};

mod config;
mod db;
mod handlers;
mod routes;
mod seed;
mod state;
mod tests;

/// Content type for the static asset extensions the frontend emits.
fn content_type(path: &str) -> &'static str {
    match path.rsplit('.').next().unwrap_or("") {
        "html" => "text/html; charset=utf-8",
        "js" => "text/javascript; charset=utf-8",
        "css" => "text/css; charset=utf-8",
        "wasm" => "application/wasm",
        "json" | "map" => "application/json",
        "svg" => "image/svg+xml",
        "png" => "image/png",
        "ico" => "image/x-icon",
        "txt" => "text/plain; charset=utf-8",
        _ => "application/octet-stream",
    }
}

/// Fallback for paths not matched by the API.
///
/// Serves the requested file from the static directory when it exists. The
/// frontend is a client-side-routed SPA, so any other path returns the shell
/// document with a `200` so the browser can resolve deep links. Unknown
/// `/api/*` paths stay `404` so clients never mistake HTML for JSON.
async fn spa_fallback(static_dir: String, uri: Uri) -> Response {
    if uri.path().starts_with("/api/") {
        return (StatusCode::NOT_FOUND, "Not Found").into_response();
    }

    let relative = uri.path().trim_start_matches('/');
    if !relative.is_empty() && !relative.split('/').any(|s| s == "..") {
        let file = format!("{static_dir}/{relative}");
        if let Ok(bytes) = tokio::fs::read(&file).await {
            return ([(header::CONTENT_TYPE, content_type(relative))], bytes).into_response();
        }
    }

    match tokio::fs::read(format!("{static_dir}/index.html")).await {
        Ok(index) => ([(header::CONTENT_TYPE, "text/html; charset=utf-8")], index).into_response(),
        Err(_) => (
            StatusCode::SERVICE_UNAVAILABLE,
            "Frontend assets are not built. Run `trunk build` in crates/frontend.",
        )
            .into_response(),
    }
}

#[tokio::main]
async fn main() {
    let config = config::Config::from_env();
    let state = seed::demo_state();
    let api_router = routes::build_router(state.clone());

    let fallback_dir = config.static_dir.clone();
    let app = api_router.fallback(move |uri: Uri| spa_fallback(fallback_dir.clone(), uri));

    if config.database_url.is_some() {
        tokio::spawn(async move {
            db::init_postgres_db(&state).await;
        });
    }

    let addr = SocketAddr::from((config.host, config.port));
    println!("Codeza listening on http://{addr}");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
