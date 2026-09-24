//! Integration tests for the packages endpoints.
use crate::routes::api_router;
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt; // for `oneshot`

#[tokio::test]
async fn test_package_flow() {
    let app = api_router();

    // Upload Package (mock)
    let payload = shared::CreatePackageOption {
        name: "test-pkg".to_string(),
        version: "0.1.0".to_string(),
        package_type: "npm".to_string(),
    };
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/packages/admin")
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_string(&payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    // List Packages
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/packages/admin")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let packages: Vec<shared::Package> = serde_json::from_slice(&body).unwrap();
    // Init state has 1 package. So now 2.
    assert_eq!(packages.len(), 2);
}
