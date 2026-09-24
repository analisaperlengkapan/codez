//! Integration tests for webhooks, secrets, deploy keys and LFS locks.

use axum::http::StatusCode;
use shared::{DeployKey, Secret, Webhook, WebhookDelivery};

use super::TestApp;

#[tokio::test]
async fn hook_can_be_created_and_listed() {
    let app = TestApp::new();

    let hooks: Vec<Webhook> = app.get_json("/api/v1/repos/admin/codeza/hooks").await;
    assert_eq!(hooks.len(), 1);

    let hook: Webhook = app
        .post_created(
            "/api/v1/repos/admin/codeza/hooks",
            serde_json::json!({
                "url": "https://hooks.example.com/x",
                "events": ["push"],
                "active": true
            }),
        )
        .await;
    assert_eq!(hook.url, "https://hooks.example.com/x");

    let hooks: Vec<Webhook> = app.get_json("/api/v1/repos/admin/codeza/hooks").await;
    assert_eq!(hooks.len(), 2);
}

#[tokio::test]
async fn hook_deliveries_starts_empty() {
    let app = TestApp::new();
    let deliveries: Vec<WebhookDelivery> = app
        .get_json("/api/v1/repos/admin/codeza/hooks/1/deliveries")
        .await;
    assert!(deliveries.is_empty());
}

#[tokio::test]
async fn secrets_are_created_and_listed() {
    let app = TestApp::new();

    let secret: Secret = app
        .post_created(
            "/api/v1/repos/admin/codeza/secrets",
            serde_json::json!({ "name": "DEPLOY_KEY", "data": "s3cret" }),
        )
        .await;
    assert_eq!(secret.name, "DEPLOY_KEY");

    let secrets: Vec<Secret> = app.get_json("/api/v1/repos/admin/codeza/secrets").await;
    assert!(!secrets.is_empty());
}

#[tokio::test]
async fn deploy_keys_are_created_and_listed() {
    let app = TestApp::new();

    let key: DeployKey = app
        .post_created(
            "/api/v1/repos/admin/codeza/keys",
            serde_json::json!({ "title": "CI", "key": "ssh-rsa AAAA" }),
        )
        .await;
    assert_eq!(key.title, "CI");

    let keys: Vec<DeployKey> = app.get_json("/api/v1/repos/admin/codeza/keys").await;
    assert!(!keys.is_empty());
}

#[tokio::test]
async fn lfs_lock_create_list_and_unlock() {
    let app = TestApp::new();

    let lock: shared::LfsLock = app
        .post_created(
            "/api/v1/repos/admin/codeza/git/lfs/locks",
            serde_json::json!({ "path": "big.bin" }),
        )
        .await;
    assert_eq!(lock.path, "big.bin");

    // Locking the same path again conflicts.
    app.post_json(
        "/api/v1/repos/admin/codeza/git/lfs/locks",
        serde_json::json!({ "path": "big.bin" }),
    )
    .await
    .assert_status(StatusCode::CONFLICT);

    let locks: Vec<shared::LfsLock> = app
        .get_json("/api/v1/repos/admin/codeza/git/lfs/locks")
        .await;
    assert_eq!(locks.len(), 1);

    app.post(&format!(
        "/api/v1/repos/admin/codeza/git/lfs/locks/{}/unlock",
        lock.id
    ))
    .await
    .assert_status(StatusCode::NO_CONTENT);

    let locks: Vec<shared::LfsLock> = app
        .get_json("/api/v1/repos/admin/codeza/git/lfs/locks")
        .await;
    assert!(locks.is_empty());
}
