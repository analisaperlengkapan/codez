//! Integration tests for pull requests, reviews and the detail endpoint.

use axum::http::StatusCode;
use shared::{DiffFile, PullRequest, Review};

use super::TestApp;

#[tokio::test]
async fn get_pull_returns_the_seeded_pull() {
    let app = TestApp::new();
    let pr: PullRequest = app.get_json("/api/v1/repos/admin/codeza/pulls/1").await;
    assert_eq!(pr.number, 1);
    assert_eq!(pr.title, "First PR");
    assert_eq!(pr.repo_id, 1);
}

#[tokio::test]
async fn get_pull_is_404_for_unknown_number_and_repo() {
    let app = TestApp::new();

    let missing: Option<shared::PullRequest> =
        app.get_json("/api/v1/repos/admin/codeza/pulls/999").await;
    assert!(missing.is_none());

    let missing: Option<shared::PullRequest> = app
        .get_json("/api/v1/repos/admin/nonexistent/pulls/1")
        .await;
    assert!(missing.is_none());
}

#[tokio::test]
async fn create_then_fetch_pull_round_trips() {
    let app = TestApp::new();

    let created: PullRequest = app
        .post_created(
            "/api/v1/repos/admin/codeza/pulls",
            serde_json::json!({ "title": "Round trip", "head": "feature", "base": "main" }),
        )
        .await;

    let fetched: PullRequest = app
        .get_json(&format!(
            "/api/v1/repos/admin/codeza/pulls/{}",
            created.number
        ))
        .await;
    assert_eq!(fetched.title, "Round trip");
    assert_eq!(fetched.id, created.id);
}

#[tokio::test]
async fn create_pull_rejects_missing_branch_and_unknown_repo() {
    let app = TestApp::new();

    app.post_json(
        "/api/v1/repos/admin/codeza/pulls",
        serde_json::json!({ "title": "Bad", "head": "does-not-exist", "base": "main" }),
    )
    .await
    .assert_status(StatusCode::BAD_REQUEST);

    app.post_json(
        "/api/v1/repos/admin/nope/pulls",
        serde_json::json!({ "title": "Bad", "head": "feature", "base": "main" }),
    )
    .await
    .assert_status(StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn update_pull_changes_title_and_state() {
    let app = TestApp::new();

    let updated: Option<PullRequest> = app
        .patch_json(
            "/api/v1/repos/admin/codeza/pulls/1",
            serde_json::json!({ "title": "Renamed", "state": "closed" }),
        )
        .await
        .json();
    let updated = updated.expect("pull should exist");
    assert_eq!(updated.title, "Renamed");
    assert_eq!(updated.state, "closed");

    let persisted: PullRequest = app.get_json("/api/v1/repos/admin/codeza/pulls/1").await;
    assert_eq!(persisted.title, "Renamed");
}

#[tokio::test]
async fn update_pull_404s_when_missing() {
    let app = TestApp::new();
    let body: Option<PullRequest> = app
        .patch_json(
            "/api/v1/repos/admin/codeza/pulls/404",
            serde_json::json!({ "title": "x" }),
        )
        .await
        .json();
    assert!(body.is_none());
}

#[tokio::test]
async fn reviews_are_associated_with_the_pull_and_map_events_to_state() {
    let app = TestApp::new();

    let approved: Review = app
        .post_created(
            "/api/v1/repos/admin/codeza/pulls/1/reviews",
            serde_json::json!({ "body": "LGTM", "event": "APPROVE" }),
        )
        .await;
    assert_eq!(approved.state, "APPROVED");
    assert_eq!(approved.pull_request_id, 1);

    let commented: Review = app
        .post_created(
            "/api/v1/repos/admin/codeza/pulls/1/reviews",
            serde_json::json!({ "body": "note", "event": "COMMENT" }),
        )
        .await;
    assert_eq!(commented.state, "COMMENTED");

    let changes: Review = app
        .post_created(
            "/api/v1/repos/admin/codeza/pulls/1/reviews",
            serde_json::json!({ "body": "fix", "event": "REQUEST_CHANGES" }),
        )
        .await;
    assert_eq!(changes.state, "CHANGES_REQUESTED");

    let reviews: Vec<Review> = app
        .get_json("/api/v1/repos/admin/codeza/pulls/1/reviews")
        .await;
    assert_eq!(reviews.len(), 3);
}

#[tokio::test]
async fn merge_is_idempotent_and_closes_the_pull() {
    let app = TestApp::new();

    let created: PullRequest = app
        .post_created(
            "/api/v1/repos/admin/codeza/pulls",
            serde_json::json!({ "title": "Merge me", "head": "feature", "base": "main" }),
        )
        .await;

    app.post_json(
        &format!("/api/v1/repos/admin/codeza/pulls/{}/merge", created.number),
        serde_json::json!({ "do": "merge" }),
    )
    .await
    .assert_status(StatusCode::OK);

    let merged: PullRequest = app
        .get_json(&format!(
            "/api/v1/repos/admin/codeza/pulls/{}",
            created.number
        ))
        .await;
    assert!(merged.merged);
    assert_eq!(merged.state, "closed");

    app.post_json(
        &format!("/api/v1/repos/admin/codeza/pulls/{}/merge", created.number),
        serde_json::json!({ "do": "merge" }),
    )
    .await
    .assert_status(StatusCode::METHOD_NOT_ALLOWED);
}

#[tokio::test]
async fn merge_unknown_pull_is_not_found() {
    let app = TestApp::new();
    app.post_json(
        "/api/v1/repos/admin/codeza/pulls/999/merge",
        serde_json::json!({ "do": "merge" }),
    )
    .await
    .assert_status(StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn merge_closes_issues_referenced_by_closing_keywords() {
    let app = TestApp::new();

    let created: PullRequest = app
        .post_created(
            "/api/v1/repos/admin/codeza/pulls",
            serde_json::json!({
                "title": "Close things",
                "body": "Closes #1",
                "head": "feature",
                "base": "main"
            }),
        )
        .await;

    app.post_json(
        &format!("/api/v1/repos/admin/codeza/pulls/{}/merge", created.number),
        serde_json::json!({ "do": "merge" }),
    )
    .await
    .assert_status(StatusCode::OK);

    let issue: shared::Issue = app.get_json("/api/v1/repos/admin/codeza/issues/1").await;
    assert_eq!(issue.state, "closed");
}

#[tokio::test]
async fn requested_reviewer_and_files_endpoints_respond() {
    let app = TestApp::new();

    app.post_json(
        "/api/v1/repos/admin/codeza/pulls/1/requested_reviewers",
        serde_json::json!({}),
    )
    .await
    .assert_status(StatusCode::CREATED);

    let files: Vec<DiffFile> = app
        .get_json("/api/v1/repos/admin/codeza/pulls/1/files")
        .await;
    assert!(!files.is_empty());
}
