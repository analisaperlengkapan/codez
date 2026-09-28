//! Integration tests for migration, transfer, security scanning and metadata endpoints.

use axum::http::StatusCode;
use shared::{Repository, SecurityScanReport};

use super::TestApp;

#[tokio::test]
async fn migrate_creates_repo_and_imports_content_for_rich_services() {
    let app = TestApp::new();

    let repo: Repository = app
        .post_created(
            "/api/v1/repos/migrate",
            serde_json::json!({
                "clone_addr": "https://github.com/example/imported",
                "repo_name": "imported",
                "service": "github",
                "mirror": false
            }),
        )
        .await;
    assert_eq!(repo.name, "imported");
    assert!(!repo.is_mirror);

    // Rich services import an issue and a PR.
    let issues: Vec<shared::Issue> = app.get_json("/api/v1/repos/admin/imported/issues").await;
    assert_eq!(issues.len(), 1);

    let pulls: Vec<shared::PullRequest> = app.get_json("/api/v1/repos/admin/imported/pulls").await;
    assert_eq!(pulls.len(), 1);
}

#[tokio::test]
async fn plain_git_migration_imports_no_issues() {
    let app = TestApp::new();

    app.post_created::<Repository>(
        "/api/v1/repos/migrate",
        serde_json::json!({
            "clone_addr": "https://example.com/plain.git",
            "repo_name": "plain",
            "service": "git",
            "mirror": true
        }),
    )
    .await;

    let issues: Vec<shared::Issue> = app.get_json("/api/v1/repos/admin/plain/issues").await;
    assert!(issues.is_empty());
}

#[tokio::test]
async fn migrate_rejects_duplicate_repo_name() {
    let app = TestApp::new();
    app.post_json(
        "/api/v1/repos/migrate",
        serde_json::json!({
            "clone_addr": "x",
            "repo_name": "codeza",
            "service": "git",
            "mirror": false
        }),
    )
    .await
    .assert_status(StatusCode::CONFLICT);
}

#[tokio::test]
async fn migrated_pull_is_reachable_by_its_repository_scoped_number() {
    let app = TestApp::new();

    // Two rich migrations in a row: each new repo allocates its own pull
    // numbers, so the second import must still be addressable by `number`
    // (the value the list/detail links use) rather than its global `id`.
    for name in ["imported-a", "imported-b"] {
        app.post_created::<Repository>(
            "/api/v1/repos/migrate",
            serde_json::json!({
                "clone_addr": format!("https://github.com/example/{name}"),
                "repo_name": name,
                "service": "github",
                "mirror": false
            }),
        )
        .await;
    }

    let pulls: Vec<shared::PullRequest> =
        app.get_json("/api/v1/repos/admin/imported-b/pulls").await;
    assert_eq!(pulls.len(), 1);
    let imported = &pulls[0];
    assert_eq!(imported.number, 1);

    let path = format!("/api/v1/repos/admin/imported-b/pulls/{}", imported.number);

    let fetched: Option<shared::PullRequest> = app.get_json(&path).await;
    assert_eq!(fetched.expect("pull reachable by number").id, imported.id);

    let updated: Option<shared::PullRequest> = app
        .patch_json(&path, serde_json::json!({ "title": "Renamed import" }))
        .await
        .json();
    assert_eq!(
        updated.expect("pull updatable by number").title,
        "Renamed import"
    );

    app.post_json(
        &format!("{path}/merge"),
        serde_json::json!({ "do": "merge" }),
    )
    .await
    .assert_status(StatusCode::OK);
}

#[tokio::test]
async fn transfer_moves_repo_to_another_user() {
    let app = TestApp::new();

    app.post_json(
        "/api/v1/repos/admin/codeza/transfer",
        serde_json::json!({ "new_owner": "user" }),
    )
    .await
    .assert_status(StatusCode::ACCEPTED);

    let repos: Vec<Repository> = app.get_json("/api/v1/repos").await;
    assert!(repos
        .iter()
        .any(|r| r.name == "codeza" && r.owner == "user"));
}

#[tokio::test]
async fn transfer_rejects_unknown_owner() {
    let app = TestApp::new();
    app.post_json(
        "/api/v1/repos/admin/codeza/transfer",
        serde_json::json!({ "new_owner": "ghost" }),
    )
    .await
    .assert_status(StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn security_scan_returns_a_report() {
    let app = TestApp::new();

    let report: SecurityScanReport = app
        .post_json(
            "/api/v1/repos/admin/codeza/security/scan",
            serde_json::json!({}),
        )
        .await
        .json();
    assert_eq!(report.repo_name, "codeza");
}
