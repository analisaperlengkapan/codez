use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
};
use shared::{
    CreateRepoOption, Repository, CreateIssueOption, Issue, CreatePullRequestOption, PullRequest,
    CreateReleaseOption, Release, CreateCommentOption, Comment, CreateLabelOption, Label,
    CreateMilestoneOption, Milestone,
    CreateHookOption, Webhook,
    LfsLock, User
};
use crate::AppState;

pub async fn list_repos(State(state): State<AppState>) -> Json<Vec<Repository>> {
    let repos = state.repos.read().unwrap();
    Json(repos.clone())
}

pub async fn get_repo(State(state): State<AppState>, Path((owner, repo)): Path<(String, String)>) -> Json<Option<Repository>> {
    let repos = state.repos.read().unwrap();
    let r = repos.iter().find(|r| r.owner == owner && r.name == repo).cloned();
    Json(r)
}

pub async fn create_repo(State(state): State<AppState>, Json(payload): Json<CreateRepoOption>) -> (StatusCode, Json<Repository>) {
    let mut repos = state.repos.write().unwrap();
    let id = (repos.len() as u64) + 1;
    let repo = Repository::new(id, payload.name, "admin".to_string());
    repos.push(repo.clone());
    (StatusCode::CREATED, Json(repo))
}

pub async fn list_issues(State(state): State<AppState>, Path((_owner, _repo)): Path<(String, String)>) -> Json<Vec<Issue>> {
    let issues = state.issues.read().unwrap();
    Json(issues.clone())
}

pub async fn create_issue(
    State(state): State<AppState>,
    Path((_owner, _repo)): Path<(String, String)>,
    Json(payload): Json<CreateIssueOption>
) -> (StatusCode, Json<Issue>) {
    let mut issues = state.issues.write().unwrap();
    let id = (issues.len() as u64) + 1;
    let issue = Issue {
        id,
        number: id,
        title: payload.title,
        body: payload.body,
        state: "open".to_string(),
        user: User::new(1, "admin".to_string(), None),
        assignees: vec![],
    };
    issues.push(issue.clone());
    (StatusCode::CREATED, Json(issue))
}

pub async fn get_issue(State(state): State<AppState>, Path((_owner, _repo, index)): Path<(String, String, u64)>) -> Json<Option<Issue>> {
    let issues = state.issues.read().unwrap();
    let issue = issues.iter().find(|i| i.id == index).cloned();
    Json(issue)
}

pub async fn list_pulls(State(state): State<AppState>, Path((_owner, _repo)): Path<(String, String)>) -> Json<Vec<PullRequest>> {
    let pulls = state.pulls.read().unwrap();
    Json(pulls.clone())
}

pub async fn create_pull(
    State(state): State<AppState>,
    Path((_owner, _repo)): Path<(String, String)>,
    Json(payload): Json<CreatePullRequestOption>
) -> (StatusCode, Json<PullRequest>) {
    let mut pulls = state.pulls.write().unwrap();
    let id = (pulls.len() as u64) + 1;
    let pr = PullRequest {
        id,
        number: id,
        title: payload.title,
        body: payload.body,
        state: "open".to_string(),
        user: User::new(1, "admin".to_string(), None),
        merged: false,
    };
    pulls.push(pr.clone());
    (StatusCode::CREATED, Json(pr))
}

pub async fn list_releases(State(state): State<AppState>, Path((_owner, _repo)): Path<(String, String)>) -> Json<Vec<Release>> {
    let releases = state.releases.read().unwrap();
    Json(releases.clone())
}

pub async fn create_release(
    State(state): State<AppState>,
    Path((_owner, _repo)): Path<(String, String)>,
    Json(payload): Json<CreateReleaseOption>
) -> (StatusCode, Json<Release>) {
    let mut releases = state.releases.write().unwrap();
    let id = (releases.len() as u64) + 1;
    let release = Release {
        id,
        tag_name: payload.tag_name,
        name: payload.name,
        body: payload.body,
        draft: payload.draft,
        prerelease: payload.prerelease,
        created_at: "2023-01-02".to_string(),
        author: User::new(1, "admin".to_string(), None),
    };
    releases.push(release.clone());
    (StatusCode::CREATED, Json(release))
}

