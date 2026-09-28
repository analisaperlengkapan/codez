//! Integration tests for issue comments and reactions.

use axum::http::StatusCode;
use shared::Comment;

use super::TestApp;

#[tokio::test]
async fn list_seeded_comment_on_issue_one() {
    let app = TestApp::new();

    let comments: Vec<Comment> = app
        .get_json("/api/v1/repos/admin/codeza/issues/1/comments")
        .await;
    assert_eq!(comments.len(), 1);
    assert_eq!(comments[0].body, "Great idea!");
}

#[tokio::test]
async fn create_update_and_delete_comment() {
    let app = TestApp::new();

    let comment: Comment = app
        .post_created(
            "/api/v1/repos/admin/codeza/issues/1/comments",
            serde_json::json!({ "body": "Nice work" }),
        )
        .await;
    assert_eq!(comment.body, "Nice work");

    app.patch_json(
        &format!("/api/v1/repos/admin/codeza/issues/comments/{}", comment.id),
        serde_json::json!({ "body": "Edited body" }),
    )
    .await
    .assert_status(StatusCode::OK);

    app.delete(&format!(
        "/api/v1/repos/admin/codeza/issues/comments/{}",
        comment.id
    ))
    .await
    .assert_status(StatusCode::NO_CONTENT);

    app.delete(&format!(
        "/api/v1/repos/admin/codeza/issues/comments/{}",
        comment.id
    ))
    .await
    .assert_status(StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn comment_can_be_reacted_to() {
    let app = TestApp::new();

    app.post_json(
        "/api/v1/repos/admin/codeza/issues/comments/1/reactions",
        serde_json::json!({ "content": "+1" }),
    )
    .await
    .assert_status(StatusCode::CREATED);

    let comments: Vec<Comment> = app
        .get_json("/api/v1/repos/admin/codeza/issues/1/comments")
        .await;
    assert!(!comments[0].reactions.is_empty());
}

#[tokio::test]
async fn comment_on_unknown_issue_is_not_found() {
    let app = TestApp::new();
    app.post_json(
        "/api/v1/repos/admin/codeza/issues/999/comments",
        serde_json::json!({ "body": "hi" }),
    )
    .await
    .assert_status(StatusCode::NOT_FOUND);
}
