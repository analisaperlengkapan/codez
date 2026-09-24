use super::*;

pub async fn list_comments(
    State(state): State<AppState>,
    Path((owner, repo_name, index)): Path<(String, String, u64)>,
) -> Json<Vec<Comment>> {
    let repos = state.repos.read().unwrap_or_else(|e| e.into_inner());
    let repo_id = repos
        .iter()
        .find(|r| r.owner == owner && r.name == repo_name)
        .map(|r| r.id)
        .unwrap_or(0);

    let issues = state.issues.read().unwrap_or_else(|e| e.into_inner());
    let issue_id = issues
        .iter()
        .find(|i| i.repo_id == repo_id && i.number == index)
        .map(|i| i.id)
        .unwrap_or(0);

    let comments = state.comments.read().unwrap_or_else(|e| e.into_inner());
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
    let repos = state.repos.read().unwrap_or_else(|e| e.into_inner());
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

    let issues = state.issues.read().unwrap_or_else(|e| e.into_inner());
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
    let issue_ref = issue.unwrap();
    if issue_ref.is_locked {
        return (
            StatusCode::FORBIDDEN,
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
    let issue_id = issue_ref.id;

    let body = payload.body;
    let mut comments = state.comments.write().unwrap_or_else(|e| e.into_inner());
    let id = (comments.len() as u64) + 1;
    let comment = Comment {
        id,
        issue_id,
        body: body.clone(),
        user: User::new(1, "admin".to_string(), None),
        created_at: "2023-01-02".to_string(),
        reactions: vec![],
    };
    comments.push(comment.clone());

    // Notify mentioned users
    {
        let mentions = process_mentions(&body);
        if !mentions.is_empty() {
            let mut notifications = state
                .notifications
                .write()
                .unwrap_or_else(|e| e.into_inner());
            let users = state.users.read().unwrap_or_else(|e| e.into_inner());
            for username in mentions {
                if let Some(user) = users.iter().find(|u| u.username == username) {
                    // Don't notify self (mock admin id 1)
                    if user.id != 1 {
                        let nid = (notifications.len() as u64) + 1;
                        notifications.push(Notification {
                            id: nid,
                            subject: format!(
                                "You were mentioned in comment on issue/PR #{} in {}/{}",
                                index, owner, repo_name
                            ),
                            unread: true,
                            updated_at: "now".to_string(),
                        });
                    }
                }
            }
        }
    }

    (StatusCode::CREATED, Json(comment))
}

pub async fn update_comment(
    State(state): State<AppState>,
    Path((owner, repo_name, id)): Path<(String, String, u64)>,
    Json(payload): Json<UpdateCommentOption>,
) -> (StatusCode, Json<Option<Comment>>) {
    let repos = state.repos.read().unwrap_or_else(|e| e.into_inner());
    let repo_id = repos
        .iter()
        .find(|r| r.owner == owner && r.name == repo_name)
        .map(|r| r.id)
        .unwrap_or(0);

    // Validate ownership requires joining with issues or checking repo_id if comment had it (it doesn't directly, it has issue_id).
    // In this mock, we can look up the issue first.
    let issues = state.issues.read().unwrap_or_else(|e| e.into_inner());
    let mut comments = state.comments.write().unwrap_or_else(|e| e.into_inner());

    if let Some(comment) = comments.iter_mut().find(|c| c.id == id) {
        if let Some(issue) = issues.iter().find(|i| i.id == comment.issue_id) {
            if issue.repo_id == repo_id {
                if issue.is_locked {
                    return (StatusCode::FORBIDDEN, Json(None));
                }
                // Check for mentions
                let mentions = process_mentions(&payload.body);
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
                                        "You were mentioned in comment on issue/PR #{} in {}/{}",
                                        issue.number, owner, repo_name
                                    ),
                                    unread: true,
                                    updated_at: "now".to_string(),
                                });
                            }
                        }
                    }
                }

                comment.body = payload.body;
                return (StatusCode::OK, Json(Some(comment.clone())));
            }
        }
    }
    (StatusCode::NOT_FOUND, Json(None))
}

pub async fn delete_comment(
    State(state): State<AppState>,
    Path((owner, repo_name, id)): Path<(String, String, u64)>,
) -> StatusCode {
    let repos = state.repos.read().unwrap_or_else(|e| e.into_inner());
    let repo_id = repos
        .iter()
        .find(|r| r.owner == owner && r.name == repo_name)
        .map(|r| r.id)
        .unwrap_or(0);

    let issues = state.issues.read().unwrap_or_else(|e| e.into_inner());
    let mut comments = state.comments.write().unwrap_or_else(|e| e.into_inner());

    if let Some(pos) = comments.iter().position(|c| c.id == id) {
        if let Some(issue) = issues.iter().find(|i| i.id == comments[pos].issue_id) {
            if issue.repo_id == repo_id {
                if issue.is_locked {
                    return StatusCode::FORBIDDEN;
                }
                comments.remove(pos);
                return StatusCode::NO_CONTENT;
            }
        }
    }
    StatusCode::NOT_FOUND
}