pub async fn list_comments(State(state): State<AppState>, Path((_owner, _repo, _index)): Path<(String, String, u64)>) -> Json<Vec<Comment>> {
    let comments = state.comments.read().unwrap();
    Json(comments.clone())
}

pub async fn create_comment(
    State(state): State<AppState>,
    Path((_owner, _repo, _index)): Path<(String, String, u64)>,
    Json(payload): Json<CreateCommentOption>
) -> (StatusCode, Json<Comment>) {
    let mut comments = state.comments.write().unwrap();
    let id = (comments.len() as u64) + 1;
    let comment = Comment {
        id,
        body: payload.body,
        user: User::new(1, "admin".to_string(), None),
        created_at: "2023-01-02".to_string(),
    };
    comments.push(comment.clone());
    (StatusCode::CREATED, Json(comment))
}

pub async fn list_labels(State(state): State<AppState>, Path((_owner, _repo)): Path<(String, String)>) -> Json<Vec<Label>> {
    let labels = state.labels.read().unwrap();
    Json(labels.clone())
}

pub async fn create_label(
    State(state): State<AppState>,
    Path((_owner, _repo)): Path<(String, String)>,
    Json(payload): Json<CreateLabelOption>
) -> (StatusCode, Json<Label>) {
    let mut labels = state.labels.write().unwrap();
    let id = (labels.len() as u64) + 1;
    let label = Label {
        id,
        name: payload.name,
        color: payload.color,
        description: payload.description,
    };
    labels.push(label.clone());
    (StatusCode::CREATED, Json(label))
}

pub async fn list_milestones(State(state): State<AppState>, Path((_owner, _repo)): Path<(String, String)>) -> Json<Vec<Milestone>> {
    let milestones = state.milestones.read().unwrap();
    Json(milestones.clone())
}

pub async fn create_milestone(
    State(state): State<AppState>,
    Path((_owner, _repo)): Path<(String, String)>,
    Json(payload): Json<CreateMilestoneOption>
) -> (StatusCode, Json<Milestone>) {
    let mut milestones = state.milestones.write().unwrap();
    let id = (milestones.len() as u64) + 1;
    let milestone = Milestone {
        id,
        title: payload.title,
        description: payload.description,
        due_on: payload.due_on,
        state: "open".to_string(),
    };
    milestones.push(milestone.clone());
    (StatusCode::CREATED, Json(milestone))
}

pub async fn list_hooks(State(state): State<AppState>, Path((_owner, _repo)): Path<(String, String)>) -> Json<Vec<Webhook>> {
    let hooks = state.hooks.read().unwrap();
    Json(hooks.clone())
}

pub async fn create_hook(
    State(state): State<AppState>,
    Path((_owner, _repo)): Path<(String, String)>,
    Json(payload): Json<CreateHookOption>
) -> (StatusCode, Json<Webhook>) {
    let mut hooks = state.hooks.write().unwrap();
    let id = (hooks.len() as u64) + 1;
    let hook = Webhook {
        id,
        url: payload.url,
        events: payload.events,
        active: payload.active,
    };
    hooks.push(hook.clone());
    (StatusCode::CREATED, Json(hook))
}

pub async fn list_lfs_locks(Path((_owner, _repo)): Path<(String, String)>) -> Json<Vec<LfsLock>> {
    let user = User::new(1, "admin".to_string(), None);
    vec![
        LfsLock { id: "1".to_string(), path: "file.bin".to_string(), owner: user, locked_at: "now".to_string() }
    ].into()
}

pub async fn create_lfs_lock(Path((_owner, _repo)): Path<(String, String)>) -> StatusCode {
    StatusCode::CREATED
}
