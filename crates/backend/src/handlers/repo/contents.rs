use super::*;

pub async fn update_topics(
    State(state): State<AppState>,
    Path((owner, repo_name)): Path<(String, String)>,
    Json(payload): Json<RepoTopicOptions>,
) -> StatusCode {
    let repos = state.repos.read().unwrap_or_else(|e| e.into_inner());
    let repo = repos
        .iter()
        .find(|r| r.owner == owner && r.name == repo_name);

    let repo_id = if let Some(r) = repo {
        r.id
    } else {
        return StatusCode::NOT_FOUND;
    };

    let mut topics = state.topics.write().unwrap_or_else(|e| e.into_inner());
    // Remove old topics
    topics.retain(|t| t.repo_id != repo_id);

    // Add new topics
    for topic_name in payload.topics {
        let id = (topics.len() as u64) + 1; // Simple ID generation, might collide if we delete, but ok for mock
        topics.push(Topic {
            id,
            repo_id,
            name: topic_name,
            created: "now".to_string(),
        });
    }

    StatusCode::NO_CONTENT
}

pub async fn list_topics(
    State(state): State<AppState>,
    Path((owner, repo_name)): Path<(String, String)>,
) -> Json<Vec<Topic>> {
    let repos = state.repos.read().unwrap_or_else(|e| e.into_inner());
    let repo_id = repos
        .iter()
        .find(|r| r.owner == owner && r.name == repo_name)
        .map(|r| r.id)
        .unwrap_or(0);

    let topics = state.topics.read().unwrap_or_else(|e| e.into_inner());
    let filtered_topics: Vec<Topic> = topics
        .iter()
        .filter(|t| t.repo_id == repo_id)
        .cloned()
        .collect();
    Json(filtered_topics)
}

pub async fn get_contents(
    State(state): State<AppState>,
    Path((owner, repo_name, path)): Path<(String, String, String)>,
    Query(query): Query<GetContentQuery>,
) -> Json<Vec<FileEntry>> {
    let repos = state.repos.read().unwrap_or_else(|e| e.into_inner());
    let repo = repos
        .iter()
        .find(|r| r.owner == owner && r.name == repo_name);
    let repo_id = repo.map(|r| r.id).unwrap_or(0);

    if repo_id == 0 {
        return Json(vec![]);
    }

    let default_branch = repo
        .and_then(|r| r.default_branch.clone())
        .unwrap_or("main".to_string());
    let target_branch = query.ref_name.unwrap_or(default_branch);

    let all_files = state
        .file_contents
        .read()
        .unwrap_or_else(|e| e.into_inner());
    let mut entries = Vec::new();
    let mut dirs = std::collections::HashSet::new();

    let prefix = if path.is_empty() || path == "/" {
        "".to_string()
    } else {
        format!("{}/", path.trim_matches('/'))
    };

    for (k_repo_id, k_branch, k_path) in all_files.keys() {
        if *k_repo_id == repo_id && k_branch == &target_branch && k_path.starts_with(&prefix) {
            let relative_path = &k_path[prefix.len()..];
            if relative_path.is_empty() {
                continue;
            }

            if let Some(idx) = relative_path.find('/') {
                // It's a directory
                let dir_name = &relative_path[..idx];
                if dirs.insert(dir_name.to_string()) {
                    entries.push(FileEntry {
                        name: dir_name.to_string(),
                        path: format!("{}{}", prefix, dir_name),
                        kind: "dir".to_string(),
                        size: 0,
                    });
                }
            } else {
                // It's a file
                let size = all_files
                    .get(&(*k_repo_id, k_branch.clone(), k_path.clone()))
                    .map(|s| s.len())
                    .unwrap_or(0) as u64;
                entries.push(FileEntry {
                    name: relative_path.to_string(),
                    path: k_path.clone(),
                    kind: "file".to_string(),
                    size,
                });
            }
        }
    }
    Json(entries)
}

pub async fn get_root_contents(
    State(state): State<AppState>,
    Path((owner, repo)): Path<(String, String)>,
    Query(query): Query<GetContentQuery>,
) -> Json<Vec<FileEntry>> {
    get_contents(
        State(state),
        Path((owner, repo, "".to_string())),
        Query(query),
    )
    .await
}

