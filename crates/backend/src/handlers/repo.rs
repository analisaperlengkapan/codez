use crate::router::AppState;
use axum::{
    extract::{Json, Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use shared::{
    Activity, Branch, CodeSearchResult, Collaborator, Comment, Commit, CreateBranchOption,
    CreateCommentOption, CreateHookOption, CreateIssueOption, CreateKeyOption, CreateLabelOption,
    CreateMilestoneOption, CreateProtectedBranchOption, CreatePullRequestOption,
    CreateReactionOption, CreateRepoOption, CreateReviewOption, CreateSecretOption,
    CreateWikiPageOption, DeployKey, DiffFile, DiffLine, FileEntry, Issue, IssueFilterOptions,
    Label, LfsLock, LfsObject, MergePullRequestOption, MigrateRepoOption, Milestone,
    MilestoneStats, Notification, PaginationOptions, ProtectedBranch, PullRequest, Reaction,
    RepoSearchOptions, RepoSettingsOption, RepoTopicOptions, RepoUserStatus, Repository, Review,
    ReviewRequest, Secret, Tag, Topic, TransferRepoOption, UpdateCommentOption, UpdateFileOption,
    UpdateIssueOption, UpdatePullRequestOption, User, Webhook, WebhookDelivery, WikiPage,
};

pub async fn get_user_repo_status(
    State(state): State<AppState>,
    Path((owner, repo_name)): Path<(String, String)>,
) -> Json<RepoUserStatus> {
    let repos = state.repos.read().unwrap();
    let repo_id = repos
        .iter()
        .find(|r| r.owner == owner && r.name == repo_name)
        .map(|r| r.id)
        .unwrap_or(0);

    let user_id = 1; // Mock current user

    let stars = state.stars.read().unwrap();
    let starred = stars
        .get(&repo_id)
        .map(|users| users.contains(&user_id))
        .unwrap_or(false);

    Json(RepoUserStatus {
        starred,
        watching: false,
    })
}

pub async fn list_repos(
    State(state): State<AppState>,
    Query(pagination): Query<PaginationOptions>,
) -> Json<Vec<Repository>> {
    let repos = state.repos.read().unwrap();
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
    let repos = state.repos.read().unwrap();
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
    let mut repos = state.repos.write().unwrap();
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
    if let Some(val) = payload.default_branch {
        repo.default_branch = val;
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
        let mut files = state.file_contents.write().unwrap();
        files.insert(
            (id, "README.md".to_string()),
            format!(
                "# {}\n\n{}",
                payload.name,
                payload.description.clone().unwrap_or_default()
            ),
        );
        if let Some(gitignores) = &payload.gitignores {
            files.insert(
                (id, ".gitignore".to_string()),
                format!("# {}\n\ntarget/\n", gitignores),
            );
        }
    }

    // Create initial commit
    let mut commits = state.commits.write().unwrap();
    commits.push(Commit {
        sha: format!("init{}", id),
        repo_id: id,
        message: "Initial commit".to_string(),
        author: User::new(1, "admin".to_string(), None),
        date: "now".to_string(),
    });

    // Log activity
    let mut activities = state.activities.write().unwrap();
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

pub async fn list_issues(
    State(state): State<AppState>,
    Path((owner, repo_name)): Path<(String, String)>,
    Query(filter): Query<IssueFilterOptions>,
) -> Json<Vec<Issue>> {
    let repos = state.repos.read().unwrap();
    let repo_id = repos
        .iter()
        .find(|r| r.owner == owner && r.name == repo_name)
        .map(|r| r.id)
        .unwrap_or(0);

    if repo_id == 0 {
        return Json(vec![]);
    }

    let issues = state.issues.read().unwrap();
    let mut filtered_issues: Vec<Issue> = issues
        .iter()
        .filter(|i| i.repo_id == repo_id)
        .cloned()
        .collect();

    if let Some(state_filter) = filter.state {
        if state_filter != "all" {
            filtered_issues.retain(|i| i.state == state_filter);
        }
    }
    if let Some(q) = filter.q {
        let q_lower = q.to_lowercase();
        filtered_issues.retain(|i| {
            i.title.to_lowercase().contains(&q_lower)
                || i.body
                    .clone()
                    .unwrap_or_default()
                    .to_lowercase()
                    .contains(&q_lower)
        });
    }
    if let Some(label_id) = filter.label_id {
        filtered_issues.retain(|i| i.labels.iter().any(|l| l.id == label_id));
    }
    if let Some(assignee) = filter.assignee_username {
        filtered_issues.retain(|i| i.assignees.iter().any(|u| u.username == assignee));
    }

    // Sort issues
    if let Some(sort) = &filter.sort {
        let direction = filter.direction.clone().unwrap_or("desc".to_string());
        match sort.as_str() {
            "created" => {
                // Mock sorting by ID since created_at is not in Issue struct, assume ID correlates with creation
                if direction == "asc" {
                    filtered_issues.sort_by(|a, b| a.id.cmp(&b.id));
                } else {
                    filtered_issues.sort_by(|a, b| b.id.cmp(&a.id));
                }
            }
            "updated" => {
                // Mock sorting by ID as proxy for updated
                if direction == "asc" {
                    filtered_issues.sort_by(|a, b| a.id.cmp(&b.id));
                } else {
                    filtered_issues.sort_by(|a, b| b.id.cmp(&a.id));
                }
            }
            "comments" => {
                // Mock sorting by ID as proxy, real impl would join comments count
                if direction == "asc" {
                    filtered_issues.sort_by(|a, b| a.id.cmp(&b.id));
                } else {
                    filtered_issues.sort_by(|a, b| b.id.cmp(&a.id));
                }
            }
            _ => {}
        }
    }

    // Pagination
    let page = filter.page.unwrap_or(1);
    let limit = filter.limit.unwrap_or(10);
    let start = ((page - 1) * limit) as usize;
    let end = (start + limit as usize).min(filtered_issues.len());

    if start >= filtered_issues.len() {
        Json(vec![])
    } else {
        Json(filtered_issues[start..end].to_vec())
    }
}

pub async fn create_issue(
    State(state): State<AppState>,
    Path((owner, repo_name)): Path<(String, String)>,
    Json(payload): Json<CreateIssueOption>,
) -> impl IntoResponse {
    let repos = state.repos.read().unwrap();
    let repo = repos
        .iter()
        .find(|r| r.owner == owner && r.name == repo_name);

    let repo_id = if let Some(r) = repo {
        r.id
    } else {
        return (
            StatusCode::NOT_FOUND,
            Json(Issue {
                id: 0,
                repo_id: 0,
                number: 0,
                title: "".to_string(),
                body: None,
                state: "".to_string(),
                user: User::new(0, "".to_string(), None),
                assignees: vec![],
                labels: vec![],
                milestone: None,
            }),
        );
    };

    let mut issues = state.issues.write().unwrap();
    let id = (issues.len() as u64) + 1;
    let issue = Issue {
        id,
        repo_id,
        number: id,
        title: payload.title.clone(),
        body: payload.body,
        state: "open".to_string(),
        user: User::new(1, "admin".to_string(), None),
        assignees: vec![],
        labels: vec![],
        milestone: None,
    };
    issues.push(issue.clone());

    // Log activity
    let mut activities = state.activities.write().unwrap();
    let activity_id = (activities.len() as u64) + 1;
    activities.push(Activity {
        id: activity_id,
        repo_id,
        user_id: 1,
        user_name: "admin".to_string(),
        op_type: "create_issue".to_string(),
        content: format!("opened issue #{} in {}/{}", id, owner, repo_name),
        created: "now".to_string(),
    });

    // Notify repository owner (mock logic)
    let mut notifications = state.notifications.write().unwrap();
    let notification_id = (notifications.len() as u64) + 1;
    notifications.push(Notification {
        id: notification_id,
        subject: format!("New issue in {}: {}", repo_name, payload.title),
        unread: true,
        updated_at: "now".to_string(),
    });

    // Trigger Webhooks
    dispatch_hooks(&state, repo_id, "issues");

    (StatusCode::CREATED, Json(issue))
}

pub async fn get_issue(
    State(state): State<AppState>,
    Path((_owner, _repo, index)): Path<(String, String, u64)>,
) -> Json<Option<Issue>> {
    let issues = state.issues.read().unwrap();
    let issue = issues.iter().find(|i| i.id == index).cloned();
    Json(issue)
}

pub async fn update_issue(
    State(state): State<AppState>,
    Path((owner, repo_name, index)): Path<(String, String, u64)>,
    Json(payload): Json<UpdateIssueOption>,
) -> (StatusCode, Json<Option<Issue>>) {
    let repos = state.repos.read().unwrap();
    let repo_id = repos
        .iter()
        .find(|r| r.owner == owner && r.name == repo_name)
        .map(|r| r.id)
        .unwrap_or(0);

    if repo_id == 0 {
        return (StatusCode::NOT_FOUND, Json(None));
    }

    let mut issues = state.issues.write().unwrap();
    let issue = issues
        .iter_mut()
        .find(|i| i.id == index && i.repo_id == repo_id);

    if let Some(i) = issue {
        if let Some(title) = payload.title {
            i.title = title;
        }
        if let Some(body) = payload.body {
            i.body = Some(body);
        }
        if let Some(state_val) = payload.state {
            i.state = state_val;
        }
        if let Some(milestone_id) = payload.milestone_id {
            if milestone_id == 0 {
                i.milestone = None;
            } else {
                // Validate milestone existence
                let milestones = state.milestones.read().unwrap();
                if let Some(m) = milestones.iter().find(|m| m.id == milestone_id) {
                    i.milestone = Some(m.clone());
                }
            }
        }
        return (StatusCode::OK, Json(Some(i.clone())));
    }
    (StatusCode::NOT_FOUND, Json(None))
}

pub async fn list_pulls(
    State(state): State<AppState>,
    Path((owner, repo_name)): Path<(String, String)>,
) -> Json<Vec<PullRequest>> {
    let repos = state.repos.read().unwrap();
    let repo_id = repos
        .iter()
        .find(|r| r.owner == owner && r.name == repo_name)
        .map(|r| r.id)
        .unwrap_or(0);

    let pulls = state.pulls.read().unwrap();
    let filtered_pulls: Vec<PullRequest> = pulls
        .iter()
        .filter(|p| p.repo_id == repo_id)
        .cloned()
        .collect();
    Json(filtered_pulls)
}

pub async fn create_pull(
    State(state): State<AppState>,
    Path((owner, repo_name)): Path<(String, String)>,
    Json(payload): Json<CreatePullRequestOption>,
) -> (StatusCode, Json<PullRequest>) {
    let repos = state.repos.read().unwrap();
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
            }),
        );
    }

    let mut pulls = state.pulls.write().unwrap();
    let id = (pulls.len() as u64) + 1;
    let pr = PullRequest {
        id,
        repo_id,
        number: id,
        title: payload.title.clone(),
        body: payload.body,
        state: "open".to_string(),
        user: User::new(1, "admin".to_string(), None),
        merged: false,
    };
    pulls.push(pr.clone());

    // Log activity
    let mut activities = state.activities.write().unwrap();
    let activity_id = (activities.len() as u64) + 1;
    activities.push(Activity {
        id: activity_id,
        repo_id,
        user_id: 1,
        user_name: "admin".to_string(),
        op_type: "create_pull_request".to_string(),
        content: format!("opened pull request #{} in {}/{}", id, owner, repo_name),
        created: "now".to_string(),
    });

    // Notify repository owner (mock logic)
    let mut notifications = state.notifications.write().unwrap();
    let notification_id = (notifications.len() as u64) + 1;
    notifications.push(Notification {
        id: notification_id,
        subject: format!("New pull request in {}: {}", repo_name, payload.title),
        unread: true,
        updated_at: "now".to_string(),
    });

    // Trigger Webhooks
    dispatch_hooks(&state, repo_id, "pull_request");

    (StatusCode::CREATED, Json(pr))
}

