use axum::{
    extract::{Json, Path},
    http::StatusCode,
};
use shared::{
    AdminStats, AdminUserEditOption, AuditLog, CreateOrgOption, CreateTeamOption,
    GitignoreTemplate, LanguageStat, LicenseTemplate, OrgMember, Organization, Repository,
    SystemNotice, Team, UpdateMemberRoleOption, User,
};

pub async fn create_org(
    State(state): State<AppState>,
    Json(payload): Json<CreateOrgOption>,
) -> (StatusCode, Json<Organization>) {
    let mut orgs = state.orgs.write().unwrap_or_else(|e| e.into_inner());
    if orgs.iter().any(|o| o.username == payload.username) {
        return (
            StatusCode::CONFLICT,
            Json(Organization {
                id: 0,
                username: "".to_string(),
                description: None,
                avatar_url: None,
                website: None,
                location: None,
                email: None,
                visibility: None,
            }),
        );
    }
    let id = (orgs.len() as u64) + 1;
    let org = Organization {
        id,
        username: payload.username,
        description: payload.description,
        avatar_url: None,
        website: payload.website,
        location: payload.location,
        email: payload.email,
        visibility: payload.visibility,
    };
    orgs.push(org.clone());
    (StatusCode::CREATED, Json(org))
}

pub async fn get_org(
    State(state): State<AppState>,
    Path(org_name): Path<String>,
) -> Json<Option<Organization>> {
    let orgs = state.orgs.read().unwrap_or_else(|e| e.into_inner());
    let org = orgs.iter().find(|o| o.username == org_name).cloned();
    Json(org)
}

pub async fn list_org_repos(
    State(state): State<AppState>,
    Path(org_name): Path<String>,
) -> Json<Vec<Repository>> {
    let repos = state.repos.read().unwrap_or_else(|e| e.into_inner());
    let filtered_repos: Vec<Repository> = repos
        .iter()
        .filter(|r| r.owner == org_name)
        .cloned()
        .collect();
    Json(filtered_repos)
}

pub async fn list_teams(State(state): State<AppState>, Path(org): Path<String>) -> Json<Vec<Team>> {
    let teams = state.teams.read().unwrap_or_else(|e| e.into_inner());
    let filtered_teams: Vec<Team> = teams
        .iter()
        .filter(|t| t.org_name == org)
        .cloned()
        .collect();
    Json(filtered_teams)
}

pub async fn create_team(
    State(state): State<AppState>,
    Path(org_name): Path<String>,
    Json(payload): Json<CreateTeamOption>,
) -> (StatusCode, Json<Team>) {
    let mut teams = state.teams.write().unwrap_or_else(|e| e.into_inner());
    let id = (teams.len() as u64) + 1;
    let team = Team {
        id,
        org_name,
        name: payload.name,
        description: payload.description,
        permission: payload.permission,
    };
    teams.push(team.clone());
    (StatusCode::CREATED, Json(team))
}

pub async fn list_org_members(
    State(state): State<AppState>,
    Path(_org_name): Path<String>,
) -> Json<Vec<OrgMember>> {
    let members = state.org_members.read().unwrap_or_else(|e| e.into_inner());
    Json(members.clone())
}

pub async fn add_org_member(Path((_org, _username)): Path<(String, String)>) -> StatusCode {
    StatusCode::CREATED
}

pub async fn update_org_member_role(
    State(state): State<AppState>,
    Path((org_name, username)): Path<(String, String)>,
    Json(payload): Json<UpdateMemberRoleOption>,
) -> StatusCode {
    let mut audit_logs = state.audit_logs.write().unwrap_or_else(|e| e.into_inner());
    let log_id = (audit_logs.len() as u64) + 1;
    audit_logs.push(AuditLog {
        id: log_id,
        actor: User::new(1, "admin".to_string(), None),
        action: "org.member_role_update".to_string(),
        target_type: "org".to_string(),
        target_name: org_name.clone(),
        details: format!(
            "Updated role of member '{}' to '{}' in org '{}'",
            username, payload.role, org_name
        ),
        ip_address: Some("127.0.0.1".to_string()),
        created_at: chrono::Utc::now().to_rfc3339(),
    });
    StatusCode::OK
}

