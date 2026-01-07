use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
};
use shared::{
    MergePullRequestOption, RepoTopicOptions, Repository, CodeSearchResult,
    Project, Secret, CreateSecretOption, DeployKey, CreateKeyOption, Collaborator, Branch,
    CreateBranchOption, Commit, Tag, LfsObject, MilestoneStats, DiffFile,
    WikiPage, CreateWikiPageOption, RepoSettingsOption, MigrateRepoOption, TransferRepoOption,
    FileEntry, CreateReactionOption, Reaction, User
};
use crate::router::AppState;

pub async fn merge_pull(
    Path((_owner, _repo, _index)): Path<(String, String, u64)>,
    Json(_payload): Json<MergePullRequestOption>
) -> StatusCode {
    StatusCode::OK
}

pub async fn list_topics(Path((_owner, _repo)): Path<(String, String)>) -> Json<Vec<shared::Topic>> {
    Json(vec![shared::Topic { id: 1, name: "rust".to_string(), created: "2023-01-01".to_string() }])
}

pub async fn update_topics_misc(
    Path((_owner, _repo)): Path<(String, String)>,
    Json(_payload): Json<RepoTopicOptions>
) -> StatusCode {
    StatusCode::NO_CONTENT
}

pub async fn search_repos_misc() -> Json<Vec<Repository>> {
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

pub async fn list_secrets(Path((_owner, _repo)): Path<(String, String)>) -> Json<Vec<Secret>> {
    let secrets = vec![
        Secret { name: "MY_TOKEN".to_string(), created_at: "2023-01-01".to_string() }
    ];
    Json(secrets)
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

pub async fn create_deploy_key_misc(
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

pub async fn get_collaborator(Path((_owner, _repo, _collaborator)): Path<(String, String, String)>) -> Json<Option<Collaborator>> {
    // Stub
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
    // Stub LFS upload
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
            author: user,
            date: "2023-01-01T12:00:00Z".to_string(),
        }
    ];
    Json(commits)
}

pub async fn create_secret_misc(
    Path((_owner, _repo)): Path<(String, String)>,
    Json(payload): Json<CreateSecretOption>
) -> (StatusCode, Json<Secret>) {
    let secret = Secret {
        name: payload.name,
        created_at: "2023-01-02".to_string(),
    };
    (StatusCode::CREATED, Json(secret))
}

pub async fn create_wiki_page_misc(
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

pub async fn get_repo_settings_misc(Path((_owner, _repo)): Path<(String, String)>) -> Json<RepoSettingsOption> {
    Json(RepoSettingsOption {
        description: Some("Description".to_string()),
        private: Some(false),
        website: None,
    })
}

pub async fn update_repo_settings_misc(
    Path((_owner, _repo)): Path<(String, String)>,
    Json(_payload): Json<RepoSettingsOption>
) -> StatusCode {
    StatusCode::OK
}

pub async fn mirror_sync_misc(Path((_owner, _repo)): Path<(String, String)>) -> StatusCode {
    StatusCode::OK
}

pub async fn migrate_repo_misc(Json(payload): Json<MigrateRepoOption>) -> (StatusCode, Json<Repository>) {
    let repo = Repository::new(4, payload.repo_name, "admin".to_string());
    (StatusCode::CREATED, Json(repo))
}

pub async fn transfer_repo_misc(Path((_owner, _repo)): Path<(String, String)>, Json(_payload): Json<TransferRepoOption>) -> StatusCode {
    StatusCode::ACCEPTED
}

pub async fn add_issue_label_misc(Path((_owner, _repo, _index)): Path<(String, String, u64)>) -> StatusCode {
    StatusCode::CREATED
}

pub async fn remove_issue_label_misc(Path((_owner, _repo, _index, _id)): Path<(String, String, u64, u64)>) -> StatusCode {
    StatusCode::NO_CONTENT
}

pub async fn update_wiki_page_misc(
    Path((_owner, _repo, _page_name)): Path<(String, String, String)>,
    Json(_payload): Json<CreateWikiPageOption>
) -> StatusCode {
    StatusCode::OK
}

pub async fn get_wiki_page_misc(Path((_owner, _repo, page_name)): Path<(String, String, String)>) -> Json<Option<WikiPage>> {
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

pub async fn star_repo_misc(Path((_owner, _repo)): Path<(String, String)>) -> StatusCode {
    StatusCode::NO_CONTENT
}

pub async fn watch_repo_misc(Path((_owner, _repo)): Path<(String, String)>) -> StatusCode {
    StatusCode::NO_CONTENT
}

pub async fn fork_repo_misc(Path((owner, repo)): Path<(String, String)>) -> Json<Repository> {
    Json(Repository::new(2, repo, owner))
}

pub async fn search_repo_code_misc(Path((_owner, _repo)): Path<(String, String)>) -> Json<Vec<CodeSearchResult>> {
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

pub async fn add_reaction_misc(
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

pub async fn get_raw_file_misc(Path((_owner, _repo, _path)): Path<(String, String, String)>) -> String {
    "fn main() { println!(\"Hello World\"); }".to_string()
}