pub async fn update_pull(
    State(state): State<AppState>,
    Path((owner, repo_name, index)): Path<(String, String, u64)>,
    Json(payload): Json<UpdatePullRequestOption>,
) -> (StatusCode, Json<Option<PullRequest>>) {
    let repos = state.repos.read().unwrap();
    let repo_id = repos
        .iter()
        .find(|r| r.owner == owner && r.name == repo_name)
        .map(|r| r.id)
        .unwrap_or(0);

    if repo_id == 0 {
        return (StatusCode::NOT_FOUND, Json(None));
    }

    let mut pulls = state.pulls.write().unwrap();
    let pr = pulls
        .iter_mut()
        .find(|p| p.number == index && p.repo_id == repo_id);

    if let Some(p) = pr {
        if let Some(title) = payload.title {
            p.title = title;
        }
        if let Some(body) = payload.body {
            p.body = Some(body);
        }
        if let Some(state_val) = payload.state {
            p.state = state_val;
        }
        return (StatusCode::OK, Json(Some(p.clone())));
    }
    (StatusCode::NOT_FOUND, Json(None))
}

pub async fn list_comments(
    State(state): State<AppState>,
    Path((owner, repo_name, index)): Path<(String, String, u64)>,
) -> Json<Vec<Comment>> {
    let repos = state.repos.read().unwrap();
    let repo_id = repos
        .iter()
        .find(|r| r.owner == owner && r.name == repo_name)
        .map(|r| r.id)
        .unwrap_or(0);

    let issues = state.issues.read().unwrap();
    let issue_id = issues
        .iter()
        .find(|i| i.repo_id == repo_id && i.number == index)
        .map(|i| i.id)
        .unwrap_or(0);

    let comments = state.comments.read().unwrap();
    let filtered_comments: Vec<Comment> = comments
        .iter()
        .filter(|c| c.issue_id == issue_id)
        .cloned()
        .collect();
    Json(filtered_comments)
}

