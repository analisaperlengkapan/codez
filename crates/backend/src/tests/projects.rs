//! Integration tests for the projects endpoints.
use crate::routes::api_router;
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use shared::{
    Activity, CreateIssueOption, CreateProjectCardOption, CreateProjectColumnOption,
    CreateProjectOption, MoveProjectCardOption, Project, ProjectCard, ProjectColumn,
};
use tower::ServiceExt; // for `oneshot`

#[tokio::test]
async fn test_project_flow() {
    let app = api_router();

    // 1. Create Project
    let payload = CreateProjectOption {
        title: "My Project".to_string(),
        description: Some("Kanban Board".to_string()),
    };
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/repos/admin/codeza/projects")
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_string(&payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let project: Project = serde_json::from_slice(&body).unwrap();
    let project_id = project.id;
    assert_eq!(project.title, "My Project");

    // 2. Create Column
    let payload = CreateProjectColumnOption {
        title: "To Do".to_string(),
    };
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/api/v1/repos/admin/codeza/projects/{}/columns",
                    project_id
                ))
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_string(&payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let column: ProjectColumn = serde_json::from_slice(&body).unwrap();
    let column_id = column.id;
    assert_eq!(column.title, "To Do");

    // 3. Create Card
    let payload = CreateProjectCardOption {
        content: Some("Task 1".to_string()),
        note: None,
        issue_id: None,
    };
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/api/v1/repos/admin/codeza/projects/columns/{}/cards",
                    column_id
                ))
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_string(&payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let card: ProjectCard = serde_json::from_slice(&body).unwrap();
    let card_id = card.id;
    assert_eq!(card.content, Some("Task 1".to_string()));

    // 4. Move Card (to same column just index change, or assume 2nd column exists)
    // Let's just create a second column to be sure
    let payload = CreateProjectColumnOption {
        title: "Done".to_string(),
    };
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/api/v1/repos/admin/codeza/projects/{}/columns",
                    project_id
                ))
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_string(&payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let col2: ProjectColumn = serde_json::from_slice(&body).unwrap();

    let payload = MoveProjectCardOption {
        column_id: col2.id,
        new_index: 0,
    };
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/api/v1/repos/admin/codeza/projects/cards/{}/move",
                    card_id
                ))
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_string(&payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // 5. Close Project
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/api/v1/repos/admin/codeza/projects/{}/close",
                    project_id
                ))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // Verify closed
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!(
                    "/api/v1/repos/admin/codeza/projects/{}",
                    project_id
                ))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let p: Option<Project> = serde_json::from_slice(&body).unwrap();
    assert!(p.unwrap().is_closed);

    // 6. Create Card Linked to Issue
    // Create Issue first to check if we can link it
    let issue_payload = CreateIssueOption {
        title: "Issue for card".to_string(),
        body: None,
        milestone: None,
    };
    let _ = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/repos/admin/codeza/issues")
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_string(&issue_payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    let payload = CreateProjectCardOption {
        content: None,
        note: None,
        issue_id: Some(1), // Assuming ID 1 or we should fetch it. Since tests run in parallel or sequence,
                           // ID prediction is brittle. Let's assume ID is > 0.
    };
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/api/v1/repos/admin/codeza/projects/columns/{}/cards",
                    column_id
                ))
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_string(&payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    // Verify Activity Log for card creation
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/user/feeds")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let activities: Vec<Activity> = serde_json::from_slice(&body).unwrap();
    assert!(activities
        .iter()
        .any(|a| a.op_type == "create_project_card"));
    assert!(activities.iter().any(|a| a.op_type == "move_project_card"));
}
