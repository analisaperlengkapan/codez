use axum::{
    extract::{Json, Path},
    http::StatusCode,
};
use shared::{
    Organization, Repository, Team, OrgMember, AdminStats, ActionWorkflow, SystemNotice,
    LicenseTemplate, GitignoreTemplate,
    AdminUserEditOption, User, LanguageStat, ProtectedBranch
};

pub async fn get_org(Path(org): Path<String>) -> Json<Option<Organization>> {
    if org == "codeza-org" {
        Json(Some(Organization {
            id: 1,
            username: "codeza-org".to_string(),
            description: Some("Codeza Organization".to_string()),
            avatar_url: None,
        }))
    } else {
        Json(None)
    }
}

pub async fn list_org_repos(Path(_org): Path<String>) -> Json<Vec<Repository>> {
    let repos = vec![
        Repository::new(1, "org-repo".to_string(), "codeza-org".to_string())
    ];
    Json(repos)
}

pub async fn list_teams(State(state): State<AppState>, Path(org): Path<String>) -> Json<Vec<Team>> {
    let teams = state.teams.read().unwrap();
    let filtered_teams: Vec<Team> = teams.iter().filter(|t| t.org_name == org).cloned().collect();
    Json(filtered_teams)
}

pub async fn list_org_members(Path(_org): Path<String>) -> Json<Vec<OrgMember>> {
    vec![
        OrgMember { user: User::new(1, "admin".to_string(), None), role: "owner".to_string() }
    ].into()
}

pub async fn add_org_member(Path((_org, _username)): Path<(String, String)>) -> StatusCode {
    StatusCode::CREATED
}

pub async fn remove_org_member(Path((_org, _username)): Path<(String, String)>) -> StatusCode {
    StatusCode::NO_CONTENT
}

pub async fn get_admin_stats(State(state): State<AppState>) -> Json<AdminStats> {
    let users_count = state.users.read().unwrap().len() as u64;
    let repos_count = state.repos.read().unwrap().len() as u64;
    let issues_count = state.issues.read().unwrap().len() as u64;
    // Orgs are not fully stateful yet, assuming mock or future implementation
    let orgs_count = 5;

    Json(AdminStats {
        users: users_count,
        repos: repos_count,
        orgs: orgs_count,
        issues: issues_count,
    })
}

pub async fn list_notices() -> Json<Vec<SystemNotice>> {
    let notices = vec![
        SystemNotice { id: 1, type_: "info".to_string(), description: "System maintenance at 00:00".to_string() }
    ];
    Json(notices)
}

use axum::extract::State;
use crate::router::AppState;

pub async fn admin_list_users(State(state): State<AppState>) -> Json<Vec<User>> {
    let users = state.users.read().unwrap();
    Json(users.clone())
}

pub async fn admin_edit_user(
    State(state): State<AppState>,
    Path(username): Path<String>,
    Json(payload): Json<AdminUserEditOption>
) -> StatusCode {
    let mut users = state.users.write().unwrap();
    if let Some(user) = users.iter_mut().find(|u| u.username == username) {
        if let Some(email) = payload.email {
            user.email = Some(email);
        }
        StatusCode::OK
    } else {
        StatusCode::NOT_FOUND
    }
}

pub async fn admin_delete_user(
    State(state): State<AppState>,
    Path(username): Path<String>
) -> StatusCode {
    let mut users = state.users.write().unwrap();
    if let Some(pos) = users.iter().position(|u| u.username == username) {
        users.remove(pos);
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}

// Miscellaneous Handlers
pub async fn list_workflows(Path((_owner, _repo)): Path<(String, String)>) -> Json<Vec<ActionWorkflow>> {
    let wfs = vec![
        ActionWorkflow { id: 1, name: "CI".to_string(), status: "success".to_string() }
    ];
    Json(wfs)
}

pub async fn list_licenses() -> Json<Vec<LicenseTemplate>> {
    vec![
        LicenseTemplate { key: "mit".to_string(), name: "MIT License".to_string(), url: "http://...".to_string() }
    ].into()
}

pub async fn list_gitignores() -> Json<Vec<GitignoreTemplate>> {
    vec![
        GitignoreTemplate { name: "Rust".to_string(), source: "target/".to_string() }
    ].into()
}

pub async fn get_repo_languages(Path((_owner, _repo)): Path<(String, String)>) -> Json<Vec<LanguageStat>> {
    vec![
        LanguageStat { language: "Rust".to_string(), percentage: 100, color: "#dea584".to_string() }
    ].into()
}

pub async fn list_branch_protections(Path((_owner, _repo)): Path<(String, String)>) -> Json<Vec<ProtectedBranch>> {
    vec![
        ProtectedBranch { name: "main".to_string(), enable_push: false, enable_force_push: false }
    ].into()
}
