//! Integration tests for organizations, teams, members and audit logs.

use axum::http::StatusCode;
use shared::{AuditLog, OrgMember, Organization, Repository, Team};

use super::TestApp;

#[tokio::test]
async fn seeded_org_is_served_with_repos_members_and_teams() {
    let app = TestApp::new();

    let org: Option<Organization> = app.get_json("/api/v1/orgs/codeza-org").await;
    assert_eq!(org.unwrap().username, "codeza-org");

    let repos: Vec<Repository> = app.get_json("/api/v1/orgs/codeza-org/repos").await;
    assert!(repos.is_empty());

    let teams: Vec<Team> = app.get_json("/api/v1/orgs/codeza-org/teams").await;
    assert_eq!(teams.len(), 1);

    let members: Vec<OrgMember> = app.get_json("/api/v1/orgs/codeza-org/members").await;
    assert_eq!(members.len(), 2);
}

#[tokio::test]
async fn unknown_org_is_null() {
    let app = TestApp::new();
    let org: Option<Organization> = app.get_json("/api/v1/orgs/ghost").await;
    assert!(org.is_none());
}

#[tokio::test]
async fn create_org_then_reject_duplicate() {
    let app = TestApp::new();

    let org: Organization = app
        .post_created(
            "/api/v1/orgs",
            serde_json::json!({ "username": "new-org", "description": "New" }),
        )
        .await;
    assert_eq!(org.username, "new-org");

    app.post_json("/api/v1/orgs", serde_json::json!({ "username": "new-org" }))
        .await
        .assert_status(StatusCode::CONFLICT);
}

#[tokio::test]
async fn team_can_be_created_under_org() {
    let app = TestApp::new();

    let team: Team = app
        .post_created(
            "/api/v1/orgs/codeza-org/teams",
            serde_json::json!({ "name": "Reviewers", "permission": "read" }),
        )
        .await;
    assert_eq!(team.name, "Reviewers");

    let teams: Vec<Team> = app.get_json("/api/v1/orgs/codeza-org/teams").await;
    assert_eq!(teams.len(), 2);
}

#[tokio::test]
async fn member_role_update_writes_an_audit_log() {
    let app = TestApp::new();

    app.put_json(
        "/api/v1/orgs/codeza-org/members/user",
        serde_json::json!({ "role": "maintainer" }),
    )
    .await
    .assert_status(StatusCode::OK);

    let logs: Vec<AuditLog> = app.get_json("/api/v1/orgs/codeza-org/audit-logs").await;
    assert!(logs.iter().any(|l| l.action == "org.member_role_update"));
}

#[tokio::test]
async fn adding_and_removing_members_responds() {
    let app = TestApp::new();

    app.post_json(
        "/api/v1/orgs/codeza-org/members/newmember",
        serde_json::json!({ "role": "developer" }),
    )
    .await
    .assert_status(StatusCode::CREATED);

    app.delete("/api/v1/orgs/codeza-org/members/newmember")
        .await
        .assert_status(StatusCode::NO_CONTENT);
}