pub async fn list_branches(
    State(state): State<AppState>,
    Path((owner, repo_name)): Path<(String, String)>,
) -> Json<Vec<Branch>> {
    let repos = state.repos.read().unwrap_or_else(|e| e.into_inner());
    let repo_id = repos
        .iter()
        .find(|r| r.owner == owner && r.name == repo_name)
        .map(|r| r.id)
        .unwrap_or(0);

    if repo_id == 0 {
        return Json(vec![]);
    }

    let files = state
        .file_contents
        .read()
        .unwrap_or_else(|e| e.into_inner());
    let mut branch_names = std::collections::HashSet::new();

    for (r_id, branch, _) in files.keys() {
        if *r_id == repo_id {
            branch_names.insert(branch.clone());
        }
    }

    // Always ensure "main" exists if files are empty but repo exists, or handle graceful fallback.
    // Ideally create_repo makes "main", so it should be there.
    if branch_names.is_empty() {
        // Fallback or empty
    }

    let user = User::new(1, "admin".to_string(), None);
    // Mock commit for branch tip
    let commit = Commit {
        sha: "mock_sha".to_string(),
        repo_id,
        message: "branch tip".to_string(),
        author: user,
        date: "now".to_string(),
    };

    let branches: Vec<Branch> = branch_names
        .into_iter()
        .map(|name| {
            Branch {
                name: name.clone(),
                repo_id,
                commit: commit.clone(),
                protected: name == "main", // Mock protection
            }
        })
        .collect();

    Json(branches)
}

pub async fn create_branch(
    State(state): State<AppState>,
    Path((owner, repo_name)): Path<(String, String)>,
    Json(payload): Json<CreateBranchOption>,
) -> (StatusCode, Json<Branch>) {
    let repos = state.repos.read().unwrap_or_else(|e| e.into_inner());
    let repo = repos
        .iter()
        .find(|r| r.owner == owner && r.name == repo_name);

    let repo_id = if let Some(r) = repo {
        r.id
    } else {
        return (
            StatusCode::NOT_FOUND,
            Json(Branch {
                repo_id: 0,
                name: "".to_string(),
                commit: Commit {
                    sha: "".to_string(),
                    repo_id: 0,
                    message: "".to_string(),
                    author: User::new(0, "".to_string(), None),
                    date: "".to_string(),
                },
                protected: false,
            }),
        );
    };

    let user = User::new(1, "admin".to_string(), None);
    let commit = Commit {
        sha: "def".to_string(),
        repo_id,
        message: "new branch".to_string(),
        author: user,
        date: "now".to_string(),
    };
    let branch = Branch {
        name: payload.name.clone(),
        repo_id,
        commit,
        protected: false,
    };

    // Copy files from base branch
    {
        let mut files = state
            .file_contents
            .write()
            .unwrap_or_else(|e| e.into_inner());

        // Check if branch already exists
        for (r_id, b_name, _) in files.keys() {
            if *r_id == repo_id && b_name == &payload.name {
                return (
                    StatusCode::CONFLICT,
                    Json(Branch {
                        repo_id: 0,
                        name: "".to_string(),
                        commit: Commit {
                            sha: "".to_string(),
                            repo_id: 0,
                            message: "".to_string(),
                            author: User::new(0, "".to_string(), None),
                            date: "".to_string(),
                        },
                        protected: false,
                    }),
                );
            }
        }

        // Validate base branch exists (has files)
        let base = payload.base.clone();
        let mut base_exists = false;
        for (r_id, b_name, _) in files.keys() {
            if *r_id == repo_id && b_name == &base {
                base_exists = true;
                break;
            }
        }

        if !base_exists {
            return (
                StatusCode::NOT_FOUND,
                Json(Branch {
                    repo_id: 0,
                    name: "".to_string(),
                    commit: Commit {
                        sha: "".to_string(),
                        repo_id: 0,
                        message: "".to_string(),
                        author: User::new(0, "".to_string(), None),
                        date: "".to_string(),
                    },
                    protected: false,
                }),
            );
        }

        let mut new_files = Vec::new();

        for ((r_id, b_name, path), content) in files.iter() {
            if *r_id == repo_id && b_name == &base {
                new_files.push((payload.name.clone(), path.clone(), content.clone()));
            }
        }

        for (b_name, path, content) in new_files {
            files.insert((repo_id, b_name, path), content);
        }
    }

    // Initialize history for the new branch with current content of base branch
    {
        let files = state
            .file_contents
            .read()
            .unwrap_or_else(|e| e.into_inner());
        let mut history = state
            .file_history
            .write()
            .unwrap_or_else(|e| e.into_inner());
        let base = payload.base.clone();

        for ((r_id, b_name, path), content) in files.iter() {
            if *r_id == repo_id && b_name == &base {
                history.insert(
                    (repo_id, payload.name.clone(), path.clone()),
                    content.clone(),
                );
            }
        }
    }

    (StatusCode::CREATED, Json(branch))
}

