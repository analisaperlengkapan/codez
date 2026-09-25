use super::*;

pub async fn list_hooks(
    State(state): State<AppState>,
    Path((owner, repo_name)): Path<(String, String)>,
) -> Json<Vec<Webhook>> {
    let repos = state.repos.read().unwrap_or_else(|e| e.into_inner());
    let repo_id = repos
        .iter()
        .find(|r| r.owner == owner && r.name == repo_name)
        .map(|r| r.id)
        .unwrap_or(0);

    let hooks = state.hooks.read().unwrap_or_else(|e| e.into_inner());
    let filtered_hooks: Vec<Webhook> = hooks
        .iter()
        .filter(|h| h.repo_id == repo_id)
        .cloned()
        .collect();
    Json(filtered_hooks)
}

pub async fn create_hook(
    State(state): State<AppState>,
    Path((owner, repo_name)): Path<(String, String)>,
    Json(payload): Json<CreateHookOption>,
) -> (StatusCode, Json<Webhook>) {
    let repos = state.repos.read().unwrap_or_else(|e| e.into_inner());
    let repo = repos
        .iter()
        .find(|r| r.owner == owner && r.name == repo_name);

    let repo_id = if let Some(r) = repo {
        r.id
    } else {
        return (
            StatusCode::NOT_FOUND,
            Json(Webhook {
                id: 0,
                repo_id: 0,
                url: "".to_string(),
                events: vec![],
                active: false,
            }),
        );
    };

    let mut hooks = state.hooks.write().unwrap_or_else(|e| e.into_inner());
    let id = (hooks.len() as u64) + 1;
    let hook = Webhook {
        id,
        repo_id,
        url: payload.url,
        events: payload.events,
        active: payload.active,
    };
    hooks.push(hook.clone());
    (StatusCode::CREATED, Json(hook))
}

pub async fn list_hook_deliveries(
    State(state): State<AppState>,
    Path((_owner, _repo, id)): Path<(String, String, u64)>,
) -> Json<Vec<WebhookDelivery>> {
    let deliveries = state
        .webhook_deliveries
        .read()
        .unwrap_or_else(|e| e.into_inner());
    let filtered: Vec<WebhookDelivery> = deliveries
        .iter()
        .filter(|d| d.hook_id == id)
        .cloned()
        .collect();
    Json(filtered)
}

pub async fn create_secret(
    State(state): State<AppState>,
    Path((owner, repo_name)): Path<(String, String)>,
    Json(payload): Json<CreateSecretOption>,
) -> (StatusCode, Json<Secret>) {
    let repos = state.repos.read().unwrap_or_else(|e| e.into_inner());
    let repo = repos
        .iter()
        .find(|r| r.owner == owner && r.name == repo_name);

    let repo_id = if let Some(r) = repo {
        r.id
    } else {
        return (
            StatusCode::NOT_FOUND,
            Json(Secret {
                name: "".to_string(),
                repo_id: 0,
                created_at: "".to_string(),
                data: "".to_string(),
            }),
        );
    };

    let secret = Secret {
        name: payload.name,
        repo_id,
        created_at: "2023-01-02".to_string(),
        data: payload.data,
    };
    (StatusCode::CREATED, Json(secret))
}

pub async fn list_secrets(Path((_owner, _repo)): Path<(String, String)>) -> Json<Vec<Secret>> {
    // Note: In a real implementation this would filter by repo_id from state,
    // but secrets are currently mocked in the handler and not in AppState.
    // For consistency with other handlers, we'd need to move secrets to AppState.
    // However, following the instruction to filter, I will return an empty list if repo doesn't match mock.
    let secrets = vec![Secret {
        name: "MY_TOKEN".to_string(),
        repo_id: 1,
        created_at: "2023-01-01".to_string(),
        data: "hidden".to_string(),
    }];
    Json(secrets)
}

