//! Integration tests for the users endpoints.
use crate::routes::api_router;
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use shared::{
    Activity, CreateIssueOption, CreatePullRequestOption, EmailAddress, Issue, PublicKey,
    PullRequest, User,
};
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
#[tokio::test]
async fn login_succeeds_with_demo_credentials() {
    let app = super::TestApp::new();
    let user: User = app
        .post_json(
            "/api/v1/users/login",
            serde_json::json!({ "username": "admin", "password": "password" }),
        )
        .await
        .json();

    assert_eq!(user.username, "admin");
    assert_eq!(user.id, 1);
}

#[tokio::test]
async fn login_rejects_wrong_password_and_unknown_user() {
    let app = super::TestApp::new();

    app.post_json(
        "/api/v1/users/login",
        serde_json::json!({ "username": "admin", "password": "nope" }),
    )
    .await
    .assert_status(StatusCode::UNAUTHORIZED);

    app.post_json(
        "/api/v1/users/login",
        serde_json::json!({ "username": "ghost", "password": "password" }),
    )
    .await
    .assert_status(StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn register_then_lookup_and_reject_duplicates() {
    let app = super::TestApp::new();
    let user: User = app
        .post_created(
            "/api/v1/users/register",
            serde_json::json!({ "username": "newbie", "email": "newbie@example.com", "password": "x" }),
        )
        .await;
    assert_eq!(user.username, "newbie");

    let fetched: Option<User> = app.get_json("/api/v1/users/newbie").await;
    assert_eq!(fetched.unwrap().id, user.id);

    app.post_json(
        "/api/v1/users/register",
        serde_json::json!({ "username": "newbie", "email": "other@example.com", "password": "x" }),
    )
    .await
    .assert_status(StatusCode::CONFLICT);

    app.post_json(
        "/api/v1/users/register",
        serde_json::json!({ "username": "other", "email": "newbie@example.com", "password": "x" }),
    )
    .await
    .assert_status(StatusCode::CONFLICT);
}

#[tokio::test]
async fn unknown_user_lookup_returns_null() {
    let app = super::TestApp::new();
    let user: Option<User> = app.get_json("/api/v1/users/nobody").await;
    assert!(user.is_none());
}

#[tokio::test]
async fn ssh_key_create_list_and_delete() {
    let app = super::TestApp::new();

    let key: PublicKey = app
        .post_created(
            "/api/v1/user/keys",
            serde_json::json!({ "title": "laptop", "key": "ssh-ed25519 AAAA" }),
        )
        .await;
    assert_eq!(key.title, "laptop");

    let keys: Vec<PublicKey> = app.get_json("/api/v1/user/keys").await;
    assert!(keys.iter().any(|k| k.id == key.id));

    app.delete(&format!("/api/v1/user/keys/{}", key.id))
        .await
        .assert_status(StatusCode::NO_CONTENT);
    app.delete(&format!("/api/v1/user/keys/{}", key.id))
        .await
        .assert_status(StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn gpg_key_lifecycle() {
    let app = super::TestApp::new();

    let key: shared::GpgKey = app
        .post_created(
            "/api/v1/user/gpg_keys",
            serde_json::json!({ "armored_public_key": "-----BEGIN PGP-----" }),
        )
        .await;

    let listed: Vec<shared::GpgKey> = app.get_json("/api/v1/user/gpg_keys").await;
    assert!(listed.iter().any(|k| k.id == key.id));

    app.post(&format!("/api/v1/user/gpg_keys/{}/verify", key.id))
        .await
        .assert_status(StatusCode::OK);

    app.delete(&format!("/api/v1/user/gpg_keys/{}", key.id))
        .await
        .assert_status(StatusCode::NO_CONTENT);
}

#[tokio::test]
async fn follow_and_unfollow_updates_both_directions() {
    let app = super::TestApp::new();

    app.post("/api/v1/users/user/follow")
        .await
        .assert_status(StatusCode::NO_CONTENT);

    let followers: Vec<User> = app.get_json("/api/v1/users/user/followers").await;
    assert!(followers.iter().any(|u| u.username == "admin"));

    let following: Vec<User> = app.get_json("/api/v1/users/admin/following").await;
    assert!(following.iter().any(|u| u.username == "user"));

    app.delete("/api/v1/users/user/follow")
        .await
        .assert_status(StatusCode::NO_CONTENT);

    let followers: Vec<User> = app.get_json("/api/v1/users/user/followers").await;
    assert!(followers.iter().all(|u| u.username != "admin"));
}

#[tokio::test]
async fn cannot_follow_yourself() {
    let app = super::TestApp::new();
    app.post("/api/v1/users/admin/follow")
        .await
        .assert_status(StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn follow_unknown_user_is_not_found() {
    let app = super::TestApp::new();
    app.post("/api/v1/users/ghost/follow")
        .await
        .assert_status(StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn settings_update_is_logged_to_the_feed() {
    let app = super::TestApp::new();
    app.patch_json(
        "/api/v1/user/settings",
        serde_json::json!({ "full_name": "Admin" }),
    )
    .await
    .assert_status(StatusCode::OK);

    let feeds: Vec<Activity> = app.get_json("/api/v1/user/feeds").await;
    assert!(feeds.iter().any(|a| a.op_type == "update_settings"));
}

#[tokio::test]
async fn oauth2_app_secret_is_redacted_when_listed() {
    let app = super::TestApp::new();

    let created: shared::OAuth2Application = app
        .post_created(
            "/api/v1/user/applications/oauth2",
            serde_json::json!({ "name": "CI", "redirect_uris": ["http://localhost/cb"] }),
        )
        .await;
    assert!(!created.client_secret.is_empty());

    let listed: Vec<shared::OAuth2Application> =
        app.get_json("/api/v1/user/applications/oauth2").await;
    let found = listed.iter().find(|a| a.id == created.id).unwrap();
    assert_eq!(found.client_secret, "****************");

    app.delete(&format!("/api/v1/user/applications/oauth2/{}", created.id))
        .await
        .assert_status(StatusCode::NO_CONTENT);
}

#[tokio::test]
async fn static_user_endpoints_return_expected_shapes() {
    let app = super::TestApp::new();

    let emails: Vec<EmailAddress> = app.get_json("/api/v1/user/emails").await;
    assert!(emails.iter().any(|e| e.primary));

    let providers: Vec<shared::OAuth2Provider> = app.get_json("/api/v1/user/oauth2").await;
    assert_eq!(providers[0].name, "github");

    let two_factor: shared::TwoFactor = app.get_json("/api/v1/user/2fa").await;
    assert!(!two_factor.enabled);

    let heatmap: Vec<shared::Contribution> = app.get_json("/api/v1/users/admin/heatmap").await;
    assert!(!heatmap.is_empty());
}

#[tokio::test]
async fn notification_can_be_marked_read() {
    let app = super::TestApp::new();

    app.post_json(
        "/api/v1/repos/admin/codeza/issues",
        serde_json::json!({ "title": "Notify me" }),
    )
    .await
    .assert_status(StatusCode::CREATED);

    let notifications: Vec<shared::Notification> = app.get_json("/api/v1/notifications").await;
    let unread = notifications
        .iter()
        .find(|n| n.unread)
        .expect("expected an unread notification");

    app.patch_json(
        &format!("/api/v1/notifications/threads/{}", unread.id),
        serde_json::json!({}),
    )
    .await
    .assert_status(StatusCode::RESET_CONTENT);

    let notifications: Vec<shared::Notification> = app.get_json("/api/v1/notifications").await;
    assert!(!notifications.iter().any(|n| n.id == unread.id && n.unread));

    app.patch_json(
        "/api/v1/notifications/threads/999999",
        serde_json::json!({}),
    )
    .await
    .assert_status(StatusCode::NOT_FOUND);
}
