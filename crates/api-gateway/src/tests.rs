
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;
use codeza_shared::Config;
use sqlx::postgres::PgPoolOptions;
use crate::routing::{AppState, build_routes};

#[tokio::test]
async fn test_auth_middleware_integration_401() {
    // Setup config
    let config = Config::default_dev();

    // Setup lazy pool (won't connect until used)
    let pool = PgPoolOptions::new()
        .connect_lazy("postgres://user:pass@localhost:5432/db")
        .expect("Failed to create lazy pool");

    let state = AppState {
        pool,
        config: std::sync::Arc::new(config),
        metrics: codeza_shared::MetricsRegistry::new(),
    };

    let app = build_routes(state.clone()).with_state(state);

    // Test 1: No token
    let response = app.clone()
        .oneshot(Request::builder().uri("/auth/user").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    // Test 2: Invalid token
    let response = app
        .oneshot(Request::builder()
            .uri("/auth/user")
            .header("Authorization", "Bearer invalid_token")
            .body(Body::empty())
            .unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}
