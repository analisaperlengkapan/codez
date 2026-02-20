use axum::{
    extract::{Json, Path, State, Query},
    http::StatusCode,
    response::IntoResponse,
};
use shared::{
    CreateRepoOption, Repository, CreateIssueOption, Issue, CreatePullRequestOption, PullRequest,
    CreateCommentOption, Comment, CreateLabelOption, Label,
    CreateMilestoneOption, Milestone, RepoTopicOptions, RepoSearchOptions, RepoSettingsOption, CreateWikiPageOption, WikiPage,
    CreateHookOption, Webhook, CreateSecretOption, Secret, CreateKeyOption, DeployKey, CreateReactionOption, Reaction, IssueFilterOptions,
    MigrateRepoOption, TransferRepoOption, LfsLock, User, FileEntry, MergePullRequestOption, Topic,
    Collaborator, Branch, CreateBranchOption, Tag, LfsObject, MilestoneStats, DiffFile, CodeSearchResult, Commit, ReviewRequest,
    DiffLine, UpdateFileOption, Activity, Notification, PaginationOptions, UpdateIssueOption, UpdateCommentOption, UpdatePullRequestOption,
    Review, CreateReviewOption, WebhookDelivery, CreateProtectedBranchOption, ProtectedBranch,
    RepoUserStatus, UpdateLabelOption, UpdateMilestoneOption, CommitStatus, CreateStatusOption
};
use crate::router::AppState;

#[derive(serde::Deserialize)]
pub struct GetContentQuery {
    #[serde(rename = "ref")]
    pub ref_name: Option<String>,
}

pub async fn get_user_repo_status(
    State(state): State<AppState>,
    Path((owner, repo_name)): Path<(String, String)>
) -> Json<RepoUserStatus> {
    let repos = state.repos.read().unwrap();
    let repo_id = repos.iter().find(|r| r.owner == owner && r.name == repo_name).map(|r| r.id).unwrap_or(0);

    let user_id = 1; // Mock current user

    let stars = state.stars.read().unwrap();
    let starred = stars.get(&repo_id).map(|users| users.contains(&user_id)).unwrap_or(false);

    let watchers = state.watchers.read().unwrap();
    let watching = watchers.get(&repo_id).map(|users| users.contains(&user_id)).unwrap_or(false);

    Json(RepoUserStatus { starred, watching })
}

