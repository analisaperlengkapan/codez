use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
};
use shared::{
    LoginOption, User, RegisterOption, UserSettingsOption, Notification, PublicKey, CreateKeyOption,
    GpgKey, CreateGpgKeyOption, Activity, EmailAddress, OAuth2Application, Package, TwoFactor, OAuth2Provider,
    Contribution
};
use crate::router::AppState;

pub async fn login_user(State(state): State<AppState>, Json(payload): Json<LoginOption>) -> (StatusCode, Json<Option<User>>) {
    let users = state.users.read().unwrap();
    if let Some(user) = users.iter().find(|u| u.username == payload.username) {
        if payload.password == "password" {
             return (StatusCode::OK, Json(Some(user.clone())));
        }
    }
    (StatusCode::UNAUTHORIZED, Json(None))
}

pub async fn register_user(State(state): State<AppState>, Json(payload): Json<RegisterOption>) -> (StatusCode, Json<User>) {
    let mut users = state.users.write().unwrap();
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

pub async fn update_user_settings(Json(_payload): Json<UserSettingsOption>) -> StatusCode {
    StatusCode::OK
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

pub async fn list_feeds() -> Json<Vec<Activity>> {
    let feeds = vec![
        Activity {
            id: 1,
            user_id: 1,
            user_name: "admin".to_string(),
            op_type: "push_branch".to_string(),
            content: "pushed to main".to_string(),
            created: "2023-01-01".to_string(),
        }
    ];
    Json(feeds)
}

pub async fn list_gpg_keys() -> Json<Vec<GpgKey>> {
    let keys = vec![
        GpgKey {
            id: 1,
            key_id: "ID".to_string(),
            primary_key_id: "PID".to_string(),
            public_key: "PUB".to_string(),
            emails: vec![],
        }
    ];
    Json(keys)
}

pub async fn create_gpg_key(Json(payload): Json<CreateGpgKeyOption>) -> (StatusCode, Json<GpgKey>) {
    let key = GpgKey {
        id: 2,
        key_id: "NEWID".to_string(),
        primary_key_id: "NEWPID".to_string(),
        public_key: payload.armored_public_key,
        emails: vec![],
    };
    (StatusCode::CREATED, Json(key))
}

pub async fn verify_gpg_key(Path(_id): Path<u64>) -> StatusCode {
    StatusCode::OK
}

pub async fn delete_ssh_key(Path(_id): Path<u64>) -> StatusCode {
    StatusCode::NO_CONTENT
}

pub async fn delete_gpg_key(Path(_id): Path<u64>) -> StatusCode {
    StatusCode::NO_CONTENT
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

pub async fn list_followers(Path(_username): Path<String>) -> Json<Vec<User>> {
    vec![User::new(2, "follower".to_string(), None)].into()
}

pub async fn list_following(Path(_username): Path<String>) -> Json<Vec<User>> {
    vec![User::new(3, "following".to_string(), None)].into()
}

pub async fn list_packages(Path(_owner): Path<String>) -> Json<Vec<Package>> {
    let pkgs = vec![
        Package { id: 1, name: "my-lib".to_string(), version: "1.0.0".to_string(), package_type: "cargo".to_string() }
    ];
    Json(pkgs)
}

pub async fn get_package_detail(Path((_owner, _type, _name, _version)): Path<(String, String, String, String)>) -> Json<Package> {
    Json(Package { id: 1, name: "pkg".to_string(), version: "1.0".to_string(), package_type: "npm".to_string() })
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
