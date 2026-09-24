use super::*;

pub async fn list_pulls(
    State(state): State<AppState>,
    Path((owner, repo_name)): Path<(String, String)>,
) -> Json<Vec<PullRequest>> {
    let repos = state.repos.read().unwrap_or_else(|e| e.into_inner());
    let repo_id = repos
        .iter()
        .find(|r| r.owner == owner && r.name == repo_name)
        .map(|r| r.id)
        .unwrap_or(0);

    let pulls = state.pulls.read().unwrap_or_else(|e| e.into_inner());
    let filtered_pulls: Vec<PullRequest> = pulls
        .iter()
        .filter(|p| p.repo_id == repo_id)
        .cloned()
        .collect();
    Json(filtered_pulls)
}

/// Repository-scoped lookup for a single pull request.
///
/// The `:index` path segment is the pull's repository-scoped `number` (the
/// value the list/detail links use), matching `update_pull`, `merge_pull` and
/// the review handlers. A missing pull is a `200` with a `null` body rather
/// than a `404`, matching the frontend `get_opt` helper.
pub async fn get_pull(
    State(state): State<AppState>,
    Path((owner, repo_name, index)): Path<(String, String, u64)>,
) -> Json<Option<PullRequest>> {
    let repos = state.repos.read().unwrap_or_else(|e| e.into_inner());
    let repo_id = repos
        .iter()
        .find(|r| r.owner == owner && r.name == repo_name)
        .map(|r| r.id)
        .unwrap_or(0);

    let pulls = state.pulls.read().unwrap_or_else(|e| e.into_inner());
    Json(
        pulls
            .iter()
            .find(|p| p.repo_id == repo_id && p.number == index)
            .cloned(),
    )
}

pub async fn create_pull(
    State(state): State<AppState>,
    Path((owner, repo_name)): Path<(String, String)>,
    Json(payload): Json<CreatePullRequestOption>,
) -> (StatusCode, Json<PullRequest>) {
    let repos = state.repos.read().unwrap_or_else(|e| e.into_inner());
    let repo_id = repos
        .iter()
        .find(|r| r.owner == owner && r.name == repo_name)
        .map(|r| r.id)
        .unwrap_or(0);

    if repo_id == 0 {
        return (
            StatusCode::NOT_FOUND,
            Json(PullRequest {
                id: 0,
                repo_id: 0,
                number: 0,
                title: "".to_string(),
                body: None,
                state: "".to_string(),
                user: User::new(0, "".to_string(), None),
                merged: false,
                head_sha: "".to_string(),
                base: "".to_string(),
                head: "".to_string(),
            }),
        );
    }

    // Validate branches exist
    {
        let files = state
            .file_contents
            .read()
            .unwrap_or_else(|e| e.into_inner());
        let has_head = files
            .keys()
            .any(|(r_id, b_name, _)| *r_id == repo_id && b_name == &payload.head);
        let has_base = files
            .keys()
            .any(|(r_id, b_name, _)| *r_id == repo_id && b_name == &payload.base);

        if !has_head || !has_base {
            return (
                StatusCode::BAD_REQUEST,
                Json(PullRequest {
                    id: 0,
                    repo_id: 0,
                    number: 0,
                    title: "".to_string(),
                    body: None,
                    state: "".to_string(),
                    user: User::new(0, "".to_string(), None),
                    merged: false,
                    head_sha: "".to_string(),
                    base: "".to_string(),
                    head: "".to_string(),
                }),
            );
        }
    }

    let mut pulls = state.pulls.write().unwrap_or_else(|e| e.into_inner());
    let id = pulls.iter().map(|p| p.id).max().unwrap_or(0) + 1;
    // `number` is scoped to the repository while `id` is global; new pulls must
    // continue each repository's own sequence so links and route lookups agree.
    let number = pulls
        .iter()
        .filter(|p| p.repo_id == repo_id)
        .map(|p| p.number)
        .max()
        .unwrap_or(0)
        + 1;
    let pr = PullRequest {
        id,
        repo_id,
        number,
        title: payload.title.clone(),
        body: payload.body.clone(),
        state: "open".to_string(),
        user: User::new(1, "admin".to_string(), None),
        merged: false,
        head_sha: format!("head_sha_{}", id),
        base: payload.base.clone(),
        head: payload.head.clone(),
    };
    pulls.push(pr.clone());

    // Log activity
    let mut activities = state.activities.write().unwrap_or_else(|e| e.into_inner());
    let activity_id = (activities.len() as u64) + 1;
    activities.push(Activity {
        id: activity_id,
        repo_id,
        user_id: 1,
        user_name: "admin".to_string(),
        op_type: "create_pull_request".to_string(),
        content: format!("opened pull request #{} in {}/{}", number, owner, repo_name),
        created: "now".to_string(),
    });

    // Notify repository owner (mock logic)
    let mut notifications = state
        .notifications
        .write()
        .unwrap_or_else(|e| e.into_inner());
    let notification_id = (notifications.len() as u64) + 1;
    notifications.push(Notification {
        id: notification_id,
        subject: format!("New pull request in {}: {}", repo_name, payload.title),
        unread: true,
        updated_at: "now".to_string(),
    });

    // Notify mentioned users
    if let Some(body) = &payload.body {
        let mentions = process_mentions(body);
        if !mentions.is_empty() {
            let users = state.users.read().unwrap_or_else(|e| e.into_inner());
            for username in mentions {
                if let Some(user) = users.iter().find(|u| u.username == username) {
                    // Don't notify self (mock admin id 1)
                    if user.id != 1 {
                        let nid = (notifications.len() as u64) + 1;
                        notifications.push(Notification {
                            id: nid,
                            subject: format!(
                                "You were mentioned in PR #{} in {}/{}",
                                number, owner, repo_name
                            ),
                            unread: true,
                            updated_at: "now".to_string(),
                        });
                    }
                }
            }
        }
    }

    // Trigger Webhooks
    if let Some(r) = repos.iter().find(|r| r.id == repo_id) {
        let event = PullRequestEvent {
            action: "opened".to_string(),
            pull_request: pr.clone(),
            repository: r.clone(),
            sender: User::new(1, "admin".to_string(), None),
        };
        dispatch_hooks(&state, repo_id, "pull_request", event);
    }

    (StatusCode::CREATED, Json(pr))
}

