//! Integration tests for instance administration.

use axum::http::StatusCode;
use shared::{AdminStats, SystemNotice, User};

use super::TestApp;

#[tokio::test]
async fn admin_stats_and_notices_are_served() {
    let app = TestApp::new();

    let stats: AdminStats = app.get_json("/api/v1/admin/stats").await;
    assert!(stats.users >= 1);
    assert!(stats.repos >= 1);

    let notices: Vec<SystemNotice> = app.get_json("/api/v1/admin/notices").await;
    assert!(!notices.is_empty());
}

#[tokio::test]
async fn admin_user_crud_flow() {
    let app = TestApp::new();

    let users: Vec<User> = app.get_json("/api/v1/admin/users").await;
    let before = users.len();

    let created: User = app
        .post_created(
            "/api/v1/admin/users",
            serde_json::json!({
                "username": "created-by-admin",
                "email": "created@example.com",
                "password": "secret"
            }),
        )
        .await;
    assert_eq!(created.username, "created-by-admin");

    let users: Vec<User> = app.get_json("/api/v1/admin/users").await;
    assert_eq!(users.len(), before + 1);

    // Edit uses POST.
    app.post_json(
        "/api/v1/admin/users/created-by-admin",
        serde_json::json!({ "email": "changed@example.com" }),
    )
    .await
    .assert_status(StatusCode::OK);

    app.delete("/api/v1/admin/users/created-by-admin")
        .await
        .assert_status(StatusCode::NO_CONTENT);

    app.delete("/api/v1/admin/users/created-by-admin")
        .await
        .assert_status(StatusCode::NOT_FOUND);
}
