//! Integration tests for the pulls endpoints.
use crate::routes::api_router;
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use shared::{CreatePullRequestOption, PullRequest, UpdatePullRequestOption};
use tower::ServiceExt; // for `oneshot`

#[tokio::test]
async fn test_update_pull_request_flow() {
    let app = api_router();

    // Create a PR first
    let payload = CreatePullRequestOption {
        title: "Test PR".to_string(),
        body: Some("Body".to_string()),
        head: "feature".to_string(),
        base: "main".to_string(),
    };
    let _ = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/repos/admin/codeza/pulls")
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_string(&payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Update the PR
    let update_payload = UpdatePullRequestOption {
        title: Some("Updated PR Title".to_string()),
        body: None,
        state: Some("closed".to_string()),
    };
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PATCH")
                .uri("/api/v1/repos/admin/codeza/pulls/1") // PR created gets ID 2 because ID 1 is in mock init? No, init has 1. Created will be 2.
                // Wait, init state has PR 1. So we can just update PR 1.
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_string(&update_payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let pr: Option<PullRequest> = serde_json::from_slice(&body).unwrap();
    let pr = pr.unwrap();
    assert_eq!(pr.title, "Updated PR Title");
    assert_eq!(pr.state, "closed");
}
#[tokio::test]
async fn test_pull_request_review_flow() {
    let app = api_router();

    // Create PR first
    let payload = CreatePullRequestOption {
        title: "PR for Review".to_string(),
        body: None,
        head: "feature".to_string(),
        base: "main".to_string(),
    };
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/repos/admin/codeza/pulls")
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_string(&payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let pr: PullRequest = serde_json::from_slice(&body).unwrap();
    let pr_number = pr.number;

    // Submit Review
    let review_payload = shared::CreateReviewOption {
        body: "Looks good to me".to_string(),
        event: "APPROVE".to_string(),
    };
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/api/v1/repos/admin/codeza/pulls/{}/reviews",
                    pr_number
                ))
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_string(&review_payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let review: shared::Review = serde_json::from_slice(&body).unwrap();
    assert_eq!(review.state, "APPROVED");
    assert_eq!(review.body, "Looks good to me");

    // List Reviews
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!(
                    "/api/v1/repos/admin/codeza/pulls/{}/reviews",
                    pr_number
                ))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let reviews: Vec<shared::Review> = serde_json::from_slice(&body).unwrap();
    assert_eq!(reviews.len(), 1);
    assert_eq!(reviews[0].state, "APPROVED");
}
