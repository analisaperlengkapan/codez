//! Codeza API Gateway
//! Entry point for all API requests

mod rate_limiter;
mod routing;
mod circuit_breaker;
mod openapi;
#[cfg(test)]
mod tests;

use axum::{
    extract::DefaultBodyLimit,
    middleware,
};
use codeza_shared::{config::Config, logging::init_logging};
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use openapi::ApiDoc;

#[tokio::main]
async fn main() {
    // Initialize logging
    init_logging();

    // Load configuration
    let config = Config::default_dev();
    tracing::info!("Starting API Gateway on {}:{}", config.server.host, config.server.port);

    // Setup database connection pool
    let pool = PgPoolOptions::new()
        .max_connections(config.database.max_connections)
        .min_connections(config.database.min_connections)
        .connect(&config.database.url)
        .await
        .expect("Failed to connect to database");

    tracing::info!("Database connected");

    // Initialize Git service
    let provider_config = routing::git::build_git_provider_config(&config)
        .expect("Failed to build git provider config");
    let provider = codeza_git_service::create_git_provider(provider_config);
    let git_service = std::sync::Arc::new(codeza_git_service::RepositoryService::new(provider));

    // Build application state
    let state = routing::AppState {
        pool,
        config: std::sync::Arc::new(config.clone()),
        metrics: codeza_shared::MetricsRegistry::new(),
        git_service,
    };

    // Build router with routes
    let app = routing::build_routes(state.clone())

        .with_state(state)
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(CorsLayer::permissive())
        .layer(DefaultBodyLimit::max(10 * 1024 * 1024)) // 10MB limit
        .layer(middleware::from_fn(codeza_shared::middleware::request_id_middleware))
        .layer(middleware::from_fn(codeza_shared::middleware::logging_middleware));

    // Run server
    let addr = SocketAddr::from((
        config.server.host.parse::<std::net::IpAddr>().unwrap(),
        config.server.port,
    ));

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind to address");

    tracing::info!("API Gateway listening on {}", addr);

    axum::serve(listener, app)
        .await
        .expect("Server error");
}