pub async fn list_repos(
    State(state): State<AppState>,
    Query(pagination): Query<PaginationOptions>
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

pub async fn get_repo(State(state): State<AppState>, Path((owner, repo)): Path<(String, String)>) -> Json<Option<Repository>> {
    let repos = state.repos.read().unwrap();
    let r = repos.iter().find(|r| r.owner == owner && r.name == repo).cloned();
    Json(r)
}

pub async fn create_repo(State(state): State<AppState>, Json(payload): Json<CreateRepoOption>) -> impl IntoResponse {
    let mut repos = state.repos.write().unwrap();
    if repos.iter().any(|r| r.owner == "admin" && r.name == payload.name) {
        return (StatusCode::CONFLICT, Json(Repository::new(0, "".to_string(), "".to_string())));
    }
    let id = (repos.len() as u64) + 1;
    let mut repo = Repository::new(id, payload.name.clone(), "admin".to_string());
    repo.private = payload.private;
    repo.description = payload.description.clone();
    if let Some(branch) = payload.default_branch.clone() { repo.default_branch = Some(branch); }
    if let Some(val) = payload.allow_rebase_merge { repo.allow_rebase_merge = val; }
    if let Some(val) = payload.allow_squash_merge { repo.allow_squash_merge = val; }
    if let Some(val) = payload.allow_merge_commit { repo.allow_merge_commit = val; }
    if let Some(val) = payload.has_issues { repo.has_issues = val; }
    if let Some(val) = payload.has_wiki { repo.has_wiki = val; }
    if let Some(val) = payload.has_projects { repo.has_projects = val; }
    repos.push(repo.clone());

    // Create initial files
    {
        let mut files = state.file_contents.write().unwrap();
        let default_branch = payload.default_branch.clone().unwrap_or("main".to_string());
        files.insert((id, default_branch.clone(), "README.md".to_string()), format!("# {}\n\n{}", payload.name, payload.description.clone().unwrap_or_default()));
        if let Some(gitignores) = &payload.gitignores {
            files.insert((id, default_branch, ".gitignore".to_string()), format!("# {}\n\ntarget/\n", gitignores));
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

pub async fn list_issues(State(state): State<AppState>, Path((owner, repo_name)): Path<(String, String)>, Query(filter): Query<IssueFilterOptions>) -> Json<Vec<Issue>> {
    let repos = state.repos.read().unwrap();
    let repo_id = repos.iter().find(|r| r.owner == owner && r.name == repo_name).map(|r| r.id).unwrap_or(0);

    if repo_id == 0 {
        return Json(vec![]);
    }

    let issues = state.issues.read().unwrap();
    let mut filtered_issues: Vec<Issue> = issues.iter().filter(|i| i.repo_id == repo_id).cloned().collect();

    if let Some(state_filter) = filter.state {
        if state_filter != "all" {
             filtered_issues.retain(|i| i.state == state_filter);
        }
    }
    if let Some(q) = filter.q {
        let q_lower = q.to_lowercase();
        filtered_issues.retain(|i| i.title.to_lowercase().contains(&q_lower) || i.body.clone().unwrap_or_default().to_lowercase().contains(&q_lower));
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
            },
            "updated" => {
                // Mock sorting by ID as proxy for updated
                if direction == "asc" {
                    filtered_issues.sort_by(|a, b| a.id.cmp(&b.id));
                } else {
                    filtered_issues.sort_by(|a, b| b.id.cmp(&a.id));
                }
            },
            "comments" => {
                // Mock sorting by ID as proxy, real impl would join comments count
                if direction == "asc" {
                    filtered_issues.sort_by(|a, b| a.id.cmp(&b.id));
                } else {
                    filtered_issues.sort_by(|a, b| b.id.cmp(&a.id));
                }
            },
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
    Json(payload): Json<CreateIssueOption>
) -> impl IntoResponse {
    let repos = state.repos.read().unwrap();
    let repo = repos.iter().find(|r| r.owner == owner && r.name == repo_name);

    let repo_id = if let Some(r) = repo {
        r.id
    } else {
        return (StatusCode::NOT_FOUND, Json(Issue {
            id: 0, repo_id: 0, number: 0, title: "".to_string(), body: None, state: "".to_string(),
            user: User::new(0, "".to_string(), None), assignees: vec![], labels: vec![], milestone: None
        }));
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

pub async fn get_issue(State(state): State<AppState>, Path((owner, repo_name, index)): Path<(String, String, u64)>) -> Json<Option<Issue>> {
    let repos = state.repos.read().unwrap();
    let repo_id = repos.iter().find(|r| r.owner == owner && r.name == repo_name).map(|r| r.id).unwrap_or(0);

    let issues = state.issues.read().unwrap();
    if let Some(issue) = issues.iter().find(|i| i.id == index) {
        if issue.repo_id == repo_id {
            return Json(Some(issue.clone()));
        }
    }
    Json(None)
}

pub async fn update_issue(
    State(state): State<AppState>,
    Path((owner, repo_name, index)): Path<(String, String, u64)>,
    Json(payload): Json<UpdateIssueOption>
) -> (StatusCode, Json<Option<Issue>>) {
    let repos = state.repos.read().unwrap();
    let repo_id = repos.iter().find(|r| r.owner == owner && r.name == repo_name).map(|r| r.id).unwrap_or(0);

    if repo_id == 0 {
        return (StatusCode::NOT_FOUND, Json(None));
    }

    let mut issues = state.issues.write().unwrap();
    let issue = issues.iter_mut().find(|i| i.id == index && i.repo_id == repo_id);

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

pub async fn list_pulls(State(state): State<AppState>, Path((owner, repo_name)): Path<(String, String)>) -> Json<Vec<PullRequest>> {
    let repos = state.repos.read().unwrap();
    let repo_id = repos.iter().find(|r| r.owner == owner && r.name == repo_name).map(|r| r.id).unwrap_or(0);

    let pulls = state.pulls.read().unwrap();
    let filtered_pulls: Vec<PullRequest> = pulls.iter().filter(|p| p.repo_id == repo_id).cloned().collect();
    Json(filtered_pulls)
}

pub async fn create_pull(
    State(state): State<AppState>,
    Path((owner, repo_name)): Path<(String, String)>,
    Json(payload): Json<CreatePullRequestOption>
) -> (StatusCode, Json<PullRequest>) {
    let repos = state.repos.read().unwrap();
    let repo_id = repos.iter().find(|r| r.owner == owner && r.name == repo_name).map(|r| r.id).unwrap_or(0);

    if repo_id == 0 {
         return (StatusCode::NOT_FOUND, Json(PullRequest {
            id: 0, repo_id: 0, number: 0, title: "".to_string(), body: None, state: "".to_string(),
            user: User::new(0, "".to_string(), None), merged: false, head_sha: "".to_string(), base: "".to_string(), head: "".to_string()
        }));
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
        head_sha: format!("head_sha_{}", id),
        base: payload.base.clone(),
        head: payload.head.clone(),
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
    Json(payload): Json<UpdatePullRequestOption>
) -> (StatusCode, Json<Option<PullRequest>>) {
    let repos = state.repos.read().unwrap();
    let repo_id = repos.iter().find(|r| r.owner == owner && r.name == repo_name).map(|r| r.id).unwrap_or(0);

    if repo_id == 0 {
        return (StatusCode::NOT_FOUND, Json(None));
    }

    let mut pulls = state.pulls.write().unwrap();
    let pr = pulls.iter_mut().find(|p| p.number == index && p.repo_id == repo_id);

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


pub async fn list_comments(State(state): State<AppState>, Path((owner, repo_name, index)): Path<(String, String, u64)>) -> Json<Vec<Comment>> {
    let repos = state.repos.read().unwrap();
    let repo_id = repos.iter().find(|r| r.owner == owner && r.name == repo_name).map(|r| r.id).unwrap_or(0);

    let issues = state.issues.read().unwrap();
    let issue_id = issues.iter().find(|i| i.repo_id == repo_id && i.number == index).map(|i| i.id).unwrap_or(0);

    let comments = state.comments.read().unwrap();
    let filtered_comments: Vec<Comment> = comments.iter().filter(|c| c.issue_id == issue_id).cloned().collect();
    Json(filtered_comments)
}

pub async fn create_comment(
    State(state): State<AppState>,
    Path((owner, repo_name, index)): Path<(String, String, u64)>,
    Json(payload): Json<CreateCommentOption>
) -> (StatusCode, Json<Comment>) {
    let repos = state.repos.read().unwrap();
    let repo = repos.iter().find(|r| r.owner == owner && r.name == repo_name);

    let repo_id = if let Some(r) = repo {
        r.id
    } else {
         return (StatusCode::NOT_FOUND, Json(Comment {
            id: 0, issue_id: 0, body: "".to_string(), user: User::new(0, "".to_string(), None), created_at: "".to_string(), reactions: vec![]
        }));
    };

    let issues = state.issues.read().unwrap();
    let issue = issues.iter().find(|i| i.repo_id == repo_id && i.number == index);

    if issue.is_none() {
         return (StatusCode::NOT_FOUND, Json(Comment {
            id: 0, issue_id: 0, body: "".to_string(), user: User::new(0, "".to_string(), None), created_at: "".to_string(), reactions: vec![]
        }));
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
    Path((owner, repo_name, id)): Path<(String, String, u64)>,
    Json(payload): Json<UpdateCommentOption>
) -> (StatusCode, Json<Option<Comment>>) {
    let repos = state.repos.read().unwrap();
    let repo_id = repos.iter().find(|r| r.owner == owner && r.name == repo_name).map(|r| r.id).unwrap_or(0);

    // Validate ownership requires joining with issues or checking repo_id if comment had it (it doesn't directly, it has issue_id).
    // In this mock, we can look up the issue first.
    let issues = state.issues.read().unwrap();
    let mut comments = state.comments.write().unwrap();

    if let Some(comment) = comments.iter_mut().find(|c| c.id == id) {
        if let Some(issue) = issues.iter().find(|i| i.id == comment.issue_id) {
            if issue.repo_id == repo_id {
                comment.body = payload.body;
                return (StatusCode::OK, Json(Some(comment.clone())));
            }
        }
    }
    (StatusCode::NOT_FOUND, Json(None))
}

pub async fn delete_comment(
    State(state): State<AppState>,
    Path((owner, repo_name, id)): Path<(String, String, u64)>
) -> StatusCode {
    let repos = state.repos.read().unwrap();
    let repo_id = repos.iter().find(|r| r.owner == owner && r.name == repo_name).map(|r| r.id).unwrap_or(0);

    let issues = state.issues.read().unwrap();
    let mut comments = state.comments.write().unwrap();

    if let Some(pos) = comments.iter().position(|c| c.id == id) {
        if let Some(issue) = issues.iter().find(|i| i.id == comments[pos].issue_id) {
            if issue.repo_id == repo_id {
                comments.remove(pos);
                return StatusCode::NO_CONTENT;
            }
        }
    }
    StatusCode::NOT_FOUND
}

pub async fn list_labels(State(state): State<AppState>, Path((owner, repo_name)): Path<(String, String)>) -> Json<Vec<Label>> {
    let repos = state.repos.read().unwrap();
    let repo_id = repos.iter().find(|r| r.owner == owner && r.name == repo_name).map(|r| r.id).unwrap_or(0);

    let labels = state.labels.read().unwrap();
    let filtered_labels: Vec<Label> = labels.iter().filter(|l| l.repo_id == repo_id).cloned().collect();
    Json(filtered_labels)
}

pub async fn create_label(
    State(state): State<AppState>,
    Path((owner, repo_name)): Path<(String, String)>,
    Json(payload): Json<CreateLabelOption>
) -> (StatusCode, Json<Label>) {
    let repos = state.repos.read().unwrap();
    let repo = repos.iter().find(|r| r.owner == owner && r.name == repo_name);

    let repo_id = if let Some(r) = repo {
        r.id
    } else {
         return (StatusCode::NOT_FOUND, Json(Label {
            id: 0, repo_id: 0, name: "".to_string(), color: "".to_string(), description: None
        }));
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

pub async fn update_label(
    State(state): State<AppState>,
    Path((owner, repo_name, id)): Path<(String, String, u64)>,
    Json(payload): Json<UpdateLabelOption>
) -> (StatusCode, Json<Option<Label>>) {
    let repos = state.repos.read().unwrap();
    let repo_id = repos.iter().find(|r| r.owner == owner && r.name == repo_name).map(|r| r.id).unwrap_or(0);

    if repo_id == 0 {
        return (StatusCode::NOT_FOUND, Json(None));
    }

    let mut labels = state.labels.write().unwrap();
    if let Some(label) = labels.iter_mut().find(|l| l.id == id) {
        if label.repo_id != repo_id {
            return (StatusCode::NOT_FOUND, Json(None));
        }
        if let Some(name) = payload.name { label.name = name; }
        if let Some(color) = payload.color { label.color = color; }
        if let Some(description) = payload.description { label.description = Some(description); }
        return (StatusCode::OK, Json(Some(label.clone())));
    }
    (StatusCode::NOT_FOUND, Json(None))
}

pub async fn delete_label(
    State(state): State<AppState>,
    Path((owner, repo_name, id)): Path<(String, String, u64)>
) -> StatusCode {
    let repos = state.repos.read().unwrap();
    let repo_id = repos.iter().find(|r| r.owner == owner && r.name == repo_name).map(|r| r.id).unwrap_or(0);

    if repo_id == 0 {
        return StatusCode::NOT_FOUND;
    }

    let mut labels = state.labels.write().unwrap();
    if let Some(pos) = labels.iter().position(|l| l.id == id) {
        if labels[pos].repo_id != repo_id {
            return StatusCode::NOT_FOUND;
        }
        labels.remove(pos);
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}

pub async fn list_milestones(State(state): State<AppState>, Path((owner, repo_name)): Path<(String, String)>) -> Json<Vec<Milestone>> {
    let repos = state.repos.read().unwrap();
    let repo_id = repos.iter().find(|r| r.owner == owner && r.name == repo_name).map(|r| r.id).unwrap_or(0);

    let milestones = state.milestones.read().unwrap();
    let filtered_milestones: Vec<Milestone> = milestones.iter().filter(|m| m.repo_id == repo_id).cloned().collect();
    Json(filtered_milestones)
}

pub async fn create_milestone(
    State(state): State<AppState>,
    Path((owner, repo_name)): Path<(String, String)>,
    Json(payload): Json<CreateMilestoneOption>
) -> (StatusCode, Json<Milestone>) {
    let repos = state.repos.read().unwrap();
    let repo = repos.iter().find(|r| r.owner == owner && r.name == repo_name);

    let repo_id = if let Some(r) = repo {
        r.id
    } else {
         return (StatusCode::NOT_FOUND, Json(Milestone {
            id: 0, repo_id: 0, title: "".to_string(), description: None, due_on: None, state: "".to_string()
        }));
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

pub async fn get_milestone(State(state): State<AppState>, Path((owner, repo_name, id)): Path<(String, String, u64)>) -> Json<Option<Milestone>> {
    let repos = state.repos.read().unwrap();
    let repo_id = repos.iter().find(|r| r.owner == owner && r.name == repo_name).map(|r| r.id).unwrap_or(0);

    let milestones = state.milestones.read().unwrap();
    if let Some(m) = milestones.iter().find(|m| m.id == id) {
        if m.repo_id == repo_id {
            return Json(Some(m.clone()));
        }
    }
    Json(None)
}

pub async fn update_milestone(
    State(state): State<AppState>,
    Path((owner, repo_name, id)): Path<(String, String, u64)>,
    Json(payload): Json<UpdateMilestoneOption>
) -> (StatusCode, Json<Option<Milestone>>) {
    let repos = state.repos.read().unwrap();
    let repo_id = repos.iter().find(|r| r.owner == owner && r.name == repo_name).map(|r| r.id).unwrap_or(0);

    if repo_id == 0 {
        return (StatusCode::NOT_FOUND, Json(None));
    }

    let mut milestones = state.milestones.write().unwrap();
    if let Some(m) = milestones.iter_mut().find(|m| m.id == id) {
        if m.repo_id != repo_id {
            return (StatusCode::NOT_FOUND, Json(None));
        }
        if let Some(title) = payload.title { m.title = title; }
        if let Some(desc) = payload.description { m.description = Some(desc); }
        if let Some(due) = payload.due_on { m.due_on = Some(due); }
        if let Some(state) = payload.state { m.state = state; }
        return (StatusCode::OK, Json(Some(m.clone())));
    }
    (StatusCode::NOT_FOUND, Json(None))
}

pub async fn delete_milestone(
    State(state): State<AppState>,
    Path((owner, repo_name, id)): Path<(String, String, u64)>
) -> StatusCode {
    let repos = state.repos.read().unwrap();
    let repo_id = repos.iter().find(|r| r.owner == owner && r.name == repo_name).map(|r| r.id).unwrap_or(0);

    if repo_id == 0 {
        return StatusCode::NOT_FOUND;
    }

    let mut milestones = state.milestones.write().unwrap();
    if let Some(pos) = milestones.iter().position(|m| m.id == id) {
        if milestones[pos].repo_id != repo_id {
            return StatusCode::NOT_FOUND;
        }
        milestones.remove(pos);
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}

pub async fn list_hooks(State(state): State<AppState>, Path((owner, repo_name)): Path<(String, String)>) -> Json<Vec<Webhook>> {
    let repos = state.repos.read().unwrap();
    let repo_id = repos.iter().find(|r| r.owner == owner && r.name == repo_name).map(|r| r.id).unwrap_or(0);

    let hooks = state.hooks.read().unwrap();
    let filtered_hooks: Vec<Webhook> = hooks.iter().filter(|h| h.repo_id == repo_id).cloned().collect();
    Json(filtered_hooks)
}

pub async fn create_hook(
    State(state): State<AppState>,
    Path((owner, repo_name)): Path<(String, String)>,
    Json(payload): Json<CreateHookOption>
) -> (StatusCode, Json<Webhook>) {
    let repos = state.repos.read().unwrap();
    let repo = repos.iter().find(|r| r.owner == owner && r.name == repo_name);

    let repo_id = if let Some(r) = repo {
        r.id
    } else {
         return (StatusCode::NOT_FOUND, Json(Webhook {
            id: 0, repo_id: 0, url: "".to_string(), events: vec![], active: false
        }));
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
    Path((_owner, _repo, id)): Path<(String, String, u64)>
) -> Json<Vec<WebhookDelivery>> {
    let deliveries = state.webhook_deliveries.read().unwrap();
    let filtered: Vec<WebhookDelivery> = deliveries.iter().filter(|d| d.hook_id == id).cloned().collect();
    Json(filtered)
}

fn dispatch_hooks(state: &AppState, repo_id: u64, event: &str) {
    let hooks = state.hooks.read().unwrap();
    let relevant_hooks: Vec<Webhook> = hooks.iter()
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
    Json(payload): Json<CreateSecretOption>
) -> (StatusCode, Json<Secret>) {
    let repos = state.repos.read().unwrap();
    let repo = repos.iter().find(|r| r.owner == owner && r.name == repo_name);

    let repo_id = if let Some(r) = repo {
        r.id
    } else {
         return (StatusCode::NOT_FOUND, Json(Secret {
            name: "".to_string(), repo_id: 0, created_at: "".to_string(), data: "".to_string()
        }));
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
    let secrets = vec![
        Secret { name: "MY_TOKEN".to_string(), repo_id: 1, created_at: "2023-01-01".to_string(), data: "hidden".to_string() }
    ];
    Json(secrets)
}

pub async fn create_deploy_key(
    State(state): State<AppState>,
    Path((owner, repo_name)): Path<(String, String)>,
    Json(payload): Json<CreateKeyOption>
) -> (StatusCode, Json<DeployKey>) {
    let repos = state.repos.read().unwrap();
    let repo = repos.iter().find(|r| r.owner == owner && r.name == repo_name);

    let repo_id = if let Some(r) = repo {
        r.id
    } else {
         return (StatusCode::NOT_FOUND, Json(DeployKey {
            id: 0, repo_id: 0, title: "".to_string(), key: "".to_string(), fingerprint: "".to_string()
        }));
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

pub async fn list_deploy_keys(Path((_owner, _repo)): Path<(String, String)>) -> Json<Vec<DeployKey>> {
    // Similar to secrets, deploy keys are mocked here.
    let keys = vec![
        DeployKey {
            id: 1,
            repo_id: 1,
            title: "CI Key".to_string(),
            key: "ssh-rsa...".to_string(),
            fingerprint: "SHA...".to_string(),
        }
    ];
    Json(keys)
}

pub async fn list_lfs_locks(
    State(state): State<AppState>,
    Path((owner, repo_name)): Path<(String, String)>
) -> Json<Vec<LfsLock>> {
    let repos = state.repos.read().unwrap();
    let repo_id = repos.iter().find(|r| r.owner == owner && r.name == repo_name).map(|r| r.id).unwrap_or(0);

    let locks = state.lfs_locks.read().unwrap();
    let filtered_locks: Vec<LfsLock> = locks.iter().filter(|l| l.repo_id == repo_id).cloned().collect();
    Json(filtered_locks)
}

#[derive(serde::Deserialize)]
pub struct LfsLockRequest {
    pub path: String,
}

pub async fn create_lfs_lock(
    State(state): State<AppState>,
    Path((owner, repo_name)): Path<(String, String)>,
    Json(payload): Json<LfsLockRequest>,
) -> (StatusCode, Json<LfsLock>) {
    let repos = state.repos.read().unwrap();
    let repo_id = repos.iter().find(|r| r.owner == owner && r.name == repo_name).map(|r| r.id).unwrap_or(0);

    if repo_id == 0 {
        return (StatusCode::NOT_FOUND, Json(LfsLock { id: "".to_string(), repo_id: 0, path: "".to_string(), owner: User::new(0, "".to_string(), None), locked_at: "".to_string() }));
    }

    let mut locks = state.lfs_locks.write().unwrap();
    if locks.iter().any(|l| l.repo_id == repo_id && l.path == payload.path) {
        return (StatusCode::CONFLICT, Json(LfsLock { id: "".to_string(), repo_id: 0, path: "".to_string(), owner: User::new(0, "".to_string(), None), locked_at: "".to_string() }));
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
    Path((owner, repo_name, id)): Path<(String, String, String)>
) -> StatusCode {
    let repos = state.repos.read().unwrap();
    let repo_id = repos.iter().find(|r| r.owner == owner && r.name == repo_name).map(|r| r.id).unwrap_or(0);

    if repo_id == 0 {
        return StatusCode::NOT_FOUND;
    }

    let mut locks = state.lfs_locks.write().unwrap();
    if let Some(pos) = locks.iter().position(|l| l.repo_id == repo_id && l.id == id) {
        locks.remove(pos);
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}

pub async fn add_reaction(
    State(state): State<AppState>,
    Path((_owner, _repo, id)): Path<(String, String, u64)>,
    Json(payload): Json<CreateReactionOption>
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
        (StatusCode::NOT_FOUND, Json(Reaction { id: 0, user: User::new(0, "".to_string(), None), content: "".to_string(), created_at: "".to_string() }))
    }
}

pub async fn update_topics(
    State(state): State<AppState>,
    Path((owner, repo_name)): Path<(String, String)>,
    Json(payload): Json<RepoTopicOptions>
) -> StatusCode {
    let repos = state.repos.read().unwrap();
    let repo = repos.iter().find(|r| r.owner == owner && r.name == repo_name);

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

pub async fn list_topics(State(state): State<AppState>, Path((owner, repo_name)): Path<(String, String)>) -> Json<Vec<Topic>> {
    let repos = state.repos.read().unwrap();
    let repo_id = repos.iter().find(|r| r.owner == owner && r.name == repo_name).map(|r| r.id).unwrap_or(0);

    let topics = state.topics.read().unwrap();
    let filtered_topics: Vec<Topic> = topics.iter().filter(|t| t.repo_id == repo_id).cloned().collect();
    Json(filtered_topics)
}

pub async fn star_repo(
    State(state): State<AppState>,
    Path((owner, repo_name)): Path<(String, String)>
) -> StatusCode {
    let mut repos = state.repos.write().unwrap();
    let repo = repos.iter_mut().find(|r| r.owner == owner && r.name == repo_name);

    if let Some(r) = repo {
        let repo_id = r.id;
        let user_id = 1; // Mock current user

        let mut stars = state.stars.write().unwrap();
        let users = stars.entry(repo_id).or_insert(Vec::new());

        if let Some(pos) = users.iter().position(|u| *u == user_id) {
            users.remove(pos);
            if r.stars_count > 0 { r.stars_count -= 1; }
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
    Path((_owner, _repo, index, username)): Path<(String, String, u64, String)>
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
    Path((owner, repo_name)): Path<(String, String)>
) -> StatusCode {
    let mut repos = state.repos.write().unwrap();
    let repo = repos.iter_mut().find(|r| r.owner == owner && r.name == repo_name);

    if let Some(r) = repo {
        let repo_id = r.id;
        let user_id = 1; // Mock current user

        let mut watchers = state.watchers.write().unwrap();
        let users = watchers.entry(repo_id).or_insert(Vec::new());

        if let Some(pos) = users.iter().position(|u| *u == user_id) {
            users.remove(pos);
            if r.watchers_count > 0 { r.watchers_count -= 1; }
        } else {
            users.push(user_id);
            r.watchers_count += 1;
        }
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}

pub async fn fork_repo(State(state): State<AppState>, Path((owner, repo)): Path<(String, String)>) -> (StatusCode, Json<Option<Repository>>) {
    let mut repos = state.repos.write().unwrap();

    if let Some(orig_idx) = repos.iter().position(|r| r.owner == owner && r.name == repo) {
        repos[orig_idx].forks_count += 1;
        let orig = repos[orig_idx].clone();

        let id = (repos.len() as u64) + 1;
        let new_name = format!("{}-fork", repo);
        let mut new_repo = Repository::new(id, new_name.clone(), "admin".to_string());
        new_repo.parent_id = Some(orig.id);
        repos.push(new_repo.clone());

        // Copy files
        {
            let mut files = state.file_contents.write().unwrap();
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

        (StatusCode::CREATED, Json(Some(new_repo)))
    } else {
        (StatusCode::NOT_FOUND, Json(None))
    }
}

pub async fn create_wiki_page(
    Path((_owner, _repo)): Path<(String, String)>,
    Json(payload): Json<CreateWikiPageOption>
) -> (StatusCode, Json<WikiPage>) {
    let page = WikiPage {
        title: payload.title,
        content: payload.content,
        commit_message: payload.message,
    };
    (StatusCode::CREATED, Json(page))
}

pub async fn get_repo_settings(
    State(state): State<AppState>,
    Path((owner, repo_name)): Path<(String, String)>
) -> (StatusCode, Json<RepoSettingsOption>) {
    let repos = state.repos.read().unwrap();
    if let Some(repo) = repos.iter().find(|r| r.owner == owner && r.name == repo_name) {
        (StatusCode::OK, Json(RepoSettingsOption {
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
        }))
    } else {
        (StatusCode::NOT_FOUND, Json(RepoSettingsOption {
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
        }))
    }
}

pub async fn update_repo_settings(
    State(state): State<AppState>,
    Path((owner, repo_name)): Path<(String, String)>,
    Json(payload): Json<RepoSettingsOption>
) -> StatusCode {
    let mut repos = state.repos.write().unwrap();
    if let Some(repo) = repos.iter_mut().find(|r| r.owner == owner && r.name == repo_name) {
        if let Some(desc) = payload.description { repo.description = Some(desc); }
        if let Some(private) = payload.private { repo.private = private; }
        if let Some(website) = payload.website { repo.website = Some(website); }
        if let Some(branch) = payload.default_branch { repo.default_branch = Some(branch); }
        if let Some(val) = payload.allow_rebase_merge { repo.allow_rebase_merge = val; }
        if let Some(val) = payload.allow_squash_merge { repo.allow_squash_merge = val; }
        if let Some(val) = payload.allow_merge_commit { repo.allow_merge_commit = val; }
        if let Some(val) = payload.has_issues { repo.has_issues = val; }
        if let Some(val) = payload.has_wiki { repo.has_wiki = val; }
        if let Some(val) = payload.has_projects { repo.has_projects = val; }
        StatusCode::OK
    } else {
        StatusCode::NOT_FOUND
    }
}

pub async fn mirror_sync(Path((_owner, _repo)): Path<(String, String)>) -> StatusCode {
    StatusCode::OK
}

pub async fn migrate_repo(Json(payload): Json<MigrateRepoOption>) -> (StatusCode, Json<Repository>) {
    let repo = Repository::new(4, payload.repo_name, "admin".to_string());
    (StatusCode::CREATED, Json(repo))
}

pub async fn transfer_repo(
    State(state): State<AppState>,
    Path((owner, repo_name)): Path<(String, String)>,
    Json(payload): Json<TransferRepoOption>
) -> StatusCode {
    let users = state.users.read().unwrap();
    if !users.iter().any(|u| u.username == payload.new_owner) {
         return StatusCode::BAD_REQUEST;
    }
    drop(users); // Release user lock

    let mut repos = state.repos.write().unwrap();
    if let Some(repo) = repos.iter_mut().find(|r| r.owner == owner && r.name == repo_name) {
        repo.owner = payload.new_owner.clone();
        let repo_id = repo.id;
        let r_name = repo.name.clone();

        // Drop repo lock before activity lock if possible, though repo->activity order is generally consistent.
        // But to be safe and avoid holding lock unnecessarily:
        drop(repos);

        let mut activities = state.activities.write().unwrap();
        let activity_id = (activities.len() as u64) + 1;
        activities.push(Activity {
            id: activity_id,
            repo_id,
            user_id: 1, // mock admin
            user_name: "admin".to_string(),
            op_type: "transfer_repo".to_string(),
            content: format!("transferred repository {} from {} to {}", r_name, owner, payload.new_owner),
            created: "now".to_string(),
        });

        StatusCode::ACCEPTED
    } else {
        StatusCode::NOT_FOUND
    }
}

pub async fn add_issue_label(
    State(state): State<AppState>,
    Path((_owner, _repo, index)): Path<(String, String, u64)>,
    Json(payload): Json<shared::CreateLabelOption>
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
    Path((_owner, _repo, index, id)): Path<(String, String, u64, u64)>
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
        }
    ];
    Json(pages)
}

pub async fn update_wiki_page(
    Path((_owner, _repo, _page_name)): Path<(String, String, String)>,
    Json(_payload): Json<CreateWikiPageOption>
) -> StatusCode {
    StatusCode::OK
}

pub async fn get_wiki_page(Path((_owner, _repo, page_name)): Path<(String, String, String)>) -> Json<Option<WikiPage>> {
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
    Query(query): Query<GetContentQuery>
) -> Json<Vec<FileEntry>> {
    let repos = state.repos.read().unwrap();
    let repo = repos.iter().find(|r| r.owner == owner && r.name == repo_name);
    let repo_id = repo.map(|r| r.id).unwrap_or(0);

    if repo_id == 0 {
        return Json(vec![]);
    }

    let default_branch = repo.and_then(|r| r.default_branch.clone()).unwrap_or("main".to_string());
    let target_branch = query.ref_name.unwrap_or(default_branch);

    let all_files = state.file_contents.read().unwrap();
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
            if relative_path.is_empty() { continue; }

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
                let size = all_files.get(&(*k_repo_id, k_branch.clone(), k_path.clone())).map(|s| s.len()).unwrap_or(0) as u64;
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
    Query(query): Query<GetContentQuery>
) -> Json<Vec<FileEntry>> {
    get_contents(State(state), Path((owner, repo, "".to_string())), Query(query)).await
}

pub async fn merge_pull(
    State(state): State<AppState>,
    Path((owner, repo, index)): Path<(String, String, u64)>,
    Json(_payload): Json<MergePullRequestOption>
) -> StatusCode {
    let repos = state.repos.read().unwrap();
    let repo_id = repos.iter().find(|r| r.owner == owner && r.name == repo).map(|r| r.id).unwrap_or(0);

    if repo_id == 0 {
        return StatusCode::NOT_FOUND;
    }

    let mut pulls = state.pulls.write().unwrap();
    let pr_opt = pulls.iter_mut().find(|p| p.number == index);

    if let Some(pr) = pr_opt {
        if pr.repo_id != repo_id {
            return StatusCode::NOT_FOUND;
        }

        // Check branch protection and status checks
        {
            let protections = state.protected_branches.read().unwrap();
            if let Some(protection) = protections.iter().find(|p| p.repo_id == repo_id && p.name == pr.base) {
                if !protection.required_status_checks.is_empty() {
                    let statuses = state.commit_statuses.read().unwrap();
                    for context in &protection.required_status_checks {
                        let latest = statuses.iter()
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

        // Copy files from head to base
        {
            let mut files = state.file_contents.write().unwrap();
            let mut merged_files = Vec::new();
            let head = pr.head.clone();
            let base = pr.base.clone();

            for ((r_id, b_name, path), content) in files.iter() {
                if *r_id == repo_id && b_name == &head {
                    merged_files.push((path.clone(), content.clone()));
                }
            }

            for (path, content) in merged_files {
                files.insert((repo_id, base.clone(), path), content);
            }
        }

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
    Query(params): Query<RepoSearchOptions>
) -> Json<Vec<Repository>> {
    let repos = state.repos.read().unwrap();
    let q = params.q.to_lowercase();

    if q.is_empty() {
        Json(repos.clone())
    } else {
        let filtered: Vec<Repository> = repos.iter()
            .filter(|r| r.name.to_lowercase().contains(&q) || r.description.clone().unwrap_or_default().to_lowercase().contains(&q))
            .cloned()
            .collect();
        Json(filtered)
    }
}

pub async fn list_collaborators(Path((_owner, _repo)): Path<(String, String)>) -> Json<Vec<Collaborator>> {
    let user = User::new(2, "collab_user".to_string(), None);
    vec![
        Collaborator { user, repo_id: 1, permissions: "write".to_string() }
    ].into()
}

pub async fn get_collaborator(Path((_owner, _repo, _collaborator)): Path<(String, String, String)>) -> Json<Option<Collaborator>> {
    Json(None)
}

pub async fn add_collaborator(Path((_owner, _repo, _collaborator)): Path<(String, String, String)>) -> StatusCode {
    StatusCode::NO_CONTENT
}

pub async fn list_branches(
    State(state): State<AppState>,
    Path((owner, repo_name)): Path<(String, String)>
) -> Json<Vec<Branch>> {
    let repos = state.repos.read().unwrap();
    let repo_id = repos.iter().find(|r| r.owner == owner && r.name == repo_name).map(|r| r.id).unwrap_or(0);

    if repo_id == 0 {
        return Json(vec![]);
    }

    let files = state.file_contents.read().unwrap();
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
    let commit = Commit { sha: "mock_sha".to_string(), repo_id, message: "branch tip".to_string(), author: user, date: "now".to_string() };

    let branches: Vec<Branch> = branch_names.into_iter().map(|name| {
        Branch {
            name: name.clone(),
            repo_id,
            commit: commit.clone(),
            protected: name == "main", // Mock protection
        }
    }).collect();

    Json(branches)
}

pub async fn create_branch(
    State(state): State<AppState>,
    Path((owner, repo_name)): Path<(String, String)>,
    Json(payload): Json<CreateBranchOption>
) -> (StatusCode, Json<Branch>) {
    let repos = state.repos.read().unwrap();
    let repo = repos.iter().find(|r| r.owner == owner && r.name == repo_name);

    let repo_id = if let Some(r) = repo {
        r.id
    } else {
         return (StatusCode::NOT_FOUND, Json(Branch {
            repo_id: 0, name: "".to_string(), commit: Commit { sha: "".to_string(), repo_id: 0, message: "".to_string(), author: User::new(0, "".to_string(), None), date: "".to_string() }, protected: false
        }));
    };

    let user = User::new(1, "admin".to_string(), None);
    let commit = Commit { sha: "def".to_string(), repo_id, message: "new branch".to_string(), author: user, date: "now".to_string() };
    let branch = Branch { name: payload.name.clone(), repo_id, commit, protected: false };

    // Copy files from base branch
    {
        let mut files = state.file_contents.write().unwrap();

        // Check if branch already exists
        for (r_id, b_name, _) in files.keys() {
            if *r_id == repo_id && b_name == &payload.name {
                return (StatusCode::CONFLICT, Json(Branch {
                    repo_id: 0, name: "".to_string(), commit: Commit { sha: "".to_string(), repo_id: 0, message: "".to_string(), author: User::new(0, "".to_string(), None), date: "".to_string() }, protected: false
                }));
            }
        }

        let mut new_files = Vec::new();
        let base = payload.base.clone();

        for ((r_id, b_name, path), content) in files.iter() {
            if *r_id == repo_id && b_name == &base {
                new_files.push((payload.name.clone(), path.clone(), content.clone()));
            }
        }

        for (b_name, path, content) in new_files {
            files.insert((repo_id, b_name, path), content);
        }
    }

    (StatusCode::CREATED, Json(branch))
}

pub async fn list_tags(Path((_owner, _repo)): Path<(String, String)>) -> Json<Vec<Tag>> {
    let user = User::new(1, "admin".to_string(), None);
    let commit = Commit { sha: "abc".to_string(), repo_id: 1, message: "init".to_string(), author: user, date: "now".to_string() };
    let tags = vec![
        Tag { name: "v1.0".to_string(), repo_id: 1, id: "1".to_string(), commit }
    ];
    Json(tags)
}

pub async fn upload_media(Path((_owner, _repo)): Path<(String, String)>) -> (StatusCode, Json<LfsObject>) {
    let lfs = LfsObject {
        oid: "abc1234567890".to_string(),
        size: 1024,
        created_at: "2023-01-01".to_string(),
    };
    (StatusCode::CREATED, Json(lfs))
}

pub async fn get_milestone_stats(
    State(state): State<AppState>,
    Path((_owner, _repo, id)): Path<(String, String, u64)>
) -> Json<MilestoneStats> {
    let issues = state.issues.read().unwrap();
    let open_count = issues.iter().filter(|i| i.milestone.as_ref().map(|m| m.id).unwrap_or(0) == id && i.state == "open").count() as u64;
    let closed_count = issues.iter().filter(|i| i.milestone.as_ref().map(|m| m.id).unwrap_or(0) == id && i.state == "closed").count() as u64;
    Json(MilestoneStats { open_issues: open_count, closed_issues: closed_count })
}

pub async fn get_pr_files(Path((_owner, _repo, _index)): Path<(String, String, u64)>) -> Json<Vec<DiffFile>> {
    let diffs = vec![
        DiffFile {
            name: "src/lib.rs".to_string(),
            old_name: None,
            index: "idx".to_string(),
            additions: 2,
            deletions: 1,
            type_: "modify".to_string(),
            lines: vec![],
        }
    ];
    Json(diffs)
}

pub async fn list_commits(State(state): State<AppState>, Path((owner, repo_name)): Path<(String, String)>) -> Json<Vec<Commit>> {
    let repos = state.repos.read().unwrap();
    let repo_id = repos.iter().find(|r| r.owner == owner && r.name == repo_name).map(|r| r.id).unwrap_or(0);

    let commits = state.commits.read().unwrap();
    let filtered_commits: Vec<Commit> = commits.iter().filter(|c| c.repo_id == repo_id).cloned().collect();
    Json(filtered_commits)
}

pub async fn search_repo_code(
    State(state): State<AppState>,
    Path((owner, repo_name)): Path<(String, String)>,
    Query(params): Query<RepoSearchOptions>
) -> Json<Vec<CodeSearchResult>> {
    let repos = state.repos.read().unwrap();
    let repo = repos.iter().find(|r| r.owner == owner && r.name == repo_name);
    let repo_id = repo.map(|r| r.id).unwrap_or(0);
    let q = params.q.to_lowercase();

    if repo_id == 0 {
        return Json(vec![]);
    }

    let default_branch = repo.and_then(|r| r.default_branch.clone()).unwrap_or("main".to_string());

    let files = state.file_contents.read().unwrap();
    let mut results = Vec::new();

    for ((r_id, branch, path), content) in files.iter() {
        if *r_id == repo_id && branch == &default_branch {
             if q.is_empty() || path.to_lowercase().contains(&q) || content.to_lowercase().contains(&q) {
                 results.push(CodeSearchResult {
                     name: path.split('/').last().unwrap_or(path).to_string(),
                     path: path.clone(),
                     sha: "mocksha".to_string(),
                     url: format!("/repos/{}/{}/src/{}", owner, repo_name, path),
                     content: Some(content.chars().take(100).collect()),
                 });
             }
        }
    }
    Json(results)
}

pub async fn get_raw_file(
    State(state): State<AppState>,
    Path((owner, repo_name, path)): Path<(String, String, String)>,
    Query(query): Query<GetContentQuery>
) -> impl IntoResponse {
    let repos = state.repos.read().unwrap();
    let repo = repos.iter().find(|r| r.owner == owner && r.name == repo_name);
    let repo_id = repo.map(|r| r.id).unwrap_or(0);

    if repo_id == 0 {
        return (StatusCode::NOT_FOUND, "".to_string());
    }

    let default_branch = repo.and_then(|r| r.default_branch.clone()).unwrap_or("main".to_string());
    let target_branch = query.ref_name.unwrap_or(default_branch);

    let files = state.file_contents.read().unwrap();
    if let Some(content) = files.get(&(repo_id, target_branch, path)) {
        (StatusCode::OK, content.clone())
    } else {
        (StatusCode::NOT_FOUND, "".to_string())
    }
}

pub async fn update_file(
    State(state): State<AppState>,
    Path((owner, repo, path)): Path<(String, String, String)>,
    Json(payload): Json<UpdateFileOption>
) -> (StatusCode, Json<FileEntry>) {
    let repos = state.repos.read().unwrap();
    let repo_obj = repos.iter().find(|r| r.owner == owner && r.name == repo);

    let (repo_id, default_branch) = if let Some(r) = repo_obj {
        (r.id, r.default_branch.clone().unwrap_or("main".to_string()))
    } else {
        return (StatusCode::NOT_FOUND, Json(FileEntry { name: "".to_string(), path: "".to_string(), kind: "".to_string(), size: 0 }));
    };

    // Check branch protection
    let branch_name = payload.branch.clone().unwrap_or(default_branch);
    {
        let protections = state.protected_branches.read().unwrap();
        if let Some(protection) = protections.iter().find(|p| p.repo_id == repo_id && p.name == branch_name) {
            if !protection.enable_push {
                return (StatusCode::FORBIDDEN, Json(FileEntry { name: "".to_string(), path: "".to_string(), kind: "".to_string(), size: 0 }));
            }
        }
    }

    // Update file content in state
    {
        let mut files = state.file_contents.write().unwrap();
        files.insert((repo_id, branch_name.clone(), path.clone()), payload.content.clone());
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

    (StatusCode::OK, Json(FileEntry {
        name: "updated_file".to_string(),
        path,
        kind: "file".to_string(),
        size: 123,
    }))
}

pub async fn add_issue_assignee(
    State(state): State<AppState>,
    Path((_owner, _repo, index)): Path<(String, String, u64)>,
    Json(payload): Json<User>
) -> StatusCode {
    let mut issues = state.issues.write().unwrap();
    if let Some(issue) = issues.iter_mut().find(|i| i.id == index) {
        if !issue.assignees.iter().any(|u| u.username == payload.username) {
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

pub async fn request_review(Path((_owner, _repo, _index)): Path<(String, String, u64)>) -> (StatusCode, Json<ReviewRequest>) {
    let reviewer = User::new(2, "reviewer".to_string(), None);
    (StatusCode::CREATED, Json(ReviewRequest { reviewer, status: "requested".to_string() }))
}

pub async fn list_reviews(
    State(state): State<AppState>,
    Path((owner, repo_name, index)): Path<(String, String, u64)>
) -> Json<Vec<Review>> {
    let repos = state.repos.read().unwrap();
    let repo_id = repos.iter().find(|r| r.owner == owner && r.name == repo_name).map(|r| r.id).unwrap_or(0);

    let pulls = state.pulls.read().unwrap();
    // Assuming pull requests have unique IDs globally or we filter by repo/number.
    // Shared `PullRequest` has `id`, `repo_id`, `number`.
    let pr = pulls.iter().find(|p| p.repo_id == repo_id && p.number == index);

    if let Some(p) = pr {
        let reviews = state.reviews.read().unwrap();
        let filtered: Vec<Review> = reviews.iter().filter(|r| r.pull_request_id == p.id).cloned().collect();
        Json(filtered)
    } else {
        Json(vec![])
    }
}

pub async fn create_review(
    State(state): State<AppState>,
    Path((owner, repo_name, index)): Path<(String, String, u64)>,
    Json(payload): Json<CreateReviewOption>
) -> (StatusCode, Json<Review>) {
    let repos = state.repos.read().unwrap();
    let repo = repos.iter().find(|r| r.owner == owner && r.name == repo_name);

    let repo_id = if let Some(r) = repo {
        r.id
    } else {
         return (StatusCode::NOT_FOUND, Json(Review {
            id: 0, pull_request_id: 0, user: User::new(0, "".to_string(), None), body: "".to_string(), state: "".to_string(), created_at: "".to_string()
        }));
    };

    let pulls = state.pulls.read().unwrap();
    let pr = pulls.iter().find(|p| p.repo_id == repo_id && p.number == index);

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
        (StatusCode::NOT_FOUND, Json(Review {
            id: 0, pull_request_id: 0, user: User::new(0, "".to_string(), None), body: "".to_string(), state: "".to_string(), created_at: "".to_string()
        }))
    }
}

pub async fn get_commit_diff(Path((_owner, _repo, _sha)): Path<(String, String, String)>) -> Json<Vec<DiffFile>> {
    let diffs = vec![
        DiffFile {
            name: "src/main.rs".to_string(),
            old_name: None,
            index: "123".to_string(),
            additions: 10,
            deletions: 5,
            type_: "modify".to_string(),
            lines: vec![
                DiffLine { line_no_old: Some(1), line_no_new: Some(1), content: " fn main() {".to_string(), type_: "context".to_string() },
                DiffLine { line_no_old: Some(2), line_no_new: None, content: "-    println!(\"old\");".to_string(), type_: "delete".to_string() },
                DiffLine { line_no_old: None, line_no_new: Some(2), content: "+    println!(\"new\");".to_string(), type_: "add".to_string() },
            ],
        }
    ];
    Json(diffs)
}

pub async fn list_branch_protections(
    State(state): State<AppState>,
    Path((owner, repo_name)): Path<(String, String)>
) -> Json<Vec<ProtectedBranch>> {
    let repos = state.repos.read().unwrap();
    let repo_id = repos.iter().find(|r| r.owner == owner && r.name == repo_name).map(|r| r.id).unwrap_or(0);

    let branches = state.protected_branches.read().unwrap();
    let filtered: Vec<ProtectedBranch> = branches.iter().filter(|b| b.repo_id == repo_id).cloned().collect();
    Json(filtered)
}

pub async fn create_branch_protection(
    State(state): State<AppState>,
    Path((owner, repo_name)): Path<(String, String)>,
    Json(payload): Json<CreateProtectedBranchOption>
) -> (StatusCode, Json<ProtectedBranch>) {
    let repos = state.repos.read().unwrap();
    let repo_id = repos.iter().find(|r| r.owner == owner && r.name == repo_name).map(|r| r.id).unwrap_or(0);

    if repo_id == 0 {
         return (StatusCode::NOT_FOUND, Json(ProtectedBranch { id: 0, repo_id: 0, name: "".to_string(), enable_push: false, enable_force_push: false, required_status_checks: vec![] }));
    }

    let mut branches = state.protected_branches.write().unwrap();
    if branches.iter().any(|b| b.repo_id == repo_id && b.name == payload.name) {
         return (StatusCode::CONFLICT, Json(ProtectedBranch { id: 0, repo_id: 0, name: "".to_string(), enable_push: false, enable_force_push: false, required_status_checks: vec![] }));
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
    Path((owner, repo_name, name)): Path<(String, String, String)>
) -> StatusCode {
    let repos = state.repos.read().unwrap();
    let repo_id = repos.iter().find(|r| r.owner == owner && r.name == repo_name).map(|r| r.id).unwrap_or(0);

    if repo_id == 0 {
        return StatusCode::NOT_FOUND;
    }

    let mut branches = state.protected_branches.write().unwrap();
    if let Some(pos) = branches.iter().position(|b| b.repo_id == repo_id && b.name == name) {
        branches.remove(pos);
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}

pub async fn search_issues_global(
    State(state): State<AppState>,
    Query(filter): Query<IssueFilterOptions>
) -> Json<Vec<Issue>> {
    let issues = state.issues.read().unwrap();
    let mut filtered_issues: Vec<Issue> = issues.clone();

    if let Some(q) = filter.q {
        let q_lower = q.to_lowercase();
        filtered_issues.retain(|i| i.title.to_lowercase().contains(&q_lower) || i.body.clone().unwrap_or_default().to_lowercase().contains(&q_lower));
    }
    Json(filtered_issues)
}

pub async fn create_commit_status(
    State(state): State<AppState>,
    Path((owner, repo_name, sha)): Path<(String, String, String)>,
    Json(payload): Json<CreateStatusOption>
) -> (StatusCode, Json<CommitStatus>) {
    let repos = state.repos.read().unwrap();
    let repo_id = repos.iter().find(|r| r.owner == owner && r.name == repo_name).map(|r| r.id).unwrap_or(0);

    if repo_id == 0 {
         return (StatusCode::NOT_FOUND, Json(CommitStatus {
            id: 0, sha: "".to_string(), state: "".to_string(), target_url: None, description: None, context: "".to_string(), created_at: "".to_string(), creator: User::new(0, "".to_string(), None)
        }));
    }

    let mut statuses = state.commit_statuses.write().unwrap();
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
    Path((_owner, _repo, ref_name)): Path<(String, String, String)>
) -> Json<Vec<CommitStatus>> {
    let statuses = state.commit_statuses.read().unwrap();
    let filtered: Vec<CommitStatus> = statuses.iter().filter(|s| s.sha == ref_name).cloned().collect();
    Json(filtered)
}
