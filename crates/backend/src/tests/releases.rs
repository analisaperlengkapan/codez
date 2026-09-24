//! Integration tests for the releases endpoints.
use crate::routes::api_router;
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt; // for `oneshot`

#[tokio::test]
async fn test_release_flow() {
    let app = api_router();

    // Create Release
    let payload = shared::CreateReleaseOption {
        tag_name: "v2.0.0".to_string(),
        name: "New Release".to_string(),
        body: Some("Release Body".to_string()),
        draft: false,
        prerelease: false,
    };

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/repos/admin/codeza/releases")
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_string(&payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let release: shared::Release = serde_json::from_slice(&body).unwrap();
    assert_eq!(release.tag_name, "v2.0.0");

    // List Releases
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/repos/admin/codeza/releases")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let releases: Vec<shared::Release> = serde_json::from_slice(&body).unwrap();
    // Should have "Initial Release" (id 1) and "New Release" (id 2)
    assert!(releases.len() >= 2);
    // Sort order is desc by ID
    assert_eq!(releases[0].tag_name, "v2.0.0");
}
