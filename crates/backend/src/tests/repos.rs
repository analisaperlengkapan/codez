//! Integration tests for repository CRUD, settings, social actions and collaborators.

use axum::http::StatusCode;
use shared::{Collaborator, RepoUserStatus, Repository};

use super::TestApp;

#[tokio::test]
async fn list_and_get_seeded_repo() {
    let app = TestApp::new();

    let repos: Vec<Repository> = app.get_json("/api/v1/repos").await;
    assert_eq!(repos.len(), 2);
    assert!(repos
        .iter()
        .any(|r| r.name == "codeza" && r.owner == "admin"));

    let repo: Repository = app.get_json("/api/v1/repos/admin/codeza").await;
    assert_eq!(repo.owner, "admin");
    assert_eq!(repo.id, 1);
}

#[tokio::test]
async fn get_unknown_repo_is_null() {
    let app = TestApp::new();
    // Detail endpoints answer `200` with a `null` body for unknown resources so the
    // frontend `get_opt` helper can decode them uniformly.
    let repo: Option<Repository> = app.get_json("/api/v1/repos/admin/ghost").await;
    assert!(repo.is_none());
}

#[tokio::test]
async fn create_repo_then_fetch_it() {
    let app = TestApp::new();

    let repo: Repository = app
        .post_created(
            "/api/v1/user/repos",
            serde_json::json!({
                "name": "fresh",
                "description": "New repo",
                "private": false,
                "auto_init": true
            }),
        )
        .await;
    assert_eq!(repo.name, "fresh");

    let fetched: Repository = app.get_json("/api/v1/repos/admin/fresh").await;
    assert_eq!(fetched.id, repo.id);

    // Duplicate name is a conflict.
    app.post_json(
        "/api/v1/user/repos",
        serde_json::json!({ "name": "fresh", "private": false, "auto_init": false }),
    )
    .await
    .assert_status(StatusCode::CONFLICT);
}

#[tokio::test]
async fn star_and_watch_toggle_user_status() {
    let app = TestApp::new();

    app.post("/api/v1/repos/admin/codeza/star")
        .await
        .assert_status(StatusCode::NO_CONTENT);
    let status: RepoUserStatus = app.get_json("/api/v1/repos/admin/codeza/user_status").await;
    assert!(status.starred);

    app.post("/api/v1/repos/admin/codeza/watch")
        .await
        .assert_status(StatusCode::NO_CONTENT);
    let status: RepoUserStatus = app.get_json("/api/v1/repos/admin/codeza/user_status").await;
    assert!(status.watching);
}

#[tokio::test]
async fn fork_creates_a_copy_under_the_user() {
    let app = TestApp::new();

    let fork: Repository = app
        .post_json("/api/v1/repos/admin/codeza/fork", serde_json::json!({}))
        .await
        .json();
    assert_eq!(fork.name, "codeza-fork");
    assert_eq!(fork.parent_id, Some(1));

    let repos: Vec<Repository> = app.get_json("/api/v1/repos").await;
    assert!(repos.iter().any(|r| r.name == "codeza-fork"));
}

#[tokio::test]
async fn repo_settings_read_and_update() {
    let app = TestApp::new();

    let settings: shared::RepoSettingsOption =
        app.get_json("/api/v1/repos/admin/codeza/settings").await;
    assert_eq!(settings.private, Some(false));

    app.patch_json(
        "/api/v1/repos/admin/codeza/settings",
        serde_json::json!({ "description": "Updated description" }),
    )
    .await
    .assert_status(StatusCode::OK);

    let repo: Repository = app.get_json("/api/v1/repos/admin/codeza").await;
    assert_eq!(repo.description.as_deref(), Some("Updated description"));
}

#[tokio::test]
async fn repo_search_filters_by_name() {
    let app = TestApp::new();

    let all: Vec<Repository> = app.get_json("/api/v1/repos/search?q=").await;
    let matches: Vec<Repository> = app.get_json("/api/v1/repos/search?q=codez").await;
    assert_eq!(all.len(), 2);
    assert_eq!(matches.len(), 1);
    assert_eq!(matches[0].name, "codeza");

    let none: Vec<Repository> = app.get_json("/api/v1/repos/search?q=zzzz").await;
    assert!(none.is_empty());
}

#[tokio::test]
async fn collaborators_endpoints_respond() {
    let app = TestApp::new();

    let collaborators: Vec<Collaborator> = app
        .get_json("/api/v1/repos/admin/codeza/collaborators")
        .await;
    assert!(!collaborators.is_empty());

    app.put_json(
        "/api/v1/repos/admin/codeza/collaborators/newcollab",
        serde_json::json!({ "permission": "write" }),
    )
    .await
    .assert_status(StatusCode::NO_CONTENT);
}

#[tokio::test]
async fn mirror_sync_and_pulse_respond() {
    let app = TestApp::new();

    app.post("/api/v1/repos/admin/codeza/mirror-sync")
        .await
        .assert_status(StatusCode::OK);

    let pulse: shared::RepoPulseStats = app.get_json("/api/v1/repos/admin/codeza/pulse").await;
    assert_eq!(pulse.period, "weekly");
}
