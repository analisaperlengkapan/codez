use super::*;

pub async fn list_issues(
    State(state): State<AppState>,
    Path((owner, repo_name)): Path<(String, String)>,
    Query(filter): Query<IssueFilterOptions>,
) -> Json<Vec<Issue>> {
    let repos = state.repos.read().unwrap_or_else(|e| e.into_inner());
    let repo_id = repos
        .iter()
        .find(|r| r.owner == owner && r.name == repo_name)
        .map(|r| r.id)
        .unwrap_or(0);

    if repo_id == 0 {
        return Json(vec![]);
    }

    let issues = state.issues.read().unwrap_or_else(|e| e.into_inner());
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
    if let Some(milestone_id) = filter.milestone_id {
        filtered_issues.retain(|i| i.milestone.as_ref().map(|m| m.id) == Some(milestone_id));
    }

    // Sort issues
    if let Some(sort) = &filter.sort {
        let direction = filter.direction.clone().unwrap_or("desc".to_string());
        match sort.as_str() {
            "created" => {
                // Mock sorting by ID since created_at is not in Issue struct, assume ID correlates with creation
                if direction == "asc" {
                    filtered_issues.sort_by_key(|a| a.id);
                } else {
                    filtered_issues.sort_by_key(|i| std::cmp::Reverse(i.id));
                }
            }
            "updated" => {
                // Mock sorting by ID as proxy for updated
                if direction == "asc" {
                    filtered_issues.sort_by_key(|a| a.id);
                } else {
                    filtered_issues.sort_by_key(|i| std::cmp::Reverse(i.id));
                }
            }
            "comments" => {
                // Mock sorting by ID as proxy, real impl would join comments count
                if direction == "asc" {
                    filtered_issues.sort_by_key(|a| a.id);
                } else {
                    filtered_issues.sort_by_key(|i| std::cmp::Reverse(i.id));
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
    let repos = state.repos.read().unwrap_or_else(|e| e.into_inner());
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
                is_locked: false,
            }),
        );
    };

    let milestone = if let Some(mid) = payload.milestone {
        let milestones = state.milestones.read().unwrap_or_else(|e| e.into_inner());
        milestones.iter().find(|m| m.id == mid).cloned()
    } else {
        None
    };

    let mut issues = state.issues.write().unwrap_or_else(|e| e.into_inner());
    let id = (issues.len() as u64) + 1;
    let issue = Issue {
        id,
        repo_id,
        number: id,
        title: payload.title.clone(),
        body: payload.body.clone(),
        state: "open".to_string(),
        user: User::new(1, "admin".to_string(), None),
        assignees: vec![],
        labels: vec![],
        milestone,
        is_locked: false,
    };
    issues.push(issue.clone());

    // Log activity
    let mut activities = state.activities.write().unwrap_or_else(|e| e.into_inner());
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
    let mut notifications = state
        .notifications
        .write()
        .unwrap_or_else(|e| e.into_inner());
    let notification_id = (notifications.len() as u64) + 1;
    notifications.push(Notification {
        id: notification_id,
        subject: format!("New issue in {}: {}", repo_name, payload.title),
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
                                "You were mentioned in issue #{} in {}/{}",
                                id, owner, repo_name
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
    if let Some(r) = repo {
        let event = IssueEvent {
            action: "opened".to_string(),
            issue: issue.clone(),
            repository: r.clone(),
            sender: User::new(1, "admin".to_string(), None),
        };
        dispatch_hooks(&state, repo_id, "issues", event);
    }

    (StatusCode::CREATED, Json(issue))
}

pub async fn get_issue(
    State(state): State<AppState>,
    Path((owner, repo_name, index)): Path<(String, String, u64)>,
) -> Json<Option<Issue>> {
    let repos = state.repos.read().unwrap_or_else(|e| e.into_inner());
    let repo_id = repos
        .iter()
        .find(|r| r.owner == owner && r.name == repo_name)
        .map(|r| r.id)
        .unwrap_or(0);

    let issues = state.issues.read().unwrap_or_else(|e| e.into_inner());
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
    Json(payload): Json<UpdateIssueOption>,
) -> (StatusCode, Json<Option<Issue>>) {
    let repos = state.repos.read().unwrap_or_else(|e| e.into_inner());
    let repo_id = repos
        .iter()
        .find(|r| r.owner == owner && r.name == repo_name)
        .map(|r| r.id)
        .unwrap_or(0);

    if repo_id == 0 {
        return (StatusCode::NOT_FOUND, Json(None));
    }

    let mut issues = state.issues.write().unwrap_or_else(|e| e.into_inner());
    let issue = issues
        .iter_mut()
        .find(|i| i.id == index && i.repo_id == repo_id);

    if let Some(i) = issue {
        if let Some(title) = payload.title {
            i.title = title;
        }
        if let Some(body) = payload.body {
            // Check for mentions in new body
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
                                    "You were mentioned in issue #{} in {}/{}",
                                    index, owner, repo_name
                                ),
                                unread: true,
                                updated_at: "now".to_string(),
                            });
                        }
                    }
                }
            }
            i.body = Some(body);
        }
        if let Some(state_val) = payload.state {
            if i.state != state_val {
                // Log activity for state change
                let mut activities = state.activities.write().unwrap_or_else(|e| e.into_inner());
                let activity_id = (activities.len() as u64) + 1;
                // TODO: Extract actual authenticated user from request context once auth middleware is implemented.
                // Currently using mock admin (ID 1) to maintain consistency with existing handlers.
                activities.push(Activity {
                    id: activity_id,
                    repo_id,
                    user_id: 1, // mock admin
                    user_name: "admin".to_string(),
                    op_type: if state_val == "closed" {
                        "close_issue".to_string()
                    } else {
                        "reopen_issue".to_string()
                    },
                    content: format!(
                        "{} issue #{} in {}/{}",
                        if state_val == "closed" {
                            "closed"
                        } else {
                            "reopened"
                        },
                        index,
                        owner,
                        repo_name
                    ),
                    created: "now".to_string(),
                });
            }
            i.state = state_val;
        }
        if let Some(milestone_id) = payload.milestone_id {
            if milestone_id == 0 {
                i.milestone = None;
            } else {
                // Validate milestone existence
                let milestones = state.milestones.read().unwrap_or_else(|e| e.into_inner());
                if let Some(m) = milestones
                    .iter()
                    .find(|m| m.id == milestone_id && m.repo_id == repo_id)
                {
                    i.milestone = Some(m.clone());
                }
            }
        }
        return (StatusCode::OK, Json(Some(i.clone())));
    }
    (StatusCode::NOT_FOUND, Json(None))
}

