//! Integration tests for releases and release assets.

use axum::http::StatusCode;
use shared::{Release, ReleaseAsset};

use super::TestApp;

#[tokio::test]
async fn list_and_get_seeded_release() {
    let app = TestApp::new();

    let releases: Vec<Release> = app.get_json("/api/v1/repos/admin/codeza/releases").await;
    assert_eq!(releases.len(), 1);

    let release: Release = app.get_json("/api/v1/repos/admin/codeza/releases/1").await;
    assert_eq!(release.tag_name, "v1.0.0");
}

#[tokio::test]
async fn get_release_is_404_when_missing() {
    let app = TestApp::new();
    let missing: Option<shared::Release> = app
        .get_json("/api/v1/repos/admin/codeza/releases/999")
        .await;
    assert!(missing.is_none());
}

#[tokio::test]
async fn release_crud_flow() {
    let app = TestApp::new();

    let created: Release = app
        .post_created(
            "/api/v1/repos/admin/codeza/releases",
            serde_json::json!({
                "tag_name": "v2.0.0",
                "name": "Second",
                "body": "Notes",
                "draft": false,
                "prerelease": false
            }),
        )
        .await;
    assert_eq!(created.tag_name, "v2.0.0");

    app.patch_json(
        &format!("/api/v1/repos/admin/codeza/releases/{}", created.id),
        serde_json::json!({ "name": "Second (edited)" }),
    )
    .await
    .assert_status(StatusCode::OK);

    let fetched: Release = app
        .get_json(&format!(
            "/api/v1/repos/admin/codeza/releases/{}",
            created.id
        ))
        .await;
    assert_eq!(fetched.name, "Second (edited)");

    app.delete(&format!(
        "/api/v1/repos/admin/codeza/releases/{}",
        created.id
    ))
    .await
    .assert_status(StatusCode::NO_CONTENT);

    app.delete(&format!(
        "/api/v1/repos/admin/codeza/releases/{}",
        created.id
    ))
    .await
    .assert_status(StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn release_asset_upload_then_download_round_trips() {
    let app = TestApp::new();

    let asset: Option<ReleaseAsset> = app
        .post_bytes(
            "/api/v1/repos/admin/codeza/releases/1/assets",
            b"hello asset".to_vec(),
        )
        .await
        .json();
    let asset = asset.expect("asset should be created");
    assert_eq!(asset.size, 11);

    let downloaded = app
        .get(&format!(
            "/api/v1/repos/admin/codeza/releases/1/assets/{}",
            asset.id
        ))
        .await;
    downloaded.assert_status(StatusCode::OK);
    assert_eq!(downloaded.text(), "hello asset");

    let release: Release = app.get_json("/api/v1/repos/admin/codeza/releases/1").await;
    assert!(release.assets.iter().any(|a| a.id == asset.id));
}

#[tokio::test]
async fn release_asset_upload_404s_for_unknown_release() {
    let app = TestApp::new();
    let asset: Option<ReleaseAsset> = app
        .post_bytes(
            "/api/v1/repos/admin/codeza/releases/999/assets",
            b"x".to_vec(),
        )
        .await
        .json();
    assert!(asset.is_none());
}
