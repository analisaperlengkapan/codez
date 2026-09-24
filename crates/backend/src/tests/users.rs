//! Integration tests for the users endpoints.
use crate::routes::api_router;
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use shared::{CreateIssueOption, CreatePullRequestOption, Issue, PullRequest};
use tower::ServiceExt; // for `oneshot`

#[tokio::test]
async fn test_user_dashboard_flow() {
    let app = api_router();

    // 1. Create an issue assigned to current user (admin, id=1)
    // Note: create_issue currently doesn't assign automatically. We need to assign it.
    // Step 1.1 Create issue
    let payload = CreateIssueOption {
        title: "Assigned Task".to_string(),
        body: None,
        milestone: None,
    };
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/repos/admin/codeza/issues")
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_string(&payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let issue: Issue = serde_json::from_slice(&body).unwrap();
    let issue_id = issue.id;

    // Step 1.2 Assign to admin (username "admin")
    let user_payload = shared::User::new(1, "admin".to_string(), None);
    let _ = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/api/v1/repos/admin/codeza/issues/{}/assignees",
                    issue_id
                ))
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_string(&user_payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    // 2. Create a PR by current user (admin is default creator in mock)
    let pr_payload = CreatePullRequestOption {
        title: "My PR".to_string(),
        body: None,
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
                .body(Body::from(serde_json::to_string(&pr_payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    // 3. Verify list_user_issues
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/user/issues?state=open")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let my_issues: Vec<Issue> = serde_json::from_slice(&body).unwrap();
    // Should find "Assigned Task"
    assert!(my_issues.iter().any(|i| i.title == "Assigned Task"));

    // 4. Verify list_user_pulls
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/user/pulls?state=open")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let my_pulls: Vec<PullRequest> = serde_json::from_slice(&body).unwrap();
    // Should find "My PR" (or "First PR" from init)
    assert!(my_pulls.iter().any(|p| p.title == "My PR"));
}