pub async fn update_pull(
    State(state): State<AppState>,
    Path((owner, repo_name, index)): Path<(String, String, u64)>,
    Json(payload): Json<UpdatePullRequestOption>,
) -> (StatusCode, Json<Option<PullRequest>>) {
    let repos = state.repos.read().unwrap_or_else(|e| e.into_inner());
    let repo_id = repos
        .iter()
        .find(|r| r.owner == owner && r.name == repo_name)
        .map(|r| r.id)
        .unwrap_or(0);

    if repo_id == 0 {
        return (StatusCode::NOT_FOUND, Json(None));
    }

    let mut pulls = state.pulls.write().unwrap_or_else(|e| e.into_inner());
    let pr = pulls
        .iter_mut()
        .find(|p| p.number == index && p.repo_id == repo_id);

    if let Some(p) = pr {
        if let Some(title) = payload.title {
            p.title = title;
        }
        if let Some(body) = payload.body {
            // Check for mentions
            let mentions = process_mentions(&body);
            if !mentions.is_empty() {
                let mut notifications = state
                    .notifications
                    .write()
                    .unwrap_or_else(|e| e.into_inner());
                let users = state.users.read().unwrap_or_else(|e| e.into_inner());
                for username in mentions {
                    if let Some(user) = users.iter().find(|u| u.username == username) {
                        if user.id != 1 {
                            let nid = (notifications.len() as u64) + 1;
                            notifications.push(Notification {
                                id: nid,
                                subject: format!(
                                    "You were mentioned in PR #{} in {}/{}",
                                    index, owner, repo_name
                                ),
                                unread: true,
                                updated_at: "now".to_string(),
                            });
                        }
                    }
                }
            }
            p.body = Some(body);
        }
        if let Some(state_val) = payload.state {
            p.state = state_val;
        }
        return (StatusCode::OK, Json(Some(p.clone())));
    }
    (StatusCode::NOT_FOUND, Json(None))
}

pub async fn add_reaction(
    State(state): State<AppState>,
    Path((_owner, _repo, id)): Path<(String, String, u64)>,
    Json(payload): Json<CreateReactionOption>,
) -> (StatusCode, Json<Reaction>) {
    let mut comments = state.comments.write().unwrap_or_else(|e| e.into_inner());
    if let Some(comment) = comments.iter_mut().find(|c| c.id == id) {
        let user = User::new(1, "admin".to_string(), None);
        let reaction_id = (comment.reactions.len() as u64) + 1;
        let reaction = Reaction {
            id: reaction_id,
            user,
            content: payload.content,
            created_at: "now".to_string(),
        };
        comment.reactions.push(reaction.clone());
        (StatusCode::CREATED, Json(reaction))
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(Reaction {
                id: 0,
                user: User::new(0, "".to_string(), None),
                content: "".to_string(),
                created_at: "".to_string(),
            }),
        )
    }
}

