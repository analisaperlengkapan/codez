use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
};
use shared::{Discussion, CreateDiscussionOption, User, DiscussionComment, CreateDiscussionCommentOption, UpdateDiscussionOption};
use crate::router::AppState;
use chrono::Utc;

pub async fn list_discussions(
    State(state): State<AppState>,
    Path((owner, repo_name)): Path<(String, String)>
) -> Json<Vec<Discussion>> {
    let repos = state.repos.read().unwrap();
    let repo_id = repos.iter().find(|r| r.owner == owner && r.name == repo_name).map(|r| r.id).unwrap_or(0);

    let discussions = state.discussions.read().unwrap();
    let filtered: Vec<Discussion> = discussions.iter().filter(|d| d.repo_id == repo_id).cloned().collect();
    Json(filtered)
}

pub async fn create_discussion(
    State(state): State<AppState>,
    Path((owner, repo_name)): Path<(String, String)>,
    Json(payload): Json<CreateDiscussionOption>
) -> (StatusCode, Json<Discussion>) {
    let repos = state.repos.read().unwrap();
    let repo = repos.iter().find(|r| r.owner == owner && r.name == repo_name);

    let repo_id = if let Some(r) = repo {
        r.id
    } else {
        return (StatusCode::NOT_FOUND, Json(Discussion {
            id: 0, repo_id: 0, number: 0, title: "".to_string(), body: "".to_string(),
            user: User::new(0, "".to_string(), None), created_at: "".to_string(), updated_at: "".to_string(),
            is_locked: false, category: "".to_string()
        }));
    };

    let mut discussions = state.discussions.write().unwrap();
    let id = (discussions.len() as u64) + 1;
    let number = discussions.iter().filter(|d| d.repo_id == repo_id).count() as u64 + 1;

    // Use existing admin user if available, otherwise mock.
    // In a real application, this would come from the authenticated user context.
    let user_name = "admin";
    let users = state.users.read().unwrap();
    let user = users.iter().find(|u| u.username == user_name).cloned()
        .unwrap_or_else(|| User::new(1, "admin".to_string(), Some("admin@codeza.com".to_string())));

    let discussion = Discussion {
        id,
        repo_id,
        number,
        title: payload.title,
        body: payload.body,
        user,
        created_at: Utc::now().to_rfc3339(),
        updated_at: Utc::now().to_rfc3339(),
        is_locked: false,
        category: payload.category,
    };
    discussions.push(discussion.clone());
    (StatusCode::CREATED, Json(discussion))
}

pub async fn get_discussion(
    State(state): State<AppState>,
    Path((owner, repo_name, id)): Path<(String, String, u64)>
) -> Json<Option<Discussion>> {
    let repos = state.repos.read().unwrap();
    let repo_id = repos.iter().find(|r| r.owner == owner && r.name == repo_name).map(|r| r.id).unwrap_or(0);

    let discussions = state.discussions.read().unwrap();
    if let Some(discussion) = discussions.iter().find(|d| d.id == id) {
        if discussion.repo_id == repo_id {
            return Json(Some(discussion.clone()));
        }
    }
    Json(None)
}

pub async fn update_discussion(
    State(state): State<AppState>,
    Path((owner, repo_name, id)): Path<(String, String, u64)>,
    Json(payload): Json<UpdateDiscussionOption>
) -> (StatusCode, Json<Option<Discussion>>) {
    let repos = state.repos.read().unwrap();
    let repo_id = repos.iter().find(|r| r.owner == owner && r.name == repo_name).map(|r| r.id).unwrap_or(0);

    let mut discussions = state.discussions.write().unwrap();
    if let Some(discussion) = discussions.iter_mut().find(|d| d.id == id) {
        if discussion.repo_id != repo_id {
            return (StatusCode::NOT_FOUND, Json(None));
        }
        if let Some(title) = payload.title { discussion.title = title; }
        if let Some(body) = payload.body { discussion.body = body; }
        if let Some(category) = payload.category { discussion.category = category; }
        if let Some(is_locked) = payload.is_locked { discussion.is_locked = is_locked; }
        discussion.updated_at = Utc::now().to_rfc3339();
        return (StatusCode::OK, Json(Some(discussion.clone())));
    }
    (StatusCode::NOT_FOUND, Json(None))
}

pub async fn delete_discussion(
    State(state): State<AppState>,
    Path((owner, repo_name, id)): Path<(String, String, u64)>
) -> StatusCode {
    let repos = state.repos.read().unwrap();
    let repo_id = repos.iter().find(|r| r.owner == owner && r.name == repo_name).map(|r| r.id).unwrap_or(0);

    let mut discussions = state.discussions.write().unwrap();
    if let Some(pos) = discussions.iter().position(|d| d.id == id) {
        if discussions[pos].repo_id != repo_id {
            return StatusCode::NOT_FOUND;
        }
        discussions.remove(pos);

        // Cleanup comments
        let mut comments = state.discussion_comments.write().unwrap();
        comments.retain(|c| c.discussion_id != id);

        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}

pub async fn list_discussion_comments(
    State(state): State<AppState>,
    Path((_owner, _repo, id)): Path<(String, String, u64)>
) -> Json<Vec<DiscussionComment>> {
    let comments = state.discussion_comments.read().unwrap();
    let filtered: Vec<DiscussionComment> = comments.iter().filter(|c| c.discussion_id == id).cloned().collect();
    Json(filtered)
}

pub async fn create_discussion_comment(
    State(state): State<AppState>,
    Path((_owner, _repo, id)): Path<(String, String, u64)>,
    Json(payload): Json<CreateDiscussionCommentOption>
) -> (StatusCode, Json<DiscussionComment>) {
    {
        let discussions = state.discussions.read().unwrap();
        if !discussions.iter().any(|d| d.id == id) {
            return (StatusCode::NOT_FOUND, Json(DiscussionComment {
                id: 0, discussion_id: 0, body: "".to_string(),
                user: User::new(0, "".to_string(), None),
                created_at: "".to_string(), updated_at: "".to_string()
            }));
        }
    }

    let mut comments = state.discussion_comments.write().unwrap();
    let comment_id = (comments.len() as u64) + 1;

    let user_name = "admin";
    let users = state.users.read().unwrap();
    let user = users.iter().find(|u| u.username == user_name).cloned()
        .unwrap_or_else(|| User::new(1, "admin".to_string(), Some("admin@codeza.com".to_string())));

    let comment = DiscussionComment {
        id: comment_id,
        discussion_id: id,
        body: payload.body,
        user,
        created_at: Utc::now().to_rfc3339(),
        updated_at: Utc::now().to_rfc3339(),
    };
    comments.push(comment.clone());
    (StatusCode::CREATED, Json(comment))
}
