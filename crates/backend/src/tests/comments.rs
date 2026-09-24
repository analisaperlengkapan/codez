//! Integration tests for the comments endpoints.
use crate::routes::api_router;
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use shared::{Comment, CreateCommentOption, UpdateCommentOption};
use tower::ServiceExt; // for `oneshot`

#[tokio::test]
async fn test_comment_update_delete_flow() {
    let app = api_router();

    // Create comment first
    let payload = CreateCommentOption {
        body: "Initial comment".to_string(),
    };
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/repos/admin/codeza/issues/1/comments")
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_string(&payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let comment: Comment = serde_json::from_slice(&body).unwrap();
    let comment_id = comment.id;

    // Update comment
    let update_payload = UpdateCommentOption {
        body: "Updated comment".to_string(),
    };
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PATCH")
                .uri(format!(
                    "/api/v1/repos/admin/codeza/issues/comments/{}",
                    comment_id
                ))
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
    let updated_comment: Option<Comment> = serde_json::from_slice(&body).unwrap();
    assert_eq!(updated_comment.unwrap().body, "Updated comment");

    // Delete comment
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(format!(
                    "/api/v1/repos/admin/codeza/issues/comments/{}",
                    comment_id
                ))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}
#[tokio::test]
async fn test_comment_reaction_flow() {
    let app = api_router();

    // Add reaction to comment 1 (mock init state)
    let payload = shared::CreateReactionOption {
        content: "+1".to_string(),
    };
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/repos/admin/codeza/issues/comments/1/reactions")
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
    let reaction: shared::Reaction = serde_json::from_slice(&body).unwrap();
    assert_eq!(reaction.content, "+1");
}
