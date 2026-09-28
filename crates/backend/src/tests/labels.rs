//! Integration tests for issue labels.

use axum::http::StatusCode;
use shared::Label;

use super::TestApp;

#[tokio::test]
async fn label_crud_flow() {
    let app = TestApp::new();

    let label: Label = app
        .post_created(
            "/api/v1/repos/admin/codeza/labels",
            serde_json::json!({ "name": "triage", "color": "#ff0000", "description": "needs triage" }),
        )
        .await;
    assert_eq!(label.name, "triage");

    let labels: Vec<Label> = app.get_json("/api/v1/repos/admin/codeza/labels").await;
    assert!(labels.iter().any(|l| l.id == label.id));

    app.patch_json(
        &format!("/api/v1/repos/admin/codeza/labels/{}", label.id),
        serde_json::json!({ "name": "triage-2" }),
    )
    .await
    .assert_status(StatusCode::OK);

    app.delete(&format!("/api/v1/repos/admin/codeza/labels/{}", label.id))
        .await
        .assert_status(StatusCode::NO_CONTENT);

    app.delete(&format!("/api/v1/repos/admin/codeza/labels/{}", label.id))
        .await
        .assert_status(StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn issue_labels_can_be_added_and_removed() {
    let app = TestApp::new();

    app.post_json(
        "/api/v1/repos/admin/codeza/issues/1/labels",
        serde_json::json!({ "name": "bug", "color": "#aa0000" }),
    )
    .await
    .assert_status(StatusCode::CREATED);

    let issue: shared::Issue = app.get_json("/api/v1/repos/admin/codeza/issues/1").await;
    assert!(issue.labels.iter().any(|l| l.name == "bug"));

    // The handler attaches labels with a placeholder id, so remove by that id.
    let attached_id = issue
        .labels
        .iter()
        .find(|l| l.name == "bug")
        .expect("label attached")
        .id;
    app.delete(&format!(
        "/api/v1/repos/admin/codeza/issues/1/labels/{attached_id}"
    ))
    .await
    .assert_status(StatusCode::NO_CONTENT);

    let issue: shared::Issue = app.get_json("/api/v1/repos/admin/codeza/issues/1").await;
    assert!(issue.labels.iter().all(|l| l.name != "bug"));
}