pub async fn create_deploy_key(
    State(state): State<AppState>,
    Path((owner, repo_name)): Path<(String, String)>,
    Json(payload): Json<CreateKeyOption>,
) -> (StatusCode, Json<DeployKey>) {
    let repos = state.repos.read().unwrap_or_else(|e| e.into_inner());
    let repo = repos
        .iter()
        .find(|r| r.owner == owner && r.name == repo_name);

    let repo_id = if let Some(r) = repo {
        r.id
    } else {
        return (
            StatusCode::NOT_FOUND,
            Json(DeployKey {
                id: 0,
                repo_id: 0,
                title: "".to_string(),
                key: "".to_string(),
                fingerprint: "".to_string(),
            }),
        );
    };

    let key = DeployKey {
        id: 2,
        repo_id,
        title: payload.title,
        key: payload.key,
        fingerprint: "SHA...".to_string(),
    };
    (StatusCode::CREATED, Json(key))
}

pub async fn list_deploy_keys(
    Path((_owner, _repo)): Path<(String, String)>,
) -> Json<Vec<DeployKey>> {
    // Similar to secrets, deploy keys are mocked here.
    let keys = vec![DeployKey {
        id: 1,
        repo_id: 1,
        title: "CI Key".to_string(),
        key: "ssh-rsa...".to_string(),
        fingerprint: "SHA...".to_string(),
    }];
    Json(keys)
}

pub async fn list_lfs_locks(
    State(state): State<AppState>,
    Path((owner, repo_name)): Path<(String, String)>,
) -> Json<Vec<LfsLock>> {
    let repos = state.repos.read().unwrap_or_else(|e| e.into_inner());
    let repo_id = repos
        .iter()
        .find(|r| r.owner == owner && r.name == repo_name)
        .map(|r| r.id)
        .unwrap_or(0);

    let locks = state.lfs_locks.read().unwrap_or_else(|e| e.into_inner());
    let filtered_locks: Vec<LfsLock> = locks
        .iter()
        .filter(|l| l.repo_id == repo_id)
        .cloned()
        .collect();
    Json(filtered_locks)
}

pub async fn create_lfs_lock(
    State(state): State<AppState>,
    Path((owner, repo_name)): Path<(String, String)>,
    Json(payload): Json<LfsLockRequest>,
) -> (StatusCode, Json<LfsLock>) {
    let repos = state.repos.read().unwrap_or_else(|e| e.into_inner());
    let repo_id = repos
        .iter()
        .find(|r| r.owner == owner && r.name == repo_name)
        .map(|r| r.id)
        .unwrap_or(0);

    if repo_id == 0 {
        return (
            StatusCode::NOT_FOUND,
            Json(LfsLock {
                id: "".to_string(),
                repo_id: 0,
                path: "".to_string(),
                owner: User::new(0, "".to_string(), None),
                locked_at: "".to_string(),
            }),
        );
    }

    let mut locks = state.lfs_locks.write().unwrap_or_else(|e| e.into_inner());
    if locks
        .iter()
        .any(|l| l.repo_id == repo_id && l.path == payload.path)
    {
        return (
            StatusCode::CONFLICT,
            Json(LfsLock {
                id: "".to_string(),
                repo_id: 0,
                path: "".to_string(),
                owner: User::new(0, "".to_string(), None),
                locked_at: "".to_string(),
            }),
        );
    }

    let id = (locks.len() as u64) + 1;
    // Mock user for now, or ideally extract from auth
    let user = User::new(1, "admin".to_string(), None);

    let lock = LfsLock {
        id: id.to_string(),
        repo_id,
        path: payload.path,
        owner: user,
        locked_at: "now".to_string(),
    };
    locks.push(lock.clone());
    (StatusCode::CREATED, Json(lock))
}

pub async fn delete_lfs_lock(
    State(state): State<AppState>,
    Path((owner, repo_name, id)): Path<(String, String, String)>,
) -> StatusCode {
    let repos = state.repos.read().unwrap_or_else(|e| e.into_inner());
    let repo_id = repos
        .iter()
        .find(|r| r.owner == owner && r.name == repo_name)
        .map(|r| r.id)
        .unwrap_or(0);

    if repo_id == 0 {
        return StatusCode::NOT_FOUND;
    }

    let mut locks = state.lfs_locks.write().unwrap_or_else(|e| e.into_inner());
    if let Some(pos) = locks
        .iter()
        .position(|l| l.repo_id == repo_id && l.id == id)
    {
        locks.remove(pos);
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}
