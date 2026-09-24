//! Integration tests for the contents endpoints.
use crate::routes::api_router;
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use shared::{Activity, FileEntry, UpdateFileOption};
use tower::ServiceExt; // for `oneshot`

#[tokio::test]
async fn test_update_file_flow() {
    let app = api_router();

    let payload = UpdateFileOption {
        content: "fn main() {}".to_string(),
        message: "Update main.rs".to_string(),
        sha: "old_sha".to_string(),
        branch: Some("main".to_string()),
    };

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri("/api/v1/repos/admin/codeza/contents/src/main.rs")
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_string(&payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let file: FileEntry = serde_json::from_slice(&body).unwrap();
    assert_eq!(file.path, "src/main.rs");

    // Verify Commit (via Activity or Commit List)
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/user/feeds")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let activities: Vec<Activity> = serde_json::from_slice(&body).unwrap();
    let found = activities
        .iter()
        .any(|a| a.op_type == "update_file" && a.content.contains("updated file src/main.rs"));
    assert!(found);
}
