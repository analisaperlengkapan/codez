//! Integration tests for milestones.

use axum::http::StatusCode;
use shared::{Milestone, MilestoneStats};

use super::TestApp;

#[tokio::test]
async fn milestone_lifecycle_and_stats() {
    let app = TestApp::new();

    let milestone: Milestone = app
        .post_created(
            "/api/v1/repos/admin/codeza/milestones",
            serde_json::json!({ "title": "v1.0", "description": "First", "due_on": "2026-01-01" }),
        )
        .await;
    assert_eq!(milestone.title, "v1.0");

    let listed: Vec<Milestone> = app.get_json("/api/v1/repos/admin/codeza/milestones").await;
    assert!(listed.iter().any(|m| m.id == milestone.id));

    let fetched: Milestone = app
        .get_json(&format!(
            "/api/v1/repos/admin/codeza/milestones/{}",
            milestone.id
        ))
        .await;
    assert_eq!(fetched.id, milestone.id);

    app.patch_json(
        &format!("/api/v1/repos/admin/codeza/milestones/{}", milestone.id),
        serde_json::json!({ "title": "v1.1" }),
    )
    .await
    .assert_status(StatusCode::OK);

    let stats: MilestoneStats = app
        .get_json(&format!(
            "/api/v1/repos/admin/codeza/milestones/{}/stats",
            milestone.id
        ))
        .await;
    assert_eq!(stats.closed_issues + stats.open_issues, 0);

    app.delete(&format!(
        "/api/v1/repos/admin/codeza/milestones/{}",
        milestone.id
    ))
    .await
    .assert_status(StatusCode::NO_CONTENT);

    app.delete(&format!(
        "/api/v1/repos/admin/codeza/milestones/{}",
        milestone.id
    ))
    .await
    .assert_status(StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn milestone_can_be_attached_to_an_issue() {
    let app = TestApp::new();

    let milestone: Milestone = app
        .post_created(
            "/api/v1/repos/admin/codeza/milestones",
            serde_json::json!({ "title": "Sprint 1" }),
        )
        .await;

    app.patch_json(
        "/api/v1/repos/admin/codeza/issues/1",
        serde_json::json!({ "milestone_id": milestone.id }),
    )
    .await
    .assert_status(StatusCode::OK);

    let stats: MilestoneStats = app
        .get_json(&format!(
            "/api/v1/repos/admin/codeza/milestones/{}/stats",
            milestone.id
        ))
        .await;
    assert!(stats.open_issues + stats.closed_issues >= 1);
}
