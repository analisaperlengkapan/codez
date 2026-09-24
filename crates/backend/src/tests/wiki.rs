//! Integration tests for the wiki endpoints.
use crate::routes::api_router;
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt; // for `oneshot`

#[tokio::test]
async fn test_wiki_flow() {
    let app = api_router();

    // Create Wiki Page
    let payload = shared::CreateWikiPageOption {
        title: "Docs".to_string(),
        content: "Documentation content".to_string(),
        message: Some("Init docs".to_string()),
    };

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/repos/admin/codeza/wiki/pages")
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_string(&payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    // List Wiki Pages
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/repos/admin/codeza/wiki/pages")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    // Note: Logic in handler uses mock static list, so we just verify API contract here.
}