pub async fn create_comment(
    State(state): State<AppState>,
    Path((owner, repo_name, index)): Path<(String, String, u64)>,
    Json(payload): Json<CreateCommentOption>,
) -> (StatusCode, Json<Comment>) {
    let repos = state.repos.read().unwrap();
    let repo = repos
        .iter()
        .find(|r| r.owner == owner && r.name == repo_name);

    let repo_id = if let Some(r) = repo {
        r.id
    } else {
        return (
            StatusCode::NOT_FOUND,
            Json(Comment {
                id: 0,
                issue_id: 0,
                body: "".to_string(),
                user: User::new(0, "".to_string(), None),
                created_at: "".to_string(),
                reactions: vec![],
            }),
        );
    };

    let issues = state.issues.read().unwrap();
    let issue = issues
        .iter()
        .find(|i| i.repo_id == repo_id && i.number == index);

    if issue.is_none() {
        return (
            StatusCode::NOT_FOUND,
            Json(Comment {
                id: 0,
                issue_id: 0,
                body: "".to_string(),
                user: User::new(0, "".to_string(), None),
                created_at: "".to_string(),
                reactions: vec![],
            }),
        );
    }
    let issue_id = issue.unwrap().id;

    let mut comments = state.comments.write().unwrap();
    let id = (comments.len() as u64) + 1;
    let comment = Comment {
        id,
        issue_id,
        body: payload.body,
        user: User::new(1, "admin".to_string(), None),
        created_at: "2023-01-02".to_string(),
        reactions: vec![],
    };
    comments.push(comment.clone());
    (StatusCode::CREATED, Json(comment))
}

pub async fn update_comment(
    State(state): State<AppState>,
    Path((_owner, _repo, id)): Path<(String, String, u64)>,
    Json(payload): Json<UpdateCommentOption>,
) -> (StatusCode, Json<Option<Comment>>) {
    let mut comments = state.comments.write().unwrap();
    if let Some(comment) = comments.iter_mut().find(|c| c.id == id) {
        comment.body = payload.body;
        return (StatusCode::OK, Json(Some(comment.clone())));
    }
    (StatusCode::NOT_FOUND, Json(None))
}