pub async fn list_tags(Path((_owner, _repo)): Path<(String, String)>) -> Json<Vec<Tag>> {
    let user = User::new(1, "admin".to_string(), None);
    let commit = Commit {
        sha: "abc".to_string(),
        repo_id: 1,
        message: "init".to_string(),
        author: user,
        date: "now".to_string(),
    };
    let tags = vec![Tag {
        name: "v1.0".to_string(),
        repo_id: 1,
        id: "1".to_string(),
        commit,
    }];
    Json(tags)
}

pub async fn upload_media(
    Path((_owner, _repo)): Path<(String, String)>,
) -> (StatusCode, Json<LfsObject>) {
    let lfs = LfsObject {
        oid: "abc1234567890".to_string(),
        size: 1024,
        created_at: "2023-01-01".to_string(),
    };
    (StatusCode::CREATED, Json(lfs))
}

pub async fn list_commits(
    State(state): State<AppState>,
    Path((owner, repo_name)): Path<(String, String)>,
) -> Json<Vec<Commit>> {
    let repos = state.repos.read().unwrap_or_else(|e| e.into_inner());
    let repo_id = repos
        .iter()
        .find(|r| r.owner == owner && r.name == repo_name)
        .map(|r| r.id)
        .unwrap_or(0);

    let commits = state.commits.read().unwrap_or_else(|e| e.into_inner());
    let filtered_commits: Vec<Commit> = commits
        .iter()
        .filter(|c| c.repo_id == repo_id)
        .cloned()
        .collect();
    Json(filtered_commits)
}

pub async fn search_repo_code(
    State(state): State<AppState>,
    Path((owner, repo_name)): Path<(String, String)>,
    Query(params): Query<RepoSearchOptions>,
) -> Json<Vec<CodeSearchResult>> {
    let repos = state.repos.read().unwrap_or_else(|e| e.into_inner());
    let repo = repos
        .iter()
        .find(|r| r.owner == owner && r.name == repo_name);
    let repo_id = repo.map(|r| r.id).unwrap_or(0);
    let q = params.q.to_lowercase();

    if repo_id == 0 {
        return Json(vec![]);
    }

    let default_branch = repo
        .and_then(|r| r.default_branch.clone())
        .unwrap_or("main".to_string());

    let files = state
        .file_contents
        .read()
        .unwrap_or_else(|e| e.into_inner());
    let mut results = Vec::new();

    for ((r_id, branch, path), content) in files.iter() {
        if *r_id == repo_id
            && branch == &default_branch
            && (q.is_empty()
                || path.to_lowercase().contains(&q)
                || content.to_lowercase().contains(&q))
        {
            results.push(CodeSearchResult {
                name: path.split('/').next_back().unwrap_or(path).to_string(),
                path: path.clone(),
                sha: "mocksha".to_string(),
                url: format!("/repos/{}/{}/src/{}", owner, repo_name, path),
                content: Some(content.chars().take(100).collect()),
            });
        }
    }
    Json(results)
}

pub async fn get_raw_file(
    State(state): State<AppState>,
    Path((owner, repo_name, path)): Path<(String, String, String)>,
    Query(query): Query<GetContentQuery>,
) -> impl IntoResponse {
    let repos = state.repos.read().unwrap_or_else(|e| e.into_inner());
    let repo = repos
        .iter()
        .find(|r| r.owner == owner && r.name == repo_name);
    let repo_id = repo.map(|r| r.id).unwrap_or(0);

    if repo_id == 0 {
        return (StatusCode::NOT_FOUND, "".to_string());
    }

    let default_branch = repo
        .and_then(|r| r.default_branch.clone())
        .unwrap_or("main".to_string());
    let target_branch = query.ref_name.unwrap_or(default_branch);

    let files = state
        .file_contents
        .read()
        .unwrap_or_else(|e| e.into_inner());
    if let Some(content) = files.get(&(repo_id, target_branch, path)) {
        (StatusCode::OK, content.clone())
    } else {
        (StatusCode::NOT_FOUND, "".to_string())
    }
}

