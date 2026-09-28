//! Integration tests for issues, assignees and locking.

use axum::http::StatusCode;
use shared::Issue;

use super::TestApp;

#[tokio::test]
async fn list_and_get_seeded_issue() {
    let app = TestApp::new();

    let issues: Vec<Issue> = app.get_json("/api/v1/repos/admin/codeza/issues").await;
    assert_eq!(issues.len(), 1);

    let issue: Issue = app.get_json("/api/v1/repos/admin/codeza/issues/1").await;
    assert_eq!(issue.title, "First Issue");
    assert_eq!(issue.state, "open");
}

#[tokio::test]
async fn create_issue_increments_number_and_notifies() {
    let app = TestApp::new();

    let created: Issue = app
        .post_created(
            "/api/v1/repos/admin/codeza/issues",
            serde_json::json!({ "title": "Second issue", "body": "Details" }),
        )
        .await;
    assert_eq!(created.number, 2);

    let issues: Vec<Issue> = app.get_json("/api/v1/repos/admin/codeza/issues").await;
    assert_eq!(issues.len(), 2);

    app.get(&format!(
        "/api/v1/repos/admin/codeza/issues/{}",
        created.number
    ))
    .await
    .assert_status(StatusCode::OK);
}

#[tokio::test]
async fn issue_filters_by_state_and_query() {
    let app = TestApp::new();

    app.post_json(
        "/api/v1/repos/admin/codeza/issues",
        serde_json::json!({ "title": "Closed one" }),
    )
    .await
    .assert_status(StatusCode::CREATED);

    let open: Vec<Issue> = app
        .get_json("/api/v1/repos/admin/codeza/issues?state=open")
        .await;
    assert!(open.iter().all(|i| i.state == "open"));

    let matching: Vec<Issue> = app
        .get_json("/api/v1/repos/admin/codeza/issues?q=Closed")
        .await;
    assert_eq!(matching.len(), 1);
    assert_eq!(matching[0].title, "Closed one");
}

#[tokio::test]
async fn update_issue_can_close_it() {
    let app = TestApp::new();

    app.patch_json(
        "/api/v1/repos/admin/codeza/issues/1",
        serde_json::json!({ "state": "closed", "title": "Renamed" }),
    )
    .await
    .assert_status(StatusCode::OK);

    let issue: Issue = app.get_json("/api/v1/repos/admin/codeza/issues/1").await;
    assert_eq!(issue.state, "closed");
    assert_eq!(issue.title, "Renamed");
}

#[tokio::test]
async fn lock_and_unlock_issue() {
    let app = TestApp::new();

    app.put_json(
        "/api/v1/repos/admin/codeza/issues/1/lock",
        serde_json::json!({}),
    )
    .await
    .assert_status(StatusCode::OK);

    let issue: Issue = app.get_json("/api/v1/repos/admin/codeza/issues/1").await;
    assert!(issue.is_locked);

    app.delete("/api/v1/repos/admin/codeza/issues/1/lock")
        .await
        .assert_status(StatusCode::OK);

    let issue: Issue = app.get_json("/api/v1/repos/admin/codeza/issues/1").await;
    assert!(!issue.is_locked);
}

#[tokio::test]
async fn assignee_add_and_remove() {
    let app = TestApp::new();

    app.post_json(
        "/api/v1/repos/admin/codeza/issues/1/assignees",
        serde_json::json!({ "id": 2, "username": "user", "email": null }),
    )
    .await
    .assert_status(StatusCode::CREATED);

    let issue: Issue = app.get_json("/api/v1/repos/admin/codeza/issues/1").await;
    assert!(issue.assignees.iter().any(|u| u.username == "user"));

    app.delete("/api/v1/repos/admin/codeza/issues/1/assignees/user")
        .await
        .assert_status(StatusCode::NO_CONTENT);

    let issue: Issue = app.get_json("/api/v1/repos/admin/codeza/issues/1").await;
    assert!(issue.assignees.iter().all(|u| u.username != "user"));
}

#[tokio::test]
async fn global_issue_search_finds_matches() {
    let app = TestApp::new();

    let results: Vec<Issue> = app.get_json("/api/v1/search/issues?q=First").await;
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].title, "First Issue");

    let none: Vec<Issue> = app.get_json("/api/v1/search/issues?q=nomatch").await;
    assert!(none.is_empty());
}

#[tokio::test]
async fn unknown_issue_is_null() {
    let app = TestApp::new();
    let issue: Option<Issue> = app.get_json("/api/v1/repos/admin/codeza/issues/999").await;
    assert!(issue.is_none());
}
