//! Instance administration: stats, notices and user management.

use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
};
use shared::{AdminStats, AdminUserEditOption, SystemNotice, User};

use crate::state::AppState;

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