pub async fn update_file(
    State(state): State<AppState>,
    Path((owner, repo, path)): Path<(String, String, String)>,
    Json(payload): Json<UpdateFileOption>,
) -> (StatusCode, Json<FileEntry>) {
    let repos = state.repos.read().unwrap_or_else(|e| e.into_inner());
    let repo_obj = repos.iter().find(|r| r.owner == owner && r.name == repo);

    let (repo_id, default_branch) = if let Some(r) = repo_obj {
        (r.id, r.default_branch.clone().unwrap_or("main".to_string()))
    } else {
        return (
            StatusCode::NOT_FOUND,
            Json(FileEntry {
                name: "".to_string(),
                path: "".to_string(),
                kind: "".to_string(),
                size: 0,
            }),
        );
    };

    // Check branch protection
    let branch_name = payload.branch.clone().unwrap_or(default_branch);
    {
        let protections = state
            .protected_branches
            .read()
            .unwrap_or_else(|e| e.into_inner());
        if let Some(protection) = protections
            .iter()
            .find(|p| p.repo_id == repo_id && p.name == branch_name)
        {
            if !protection.enable_push {
                return (
                    StatusCode::FORBIDDEN,
                    Json(FileEntry {
                        name: "".to_string(),
                        path: "".to_string(),
                        kind: "".to_string(),
                        size: 0,
                    }),
                );
            }
        }
    }

    // Update file content in state
    {
        let mut files = state
            .file_contents
            .write()
            .unwrap_or_else(|e| e.into_inner());
        files.insert(
            (repo_id, branch_name.clone(), path.clone()),
            payload.content.clone(),
        );
    }

    // Create a commit for the file update
    let mut commits = state.commits.write().unwrap_or_else(|e| e.into_inner());
    let commit_message = if payload.message.is_empty() {
        format!("Update {}", path)
    } else {
        payload.message.clone()
    };

    let commit_id = commits.len() + 1;
    commits.push(Commit {
        sha: format!("update{}", commit_id),
        repo_id,
        message: commit_message.clone(),
        author: User::new(1, "admin".to_string(), None),
        date: "now".to_string(),
    });

    // Log activity
    let mut activities = state.activities.write().unwrap_or_else(|e| e.into_inner());
    let activity_id = (activities.len() as u64) + 1;
    activities.push(Activity {
        id: activity_id,
        repo_id,
        user_id: 1,
        user_name: "admin".to_string(),
        op_type: "update_file".to_string(),
        content: format!("updated file {} in {}/{}", path, owner, repo),
        created: "now".to_string(),
    });

    // Trigger Webhooks
    if let Some(r) = repo_obj {
        let user = User::new(1, "admin".to_string(), None);
        let commit = Commit {
            sha: format!("update{}", commit_id),
            repo_id,
            message: commit_message,
            author: user.clone(),
            date: "now".to_string(),
        };

        let event = PushEvent {
            r#ref: format!("refs/heads/{}", branch_name),
            before: "0000000000000000000000000000000000000000".to_string(), // Mock
            after: commit.sha.clone(),
            repository: r.clone(),
            pusher: user.clone(),
            commits: vec![commit],
        };
        dispatch_hooks(&state, repo_id, "push", event);
    }

    (
        StatusCode::OK,
        Json(FileEntry {
            name: "updated_file".to_string(),
            path,
            kind: "file".to_string(),
            size: 123,
        }),
    )
}

pub async fn get_commit_diff(
    Path((_owner, _repo, _sha)): Path<(String, String, String)>,
) -> Json<Vec<DiffFile>> {
    let diffs = vec![DiffFile {
        name: "src/main.rs".to_string(),
        old_name: None,
        index: "123".to_string(),
        additions: 10,
        deletions: 5,
        type_: "modify".to_string(),
        lines: vec![
            DiffLine {
                line_no_old: Some(1),
                line_no_new: Some(1),
                content: " fn main() {".to_string(),
                type_: "context".to_string(),
            },
            DiffLine {
                line_no_old: Some(2),
                line_no_new: None,
                content: "-    println!(\"old\");".to_string(),
                type_: "delete".to_string(),
            },
            DiffLine {
                line_no_old: None,
                line_no_new: Some(2),
                content: "+    println!(\"new\");".to_string(),
                type_: "add".to_string(),
            },
        ],
    }];
    Json(diffs)
}