pub async fn lock_issue(
    State(state): State<AppState>,
    Path((owner, repo_name, index)): Path<(String, String, u64)>,
) -> StatusCode {
    let repos = state.repos.read().unwrap_or_else(|e| e.into_inner());
    let repo_id = repos
        .iter()
        .find(|r| r.owner == owner && r.name == repo_name)
        .map(|r| r.id)
        .unwrap_or(0);

    let mut issues = state.issues.write().unwrap_or_else(|e| e.into_inner());
    if let Some(issue) = issues
        .iter_mut()
        .find(|i| i.repo_id == repo_id && i.id == index)
    {
        issue.is_locked = true;
        StatusCode::OK
    } else {
        StatusCode::NOT_FOUND
    }
}

pub async fn unlock_issue(
    State(state): State<AppState>,
    Path((owner, repo_name, index)): Path<(String, String, u64)>,
) -> StatusCode {
    let repos = state.repos.read().unwrap_or_else(|e| e.into_inner());
    let repo_id = repos
        .iter()
        .find(|r| r.owner == owner && r.name == repo_name)
        .map(|r| r.id)
        .unwrap_or(0);

    let mut issues = state.issues.write().unwrap_or_else(|e| e.into_inner());
    if let Some(issue) = issues
        .iter_mut()
        .find(|i| i.repo_id == repo_id && i.id == index)
    {
        issue.is_locked = false;
        StatusCode::OK
    } else {
        StatusCode::NOT_FOUND
    }
}

pub async fn list_labels(
    State(state): State<AppState>,
    Path((owner, repo_name)): Path<(String, String)>,
) -> Json<Vec<Label>> {
    let repos = state.repos.read().unwrap_or_else(|e| e.into_inner());
    let repo_id = repos
        .iter()
        .find(|r| r.owner == owner && r.name == repo_name)
        .map(|r| r.id)
        .unwrap_or(0);

    let labels = state.labels.read().unwrap_or_else(|e| e.into_inner());
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
    let repos = state.repos.read().unwrap_or_else(|e| e.into_inner());
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

    let mut labels = state.labels.write().unwrap_or_else(|e| e.into_inner());
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
    Json(payload): Json<UpdateLabelOption>,
) -> (StatusCode, Json<Option<Label>>) {
    let repos = state.repos.read().unwrap_or_else(|e| e.into_inner());
    let repo_id = repos
        .iter()
        .find(|r| r.owner == owner && r.name == repo_name)
        .map(|r| r.id)
        .unwrap_or(0);

    if repo_id == 0 {
        return (StatusCode::NOT_FOUND, Json(None));
    }

    let mut labels = state.labels.write().unwrap_or_else(|e| e.into_inner());
    if let Some(label) = labels.iter_mut().find(|l| l.id == id) {
        if label.repo_id != repo_id {
            return (StatusCode::NOT_FOUND, Json(None));
        }
        if let Some(name) = payload.name {
            label.name = name;
        }
        if let Some(color) = payload.color {
            label.color = color;
        }
        if let Some(description) = payload.description {
            label.description = Some(description);
        }
        return (StatusCode::OK, Json(Some(label.clone())));
    }
    (StatusCode::NOT_FOUND, Json(None))
}

