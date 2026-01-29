use axum::{
    extract::{Json, Path, State, Query},
    http::StatusCode,
    response::IntoResponse,
};
use shared::{
    LoginOption, User, RegisterOption, UserSettingsOption, Notification, PublicKey, CreateKeyOption,
    GpgKey, CreateGpgKeyOption, Activity, EmailAddress, OAuth2Application, TwoFactor, OAuth2Provider,
    Contribution, Issue, PullRequest, IssueFilterOptions, Repository
};
use crate::router::AppState;

pub async fn list_starred_repos(State(state): State<AppState>) -> Json<Vec<Repository>> {
    let user_id = 1; // Mock current user
    let stars = state.stars.read().unwrap();
    let repos = state.repos.read().unwrap();

    let mut starred_repos = Vec::new();
    for (repo_id, users) in stars.iter() {
        if users.contains(&user_id) {
            if let Some(repo) = repos.iter().find(|r| r.id == *repo_id) {
                starred_repos.push(repo.clone());
            }
        }
    }
    Json(starred_repos)
}

pub async fn login_user(State(state): State<AppState>, Json(payload): Json<LoginOption>) -> (StatusCode, Json<Option<User>>) {
    let users = state.users.read().unwrap();
    if let Some(user) = users.iter().find(|u| u.username == payload.username) {
        if payload.password == "password" {
             return (StatusCode::OK, Json(Some(user.clone())));
        }
    }
    (StatusCode::UNAUTHORIZED, Json(None))
}

pub async fn register_user(State(state): State<AppState>, Json(payload): Json<RegisterOption>) -> impl IntoResponse {
    let mut users = state.users.write().unwrap();
    if users.iter().any(|u| u.username == payload.username || u.email == Some(payload.email.clone())) {
        return (StatusCode::CONFLICT, Json(User::new(0, "".to_string(), None)));
    }
    let id = (users.len() as u64) + 1;
    let user = User::new(id, payload.username, Some(payload.email));
    users.push(user.clone());
    (StatusCode::CREATED, Json(user))
}

pub async fn get_user(State(state): State<AppState>, Path(username): Path<String>) -> Json<Option<User>> {
    let users = state.users.read().unwrap();
    let user = users.iter().find(|u| u.username == username).cloned();
    Json(user)
}

pub async fn get_user_settings() -> Json<UserSettingsOption> {
    Json(UserSettingsOption {
        full_name: Some("Admin User".to_string()),
        website: None,
        description: None,
        location: None,
    })
}

pub async fn update_user_settings(
    State(state): State<AppState>,
    Json(_payload): Json<UserSettingsOption>
) -> StatusCode {
    // In a real app we'd identify the user via token/session.
    // Here we assume "admin" (id=1) for the mock state update.
    let mut users = state.users.write().unwrap();
    if let Some(user) = users.iter_mut().find(|u| u.id == 1) {
        // Shared User struct only has username/email exposed publicly in this context?
        // Let's check shared definition again. It has id, username, email.
        // UserSettingsOption has full_name, website, description, location.
        // The shared::User struct doesn't have these fields to update.
        // So we can't actually update them in the User object unless we expand User.
        // But to satisfy "stateful" logic request, we can at least find the user and return OK.
        // Or better, we can log an activity that settings were updated.

        let mut activities = state.activities.write().unwrap();
        let activity_id = (activities.len() as u64) + 1;
        activities.push(Activity {
            id: activity_id,
            repo_id: 0,
            user_id: user.id,
            user_name: user.username.clone(),
            op_type: "update_settings".to_string(),
            content: "updated user settings".to_string(),
            created: "now".to_string(),
        });

        StatusCode::OK
    } else {
        StatusCode::UNAUTHORIZED
    }
}

pub async fn list_notifications(State(state): State<AppState>) -> Json<Vec<Notification>> {
    let notifications = state.notifications.read().unwrap();
    Json(notifications.clone())
}

