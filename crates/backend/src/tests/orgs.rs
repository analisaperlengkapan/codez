//! Integration tests for the orgs endpoints.
use crate::routes::api_router;
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt; // for `oneshot`

#[tokio::test]
async fn test_org_management_flow() {
    let app = api_router();

    // 1. Create Organization
    let payload = shared::CreateOrgOption {
        username: "new-org".to_string(),
        description: Some("New Org".to_string()),
        website: None,
        location: None,
        email: None,
        visibility: None,
    };
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/orgs")
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_string(&payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    // 2. Create Team
    let team_payload = shared::CreateTeamOption {
        name: "Devs".to_string(),
        description: None,
        permission: "write".to_string(),
    };
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/orgs/new-org/teams")
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_string(&team_payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    // 3. List Teams
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/orgs/new-org/teams")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let teams: Vec<shared::Team> = serde_json::from_slice(&body).unwrap();
    assert_eq!(teams.len(), 1);
    assert_eq!(teams[0].name, "Devs");
}