pub async fn delete_label(
    State(state): State<AppState>,
    Path((owner, repo_name, id)): Path<(String, String, u64)>,
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

    let mut labels = state.labels.write().unwrap_or_else(|e| e.into_inner());
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

pub async fn list_milestones(
    State(state): State<AppState>,
    Path((owner, repo_name)): Path<(String, String)>,
) -> Json<Vec<Milestone>> {
    let repos = state.repos.read().unwrap_or_else(|e| e.into_inner());
    let repo_id = repos
        .iter()
        .find(|r| r.owner == owner && r.name == repo_name)
        .map(|r| r.id)
        .unwrap_or(0);

    let milestones = state.milestones.read().unwrap_or_else(|e| e.into_inner());
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
    let repos = state.repos.read().unwrap_or_else(|e| e.into_inner());
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

    let mut milestones = state.milestones.write().unwrap_or_else(|e| e.into_inner());
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
    Path((owner, repo_name, id)): Path<(String, String, u64)>,
) -> Json<Option<Milestone>> {
    let repos = state.repos.read().unwrap_or_else(|e| e.into_inner());
    let repo_id = repos
        .iter()
        .find(|r| r.owner == owner && r.name == repo_name)
        .map(|r| r.id)
        .unwrap_or(0);

    let milestones = state.milestones.read().unwrap_or_else(|e| e.into_inner());
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
    Json(payload): Json<UpdateMilestoneOption>,
) -> (StatusCode, Json<Option<Milestone>>) {
    let repos = state.repos.read().unwrap_or_else(|e| e.into_inner());
    let repo_id = repos
        .iter()
        .find(|r| r.owner == owner && r.name == repo_name)
        .map(|r| r.id)
        .unwrap_or(0);

    if repo_id == 0 {
        return (StatusCode::NOT_FOUND, Json(None));
    }

    let mut milestones = state.milestones.write().unwrap_or_else(|e| e.into_inner());
    if let Some(m) = milestones.iter_mut().find(|m| m.id == id) {
        if m.repo_id != repo_id {
            return (StatusCode::NOT_FOUND, Json(None));
        }
        if let Some(title) = payload.title {
            m.title = title;
        }
        if let Some(desc) = payload.description {
            m.description = Some(desc);
        }
        if let Some(due) = payload.due_on {
            m.due_on = Some(due);
        }
        if let Some(state) = payload.state {
            m.state = state;
        }
        return (StatusCode::OK, Json(Some(m.clone())));
    }
    (StatusCode::NOT_FOUND, Json(None))
}

pub async fn delete_milestone(
    State(state): State<AppState>,
    Path((owner, repo_name, id)): Path<(String, String, u64)>,
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

    let mut milestones = state.milestones.write().unwrap_or_else(|e| e.into_inner());
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

pub async fn remove_issue_assignee(
    State(state): State<AppState>,
    Path((_owner, _repo, index, username)): Path<(String, String, u64, String)>,
) -> StatusCode {
    let mut issues = state.issues.write().unwrap_or_else(|e| e.into_inner());
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

pub async fn add_issue_label(
    State(state): State<AppState>,
    Path((_owner, _repo, index)): Path<(String, String, u64)>,
    Json(payload): Json<shared::CreateLabelOption>,
) -> StatusCode {
    let mut issues = state.issues.write().unwrap_or_else(|e| e.into_inner());
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
    let mut issues = state.issues.write().unwrap_or_else(|e| e.into_inner());
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

pub async fn get_milestone_stats(
    State(state): State<AppState>,
    Path((_owner, _repo, id)): Path<(String, String, u64)>,
) -> Json<MilestoneStats> {
    let issues = state.issues.read().unwrap_or_else(|e| e.into_inner());
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

pub async fn add_issue_assignee(
    State(state): State<AppState>,
    Path((_owner, _repo, index)): Path<(String, String, u64)>,
    Json(payload): Json<User>,
) -> StatusCode {
    let mut issues = state.issues.write().unwrap_or_else(|e| e.into_inner());
    if let Some(issue) = issues.iter_mut().find(|i| i.id == index) {
        if !issue
            .assignees
            .iter()
            .any(|u| u.username == payload.username)
        {
            issue.assignees.push(payload);

            // Notify assignee
            let mut notifications = state
                .notifications
                .write()
                .unwrap_or_else(|e| e.into_inner());
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

pub async fn search_issues_global(
    State(state): State<AppState>,
    Query(filter): Query<IssueFilterOptions>,
) -> Json<Vec<Issue>> {
    let issues = state.issues.read().unwrap_or_else(|e| e.into_inner());
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