pub async fn merge_pull(
    State(state): State<AppState>,
    Path((owner, repo, index)): Path<(String, String, u64)>,
    Json(_payload): Json<MergePullRequestOption>,
) -> StatusCode {
    let repos = state.repos.read().unwrap_or_else(|e| e.into_inner());
    let repo_id = repos
        .iter()
        .find(|r| r.owner == owner && r.name == repo)
        .map(|r| r.id)
        .unwrap_or(0);

    if repo_id == 0 {
        return StatusCode::NOT_FOUND;
    }

    let mut pulls = state.pulls.write().unwrap_or_else(|e| e.into_inner());
    let pr_opt = pulls
        .iter_mut()
        .find(|p| p.repo_id == repo_id && p.number == index);

    if let Some(pr) = pr_opt {
        if pr.merged {
            return StatusCode::METHOD_NOT_ALLOWED;
        }

        // Check branch protection and status checks
        {
            let protections = state
                .protected_branches
                .read()
                .unwrap_or_else(|e| e.into_inner());
            if let Some(protection) = protections
                .iter()
                .find(|p| p.repo_id == repo_id && p.name == pr.base)
            {
                if !protection.required_status_checks.is_empty() {
                    let statuses = state
                        .commit_statuses
                        .read()
                        .unwrap_or_else(|e| e.into_inner());
                    for context in &protection.required_status_checks {
                        let latest = statuses
                            .iter()
                            .filter(|s| s.sha == pr.head_sha && &s.context == context)
                            .max_by_key(|s| s.id);

                        match latest {
                            Some(status) if status.state == "success" => continue,
                            _ => return StatusCode::CONFLICT,
                        }
                    }
                }
            }
        }

        // Copy modified files from head to base
        {
            let mut files = state
                .file_contents
                .write()
                .unwrap_or_else(|e| e.into_inner());
            let history = state.file_history.read().unwrap_or_else(|e| e.into_inner());
            let mut merged_files = Vec::new();
            let head = pr.head.clone();
            let base = pr.base.clone();

            for ((r_id, b_name, path), head_content) in files.iter() {
                if *r_id == repo_id && b_name == &head {
                    let history_key = (repo_id, head.clone(), path.clone());
                    let original_content_opt = history.get(&history_key);

                    // Check if file was modified on head compared to history baseline
                    let head_modified = match original_content_opt {
                        Some(original_content) => original_content != head_content,
                        None => true, // New file on head
                    };

                    if head_modified {
                        // Check for conflict: was it also modified on base?
                        let base_content_opt = files.get(&(repo_id, base.clone(), path.clone()));
                        let base_modified = match (base_content_opt, original_content_opt) {
                            (Some(base_content), Some(original_content)) => {
                                base_content != original_content
                            }
                            (Some(_), None) => true, // Created on both? Conflict unless identical
                            (None, Some(_)) => true, // Deleted on base? Conflict
                            (None, None) => false,   // Shouldn't happen if head is new
                        };

                        if base_modified {
                            // Simple conflict check: if content differs, it's a conflict
                            if base_content_opt != Some(head_content) {
                                return StatusCode::CONFLICT;
                            }
                        }

                        merged_files.push((path.clone(), head_content.clone()));
                    }
                }
            }

            for (path, content) in merged_files {
                files.insert((repo_id, base.clone(), path), content);
            }
        }

        pr.merged = true;
        pr.state = "closed".to_string();

        // Process closing keywords
        {
            let text = format!("{} {}", pr.title, pr.body.clone().unwrap_or_default());
            let closed_issues = process_closers(&text);
            if !closed_issues.is_empty() {
                let mut issues = state.issues.write().unwrap_or_else(|e| e.into_inner());
                let mut activities = state.activities.write().unwrap_or_else(|e| e.into_inner());
                for issue_id in closed_issues {
                    if let Some(issue) = issues
                        .iter_mut()
                        .find(|i| i.repo_id == repo_id && i.number == issue_id)
                    {
                        if issue.state != "closed" {
                            issue.state = "closed".to_string();

                            // Log activity
                            let activity_id = (activities.len() as u64) + 1;
                            activities.push(Activity {
                                id: activity_id,
                                repo_id,
                                user_id: 1, // mock admin
                                user_name: "admin".to_string(),
                                op_type: "close_issue".to_string(),
                                content: format!(
                                    "closed issue #{} via PR #{}",
                                    issue.number, index
                                ),
                                created: "now".to_string(),
                            });
                        }
                    }
                }
            }
        }

        // Create merge commit
        let mut commits = state.commits.write().unwrap_or_else(|e| e.into_inner());
        commits.push(Commit {
            sha: format!("merge{}", index),
            repo_id,
            message: format!("Merge pull request #{} from {}", index, pr.title),
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
            op_type: "merge_pull_request".to_string(),
            content: format!("merged pull request #{} in {}/{}", index, owner, repo),
            created: "now".to_string(),
        });

        StatusCode::OK
    } else {
        StatusCode::NOT_FOUND
    }
}