pub async fn delete_comment(
    State(state): State<AppState>,
    Path((_owner, _repo, id)): Path<(String, String, u64)>,
) -> StatusCode {
    let mut comments = state.comments.write().unwrap();
    if let Some(pos) = comments.iter().position(|c| c.id == id) {
        comments.remove(pos);
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}

pub async fn list_labels(
    State(state): State<AppState>,
    Path((owner, repo_name)): Path<(String, String)>,
) -> Json<Vec<Label>> {
    let repos = state.repos.read().unwrap();
    let repo_id = repos
        .iter()
        .find(|r| r.owner == owner && r.name == repo_name)
        .map(|r| r.id)
        .unwrap_or(0);

    let labels = state.labels.read().unwrap();
    let filtered_labels: Vec<Label> = labels
        .iter()
        .filter(|l| l.repo_id == repo_id)
        .cloned()
        .collect();
    Json(filtered_labels)
}

pub async fn create_label(
    State(state): State<AppState>,
    Path((owner, repo_name)): Path<(String, String)>,
    Json(payload): Json<CreateLabelOption>,
) -> (StatusCode, Json<Label>) {
    let repos = state.repos.read().unwrap();
    let repo = repos
        .iter()
        .find(|r| r.owner == owner && r.name == repo_name);

    let repo_id = if let Some(r) = repo {
        r.id
    } else {
        return (
            StatusCode::NOT_FOUND,
            Json(Label {
                id: 0,
                repo_id: 0,
                name: "".to_string(),
                color: "".to_string(),
                description: None,
            }),
        );
    };

    let mut labels = state.labels.write().unwrap();
    let id = (labels.len() as u64) + 1;
    let label = Label {
        id,
        repo_id,
        name: payload.name,
        color: payload.color,
        description: payload.description,
    };
    labels.push(label.clone());
    (StatusCode::CREATED, Json(label))
}

pub async fn list_milestones(
    State(state): State<AppState>,
    Path((owner, repo_name)): Path<(String, String)>,
) -> Json<Vec<Milestone>> {
    let repos = state.repos.read().unwrap();
    let repo_id = repos
        .iter()
        .find(|r| r.owner == owner && r.name == repo_name)
        .map(|r| r.id)
        .unwrap_or(0);

    let milestones = state.milestones.read().unwrap();
    let filtered_milestones: Vec<Milestone> = milestones
        .iter()
        .filter(|m| m.repo_id == repo_id)
        .cloned()
        .collect();
    Json(filtered_milestones)
}

pub async fn create_milestone(
    State(state): State<AppState>,
    Path((owner, repo_name)): Path<(String, String)>,
    Json(payload): Json<CreateMilestoneOption>,
) -> (StatusCode, Json<Milestone>) {
    let repos = state.repos.read().unwrap();
    let repo = repos
        .iter()
        .find(|r| r.owner == owner && r.name == repo_name);

    let repo_id = if let Some(r) = repo {
        r.id
    } else {
        return (
            StatusCode::NOT_FOUND,
            Json(Milestone {
                id: 0,
                repo_id: 0,
                title: "".to_string(),
                description: None,
                due_on: None,
                state: "".to_string(),
            }),
        );
    };

    let mut milestones = state.milestones.write().unwrap();
    let id = (milestones.len() as u64) + 1;
    let milestone = Milestone {
        id,
        repo_id,
        title: payload.title,
        description: payload.description,
        due_on: payload.due_on,
        state: "open".to_string(),
    };
    milestones.push(milestone.clone());
    (StatusCode::CREATED, Json(milestone))
}

pub async fn get_milestone(
    State(state): State<AppState>,
    Path((_owner, _repo, id)): Path<(String, String, u64)>,
) -> Json<Option<Milestone>> {
    let milestones = state.milestones.read().unwrap();
    let m = milestones.iter().find(|m| m.id == id).cloned();
    Json(m)
}

pub async fn list_hooks(
    State(state): State<AppState>,
    Path((owner, repo_name)): Path<(String, String)>,
) -> Json<Vec<Webhook>> {
    let repos = state.repos.read().unwrap();
    let repo_id = repos
        .iter()
        .find(|r| r.owner == owner && r.name == repo_name)
        .map(|r| r.id)
        .unwrap_or(0);

    let hooks = state.hooks.read().unwrap();
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
    let repos = state.repos.read().unwrap();
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

    let mut hooks = state.hooks.write().unwrap();
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
    let deliveries = state.webhook_deliveries.read().unwrap();
    let filtered: Vec<WebhookDelivery> = deliveries
        .iter()
        .filter(|d| d.hook_id == id)
        .cloned()
        .collect();
    Json(filtered)
}

fn dispatch_hooks(state: &AppState, repo_id: u64, event: &str) {
    let hooks = state.hooks.read().unwrap();
    let relevant_hooks: Vec<Webhook> = hooks
        .iter()
        .filter(|h| h.repo_id == repo_id && h.active && h.events.contains(&event.to_string()))
        .cloned()
        .collect();

    if relevant_hooks.is_empty() {
        return;
    }

    let mut deliveries = state.webhook_deliveries.write().unwrap();
    for hook in relevant_hooks {
        let delivery_id = (deliveries.len() as u64) + 1;
        deliveries.push(WebhookDelivery {
            id: delivery_id,
            hook_id: hook.id,
            event: event.to_string(),
            status: "success".to_string(), // Mock success
            request_url: hook.url.clone(),
            response_status: 200,
            delivered_at: "now".to_string(),
        });
    }
}

pub async fn create_secret(
    State(state): State<AppState>,
    Path((owner, repo_name)): Path<(String, String)>,
    Json(payload): Json<CreateSecretOption>,
) -> (StatusCode, Json<Secret>) {
    let repos = state.repos.read().unwrap();
    let repo_id = repos
        .iter()
        .find(|r| r.owner == owner && r.name == repo_name)
        .map(|r| r.id)
        .unwrap_or(0);

    if repo_id == 0 {
        return (
            StatusCode::NOT_FOUND,
            Json(Secret {
                name: "".to_string(),
                repo_id: 0,
                created_at: "".to_string(),
            }),
        );
    }

    let mut secrets = state.secrets.write().unwrap();
    if secrets
        .iter()
        .any(|s| s.repo_id == repo_id && s.name == payload.name)
    {
        return (
            StatusCode::CONFLICT,
            Json(Secret {
                name: "".to_string(),
                repo_id: 0,
                created_at: "".to_string(),
            }),
        );
    }

    let secret = Secret {
        name: payload.name,
        repo_id,
        created_at: "now".to_string(),
    };
    secrets.push(secret.clone());
    (StatusCode::CREATED, Json(secret))
}

pub async fn list_secrets(
    State(state): State<AppState>,
    Path((owner, repo_name)): Path<(String, String)>,
) -> Json<Vec<Secret>> {
    let repos = state.repos.read().unwrap();
    let repo_id = repos
        .iter()
        .find(|r| r.owner == owner && r.name == repo_name)
        .map(|r| r.id)
        .unwrap_or(0);

    let secrets = state.secrets.read().unwrap();
    let filtered: Vec<Secret> = secrets
        .iter()
        .filter(|s| s.repo_id == repo_id)
        .cloned()
        .collect();
    Json(filtered)
}

pub async fn delete_secret(
    State(state): State<AppState>,
    Path((owner, repo_name, name)): Path<(String, String, String)>,
) -> StatusCode {
    let repos = state.repos.read().unwrap();
    let repo_id = repos
        .iter()
        .find(|r| r.owner == owner && r.name == repo_name)
        .map(|r| r.id)
        .unwrap_or(0);

    if repo_id == 0 {
        return StatusCode::NOT_FOUND;
    }

    let mut secrets = state.secrets.write().unwrap();
    if let Some(pos) = secrets
        .iter()
        .position(|s| s.repo_id == repo_id && s.name == name)
    {
        secrets.remove(pos);
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}

pub async fn create_deploy_key(
    State(state): State<AppState>,
    Path((owner, repo_name)): Path<(String, String)>,
    Json(payload): Json<CreateKeyOption>,
) -> (StatusCode, Json<DeployKey>) {
    let repos = state.repos.read().unwrap();
    let repo_id = repos
        .iter()
        .find(|r| r.owner == owner && r.name == repo_name)
        .map(|r| r.id)
        .unwrap_or(0);

    if repo_id == 0 {
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
    }

    let mut keys = state.deploy_keys.write().unwrap();
    let id = (keys.len() as u64) + 1;
    let key = DeployKey {
        id,
        repo_id,
        title: payload.title,
        key: payload.key,
        fingerprint: "SHA256:deploy".to_string(),
    };
    keys.push(key.clone());
    (StatusCode::CREATED, Json(key))
}

pub async fn list_deploy_keys(
    State(state): State<AppState>,
    Path((owner, repo_name)): Path<(String, String)>,
) -> Json<Vec<DeployKey>> {
    let repos = state.repos.read().unwrap();
    let repo_id = repos
        .iter()
        .find(|r| r.owner == owner && r.name == repo_name)
        .map(|r| r.id)
        .unwrap_or(0);

    let keys = state.deploy_keys.read().unwrap();
    let filtered: Vec<DeployKey> = keys
        .iter()
        .filter(|k| k.repo_id == repo_id)
        .cloned()
        .collect();
    Json(filtered)
}

pub async fn delete_deploy_key(
    State(state): State<AppState>,
    Path((owner, repo_name, id)): Path<(String, String, u64)>,
) -> StatusCode {
    let repos = state.repos.read().unwrap();
    let repo_id = repos
        .iter()
        .find(|r| r.owner == owner && r.name == repo_name)
        .map(|r| r.id)
        .unwrap_or(0);

    if repo_id == 0 {
        return StatusCode::NOT_FOUND;
    }

    let mut keys = state.deploy_keys.write().unwrap();
    if let Some(pos) = keys.iter().position(|k| k.repo_id == repo_id && k.id == id) {
        keys.remove(pos);
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}

pub async fn list_lfs_locks(
    State(state): State<AppState>,
    Path((owner, repo_name)): Path<(String, String)>,
) -> Json<Vec<LfsLock>> {
    let repos = state.repos.read().unwrap();
    let repo_id = repos
        .iter()
        .find(|r| r.owner == owner && r.name == repo_name)
        .map(|r| r.id)
        .unwrap_or(0);

    let locks = state.lfs_locks.read().unwrap();
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
) -> StatusCode {
    let repos = state.repos.read().unwrap();
    let repo = repos
        .iter()
        .find(|r| r.owner == owner && r.name == repo_name);

    let repo_id = if let Some(r) = repo {
        r.id
    } else {
        return StatusCode::NOT_FOUND;
    };

    let mut locks = state.lfs_locks.write().unwrap();
    let id = (locks.len() as u64) + 1;
    let user = User::new(1, "admin".to_string(), None);
    locks.push(LfsLock {
        id: id.to_string(),
        repo_id,
        path: format!("file{}.bin", id),
        owner: user,
        locked_at: "now".to_string(),
    });
    StatusCode::CREATED
}

pub async fn add_reaction(
    State(state): State<AppState>,
    Path((_owner, _repo, id)): Path<(String, String, u64)>,
    Json(payload): Json<CreateReactionOption>,
) -> (StatusCode, Json<Reaction>) {
    let mut comments = state.comments.write().unwrap();
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

pub async fn update_topics(
    State(state): State<AppState>,
    Path((owner, repo_name)): Path<(String, String)>,
    Json(payload): Json<RepoTopicOptions>,
) -> StatusCode {
    let repos = state.repos.read().unwrap();
    let repo = repos
        .iter()
        .find(|r| r.owner == owner && r.name == repo_name);

    let repo_id = if let Some(r) = repo {
        r.id
    } else {
        return StatusCode::NOT_FOUND;
    };

    let mut topics = state.topics.write().unwrap();
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
    let repos = state.repos.read().unwrap();
    let repo_id = repos
        .iter()
        .find(|r| r.owner == owner && r.name == repo_name)
        .map(|r| r.id)
        .unwrap_or(0);

    let topics = state.topics.read().unwrap();
    let filtered_topics: Vec<Topic> = topics
        .iter()
        .filter(|t| t.repo_id == repo_id)
        .cloned()
        .collect();
    Json(filtered_topics)
}

pub async fn star_repo(
    State(state): State<AppState>,
    Path((owner, repo_name)): Path<(String, String)>,
) -> StatusCode {
    let mut repos = state.repos.write().unwrap();
    let repo = repos
        .iter_mut()
        .find(|r| r.owner == owner && r.name == repo_name);

    if let Some(r) = repo {
        let repo_id = r.id;
        let user_id = 1; // Mock current user

        let mut stars = state.stars.write().unwrap();
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

pub async fn remove_issue_assignee(
    State(state): State<AppState>,
    Path((_owner, _repo, index, username)): Path<(String, String, u64, String)>,
) -> StatusCode {
    let mut issues = state.issues.write().unwrap();
    if let Some(issue) = issues.iter_mut().find(|i| i.id == index) {
        if let Some(pos) = issue.assignees.iter().position(|u| u.username == username) {
            issue.assignees.remove(pos);
            StatusCode::NO_CONTENT
        } else {
            StatusCode::NOT_FOUND
        }
    } else {
        StatusCode::NOT_FOUND
    }
}

pub async fn watch_repo(
    State(state): State<AppState>,
    Path((owner, repo_name)): Path<(String, String)>,
) -> StatusCode {
    let mut repos = state.repos.write().unwrap();
    if let Some(repo) = repos
        .iter_mut()
        .find(|r| r.owner == owner && r.name == repo_name)
    {
        repo.watchers_count += 1;
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}

pub async fn fork_repo(
    State(state): State<AppState>,
    Path((owner, repo)): Path<(String, String)>,
) -> Json<Repository> {
    let mut repos = state.repos.write().unwrap();

    let original_repo = repos
        .iter()
        .find(|r| r.owner == owner && r.name == repo)
        .cloned();

    if let Some(orig) = original_repo {
        let id = (repos.len() as u64) + 1;
        let new_name = format!("{}-fork", repo);
        let mut new_repo = Repository::new(id, new_name.clone(), "admin".to_string());
        new_repo.parent_id = Some(orig.id);
        repos.push(new_repo.clone());

        // Copy files
        {
            let mut files = state.file_contents.write().unwrap();
            let mut new_files = Vec::new();

            for ((r_id, path), content) in files.iter() {
                if *r_id == orig.id {
                    new_files.push((path.clone(), content.clone()));
                }
            }

            for (path, content) in new_files {
                files.insert((id, path), content);
            }
        }

        // Log activity
        let mut activities = state.activities.write().unwrap();
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

        Json(new_repo)
    } else {
        Json(Repository::new(0, "error".to_string(), "error".to_string()))
    }
}

pub async fn create_wiki_page(
    Path((_owner, _repo)): Path<(String, String)>,
    Json(payload): Json<CreateWikiPageOption>,
) -> (StatusCode, Json<WikiPage>) {
    let page = WikiPage {
        title: payload.title,
        content: payload.content,
        commit_message: payload.message,
    };
    (StatusCode::CREATED, Json(page))
}

pub async fn get_repo_settings(
    Path((_owner, _repo)): Path<(String, String)>,
) -> Json<RepoSettingsOption> {
    Json(RepoSettingsOption {
        description: Some("Description".to_string()),
        private: Some(false),
        website: None,
        default_branch: Some("main".to_string()),
        allow_rebase_merge: Some(true),
        allow_squash_merge: Some(true),
        allow_merge_commit: Some(true),
        has_issues: Some(true),
        has_wiki: Some(true),
        has_projects: Some(true),
    })
}

pub async fn update_repo_settings(
    State(state): State<AppState>,
    Path((owner, repo_name)): Path<(String, String)>,
    Json(payload): Json<RepoSettingsOption>,
) -> StatusCode {
    let mut repos = state.repos.write().unwrap();
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
        if let Some(val) = payload.default_branch {
            repo.default_branch = val;
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

        // Handle website update if Repo struct supported it, currently it doesn't in shared definition
        // but we can at least return OK after modifying what we can.
        StatusCode::OK
    } else {
        StatusCode::NOT_FOUND
    }
}

pub async fn mirror_sync(Path((_owner, _repo)): Path<(String, String)>) -> StatusCode {
    StatusCode::OK
}

pub async fn migrate_repo(
    Json(payload): Json<MigrateRepoOption>,
) -> (StatusCode, Json<Repository>) {
    let repo = Repository::new(4, payload.repo_name, "admin".to_string());
    (StatusCode::CREATED, Json(repo))
}

pub async fn transfer_repo(
    Path((_owner, _repo)): Path<(String, String)>,
    Json(_payload): Json<TransferRepoOption>,
) -> StatusCode {
    StatusCode::ACCEPTED
}

pub async fn add_issue_label(
    State(state): State<AppState>,
    Path((_owner, _repo, index)): Path<(String, String, u64)>,
    Json(payload): Json<shared::CreateLabelOption>,
) -> StatusCode {
    let mut issues = state.issues.write().unwrap();
    if let Some(issue) = issues.iter_mut().find(|i| i.id == index) {
        issue.labels.push(Label {
            id: 100, // mock ID
            repo_id: issue.repo_id,
            name: payload.name,
            color: payload.color,
            description: payload.description,
        });
        StatusCode::CREATED
    } else {
        StatusCode::NOT_FOUND
    }
}

pub async fn remove_issue_label(
    State(state): State<AppState>,
    Path((_owner, _repo, index, id)): Path<(String, String, u64, u64)>,
) -> StatusCode {
    let mut issues = state.issues.write().unwrap();
    if let Some(issue) = issues.iter_mut().find(|i| i.id == index) {
        if let Some(pos) = issue.labels.iter().position(|l| l.id == id) {
            issue.labels.remove(pos);
            StatusCode::NO_CONTENT
        } else {
            StatusCode::NOT_FOUND
        }
    } else {
        StatusCode::NOT_FOUND
    }
}

pub async fn list_wiki_pages(Path((_owner, _repo)): Path<(String, String)>) -> Json<Vec<WikiPage>> {
    let pages = vec![
        WikiPage {
            title: "Home".to_string(),
            content: "Welcome to the wiki!".to_string(),
            commit_message: None,
        },
        WikiPage {
            title: "Installation".to_string(),
            content: "How to install...".to_string(),
            commit_message: None,
        },
    ];
    Json(pages)
}

pub async fn update_wiki_page(
    Path((_owner, _repo, _page_name)): Path<(String, String, String)>,
    Json(_payload): Json<CreateWikiPageOption>,
) -> StatusCode {
    StatusCode::OK
}

pub async fn get_wiki_page(
    Path((_owner, _repo, page_name)): Path<(String, String, String)>,
) -> Json<Option<WikiPage>> {
    if page_name == "Home" {
        Json(Some(WikiPage {
            title: "Home".to_string(),
            content: "Welcome to the wiki!".to_string(),
            commit_message: None,
        }))
    } else if page_name == "Installation" {
        Json(Some(WikiPage {
            title: "Installation".to_string(),
            content: "How to install...".to_string(),
            commit_message: None,
        }))
    } else {
        Json(None)
    }
}

pub async fn get_contents(
    State(state): State<AppState>,
    Path((owner, repo_name, path)): Path<(String, String, String)>,
) -> Json<Vec<FileEntry>> {
    let repos = state.repos.read().unwrap();
    let repo_id = repos
        .iter()
        .find(|r| r.owner == owner && r.name == repo_name)
        .map(|r| r.id)
        .unwrap_or(0);

    if repo_id == 0 {
        return Json(vec![]);
    }

    let all_files = state.file_contents.read().unwrap();
    let mut entries = Vec::new();
    let mut dirs = std::collections::HashSet::new();

    let prefix = if path.is_empty() || path == "/" {
        "".to_string()
    } else {
        format!("{}/", path.trim_matches('/'))
    };

    for (k_repo_id, k_path) in all_files.keys() {
        if *k_repo_id == repo_id && k_path.starts_with(&prefix) {
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
                    .get(&(*k_repo_id, k_path.clone()))
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
) -> Json<Vec<FileEntry>> {
    get_contents(State(state), Path((owner, repo, "".to_string()))).await
}

pub async fn merge_pull(
    State(state): State<AppState>,
    Path((owner, repo, index)): Path<(String, String, u64)>,
    Json(_payload): Json<MergePullRequestOption>,
) -> StatusCode {
    let mut pulls = state.pulls.write().unwrap();
    let pr_opt = pulls.iter_mut().find(|p| p.number == index);

    if let Some(pr) = pr_opt {
        let repo_id = pr.repo_id;
        pr.merged = true;
        pr.state = "closed".to_string();

        // Create merge commit
        let mut commits = state.commits.write().unwrap();
        commits.push(Commit {
            sha: format!("merge{}", index),
            repo_id,
            message: format!("Merge pull request #{} from {}", index, pr.title),
            author: User::new(1, "admin".to_string(), None),
            date: "now".to_string(),
        });

        // Log activity
        let mut activities = state.activities.write().unwrap();
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

pub async fn search_repos(
    State(state): State<AppState>,
    Query(params): Query<RepoSearchOptions>,
) -> Json<Vec<Repository>> {
    let repos = state.repos.read().unwrap();
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
    State(state): State<AppState>,
    Path((owner, repo_name)): Path<(String, String)>,
) -> Json<Vec<Collaborator>> {
    let repos = state.repos.read().unwrap();
    let repo_id = repos
        .iter()
        .find(|r| r.owner == owner && r.name == repo_name)
        .map(|r| r.id)
        .unwrap_or(0);

    let collaborators = state.collaborators.read().unwrap();
    let filtered: Vec<Collaborator> = collaborators
        .iter()
        .filter(|c| c.repo_id == repo_id)
        .cloned()
        .collect();
    Json(filtered)
}

pub async fn get_collaborator(
    State(state): State<AppState>,
    Path((owner, repo_name, username)): Path<(String, String, String)>,
) -> Json<Option<Collaborator>> {
    let repos = state.repos.read().unwrap();
    let repo_id = repos
        .iter()
        .find(|r| r.owner == owner && r.name == repo_name)
        .map(|r| r.id)
        .unwrap_or(0);

    let collaborators = state.collaborators.read().unwrap();
    let collaborator = collaborators
        .iter()
        .find(|c| c.repo_id == repo_id && c.user.username == username)
        .cloned();
    Json(collaborator)
}

pub async fn add_collaborator(
    State(state): State<AppState>,
    Path((owner, repo_name, username)): Path<(String, String, String)>,
) -> StatusCode {
    let repos = state.repos.read().unwrap();
    let repo_id = repos
        .iter()
        .find(|r| r.owner == owner && r.name == repo_name)
        .map(|r| r.id)
        .unwrap_or(0);

    if repo_id == 0 {
        return StatusCode::NOT_FOUND;
    }

    // Check if user exists (mock check, but looking in state.users)
    let users = state.users.read().unwrap();
    let user = users.iter().find(|u| u.username == username);

    if let Some(u) = user {
        let mut collaborators = state.collaborators.write().unwrap();
        // Check if already exists
        if !collaborators
            .iter()
            .any(|c| c.repo_id == repo_id && c.user.username == username)
        {
            collaborators.push(Collaborator {
                repo_id,
                user: u.clone(),
                permissions: "write".to_string(), // Default permission
            });
        }
        StatusCode::NO_CONTENT
    } else {
        // In Gitea/GitHub, you can invite by email, but here we enforce existing user
        StatusCode::NOT_FOUND
    }
}

pub async fn remove_collaborator(
    State(state): State<AppState>,
    Path((owner, repo_name, username)): Path<(String, String, String)>,
) -> StatusCode {
    let repos = state.repos.read().unwrap();
    let repo_id = repos
        .iter()
        .find(|r| r.owner == owner && r.name == repo_name)
        .map(|r| r.id)
        .unwrap_or(0);

    if repo_id == 0 {
        return StatusCode::NOT_FOUND;
    }

    let mut collaborators = state.collaborators.write().unwrap();
    if let Some(pos) = collaborators
        .iter()
        .position(|c| c.repo_id == repo_id && c.user.username == username)
    {
        collaborators.remove(pos);
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}

pub async fn list_branches(Path((_owner, _repo)): Path<(String, String)>) -> Json<Vec<Branch>> {
    let user = User::new(1, "admin".to_string(), None);
    let commit = Commit {
        sha: "abc".to_string(),
        repo_id: 1,
        message: "init".to_string(),
        author: user,
        date: "now".to_string(),
    };
    let branches = vec![Branch {
        name: "main".to_string(),
        repo_id: 1,
        commit,
        protected: true,
    }];
    Json(branches)
}

pub async fn create_branch(
    State(state): State<AppState>,
    Path((owner, repo_name)): Path<(String, String)>,
    Json(payload): Json<CreateBranchOption>,
) -> (StatusCode, Json<Branch>) {
    let repos = state.repos.read().unwrap();
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
        name: payload.name,
        repo_id,
        commit,
        protected: false,
    };
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

pub async fn get_milestone_stats(
    State(state): State<AppState>,
    Path((_owner, _repo, id)): Path<(String, String, u64)>,
) -> Json<MilestoneStats> {
    let issues = state.issues.read().unwrap();
    let open_count = issues
        .iter()
        .filter(|i| i.milestone.as_ref().map(|m| m.id).unwrap_or(0) == id && i.state == "open")
        .count() as u64;
    let closed_count = issues
        .iter()
        .filter(|i| i.milestone.as_ref().map(|m| m.id).unwrap_or(0) == id && i.state == "closed")
        .count() as u64;
    Json(MilestoneStats {
        open_issues: open_count,
        closed_issues: closed_count,
    })
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

pub async fn list_commits(
    State(state): State<AppState>,
    Path((owner, repo_name)): Path<(String, String)>,
) -> Json<Vec<Commit>> {
    let repos = state.repos.read().unwrap();
    let repo_id = repos
        .iter()
        .find(|r| r.owner == owner && r.name == repo_name)
        .map(|r| r.id)
        .unwrap_or(0);

    let commits = state.commits.read().unwrap();
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
    let repos = state.repos.read().unwrap();
    let repo_id = repos
        .iter()
        .find(|r| r.owner == owner && r.name == repo_name)
        .map(|r| r.id)
        .unwrap_or(0);
    let q = params.q.to_lowercase();

    if repo_id == 0 {
        return Json(vec![]);
    }

    let files = state.file_contents.read().unwrap();
    let mut results = Vec::new();

    for ((r_id, path), content) in files.iter() {
        if *r_id == repo_id
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
) -> impl IntoResponse {
    let repos = state.repos.read().unwrap();
    let repo_id = repos
        .iter()
        .find(|r| r.owner == owner && r.name == repo_name)
        .map(|r| r.id)
        .unwrap_or(0);

    if repo_id == 0 {
        return (StatusCode::NOT_FOUND, "".to_string());
    }

    let files = state.file_contents.read().unwrap();
    if let Some(content) = files.get(&(repo_id, path)) {
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
    let repos = state.repos.read().unwrap();
    let repo_obj = repos.iter().find(|r| r.owner == owner && r.name == repo);

    let repo_id = if let Some(r) = repo_obj {
        r.id
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

    // Update file content in state
    {
        let mut files = state.file_contents.write().unwrap();
        files.insert((repo_id, path.clone()), payload.content.clone());
    }

    // Create a commit for the file update
    let mut commits = state.commits.write().unwrap();
    let commit_message = if payload.message.is_empty() {
        format!("Update {}", path)
    } else {
        payload.message.clone()
    };

    let commit_id = commits.len() + 1;
    commits.push(Commit {
        sha: format!("update{}", commit_id),
        repo_id,
        message: commit_message,
        author: User::new(1, "admin".to_string(), None),
        date: "now".to_string(),
    });

    // Log activity
    let mut activities = state.activities.write().unwrap();
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
    dispatch_hooks(&state, repo_id, "push");

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

pub async fn add_issue_assignee(
    State(state): State<AppState>,
    Path((_owner, _repo, index)): Path<(String, String, u64)>,
    Json(payload): Json<User>,
) -> StatusCode {
    let mut issues = state.issues.write().unwrap();
    if let Some(issue) = issues.iter_mut().find(|i| i.id == index) {
        if !issue
            .assignees
            .iter()
            .any(|u| u.username == payload.username)
        {
            issue.assignees.push(payload);

            // Notify assignee
            let mut notifications = state.notifications.write().unwrap();
            let notification_id = (notifications.len() as u64) + 1;
            notifications.push(Notification {
                id: notification_id,
                subject: format!("You were assigned to issue #{}", issue.number),
                unread: true,
                updated_at: "now".to_string(),
            });
        }
        StatusCode::CREATED
    } else {
        StatusCode::NOT_FOUND
    }
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
    let repos = state.repos.read().unwrap();
    let repo_id = repos
        .iter()
        .find(|r| r.owner == owner && r.name == repo_name)
        .map(|r| r.id)
        .unwrap_or(0);

    let pulls = state.pulls.read().unwrap();
    // Assuming pull requests have unique IDs globally or we filter by repo/number.
    // Shared `PullRequest` has `id`, `repo_id`, `number`.
    let pr = pulls
        .iter()
        .find(|p| p.repo_id == repo_id && p.number == index);

    if let Some(p) = pr {
        let reviews = state.reviews.read().unwrap();
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
    let repos = state.repos.read().unwrap();
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

    let pulls = state.pulls.read().unwrap();
    let pr = pulls
        .iter()
        .find(|p| p.repo_id == repo_id && p.number == index);

    if let Some(p) = pr {
        let mut reviews = state.reviews.write().unwrap();
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
    Path((_owner, _repo)): Path<(String, String)>,
) -> Json<Vec<ProtectedBranch>> {
    let branches = state.protected_branches.read().unwrap();
    // In real impl we would filter by repo ID
    Json(branches.clone())
}

pub async fn create_branch_protection(
    State(state): State<AppState>,
    Path((_owner, _repo)): Path<(String, String)>,
    Json(payload): Json<CreateProtectedBranchOption>,
) -> (StatusCode, Json<ProtectedBranch>) {
    let mut branches = state.protected_branches.write().unwrap();
    if branches.iter().any(|b| b.name == payload.name) {
        return (
            StatusCode::CONFLICT,
            Json(ProtectedBranch {
                name: "".to_string(),
                enable_push: false,
                enable_force_push: false,
            }),
        );
    }
    let protection = ProtectedBranch {
        name: payload.name,
        enable_push: payload.enable_push,
        enable_force_push: payload.enable_force_push,
    };
    branches.push(protection.clone());
    (StatusCode::CREATED, Json(protection))
}

pub async fn delete_branch_protection(
    State(state): State<AppState>,
    Path((_owner, _repo, name)): Path<(String, String, String)>,
) -> StatusCode {
    let mut branches = state.protected_branches.write().unwrap();
    if let Some(pos) = branches.iter().position(|b| b.name == name) {
        branches.remove(pos);
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}

pub async fn search_issues_global(
    State(state): State<AppState>,
    Query(filter): Query<IssueFilterOptions>,
) -> Json<Vec<Issue>> {
    let issues = state.issues.read().unwrap();
    let mut filtered_issues: Vec<Issue> = issues.clone();

    if let Some(q) = filter.q {
        let q_lower = q.to_lowercase();
        filtered_issues.retain(|i| {
            i.title.to_lowercase().contains(&q_lower)
                || i.body
                    .clone()
                    .unwrap_or_default()
                    .to_lowercase()
                    .contains(&q_lower)
        });
    }
    Json(filtered_issues)
}