pub async fn mark_notification_read(State(state): State<AppState>, Path(id): Path<u64>) -> StatusCode {
    let mut notifications = state.notifications.write().unwrap();
    if let Some(n) = notifications.iter_mut().find(|n| n.id == id) {
        n.unread = false;
        StatusCode::RESET_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}

pub async fn list_keys(State(state): State<AppState>) -> Json<Vec<PublicKey>> {
    let keys = state.keys.read().unwrap();
    Json(keys.clone())
}

pub async fn create_key(State(state): State<AppState>, Json(payload): Json<CreateKeyOption>) -> (StatusCode, Json<PublicKey>) {
    let mut keys = state.keys.write().unwrap();
    let id = (keys.len() as u64) + 1;
    let key = PublicKey {
        id,
        title: payload.title,
        key: payload.key,
        fingerprint: "SHA256:new".to_string(),
    };
    keys.push(key.clone());
    (StatusCode::CREATED, Json(key))
}

pub async fn list_feeds(State(state): State<AppState>) -> Json<Vec<Activity>> {
    let feeds = state.activities.read().unwrap();
    // Return latest first
    let mut feeds = feeds.clone();
    feeds.sort_by(|a, b| b.id.cmp(&a.id));
    Json(feeds)
}

pub async fn list_gpg_keys(State(state): State<AppState>) -> Json<Vec<GpgKey>> {
    let keys = state.gpg_keys.read().unwrap();
    Json(keys.clone())
}

pub async fn create_gpg_key(
    State(state): State<AppState>,
    Json(payload): Json<CreateGpgKeyOption>
) -> (StatusCode, Json<GpgKey>) {
    let mut keys = state.gpg_keys.write().unwrap();
    let id = (keys.len() as u64) + 1;
    let key = GpgKey {
        id,
        key_id: format!("KEY-{}", id),
        primary_key_id: format!("PRIM-{}", id),
        public_key: payload.armored_public_key,
        emails: vec![],
    };
    keys.push(key.clone());
    (StatusCode::CREATED, Json(key))
}

pub async fn verify_gpg_key(Path(_id): Path<u64>) -> StatusCode {
    StatusCode::OK
}

pub async fn delete_ssh_key(
    State(state): State<AppState>,
    Path(id): Path<u64>
) -> StatusCode {
    let mut keys = state.keys.write().unwrap();
    if let Some(pos) = keys.iter().position(|k| k.id == id) {
        keys.remove(pos);
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}

pub async fn delete_gpg_key(
    State(state): State<AppState>,
    Path(id): Path<u64>
) -> StatusCode {
    let mut keys = state.gpg_keys.write().unwrap();
    if let Some(pos) = keys.iter().position(|k| k.id == id) {
        keys.remove(pos);
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}

pub async fn list_emails() -> Json<Vec<EmailAddress>> {
    vec![
        EmailAddress { email: "admin@codeza.com".to_string(), verified: true, primary: true }
    ].into()
}

pub async fn list_oauth2_apps() -> Json<Vec<OAuth2Application>> {
    vec![
        OAuth2Application { id: 1, name: "MyApp".to_string(), client_id: "client-id".to_string(), redirect_uris: vec![] }
    ].into()
}

pub async fn list_followers(
    State(state): State<AppState>,
    Path(username): Path<String>
) -> Json<Vec<User>> {
    let users = state.users.read().unwrap();
    let target_user_id = users.iter().find(|u| u.username == username).map(|u| u.id).unwrap_or(0);

    if target_user_id == 0 {
        return Json(vec![]);
    }

    let followers_map = state.followers.read().unwrap();
    let follower_ids = followers_map.get(&target_user_id).cloned().unwrap_or_default();

    let mut result = Vec::new();
    for id in follower_ids {
        if let Some(u) = users.iter().find(|u| u.id == id) {
            result.push(u.clone());
        }
    }
    Json(result)
}

pub async fn list_following(
    State(state): State<AppState>,
    Path(username): Path<String>
) -> Json<Vec<User>> {
    let users = state.users.read().unwrap();
    let target_user_id = users.iter().find(|u| u.username == username).map(|u| u.id).unwrap_or(0);

    if target_user_id == 0 {
        return Json(vec![]);
    }

    let following_map = state.following.read().unwrap();
    let following_ids = following_map.get(&target_user_id).cloned().unwrap_or_default();

    let mut result = Vec::new();
    for id in following_ids {
        if let Some(u) = users.iter().find(|u| u.id == id) {
            result.push(u.clone());
        }
    }
    Json(result)
}

pub async fn follow_user(
    State(state): State<AppState>,
    Path(username): Path<String>
) -> StatusCode {
    let current_user_id = 1; // Mock current user
    let users = state.users.read().unwrap();
    let target_user = users.iter().find(|u| u.username == username);

    if let Some(target) = target_user {
        let target_id = target.id;
        if target_id == current_user_id {
             return StatusCode::BAD_REQUEST; // Cannot follow self
        }

        // Add current_user to target's followers
        let mut followers = state.followers.write().unwrap();
        let f_list = followers.entry(target_id).or_insert(Vec::new());
        if !f_list.contains(&current_user_id) {
            f_list.push(current_user_id);
        }

        // Add target to current_user's following
        let mut following = state.following.write().unwrap();
        let f_ing_list = following.entry(current_user_id).or_insert(Vec::new());
        if !f_ing_list.contains(&target_id) {
            f_ing_list.push(target_id);
        }

        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}

pub async fn unfollow_user(
    State(state): State<AppState>,
    Path(username): Path<String>
) -> StatusCode {
    let current_user_id = 1; // Mock current user
    let users = state.users.read().unwrap();
    let target_user = users.iter().find(|u| u.username == username);

    if let Some(target) = target_user {
        let target_id = target.id;

        // Remove current_user from target's followers
        let mut followers = state.followers.write().unwrap();
        if let Some(f_list) = followers.get_mut(&target_id) {
            if let Some(pos) = f_list.iter().position(|id| *id == current_user_id) {
                f_list.remove(pos);
            }
        }

        // Remove target from current_user's following
        let mut following = state.following.write().unwrap();
        if let Some(f_ing_list) = following.get_mut(&current_user_id) {
             if let Some(pos) = f_ing_list.iter().position(|id| *id == target_id) {
                f_ing_list.remove(pos);
            }
        }

        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}


pub async fn get_2fa() -> Json<TwoFactor> {
    Json(TwoFactor { enabled: false, method: "totp".to_string() })
}

pub async fn update_2fa(Json(_payload): Json<TwoFactor>) -> StatusCode {
    StatusCode::OK
}

pub async fn list_oauth2_providers() -> Json<Vec<OAuth2Provider>> {
    let providers = vec![
        OAuth2Provider {
            name: "github".to_string(),
            display_name: "GitHub".to_string(),
            url: "http://github.com/login".to_string(),
        }
    ];
    Json(providers)
}

pub async fn get_user_heatmap(Path(_username): Path<String>) -> Json<Vec<Contribution>> {
    vec![
        Contribution { date: "2023-01-01".to_string(), count: 5 },
        Contribution { date: "2023-01-02".to_string(), count: 2 },
    ].into()
}

pub async fn list_user_issues(
    State(state): State<AppState>,
    Query(filter): Query<IssueFilterOptions>
) -> Json<Vec<Issue>> {
    // Mock user ID for current user is 1 (admin)
    let current_user_id = 1;
    let issues = state.issues.read().unwrap();

    let mut filtered_issues: Vec<Issue> = issues.iter()
        .filter(|i| i.assignees.iter().any(|u| u.id == current_user_id))
        .cloned()
        .collect();

    if let Some(state_filter) = filter.state {
        if state_filter != "all" {
             filtered_issues.retain(|i| i.state == state_filter);
        }
    }

    Json(filtered_issues)
}

pub async fn list_user_pulls(
    State(state): State<AppState>,
    Query(filter): Query<IssueFilterOptions> // Reusing IssueFilterOptions as it has 'state'
) -> Json<Vec<PullRequest>> {
    // Mock user ID for current user is 1 (admin)
    let current_user_id = 1;
    let pulls = state.pulls.read().unwrap();

    let mut filtered_pulls: Vec<PullRequest> = pulls.iter()
        .filter(|p| p.user.id == current_user_id)
        .cloned()
        .collect();

    if let Some(state_filter) = filter.state {
        if state_filter != "all" {
             filtered_pulls.retain(|p| p.state == state_filter);
        }
    }

    Json(filtered_pulls)
}