pub async fn list_branch_protections(
    State(state): State<AppState>,
    Path((owner, repo_name)): Path<(String, String)>,
) -> Json<Vec<ProtectedBranch>> {
    let repos = state.repos.read().unwrap_or_else(|e| e.into_inner());
    let repo_id = repos
        .iter()
        .find(|r| r.owner == owner && r.name == repo_name)
        .map(|r| r.id)
        .unwrap_or(0);

    let branches = state
        .protected_branches
        .read()
        .unwrap_or_else(|e| e.into_inner());
    let filtered: Vec<ProtectedBranch> = branches
        .iter()
        .filter(|b| b.repo_id == repo_id)
        .cloned()
        .collect();
    Json(filtered)
}

pub async fn create_branch_protection(
    State(state): State<AppState>,
    Path((owner, repo_name)): Path<(String, String)>,
    Json(payload): Json<CreateProtectedBranchOption>,
) -> (StatusCode, Json<ProtectedBranch>) {
    let repos = state.repos.read().unwrap_or_else(|e| e.into_inner());
    let repo_id = repos
        .iter()
        .find(|r| r.owner == owner && r.name == repo_name)
        .map(|r| r.id)
        .unwrap_or(0);

    if repo_id == 0 {
        return (
            StatusCode::NOT_FOUND,
            Json(ProtectedBranch {
                id: 0,
                repo_id: 0,
                name: "".to_string(),
                enable_push: false,
                enable_force_push: false,
                required_status_checks: vec![],
            }),
        );
    }

    let mut branches = state
        .protected_branches
        .write()
        .unwrap_or_else(|e| e.into_inner());
    if branches
        .iter()
        .any(|b| b.repo_id == repo_id && b.name == payload.name)
    {
        return (
            StatusCode::CONFLICT,
            Json(ProtectedBranch {
                id: 0,
                repo_id: 0,
                name: "".to_string(),
                enable_push: false,
                enable_force_push: false,
                required_status_checks: vec![],
            }),
        );
    }

    let id = branches.iter().map(|b| b.id).max().unwrap_or(0) + 1;
    let protection = ProtectedBranch {
        id,
        repo_id,
        name: payload.name,
        enable_push: payload.enable_push,
        enable_force_push: payload.enable_force_push,
        required_status_checks: payload.required_status_checks.unwrap_or_default(),
    };
    branches.push(protection.clone());
    (StatusCode::CREATED, Json(protection))
}

pub async fn delete_branch_protection(
    State(state): State<AppState>,
    Path((owner, repo_name, name)): Path<(String, String, String)>,
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

    let mut branches = state
        .protected_branches
        .write()
        .unwrap_or_else(|e| e.into_inner());
    if let Some(pos) = branches
        .iter()
        .position(|b| b.repo_id == repo_id && b.name == name)
    {
        branches.remove(pos);
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}

pub async fn create_commit_status(
    State(state): State<AppState>,
    Path((owner, repo_name, sha)): Path<(String, String, String)>,
    Json(payload): Json<CreateStatusOption>,
) -> (StatusCode, Json<CommitStatus>) {
    let repos = state.repos.read().unwrap_or_else(|e| e.into_inner());
    let repo_id = repos
        .iter()
        .find(|r| r.owner == owner && r.name == repo_name)
        .map(|r| r.id)
        .unwrap_or(0);

    if repo_id == 0 {
        return (
            StatusCode::NOT_FOUND,
            Json(CommitStatus {
                id: 0,
                sha: "".to_string(),
                state: "".to_string(),
                target_url: None,
                description: None,
                context: "".to_string(),
                created_at: "".to_string(),
                creator: User::new(0, "".to_string(), None),
            }),
        );
    }

    let mut statuses = state
        .commit_statuses
        .write()
        .unwrap_or_else(|e| e.into_inner());
    let id = (statuses.len() as u64) + 1;
    let status = CommitStatus {
        id,
        sha: sha.clone(),
        state: payload.state,
        target_url: payload.target_url,
        description: payload.description,
        context: payload.context.unwrap_or("default".to_string()),
        created_at: "now".to_string(),
        creator: User::new(1, "admin".to_string(), None),
    };
    statuses.push(status.clone());
    (StatusCode::CREATED, Json(status))
}

pub async fn list_commit_statuses(
    State(state): State<AppState>,
    Path((_owner, _repo, ref_name)): Path<(String, String, String)>,
) -> Json<Vec<CommitStatus>> {
    let statuses = state
        .commit_statuses
        .read()
        .unwrap_or_else(|e| e.into_inner());
    let filtered: Vec<CommitStatus> = statuses
        .iter()
        .filter(|s| s.sha == ref_name)
        .cloned()
        .collect();
    Json(filtered)
}