pub async fn get_pr_files(
    Path((_owner, _repo, _index)): Path<(String, String, u64)>,
) -> Json<Vec<DiffFile>> {
    let diffs = vec![DiffFile {
        name: "src/lib.rs".to_string(),
        old_name: None,
        index: "idx".to_string(),
        additions: 2,
        deletions: 1,
        type_: "modify".to_string(),
        lines: vec![],
    }];
    Json(diffs)
}

pub async fn request_review(
    Path((_owner, _repo, _index)): Path<(String, String, u64)>,
) -> (StatusCode, Json<ReviewRequest>) {
    let reviewer = User::new(2, "reviewer".to_string(), None);
    (
        StatusCode::CREATED,
        Json(ReviewRequest {
            reviewer,
            status: "requested".to_string(),
        }),
    )
}

pub async fn list_reviews(
    State(state): State<AppState>,
    Path((owner, repo_name, index)): Path<(String, String, u64)>,
) -> Json<Vec<Review>> {
    let repos = state.repos.read().unwrap_or_else(|e| e.into_inner());
    let repo_id = repos
        .iter()
        .find(|r| r.owner == owner && r.name == repo_name)
        .map(|r| r.id)
        .unwrap_or(0);

    let pulls = state.pulls.read().unwrap_or_else(|e| e.into_inner());
    // Assuming pull requests have unique IDs globally or we filter by repo/number.
    // Shared `PullRequest` has `id`, `repo_id`, `number`.
    let pr = pulls
        .iter()
        .find(|p| p.repo_id == repo_id && p.number == index);

    if let Some(p) = pr {
        let reviews = state.reviews.read().unwrap_or_else(|e| e.into_inner());
        let filtered: Vec<Review> = reviews
            .iter()
            .filter(|r| r.pull_request_id == p.id)
            .cloned()
            .collect();
        Json(filtered)
    } else {
        Json(vec![])
    }
}

pub async fn create_review(
    State(state): State<AppState>,
    Path((owner, repo_name, index)): Path<(String, String, u64)>,
    Json(payload): Json<CreateReviewOption>,
) -> (StatusCode, Json<Review>) {
    let repos = state.repos.read().unwrap_or_else(|e| e.into_inner());
    let repo = repos
        .iter()
        .find(|r| r.owner == owner && r.name == repo_name);

    let repo_id = if let Some(r) = repo {
        r.id
    } else {
        return (
            StatusCode::NOT_FOUND,
            Json(Review {
                id: 0,
                pull_request_id: 0,
                user: User::new(0, "".to_string(), None),
                body: "".to_string(),
                state: "".to_string(),
                created_at: "".to_string(),
            }),
        );
    };

    let pulls = state.pulls.read().unwrap_or_else(|e| e.into_inner());
    let pr = pulls
        .iter()
        .find(|p| p.repo_id == repo_id && p.number == index);

    if let Some(p) = pr {
        let mut reviews = state.reviews.write().unwrap_or_else(|e| e.into_inner());
        let id = (reviews.len() as u64) + 1;
        let state_val = match payload.event.as_str() {
            "APPROVE" => "APPROVED",
            "REQUEST_CHANGES" => "CHANGES_REQUESTED",
            _ => "COMMENTED",
        };
        let review = Review {
            id,
            pull_request_id: p.id,
            user: User::new(1, "admin".to_string(), None), // Mock user
            body: payload.body,
            state: state_val.to_string(),
            created_at: "now".to_string(),
        };
        reviews.push(review.clone());
        (StatusCode::CREATED, Json(review))
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(Review {
                id: 0,
                pull_request_id: 0,
                user: User::new(0, "".to_string(), None),
                body: "".to_string(),
                state: "".to_string(),
                created_at: "".to_string(),
            }),
        )
    }
}
