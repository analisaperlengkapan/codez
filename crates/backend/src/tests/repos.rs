//! Integration tests for the repos endpoints.
use crate::routes::api_router;
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use shared::{
    Activity, CreateIssueOption, CreateProtectedBranchOption, CreatePullRequestOption,
    CreateRepoOption, CreateStatusOption, MergePullRequestOption, PullRequest, Repository,
    UpdateIssueOption,
};
use tower::ServiceExt; // for `oneshot`

#[tokio::test]
async fn test_create_repo_flow() {
    let app = api_router();

    let payload = CreateRepoOption {
        name: "test-repo".to_string(),
        description: None,
        private: false,
        auto_init: true,
        gitignores: None,
        license: None,
        readme: None,
        default_branch: None,
        allow_rebase_merge: None,
        allow_squash_merge: None,
        allow_merge_commit: None,
        has_issues: None,
        has_wiki: None,
        has_projects: None,
    };

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/user/repos")
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
    let repo: Repository = serde_json::from_slice(&body).unwrap();
    assert_eq!(repo.name, "test-repo");

    // Verify Activity Log
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

    assert_eq!(response.status(), StatusCode::OK);
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let activities: Vec<Activity> = serde_json::from_slice(&body).unwrap();

    let found = activities
        .iter()
        .any(|a| a.content.contains("created repository test-repo"));
    assert!(found, "Should find creation activity in feed");
}
#[tokio::test]
async fn test_star_repo_flow() {
    let app = api_router();

    // 1. Star a repo
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/repos/admin/codeza/star")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    // 2. Check User Status
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/repos/admin/codeza/user_status")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let status: shared::RepoUserStatus = serde_json::from_slice(&body).unwrap();
    assert!(status.starred);

    // 3. List Starred Repos
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/user/starred")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let repos: Vec<Repository> = serde_json::from_slice(&body).unwrap();
    assert!(repos.iter().any(|r| r.name == "codeza"));

    // 4. Unstar
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/repos/admin/codeza/star")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    // 5. Verify Unstarred
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/repos/admin/codeza/user_status")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let status: shared::RepoUserStatus = serde_json::from_slice(&body).unwrap();
    assert!(!status.starred);
}
#[tokio::test]
async fn test_fork_repo_flow() {
    let app = api_router();

    // 1. Fork 'codeza'
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/repos/admin/codeza/fork")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let forked_repo: Repository = serde_json::from_slice(&body).unwrap();

    assert_eq!(forked_repo.name, "codeza-fork");
    // Verify parent_id is set (assuming codeza id is 1)
    assert_eq!(forked_repo.parent_id, Some(1));

    // 2. Verify files copied
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!(
                    "/api/v1/repos/{}/{}/raw/src/main.rs",
                    forked_repo.owner, forked_repo.name
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
    let content = String::from_utf8(body.to_vec()).unwrap();
    assert!(content.contains("Welcome to codeza"));
}
#[tokio::test]
async fn test_commit_status_protection_flow() {
    let app = api_router();

    // 1. Create Protected Branch
    let pb_payload = CreateProtectedBranchOption {
        name: "main".to_string(),
        enable_push: false,
        enable_force_push: false,
        required_status_checks: Some(vec!["ci/test".to_string()]),
    };
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/repos/admin/codeza/branch_protections")
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_string(&pb_payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    // 2. Create PR (target: main)
    let pr_payload = CreatePullRequestOption {
        title: "Protected PR".to_string(),
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
                .body(Body::from(serde_json::to_string(&pr_payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let pr: PullRequest = serde_json::from_slice(&body).unwrap();
    let pr_number = pr.number;
    let head_sha = pr.head_sha;

    // 3. Attempt Merge (Should Fail due to missing status check)
    let merge_payload = MergePullRequestOption {
        merge_action: "merge".to_string(),
        merge_title_field: None,
        merge_message_field: None,
    };
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/api/v1/repos/admin/codeza/pulls/{}/merge",
                    pr_number
                ))
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_string(&merge_payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CONFLICT); // 409 Conflict

    // 4. Create Success Status
    let status_payload = CreateStatusOption {
        state: "success".to_string(),
        target_url: None,
        description: Some("Tests passed".to_string()),
        context: Some("ci/test".to_string()),
    };
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/v1/repos/admin/codeza/statuses/{}", head_sha))
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_string(&status_payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    // 5. Attempt Merge (Should Succeed)
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/api/v1/repos/admin/codeza/pulls/{}/merge",
                    pr_number
                ))
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_string(&merge_payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}
#[tokio::test]
async fn test_repo_pulse_flow() {
    let app = api_router();

    // 1. Create 2 Issues
    for i in 1..=2 {
        let payload = CreateIssueOption {
            title: format!("Issue {}", i),
            body: None,
            milestone: None,
        };
        let _ = app
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
    }

    // 2. Close 1 Issue (The first one created in this test is ID 2, since ID 1 exists in init)
    // Wait, init has issue 1. Created are 2 and 3.
    // Let's close issue 2.
    let update_payload = UpdateIssueOption {
        title: None,
        body: None,
        state: Some("closed".to_string()),
        milestone_id: None,
    };
    let _ = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PATCH")
                .uri("/api/v1/repos/admin/codeza/issues/2")
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_string(&update_payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    // 3. Create a PR
    let pr_payload = CreatePullRequestOption {
        title: "Pulse PR".to_string(),
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

    // 4. Fetch Pulse
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/repos/admin/codeza/pulse?period=weekly")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let stats: shared::RepoPulseStats = serde_json::from_slice(&body).unwrap();

    // Verify Stats
    // Issues: 2 created. 1 closed.
    // Note: 'active_issues' in handler counts "create_issue" events.
    // 'closed_issues' counts "close_issue" events.
    // So active_issues should be 2 (plus any from init if they had activity logs? Init has no activity logs).
    // So active_issues >= 2.
    assert!(stats.active_issues >= 2);
    assert!(stats.closed_issues >= 1);
    assert!(stats.opened_prs >= 1);
    assert!(!stats.active_authors.is_empty());
}