pub async fn remove_org_member(Path((_org, _username)): Path<(String, String)>) -> StatusCode {
    StatusCode::NO_CONTENT
}

pub async fn list_org_audit_logs(
    State(state): State<AppState>,
    Path(org_name): Path<String>,
) -> Json<Vec<AuditLog>> {
    let logs = state.audit_logs.read().unwrap_or_else(|e| e.into_inner());
    let filtered: Vec<AuditLog> = logs
        .iter()
        .filter(|l| l.target_name == org_name || l.target_type == "org")
        .cloned()
        .collect();
    Json(filtered)
}

pub async fn get_admin_stats(State(state): State<AppState>) -> Json<AdminStats> {
    let users_count = state.users.read().unwrap_or_else(|e| e.into_inner()).len() as u64;
    let repos_count = state.repos.read().unwrap_or_else(|e| e.into_inner()).len() as u64;
    let issues_count = state.issues.read().unwrap_or_else(|e| e.into_inner()).len() as u64;
    let orgs_count = state.orgs.read().unwrap_or_else(|e| e.into_inner()).len() as u64;

    Json(AdminStats {
        users: users_count,
        repos: repos_count,
        orgs: orgs_count,
        issues: issues_count,
    })
}

pub async fn list_notices() -> Json<Vec<SystemNotice>> {
    let notices = vec![SystemNotice {
        id: 1,
        type_: "info".to_string(),
        description: "System maintenance at 00:00".to_string(),
    }];
    Json(notices)
}

use crate::state::AppState;
use axum::extract::State;

pub async fn admin_list_users(State(state): State<AppState>) -> Json<Vec<User>> {
    let users = state.users.read().unwrap_or_else(|e| e.into_inner());
    Json(users.clone())
}

pub async fn admin_create_user(
    State(state): State<AppState>,
    Json(payload): Json<shared::RegisterOption>,
) -> (StatusCode, Json<User>) {
    let mut users = state.users.write().unwrap_or_else(|e| e.into_inner());
    if users
        .iter()
        .any(|u| u.username == payload.username || u.email == Some(payload.email.clone()))
    {
        return (
            StatusCode::CONFLICT,
            Json(User::new(0, "".to_string(), None)),
        );
    }
    let id = (users.len() as u64) + 1;
    let user = User::new(id, payload.username, Some(payload.email));
    users.push(user.clone());
    (StatusCode::CREATED, Json(user))
}

pub async fn admin_edit_user(
    State(state): State<AppState>,
    Path(username): Path<String>,
    Json(payload): Json<AdminUserEditOption>,
) -> StatusCode {
    let mut users = state.users.write().unwrap_or_else(|e| e.into_inner());
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
    Path(username): Path<String>,
) -> StatusCode {
    let mut users = state.users.write().unwrap_or_else(|e| e.into_inner());
    if let Some(pos) = users.iter().position(|u| u.username == username) {
        users.remove(pos);
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}

// Miscellaneous Handlers

pub async fn list_licenses() -> Json<Vec<LicenseTemplate>> {
    vec![LicenseTemplate {
        key: "mit".to_string(),
        name: "MIT License".to_string(),
        url: "http://...".to_string(),
    }]
    .into()
}

pub async fn list_gitignores() -> Json<Vec<GitignoreTemplate>> {
    vec![GitignoreTemplate {
        name: "Rust".to_string(),
        source: "target/".to_string(),
    }]
    .into()
}

pub async fn get_repo_languages(
    Path((_owner, _repo)): Path<(String, String)>,
) -> Json<Vec<LanguageStat>> {
    vec![LanguageStat {
        language: "Rust".to_string(),
        percentage: 100,
        color: "#dea584".to_string(),
    }]
    .into()
}
