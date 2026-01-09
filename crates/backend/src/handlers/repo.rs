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
    MigrateRepoOption, TransferRepoOption, LfsLock, User, FileEntry, MergePullRequestOption, Topic, Project,
    Collaborator, Branch, CreateBranchOption, Tag, LfsObject, MilestoneStats, DiffFile, CodeSearchResult, Commit, ReviewRequest,
    DiffLine
};
use crate::router::AppState;

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
            user: User::new(0, "".to_string(), None), assignees: vec![], labels: vec![], milestone: None
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
        labels: vec![],
        milestone: None,
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

pub async fn get_milestone(State(state): State<AppState>, Path((_owner, _repo, id)): Path<(String, String, u64)>) -> Json<Option<Milestone>> {
    let milestones = state.milestones.read().unwrap();
    let m = milestones.iter().find(|m| m.id == id).cloned();
    Json(m)
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

pub async fn list_secrets(Path((_owner, _repo)): Path<(String, String)>) -> Json<Vec<Secret>> {
    let secrets = vec![
        Secret { name: "MY_TOKEN".to_string(), created_at: "2023-01-01".to_string() }
    ];
    Json(secrets)
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

pub async fn list_deploy_keys(Path((_owner, _repo)): Path<(String, String)>) -> Json<Vec<DeployKey>> {
    let keys = vec![
        DeployKey {
            id: 1,
            title: "CI Key".to_string(),
            key: "ssh-rsa...".to_string(),
            fingerprint: "SHA...".to_string(),
        }
    ];
    Json(keys)
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

pub async fn list_topics(Path((_owner, _repo)): Path<(String, String)>) -> Json<Vec<Topic>> {
    Json(vec![Topic { id: 1, name: "rust".to_string(), created: "2023-01-01".to_string() }])
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

pub async fn get_contents(Path((_owner, _repo, path)): Path<(String, String, String)>) -> Json<Vec<FileEntry>> {
    let mut files = vec![];
    if path == "/" || path.is_empty() {
        files.push(FileEntry { name: "src".to_string(), path: "src".to_string(), kind: "dir".to_string(), size: 0 });
        files.push(FileEntry { name: "README.md".to_string(), path: "README.md".to_string(), kind: "file".to_string(), size: 1024 });
        files.push(FileEntry { name: "Cargo.toml".to_string(), path: "Cargo.toml".to_string(), kind: "file".to_string(), size: 256 });
    } else if path == "src" {
        files.push(FileEntry { name: "main.rs".to_string(), path: "src/main.rs".to_string(), kind: "file".to_string(), size: 512 });
        files.push(FileEntry { name: "lib.rs".to_string(), path: "src/lib.rs".to_string(), kind: "file".to_string(), size: 1024 });
    }
    Json(files)
}

pub async fn get_root_contents(Path((owner, repo)): Path<(String, String)>) -> Json<Vec<FileEntry>> {
    get_contents(Path((owner, repo, "".to_string()))).await
}

pub async fn merge_pull(
    Path((_owner, _repo, _index)): Path<(String, String, u64)>,
    Json(_payload): Json<MergePullRequestOption>
) -> StatusCode {
    StatusCode::OK
}

pub async fn search_repos() -> Json<Vec<Repository>> {
    let repos = vec![
        Repository::new(1, "searched-repo".to_string(), "user".to_string())
    ];
    Json(repos)
}

pub async fn list_projects(Path((_owner, _repo)): Path<(String, String)>) -> Json<Vec<Project>> {
    let projects = vec![
        Project {
            id: 1,
            title: "Kanban Board".to_string(),
            description: None,
            is_closed: false,
        }
    ];
    Json(projects)
}

pub async fn list_collaborators(Path((_owner, _repo)): Path<(String, String)>) -> Json<Vec<Collaborator>> {
    let user = User::new(2, "collab_user".to_string(), None);
    vec![
        Collaborator { user, permissions: "write".to_string() }
    ].into()
}

pub async fn get_collaborator(Path((_owner, _repo, _collaborator)): Path<(String, String, String)>) -> Json<Option<Collaborator>> {
    Json(None)
}

pub async fn add_collaborator(Path((_owner, _repo, _collaborator)): Path<(String, String, String)>) -> StatusCode {
    StatusCode::NO_CONTENT
}

pub async fn list_branches(Path((_owner, _repo)): Path<(String, String)>) -> Json<Vec<Branch>> {
    let user = User::new(1, "admin".to_string(), None);
    let commit = Commit { sha: "abc".to_string(), message: "init".to_string(), author: user, date: "now".to_string() };
    let branches = vec![
        Branch { name: "main".to_string(), commit, protected: true }
    ];
    Json(branches)
}

pub async fn create_branch(
    Path((_owner, _repo)): Path<(String, String)>,
    Json(payload): Json<CreateBranchOption>
) -> (StatusCode, Json<Branch>) {
    let user = User::new(1, "admin".to_string(), None);
    let commit = Commit { sha: "def".to_string(), message: "new branch".to_string(), author: user, date: "now".to_string() };
    let branch = Branch { name: payload.name, commit, protected: false };
    (StatusCode::CREATED, Json(branch))
}

pub async fn list_tags(Path((_owner, _repo)): Path<(String, String)>) -> Json<Vec<Tag>> {
    let user = User::new(1, "admin".to_string(), None);
    let commit = Commit { sha: "abc".to_string(), message: "init".to_string(), author: user, date: "now".to_string() };
    let tags = vec![
        Tag { name: "v1.0".to_string(), id: "1".to_string(), commit }
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

pub async fn get_milestone_stats(Path((_owner, _repo, _id)): Path<(String, String, u64)>) -> Json<MilestoneStats> {
    Json(MilestoneStats { open_issues: 10, closed_issues: 5 })
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

pub async fn list_commits(Path((_owner, _repo)): Path<(String, String)>) -> Json<Vec<Commit>> {
    let user = User::new(1, "admin".to_string(), None);
    let commits = vec![
        Commit {
            sha: "abc123456789".to_string(),
            message: "Initial commit".to_string(),
            author: user.clone(),
            date: "2023-01-01T12:00:00Z".to_string(),
        },
        Commit {
            sha: "def456789012".to_string(),
            message: "Refactor backend handlers".to_string(),
            author: user.clone(),
            date: "2023-01-02T14:30:00Z".to_string(),
        },
        Commit {
            sha: "ghi789012345".to_string(),
            message: "Add frontend components".to_string(),
            author: user,
            date: "2023-01-03T09:15:00Z".to_string(),
        }
    ];
    Json(commits)
}

pub async fn search_repo_code(Path((_owner, _repo)): Path<(String, String)>) -> Json<Vec<CodeSearchResult>> {
    vec![
        CodeSearchResult {
            name: "main.rs".to_string(),
            path: "src/main.rs".to_string(),
            sha: "abc".to_string(),
            url: "http://...".to_string(),
            content: Some("fn main() {}".to_string()),
        }
    ].into()
}

pub async fn get_raw_file(Path((_owner, _repo, _path)): Path<(String, String, String)>) -> String {
    "fn main() { println!(\"Hello World\"); }".to_string()
}

pub async fn add_issue_assignee(Path((_owner, _repo, _index)): Path<(String, String, u64)>) -> StatusCode {
    StatusCode::CREATED
}

pub async fn request_review(Path((_owner, _repo, _index)): Path<(String, String, u64)>) -> (StatusCode, Json<ReviewRequest>) {
    let reviewer = User::new(2, "reviewer".to_string(), None);
    (StatusCode::CREATED, Json(ReviewRequest { reviewer, status: "requested".to_string() }))
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
