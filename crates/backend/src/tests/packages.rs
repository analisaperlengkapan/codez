//! Integration tests for packages.

use axum::http::StatusCode;
use shared::Package;

use super::TestApp;

#[tokio::test]
async fn list_and_get_seeded_package() {
    let app = TestApp::new();

    let packages: Vec<Package> = app.get_json("/api/v1/packages/admin").await;
    assert_eq!(packages.len(), 1);
    assert_eq!(packages[0].name, "my-lib");

    let detail: Option<Package> = app
        .get_json("/api/v1/packages/admin/cargo/my-lib/1.0.0")
        .await;
    assert_eq!(detail.unwrap().version, "1.0.0");
}

#[tokio::test]
async fn upload_rejects_duplicate_version() {
    let app = TestApp::new();

    let package: Package = app
        .post_created(
            "/api/v1/packages/admin",
            serde_json::json!({
                "name": "new-lib",
                "version": "0.1.0",
                "package_type": "cargo"
            }),
        )
        .await;
    assert_eq!(package.name, "new-lib");

    app.post_json(
        "/api/v1/packages/admin",
        serde_json::json!({
            "name": "new-lib",
            "version": "0.1.0",
            "package_type": "cargo"
        }),
    )
    .await
    .assert_status(StatusCode::CONFLICT);
}

#[tokio::test]
async fn delete_package_removes_it() {
    let app = TestApp::new();

    app.delete("/api/v1/packages/admin/cargo/my-lib/1.0.0")
        .await
        .assert_status(StatusCode::NO_CONTENT);

    app.delete("/api/v1/packages/admin/cargo/my-lib/1.0.0")
        .await
        .assert_status(StatusCode::NOT_FOUND);
}
