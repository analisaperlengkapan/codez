//! Integration tests for projects, columns and cards.

use axum::http::StatusCode;
use shared::{Project, ProjectCard, ProjectColumn};

use super::TestApp;

#[tokio::test]
async fn project_board_flow() {
    let app = TestApp::new();

    // Create project.
    let project: Project = app
        .post_created(
            "/api/v1/repos/admin/codeza/projects",
            serde_json::json!({ "title": "Roadmap", "description": "Q3" }),
        )
        .await;
    assert_eq!(project.title, "Roadmap");

    let projects: Vec<Project> = app.get_json("/api/v1/repos/admin/codeza/projects").await;
    assert!(projects.iter().any(|p| p.id == project.id));

    app.get(&format!(
        "/api/v1/repos/admin/codeza/projects/{}",
        project.id
    ))
    .await
    .assert_status(StatusCode::OK);

    // Create column.
    let column: ProjectColumn = app
        .post_created(
            &format!("/api/v1/repos/admin/codeza/projects/{}/columns", project.id),
            serde_json::json!({ "title": "To do" }),
        )
        .await;

    let columns: Vec<ProjectColumn> = app
        .get_json(&format!(
            "/api/v1/repos/admin/codeza/projects/{}/columns",
            project.id
        ))
        .await;
    assert!(columns.iter().any(|c| c.id == column.id));

    // Create card.
    let card: ProjectCard = app
        .post_created(
            &format!(
                "/api/v1/repos/admin/codeza/projects/columns/{}/cards",
                column.id
            ),
            serde_json::json!({ "content": "Ship it", "note": "urgent" }),
        )
        .await;

    let cards: Vec<ProjectCard> = app
        .get_json(&format!(
            "/api/v1/repos/admin/codeza/projects/columns/{}/cards",
            column.id
        ))
        .await;
    assert!(cards.iter().any(|c| c.id == card.id));

    // Close then reopen.
    app.post(&format!(
        "/api/v1/repos/admin/codeza/projects/{}/close",
        project.id
    ))
    .await
    .assert_status(StatusCode::OK);

    let closed: Project = app
        .get_json(&format!(
            "/api/v1/repos/admin/codeza/projects/{}",
            project.id
        ))
        .await;
    assert!(closed.is_closed);

    app.post(&format!(
        "/api/v1/repos/admin/codeza/projects/{}/reopen",
        project.id
    ))
    .await
    .assert_status(StatusCode::OK);

    let reopened: Project = app
        .get_json(&format!(
            "/api/v1/repos/admin/codeza/projects/{}",
            project.id
        ))
        .await;
    assert!(!reopened.is_closed);
}

#[tokio::test]
async fn project_unknown_id_is_null() {
    let app = TestApp::new();
    let missing: Option<shared::Project> = app
        .get_json("/api/v1/repos/admin/codeza/projects/999")
        .await;
    assert!(missing.is_none());
}
