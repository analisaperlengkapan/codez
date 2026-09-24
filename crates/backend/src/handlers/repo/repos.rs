use super::*;

pub async fn get_user_repo_status(
    State(state): State<AppState>,
    Path((owner, repo_name)): Path<(String, String)>,
) -> Json<RepoUserStatus> {
    let repos = state.repos.read().unwrap_or_else(|e| e.into_inner());
    let repo_id = repos
        .iter()
        .find(|r| r.owner == owner && r.name == repo_name)
        .map(|r| r.id)
        .unwrap_or(0);

    let user_id = 1; // Mock current user

    let stars = state.stars.read().unwrap_or_else(|e| e.into_inner());
    let starred = stars
        .get(&repo_id)
        .map(|users| users.contains(&user_id))
        .unwrap_or(false);

    let watchers = state.watchers.read().unwrap_or_else(|e| e.into_inner());
    let watching = watchers
        .get(&repo_id)
        .map(|users| users.contains(&user_id))
        .unwrap_or(false);

    Json(RepoUserStatus { starred, watching })
}

pub async fn list_repos(
    State(state): State<AppState>,
    Query(pagination): Query<PaginationOptions>,
) -> Json<Vec<Repository>> {
    let repos = state.repos.read().unwrap_or_else(|e| e.into_inner());
    let page = pagination.page.unwrap_or(1);
    let limit = pagination.limit.unwrap_or(10);

    let start = ((page - 1) * limit) as usize;
    let end = (start + limit as usize).min(repos.len());

    if start >= repos.len() {
        Json(vec![])
    } else {
        Json(repos[start..end].to_vec())
    }
}

pub async fn get_repo(
    State(state): State<AppState>,
    Path((owner, repo)): Path<(String, String)>,
) -> Json<Option<Repository>> {
    let repos = state.repos.read().unwrap_or_else(|e| e.into_inner());
    let r = repos
        .iter()
        .find(|r| r.owner == owner && r.name == repo)
        .cloned();
    Json(r)
}

pub async fn create_repo(
    State(state): State<AppState>,
    Json(payload): Json<CreateRepoOption>,
) -> impl IntoResponse {
    let mut repos = state.repos.write().unwrap_or_else(|e| e.into_inner());
    if repos
        .iter()
        .any(|r| r.owner == "admin" && r.name == payload.name)
    {
        return (
            StatusCode::CONFLICT,
            Json(Repository::new(0, "".to_string(), "".to_string())),
        );
    }
    let id = (repos.len() as u64) + 1;
    let mut repo = Repository::new(id, payload.name.clone(), "admin".to_string());
    repo.private = payload.private;
    repo.description = payload.description.clone();
    if let Some(branch) = payload.default_branch.clone() {
        repo.default_branch = Some(branch);
    }
    if let Some(val) = payload.allow_rebase_merge {
        repo.allow_rebase_merge = val;
    }
    if let Some(val) = payload.allow_squash_merge {
        repo.allow_squash_merge = val;
    }
    if let Some(val) = payload.allow_merge_commit {
        repo.allow_merge_commit = val;
    }
    if let Some(val) = payload.has_issues {
        repo.has_issues = val;
    }
    if let Some(val) = payload.has_wiki {
        repo.has_wiki = val;
    }
    if let Some(val) = payload.has_projects {
        repo.has_projects = val;
    }
    repos.push(repo.clone());

    // Create initial files
    {
        let mut files = state
            .file_contents
            .write()
            .unwrap_or_else(|e| e.into_inner());
        let mut history = state
            .file_history
            .write()
            .unwrap_or_else(|e| e.into_inner());
        let default_branch = payload.default_branch.clone().unwrap_or("main".to_string());

        let readme_content = format!(
            "# {}\n\n{}",
            payload.name,
            payload.description.clone().unwrap_or_default()
        );
        files.insert(
            (id, default_branch.clone(), "README.md".to_string()),
            readme_content.clone(),
        );
        history.insert(
            (id, default_branch.clone(), "README.md".to_string()),
            readme_content,
        );

        if let Some(gitignores) = &payload.gitignores {
            let ignore_content = format!("# {}\n\ntarget/\n", gitignores);
            files.insert(
                (id, default_branch.clone(), ".gitignore".to_string()),
                ignore_content.clone(),
            );
            history.insert(
                (id, default_branch, ".gitignore".to_string()),
                ignore_content,
            );
        }
    }

    // Create initial commit
    let mut commits = state.commits.write().unwrap_or_else(|e| e.into_inner());
    commits.push(Commit {
        sha: format!("init{}", id),
        repo_id: id,
        message: "Initial commit".to_string(),
        author: User::new(1, "admin".to_string(), None),
        date: "now".to_string(),
    });

    // Log activity
    let mut activities = state.activities.write().unwrap_or_else(|e| e.into_inner());
    let activity_id = (activities.len() as u64) + 1;
    activities.push(Activity {
        id: activity_id,
        repo_id: id,
        user_id: 1,
        user_name: "admin".to_string(),
        op_type: "create_repo".to_string(),
        content: format!("created repository {}", payload.name),
        created: "now".to_string(),
    });

    (StatusCode::CREATED, Json(repo))
}

