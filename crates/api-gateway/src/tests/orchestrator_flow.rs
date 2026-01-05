use crate::routing::{AppState, build_routes};
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use codeza_shared::Config;
use sqlx::postgres::PgPoolOptions;
use tower::ServiceExt;

#[tokio::test]
async fn test_orchestrator_routes_wiring() {
    // Setup config
    let config = Config::default_dev();

    // Setup lazy pool (won't connect until used)
    let pool = PgPoolOptions::new()
        .connect_lazy("postgres://user:pass@localhost:5432/db")
        .expect("Failed to create lazy pool");

    // Create mock git service
    let provider_config = codeza_git_service::ProviderConfig::new(
        codeza_git_service::ProviderType::Gitea,
        "http://localhost:3000".to_string(),
        "token".to_string(),
    );
    let provider = codeza_git_service::create_git_provider(provider_config);
    let git_service = std::sync::Arc::new(codeza_git_service::RepositoryService::new(provider));

    let state = AppState {
        pool,
        config: std::sync::Arc::new(config),
        metrics: codeza_shared::MetricsRegistry::new(),
        git_service,
        registry: std::sync::Arc::new(codeza_registry::push_pull::LocalImageStorage::new()),
    };

    let app = build_routes(state.clone()).with_state(state);

    // Test: Get Manifest (should be protected)
    let superapp_id = uuid::Uuid::new_v4();
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(&format!(
                    "/api/v1/orchestrator/apps/{}/manifest",
                    superapp_id
                ))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Expect 401 Unauthorized because we didn't provide a token.
    // This confirms the route is wired up correctly in the router.
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    // Test: Create SuperApp (should be protected)
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/orchestrator/apps")
                .method("POST")
                .header("Content-Type", "application/json")
                .body(Body::from("{}")) // Invalid body but auth check happens first
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}
