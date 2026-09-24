//! Integration tests for CI action workflows and runs.

use axum::http::StatusCode;
use shared::{ActionWorkflow, WorkflowRun};

use super::TestApp;

#[tokio::test]
async fn list_seeded_workflow() {
    let app = TestApp::new();
    let workflows: Vec<ActionWorkflow> = app
        .get_json("/api/v1/repos/admin/codeza/actions/workflows")
        .await;
    assert_eq!(workflows.len(), 1);
    assert_eq!(workflows[0].name, "CI");
}

#[tokio::test]
async fn trigger_workflow_creates_a_run() {
    let app = TestApp::new();

    let run: WorkflowRun = app
        .post_json(
            "/api/v1/repos/admin/codeza/actions/workflows/1/runs",
            serde_json::json!({ "workflow_id": 1, "ref_name": "main" }),
        )
        .await
        .json();
    assert_eq!(run.workflow_id, 1);
    // A triggered run completes synchronously with logs; it must not be left
    // queued (which would render as permanently active in the Actions UI).
    assert_eq!(run.status, "success");
    assert!(!run.step_logs.is_empty());

    let runs: Vec<WorkflowRun> = app
        .get_json("/api/v1/repos/admin/codeza/actions/workflows/1/runs")
        .await;
    let stored = runs.iter().find(|r| r.id == run.id).expect("run persisted");
    assert_eq!(stored.status, "success");
    assert!(!stored.step_logs.is_empty());

    let logs: Vec<shared::WorkflowStepLog> = app
        .get_json(&format!(
            "/api/v1/repos/admin/codeza/actions/runs/{}/logs",
            run.id
        ))
        .await;
    assert!(!logs.is_empty());
}

#[tokio::test]
async fn workflow_run_logs_update_and_delete() {
    let app = TestApp::new();

    let run: WorkflowRun = app
        .post_json(
            "/api/v1/repos/admin/codeza/actions/workflows/1/runs",
            serde_json::json!({ "workflow_id": 1, "ref_name": "main" }),
        )
        .await
        .json();

    let logs = app
        .get(&format!(
            "/api/v1/repos/admin/codeza/actions/runs/{}/logs",
            run.id
        ))
        .await;
    logs.assert_status(StatusCode::OK);

    app.patch_json(
        &format!("/api/v1/repos/admin/codeza/actions/runs/{}", run.id),
        serde_json::json!({ "status": "success" }),
    )
    .await
    .assert_status(StatusCode::OK);

    app.post_json(
        &format!("/api/v1/repos/admin/codeza/actions/runs/{}/rerun", run.id),
        serde_json::json!({}),
    )
    .await
    .assert_status(StatusCode::OK);

    app.delete(&format!(
        "/api/v1/repos/admin/codeza/actions/runs/{}",
        run.id
    ))
    .await
    .assert_status(StatusCode::NO_CONTENT);

    app.delete(&format!(
        "/api/v1/repos/admin/codeza/actions/runs/{}",
        run.id
    ))
    .await
    .assert_status(StatusCode::NOT_FOUND);
}
