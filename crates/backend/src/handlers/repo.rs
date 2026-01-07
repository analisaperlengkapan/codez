use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use shared::{
    CreateRepoOption, Repository, CreateIssueOption, Issue, CreatePullRequestOption, PullRequest,
    CreateReleaseOption, Release, CreateCommentOption, Comment, CreateLabelOption, Label,
    CreateMilestoneOption, Milestone, RepoTopicOptions, RepoSettingsOption, CreateWikiPageOption, WikiPage,
    CreateHookOption, Webhook, CreateSecretOption, Secret, CreateKeyOption, DeployKey, CreateReactionOption, Reaction,
    MigrateRepoOption, TransferRepoOption, LfsLock, User
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

pub async fn create_repo(State(state): State<AppState>, Json(payload): Json<CreateRepoOption>) -> impl IntoResponse {
    let mut repos = state.repos.write().unwrap();

    // Check for duplicate repo name for the admin user (mimicking GitHub behavior)
    if repos.iter().any(|r| r.owner == "admin" && r.name == payload.name) {
        return (StatusCode::CONFLICT, Json(Repository::new(0, "".to_string(), "".to_string())));
    }

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
    Path((owner, repo_name)): Path<(String, String)>,
    Json(payload): Json<CreateIssueOption>
) -> impl IntoResponse {
    let repos = state.repos.read().unwrap();
    if !repos.iter().any(|r| r.owner == owner && r.name == repo_name) {
        return (StatusCode::NOT_FOUND, Json(Issue {
            id: 0, number: 0, title: "".to_string(), body: None, state: "".to_string(),
            user: User::new(0, "".to_string(), None), assignees: vec![]
        }));
    }

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

pub async fn create_secret(
    Path((_owner, _repo)): Path<(String, String)>,
    Json(payload): Json<CreateSecretOption>
) -> (StatusCode, Json<Secret>) {
    let secret = Secret {
        name: payload.name,
        created_at: "2023-01-02".to_string(),
    };
    (StatusCode::CREATED, Json(secret))
}

pub async fn create_deploy_key(
    Path((_owner, _repo)): Path<(String, String)>,
    Json(payload): Json<CreateKeyOption>
) -> (StatusCode, Json<DeployKey>) {
    let key = DeployKey {
        id: 2,
        title: payload.title,
        key: payload.key,
        fingerprint: "SHA...".to_string(),
    };
    (StatusCode::CREATED, Json(key))
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

pub async fn add_reaction(
    Path((_owner, _repo, _id)): Path<(String, String, u64)>,
    Json(payload): Json<CreateReactionOption>
) -> (StatusCode, Json<Reaction>) {
    let user = User::new(1, "admin".to_string(), None);
    let reaction = Reaction {
        id: 1,
        user,
        content: payload.content,
        created_at: "now".to_string(),
    };
    (StatusCode::CREATED, Json(reaction))
}

pub async fn update_topics(
    Path((_owner, _repo)): Path<(String, String)>,
    Json(_payload): Json<RepoTopicOptions>
) -> StatusCode {
    StatusCode::NO_CONTENT
}

pub async fn star_repo(Path((_owner, _repo)): Path<(String, String)>) -> StatusCode {
    StatusCode::NO_CONTENT
}

pub async fn watch_repo(Path((_owner, _repo)): Path<(String, String)>) -> StatusCode {
    StatusCode::NO_CONTENT
}

pub async fn fork_repo(Path((owner, repo)): Path<(String, String)>) -> Json<Repository> {
    Json(Repository::new(2, repo, owner))
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

pub async fn get_repo_settings(Path((_owner, _repo)): Path<(String, String)>) -> Json<RepoSettingsOption> {
    Json(RepoSettingsOption {
        description: Some("Description".to_string()),
        private: Some(false),
        website: None,
    })
}

pub async fn update_repo_settings(
    Path((_owner, _repo)): Path<(String, String)>,
    Json(_payload): Json<RepoSettingsOption>
) -> StatusCode {
    StatusCode::OK
}

pub async fn mirror_sync(Path((_owner, _repo)): Path<(String, String)>) -> StatusCode {
    StatusCode::OK
}

pub async fn migrate_repo(Json(payload): Json<MigrateRepoOption>) -> (StatusCode, Json<Repository>) {
    let repo = Repository::new(4, payload.repo_name, "admin".to_string());
    (StatusCode::CREATED, Json(repo))
}

pub async fn transfer_repo(Path((_owner, _repo)): Path<(String, String)>, Json(_payload): Json<TransferRepoOption>) -> StatusCode {
    StatusCode::ACCEPTED
}

pub async fn add_issue_label(Path((_owner, _repo, _index)): Path<(String, String, u64)>) -> StatusCode {
    StatusCode::CREATED
}

pub async fn remove_issue_label(Path((_owner, _repo, _index, _id)): Path<(String, String, u64, u64)>) -> StatusCode {
    StatusCode::NO_CONTENT
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
    } else {
        Json(None)
    }
}