pub async fn star_repo(
    State(state): State<AppState>,
    Path((owner, repo_name)): Path<(String, String)>,
) -> StatusCode {
    let mut repos = state.repos.write().unwrap_or_else(|e| e.into_inner());
    let repo = repos
        .iter_mut()
        .find(|r| r.owner == owner && r.name == repo_name);

    if let Some(r) = repo {
        let repo_id = r.id;
        let user_id = 1; // Mock current user

        let mut stars = state.stars.write().unwrap_or_else(|e| e.into_inner());
        let users = stars.entry(repo_id).or_insert(Vec::new());

        if let Some(pos) = users.iter().position(|u| *u == user_id) {
            users.remove(pos);
            if r.stars_count > 0 {
                r.stars_count -= 1;
            }
        } else {
            users.push(user_id);
            r.stars_count += 1;
        }
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}

pub async fn watch_repo(
    State(state): State<AppState>,
    Path((owner, repo_name)): Path<(String, String)>,
) -> StatusCode {
    let mut repos = state.repos.write().unwrap_or_else(|e| e.into_inner());
    let repo = repos
        .iter_mut()
        .find(|r| r.owner == owner && r.name == repo_name);

    if let Some(r) = repo {
        let repo_id = r.id;
        let user_id = 1; // Mock current user

        let mut watchers = state.watchers.write().unwrap_or_else(|e| e.into_inner());
        let users = watchers.entry(repo_id).or_insert(Vec::new());

        if let Some(pos) = users.iter().position(|u| *u == user_id) {
            users.remove(pos);
            if r.watchers_count > 0 {
                r.watchers_count -= 1;
            }
        } else {
            users.push(user_id);
            r.watchers_count += 1;
        }
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}

pub async fn fork_repo(
    State(state): State<AppState>,
    Path((owner, repo)): Path<(String, String)>,
) -> (StatusCode, Json<Option<Repository>>) {
    let mut repos = state.repos.write().unwrap_or_else(|e| e.into_inner());

    if let Some(orig_idx) = repos
        .iter()
        .position(|r| r.owner == owner && r.name == repo)
    {
        repos[orig_idx].forks_count += 1;
        let orig = repos[orig_idx].clone();

        let id = (repos.len() as u64) + 1;
        let new_name = format!("{}-fork", repo);
        let mut new_repo = Repository::new(id, new_name.clone(), "admin".to_string());
        new_repo.parent_id = Some(orig.id);
        repos.push(new_repo.clone());

        // Copy files
        {
            let mut files = state
                .file_contents
                .write()
                .unwrap_or_else(|e| e.into_inner());
            let mut new_files = Vec::new();

            for ((r_id, branch, path), content) in files.iter() {
                if *r_id == orig.id {
                    new_files.push((branch.clone(), path.clone(), content.clone()));
                }
            }

            for (branch, path, content) in new_files {
                files.insert((id, branch, path), content);
            }
        }

        // Copy history
        {
            let history = state.file_history.read().unwrap_or_else(|e| e.into_inner());
            let mut new_history = Vec::new();

            for ((r_id, branch, path), content) in history.iter() {
                if *r_id == orig.id {
                    new_history.push((branch.clone(), path.clone(), content.clone()));
                }
            }
            drop(history); // release read lock

            let mut history = state
                .file_history
                .write()
                .unwrap_or_else(|e| e.into_inner());
            for (branch, path, content) in new_history {
                history.insert((id, branch, path), content);
            }
        }

        // Log activity
        let mut activities = state.activities.write().unwrap_or_else(|e| e.into_inner());
        let activity_id = (activities.len() as u64) + 1;
        activities.push(Activity {
            id: activity_id,
            repo_id: id,
            user_id: 1,
            user_name: "admin".to_string(),
            op_type: "fork_repo".to_string(),
            content: format!("forked {}/{} to admin/{}", owner, repo, new_name),
            created: "now".to_string(),
        });

        (StatusCode::CREATED, Json(Some(new_repo)))
    } else {
        (StatusCode::NOT_FOUND, Json(None))
    }
}

pub async fn get_repo_settings(
    State(state): State<AppState>,
    Path((owner, repo_name)): Path<(String, String)>,
) -> (StatusCode, Json<RepoSettingsOption>) {
    let repos = state.repos.read().unwrap_or_else(|e| e.into_inner());
    if let Some(repo) = repos
        .iter()
        .find(|r| r.owner == owner && r.name == repo_name)
    {
        (
            StatusCode::OK,
            Json(RepoSettingsOption {
                description: repo.description.clone(),
                private: Some(repo.private),
                website: repo.website.clone(),
                default_branch: repo.default_branch.clone(),
                allow_rebase_merge: Some(repo.allow_rebase_merge),
                allow_squash_merge: Some(repo.allow_squash_merge),
                allow_merge_commit: Some(repo.allow_merge_commit),
                has_issues: Some(repo.has_issues),
                has_wiki: Some(repo.has_wiki),
                has_projects: Some(repo.has_projects),
            }),
        )
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(RepoSettingsOption {
                description: None,
                private: None,
                website: None,
                default_branch: None,
                allow_rebase_merge: None,
                allow_squash_merge: None,
                allow_merge_commit: None,
                has_issues: None,
                has_wiki: None,
                has_projects: None,
            }),
        )
    }
}

pub async fn update_repo_settings(
    State(state): State<AppState>,
    Path((owner, repo_name)): Path<(String, String)>,
    Json(payload): Json<RepoSettingsOption>,
) -> StatusCode {
    let mut repos = state.repos.write().unwrap_or_else(|e| e.into_inner());
    if let Some(repo) = repos
        .iter_mut()
        .find(|r| r.owner == owner && r.name == repo_name)
    {
        if let Some(desc) = payload.description {
            repo.description = Some(desc);
        }
        if let Some(private) = payload.private {
            repo.private = private;
        }
        if let Some(website) = payload.website {
            repo.website = Some(website);
        }
        if let Some(branch) = payload.default_branch {
            repo.default_branch = Some(branch);
        }
        if let Some(val) = payload.allow_rebase_merge {
            repo.allow_rebase_merge = val;
        }
        if let Some(val) = payload.allow_squash_merge {
            repo.allow_squash_merge = val;
        }
        if let Some(val) = payload.allow_merge_commit {
            repo.allow_merge_commit = val;
        }
        if let Some(val) = payload.has_issues {
            repo.has_issues = val;
        }
        if let Some(val) = payload.has_wiki {
            repo.has_wiki = val;
        }
        if let Some(val) = payload.has_projects {
            repo.has_projects = val;
        }
        StatusCode::OK
    } else {
        StatusCode::NOT_FOUND
    }
}

pub async fn mirror_sync(Path((_owner, _repo)): Path<(String, String)>) -> StatusCode {
    StatusCode::OK
}

pub async fn migrate_repo(
    State(state): State<AppState>,
    Json(payload): Json<MigrateRepoOption>,
) -> (StatusCode, Json<Repository>) {
    let owner = "admin".to_string(); // Mock authenticated user

    let mut repos = state.repos.write().unwrap_or_else(|e| e.into_inner());

    // Check for duplicate repo name (consistent with create_repo)
    if repos
        .iter()
        .any(|r| r.owner == owner && r.name == payload.repo_name)
    {
        return (
            StatusCode::CONFLICT,
            Json(Repository::new(0, "".to_string(), "".to_string())),
        );
    }

    let repo_id = (repos.len() as u64) + 1;

    let mut repo = Repository::new(repo_id, payload.repo_name.clone(), owner.clone());
    repo.description = Some(format!(
        "Migrated from {} ({})",
        payload.clone_addr, payload.service
    ));
    repo.is_mirror = payload.mirror;
    repos.push(repo.clone());
    drop(repos);

    // Create mock commit history based on the service
    let mut commits = state.commits.write().unwrap_or_else(|e| e.into_inner());
    commits.push(Commit {
        sha: format!("migrated-sha-{}", repo_id),
        repo_id,
        message: format!("Initial commit migrated from {}", payload.service),
        author: User::new(1, owner.clone(), Some("admin@example.com".to_string())),
        date: chrono::Utc::now().to_rfc3339(),
    });
    drop(commits);

    if payload.service != "git" {
        // Mock some issues and PRs being imported if it's from a richer platform like github/gitlab/gitea
        let mut issues = state.issues.write().unwrap_or_else(|e| e.into_inner());
        let issue_id = (issues.len() as u64) + 1;
        issues.push(Issue {
            id: issue_id,
            repo_id,
            number: 1,
            title: "Imported Issue 1".to_string(),
            body: Some(format!("This issue was imported from {}.", payload.service)),
            state: "open".to_string(),
            user: User::new(1, owner.clone(), None),
            assignees: vec![],
            labels: vec![],
            milestone: None,
            is_locked: false,
        });
        drop(issues);

        let mut pulls = state.pulls.write().unwrap_or_else(|e| e.into_inner());
        let pull_id = (pulls.len() as u64) + 1;
        pulls.push(PullRequest {
            id: pull_id,
            repo_id,
            number: 2,
            title: "Imported PR 1".to_string(),
            body: Some(format!("This PR was imported from {}.", payload.service)),
            state: "open".to_string(),
            user: User::new(1, owner.clone(), None),
            merged: false,
            head_sha: "headsha123".to_string(),
            base: "main".to_string(),
            head: "feature".to_string(),
        });
        drop(pulls);
    }

    (StatusCode::CREATED, Json(repo))
}

pub async fn transfer_repo(
    State(state): State<AppState>,
    Path((owner, repo_name)): Path<(String, String)>,
    Json(payload): Json<TransferRepoOption>,
) -> StatusCode {
    let users = state.users.read().unwrap_or_else(|e| e.into_inner());
    if !users.iter().any(|u| u.username == payload.new_owner) {
        return StatusCode::BAD_REQUEST;
    }
    drop(users); // Release user lock

    let mut repos = state.repos.write().unwrap_or_else(|e| e.into_inner());
    if let Some(repo) = repos
        .iter_mut()
        .find(|r| r.owner == owner && r.name == repo_name)
    {
        repo.owner = payload.new_owner.clone();
        let repo_id = repo.id;
        let r_name = repo.name.clone();

        // Drop repo lock before activity lock if possible, though repo->activity order is generally consistent.
        // But to be safe and avoid holding lock unnecessarily:
        drop(repos);

        let mut activities = state.activities.write().unwrap_or_else(|e| e.into_inner());
        let activity_id = (activities.len() as u64) + 1;
        activities.push(Activity {
            id: activity_id,
            repo_id,
            user_id: 1, // mock admin
            user_name: "admin".to_string(),
            op_type: "transfer_repo".to_string(),
            content: format!(
                "transferred repository {} from {} to {}",
                r_name, owner, payload.new_owner
            ),
            created: "now".to_string(),
        });

        StatusCode::ACCEPTED
    } else {
        StatusCode::NOT_FOUND
    }
}

pub async fn search_repos(
    State(state): State<AppState>,
    Query(params): Query<RepoSearchOptions>,
) -> Json<Vec<Repository>> {
    let repos = state.repos.read().unwrap_or_else(|e| e.into_inner());
    let q = params.q.to_lowercase();

    if q.is_empty() {
        Json(repos.clone())
    } else {
        let filtered: Vec<Repository> = repos
            .iter()
            .filter(|r| {
                r.name.to_lowercase().contains(&q)
                    || r.description
                        .clone()
                        .unwrap_or_default()
                        .to_lowercase()
                        .contains(&q)
            })
            .cloned()
            .collect();
        Json(filtered)
    }
}

pub async fn list_collaborators(
    Path((_owner, _repo)): Path<(String, String)>,
) -> Json<Vec<Collaborator>> {
    let user = User::new(2, "collab_user".to_string(), None);
    vec![Collaborator {
        user,
        repo_id: 1,
        permissions: "write".to_string(),
    }]
    .into()
}

pub async fn get_collaborator(
    Path((_owner, _repo, _collaborator)): Path<(String, String, String)>,
) -> Json<Option<Collaborator>> {
    Json(None)
}

pub async fn add_collaborator(
    Path((_owner, _repo, _collaborator)): Path<(String, String, String)>,
) -> StatusCode {
    StatusCode::NO_CONTENT
}
