//! Integration tests for repository discussions and their comments.

use axum::http::StatusCode;
use shared::{Discussion, DiscussionComment};

use super::TestApp;

#[tokio::test]
async fn discussion_crud_and_comments_flow() {
    let app = TestApp::new();

    let discussions: Vec<Discussion> = app.get_json("/api/v1/repos/admin/codeza/discussions").await;
    assert!(discussions.is_empty());

    let created: Discussion = app
        .post_created(
            "/api/v1/repos/admin/codeza/discussions",
            serde_json::json!({
                "title": "Ideas",
                "body": "Let's discuss",
                "category": "general"
            }),
        )
        .await;
    assert_eq!(created.title, "Ideas");

    let fetched: Discussion = app
        .get_json(&format!(
            "/api/v1/repos/admin/codeza/discussions/{}",
            created.id
        ))
        .await;
    assert_eq!(fetched.id, created.id);

    app.patch_json(
        &format!("/api/v1/repos/admin/codeza/discussions/{}", created.id),
        serde_json::json!({ "title": "Ideas (edited)", "is_locked": true }),
    )
    .await
    .assert_status(StatusCode::OK);

    let comment: DiscussionComment = app
        .post_created(
            &format!(
                "/api/v1/repos/admin/codeza/discussions/{}/comments",
                created.id
            ),
            serde_json::json!({ "body": "First!" }),
        )
        .await;
    assert_eq!(comment.body, "First!");

    let comments: Vec<DiscussionComment> = app
        .get_json(&format!(
            "/api/v1/repos/admin/codeza/discussions/{}/comments",
            created.id
        ))
        .await;
    assert_eq!(comments.len(), 1);

    app.delete(&format!(
        "/api/v1/repos/admin/codeza/discussions/{}",
        created.id
    ))
    .await
    .assert_status(StatusCode::NO_CONTENT);

    let gone: Option<Discussion> = app
        .get_json(&format!(
            "/api/v1/repos/admin/codeza/discussions/{}",
            created.id
        ))
        .await;
    assert!(gone.is_none());
}
