use axum::{
    extract::{Json, Path},
    http::StatusCode,
    routing::{get, post},
    Router,
};
use shared::{ActionWorkflow, Activity, AdminStats, Commit, Comment, CreateCommentOption, CreateHookOption, CreateIssueOption, CreateKeyOption, CreateLabelOption, CreateMilestoneOption, CreatePullRequestOption, CreateReleaseOption, CreateRepoOption, CreateSecretOption, CreateWikiPageOption, DeployKey, FileEntry, Issue, Label, LoginOption, MergePullRequestOption, Milestone, Notification, Organization, Package, Project, PublicKey, PullRequest, RegisterOption, Release, RepoActionOption, RepoSettingsOption, RepoTopicOptions, Repository, Secret, SystemNotice, Team, Topic, TwoFactor, User, UserSettingsOption, Webhook, WikiPage};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    let app = app();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn app() -> Router {
    Router::new()
        .route("/api/v1/repos", get(list_repos))
        .route("/api/v1/users/:username", get(get_user))
        .route("/api/v1/repos/:owner/:repo", get(get_repo))
        .route("/api/v1/user/repos", post(create_repo))
        .route("/api/v1/repos/:owner/:repo/issues", get(list_issues).post(create_issue))
        .route("/api/v1/repos/:owner/:repo/pulls", get(list_pulls).post(create_pull))
        .route("/api/v1/repos/:owner/:repo/contents/*path", get(get_contents))
        .route("/api/v1/repos/:owner/:repo/contents", get(get_root_contents))
        .route("/api/v1/repos/:owner/:repo/commits", get(list_commits))
        .route("/api/v1/repos/:owner/:repo/releases", get(list_releases).post(create_release))
        .route("/api/v1/users/login", post(login_user))
        .route("/api/v1/users/register", post(register_user))
        .route("/api/v1/orgs/:org", get(get_org))
        .route("/api/v1/orgs/:org/repos", get(list_org_repos))
        .route("/api/v1/repos/:owner/:repo/issues/:index", get(get_issue))
        .route("/api/v1/repos/:owner/:repo/issues/:index/comments", get(list_comments).post(create_comment))
        .route("/api/v1/repos/:owner/:repo/pulls/:index/merge", post(merge_pull))
        .route("/api/v1/repos/:owner/:repo/labels", get(list_labels).post(create_label))
        .route("/api/v1/repos/:owner/:repo/milestones", get(list_milestones).post(create_milestone))
        .route("/api/v1/repos/:owner/:repo/topics", get(list_topics).put(update_topics))
        .route("/api/v1/repos/:owner/:repo/star", post(star_repo))
        .route("/api/v1/repos/:owner/:repo/watch", post(watch_repo))
        .route("/api/v1/repos/:owner/:repo/fork", post(fork_repo))
        .route("/api/v1/repos/search", get(search_repos))
        .route("/api/v1/repos/:owner/:repo/wiki/pages/:page_name", get(get_wiki_page))
        .route("/api/v1/repos/:owner/:repo/wiki/pages", post(create_wiki_page))
        .route("/api/v1/repos/:owner/:repo/settings", get(get_repo_settings).patch(update_repo_settings))
        .route("/api/v1/user/settings", get(get_user_settings).patch(update_user_settings))
        .route("/api/v1/notifications", get(list_notifications))
        .route("/api/v1/user/keys", get(list_keys).post(create_key))
        .route("/api/v1/repos/:owner/:repo/hooks", get(list_hooks).post(create_hook))
        .route("/api/v1/orgs/:org/teams", get(list_teams))
        .route("/api/v1/repos/:owner/:repo/projects", get(list_projects))
        .route("/api/v1/admin/stats", get(get_admin_stats))
        .route("/api/v1/user/feeds", get(list_feeds))
        .route("/api/v1/repos/:owner/:repo/actions/workflows", get(list_workflows))
        .route("/api/v1/packages/:owner", get(list_packages))
        .route("/api/v1/repos/:owner/:repo/secrets", get(list_secrets).post(create_secret))
        .route("/api/v1/repos/:owner/:repo/keys", get(list_deploy_keys).post(create_deploy_key))
        .route("/api/v1/admin/notices", get(list_notices))
        .route("/api/v1/user/2fa", get(get_2fa).post(update_2fa))
        .layer(CorsLayer::permissive())
}

async fn list_repos() -> Json<Vec<Repository>> {
    let repos = vec![
        Repository::new(1, "codeza".to_string(), "admin".to_string()),
        Repository::new(2, "gitea-clone".to_string(), "user".to_string()),
    ];
    Json(repos)
}

async fn get_user(Path(username): Path<String>) -> Json<Option<User>> {
    // Mock user lookup
    if username == "admin" {
        Json(Some(User::new(1, "admin".to_string(), Some("admin@codeza.com".to_string()))))
    } else {
        Json(None)
    }
}

async fn get_repo(Path((owner, repo)): Path<(String, String)>) -> Json<Option<Repository>> {
    // Mock repo lookup
    if owner == "admin" && repo == "codeza" {
        Json(Some(Repository::new(1, "codeza".to_string(), "admin".to_string())))
    } else {
        Json(None)
    }
}

async fn create_repo(Json(payload): Json<CreateRepoOption>) -> (StatusCode, Json<Repository>) {
    // Mock creation
    let repo = Repository::new(3, payload.name, "admin".to_string());
    (StatusCode::CREATED, Json(repo))
}

async fn list_issues(Path((_owner, _repo)): Path<(String, String)>) -> Json<Vec<Issue>> {
    let user = User::new(1, "admin".to_string(), None);
    let issues = vec![
        Issue {
            id: 1,
            number: 1,
            title: "First Issue".to_string(),
            body: Some("This is a bug".to_string()),
            state: "open".to_string(),
            user,
        }
    ];
    Json(issues)
}

async fn create_issue(
    Path((_owner, _repo)): Path<(String, String)>,
    Json(payload): Json<CreateIssueOption>
) -> (StatusCode, Json<Issue>) {
    let user = User::new(1, "admin".to_string(), None);
    let issue = Issue {
        id: 2,
        number: 2,
        title: payload.title,
        body: payload.body,
        state: "open".to_string(),
        user,
    };
    (StatusCode::CREATED, Json(issue))
}

async fn list_pulls(Path((_owner, _repo)): Path<(String, String)>) -> Json<Vec<PullRequest>> {
    let user = User::new(1, "admin".to_string(), None);
    let pulls = vec![
        PullRequest {
            id: 1,
            number: 1,
            title: "First PR".to_string(),
            body: Some("Description".to_string()),
            state: "open".to_string(),
            user,
            merged: false,
        }
    ];
    Json(pulls)
}

async fn create_pull(
    Path((_owner, _repo)): Path<(String, String)>,
    Json(payload): Json<CreatePullRequestOption>
) -> (StatusCode, Json<PullRequest>) {
    let user = User::new(1, "admin".to_string(), None);
    let pr = PullRequest {
        id: 2,
        number: 2,
        title: payload.title,
        body: payload.body,
        state: "open".to_string(),
        user,
        merged: false,
    };
    (StatusCode::CREATED, Json(pr))
}

async fn get_contents(Path((_owner, _repo, path)): Path<(String, String, String)>) -> Json<Vec<FileEntry>> {
    // Mock contents based on path
    let mut files = vec![];
    if path == "/" || path.is_empty() {
        files.push(FileEntry {
            name: "src".to_string(),
            path: "src".to_string(),
            kind: "dir".to_string(),
            size: 0,
        });
        files.push(FileEntry {
            name: "README.md".to_string(),
            path: "README.md".to_string(),
            kind: "file".to_string(),
            size: 1024,
        });
    } else if path == "src" {
        files.push(FileEntry {
            name: "main.rs".to_string(),
            path: "src/main.rs".to_string(),
            kind: "file".to_string(),
            size: 512,
        });
    }
    Json(files)
}

async fn get_root_contents(Path((owner, repo)): Path<(String, String)>) -> Json<Vec<FileEntry>> {
    get_contents(Path((owner, repo, "".to_string()))).await
}

async fn list_commits(Path((_owner, _repo)): Path<(String, String)>) -> Json<Vec<Commit>> {
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

async fn list_releases(Path((_owner, _repo)): Path<(String, String)>) -> Json<Vec<Release>> {
    let user = User::new(1, "admin".to_string(), None);
    let releases = vec![
        Release {
            id: 1,
            tag_name: "v1.0.0".to_string(),
            name: "Initial Release".to_string(),
            body: Some("Description".to_string()),
            draft: false,
            prerelease: false,
            created_at: "2023-01-01".to_string(),
            author: user,
        }
    ];
    Json(releases)
}

async fn create_release(
    Path((_owner, _repo)): Path<(String, String)>,
    Json(payload): Json<CreateReleaseOption>
) -> (StatusCode, Json<Release>) {
    let user = User::new(1, "admin".to_string(), None);
    let release = Release {
        id: 2,
        tag_name: payload.tag_name,
        name: payload.name,
        body: payload.body,
        draft: payload.draft,
        prerelease: payload.prerelease,
        created_at: "2023-01-02".to_string(),
        author: user,
    };
    (StatusCode::CREATED, Json(release))
}

async fn login_user(Json(payload): Json<LoginOption>) -> (StatusCode, Json<Option<User>>) {
    if payload.username == "admin" && payload.password == "password" {
        (StatusCode::OK, Json(Some(User::new(1, "admin".to_string(), Some("admin@codeza.com".to_string())))))
    } else {
        (StatusCode::UNAUTHORIZED, Json(None))
    }
}

async fn register_user(Json(payload): Json<RegisterOption>) -> (StatusCode, Json<User>) {
    // Mock register
    let user = User::new(2, payload.username, Some(payload.email));
    (StatusCode::CREATED, Json(user))
}

async fn get_org(Path(org): Path<String>) -> Json<Option<Organization>> {
    if org == "codeza-org" {
        Json(Some(Organization {
            id: 1,
            username: "codeza-org".to_string(),
            description: Some("Codeza Organization".to_string()),
            avatar_url: None,
        }))
    } else {
        Json(None)
    }
}

async fn list_org_repos(Path(_org): Path<String>) -> Json<Vec<Repository>> {
    let repos = vec![
        Repository::new(1, "org-repo".to_string(), "codeza-org".to_string())
    ];
    Json(repos)
}

async fn get_issue(Path((_owner, _repo, index)): Path<(String, String, u64)>) -> Json<Option<Issue>> {
    let user = User::new(1, "admin".to_string(), None);
    Json(Some(Issue {
        id: index,
        number: index,
        title: "Mock Issue".to_string(),
        body: Some("Body".to_string()),
        state: "open".to_string(),
        user,
    }))
}

async fn list_comments(Path((_owner, _repo, _index)): Path<(String, String, u64)>) -> Json<Vec<Comment>> {
    let user = User::new(1, "admin".to_string(), None);
    let comments = vec![
        Comment {
            id: 1,
            body: "Great idea!".to_string(),
            user,
            created_at: "2023-01-01".to_string(),
        }
    ];
    Json(comments)
}

async fn create_comment(
    Path((_owner, _repo, _index)): Path<(String, String, u64)>,
    Json(payload): Json<CreateCommentOption>
) -> (StatusCode, Json<Comment>) {
    let user = User::new(1, "admin".to_string(), None);
    let comment = Comment {
        id: 2,
        body: payload.body,
        user,
        created_at: "2023-01-02".to_string(),
    };
    (StatusCode::CREATED, Json(comment))
}

async fn merge_pull(
    Path((_owner, _repo, _index)): Path<(String, String, u64)>,
    Json(_payload): Json<MergePullRequestOption>
) -> StatusCode {
    StatusCode::OK
}

async fn list_labels(Path((_owner, _repo)): Path<(String, String)>) -> Json<Vec<Label>> {
    let labels = vec![
        Label {
            id: 1,
            name: "bug".to_string(),
            color: "#ff0000".to_string(),
            description: None,
        }
    ];
    Json(labels)
}

async fn create_label(
    Path((_owner, _repo)): Path<(String, String)>,
    Json(payload): Json<CreateLabelOption>
) -> (StatusCode, Json<Label>) {
    let label = Label {
        id: 2,
        name: payload.name,
        color: payload.color,
        description: payload.description,
    };
    (StatusCode::CREATED, Json(label))
}

async fn list_milestones(Path((_owner, _repo)): Path<(String, String)>) -> Json<Vec<Milestone>> {
    let milestones = vec![
        Milestone {
            id: 1,
            title: "v1.0".to_string(),
            description: None,
            due_on: None,
            state: "open".to_string(),
        }
    ];
    Json(milestones)
}

async fn create_milestone(
    Path((_owner, _repo)): Path<(String, String)>,
    Json(payload): Json<CreateMilestoneOption>
) -> (StatusCode, Json<Milestone>) {
    let milestone = Milestone {
        id: 2,
        title: payload.title,
        description: payload.description,
        due_on: payload.due_on,
        state: "open".to_string(),
    };
    (StatusCode::CREATED, Json(milestone))
}

async fn list_topics(Path((_owner, _repo)): Path<(String, String)>) -> Json<Vec<Topic>> {
    let topics = vec![
        Topic { id: 1, name: "rust".to_string(), created: "2023-01-01".to_string() }
    ];
    Json(topics)
}

async fn update_topics(
    Path((_owner, _repo)): Path<(String, String)>,
    Json(_payload): Json<RepoTopicOptions>
) -> StatusCode {
    StatusCode::NO_CONTENT
}

async fn star_repo(Path((_owner, _repo)): Path<(String, String)>) -> StatusCode {
    StatusCode::NO_CONTENT
}

async fn watch_repo(Path((_owner, _repo)): Path<(String, String)>) -> StatusCode {
    StatusCode::NO_CONTENT
}

async fn fork_repo(Path((owner, repo)): Path<(String, String)>) -> Json<Repository> {
    Json(Repository::new(2, repo, owner))
}

async fn search_repos() -> Json<Vec<Repository>> {
    let repos = vec![
        Repository::new(1, "searched-repo".to_string(), "user".to_string())
    ];
    Json(repos)
}

async fn get_wiki_page(Path((_owner, _repo, page_name)): Path<(String, String, String)>) -> Json<Option<WikiPage>> {
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

async fn create_wiki_page(
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

async fn get_repo_settings(Path((_owner, _repo)): Path<(String, String)>) -> Json<RepoSettingsOption> {
    Json(RepoSettingsOption {
        description: Some("Description".to_string()),
        private: Some(false),
        website: None,
    })
}

async fn update_repo_settings(
    Path((_owner, _repo)): Path<(String, String)>,
    Json(_payload): Json<RepoSettingsOption>
) -> StatusCode {
    StatusCode::OK
}

async fn get_user_settings() -> Json<UserSettingsOption> {
    Json(UserSettingsOption {
        full_name: Some("Admin User".to_string()),
        website: None,
        description: None,
        location: None,
    })
}

async fn update_user_settings(Json(_payload): Json<UserSettingsOption>) -> StatusCode {
    StatusCode::OK
}

async fn list_notifications() -> Json<Vec<Notification>> {
    let notifications = vec![
        Notification {
            id: 1,
            subject: "Welcome to Codeza".to_string(),
            unread: true,
            updated_at: "2023-01-01".to_string(),
        }
    ];
    Json(notifications)
}

async fn list_keys() -> Json<Vec<PublicKey>> {
    let keys = vec![
        PublicKey {
            id: 1,
            title: "Laptop".to_string(),
            key: "ssh-rsa AAA...".to_string(),
            fingerprint: "SHA256:...".to_string(),
        }
    ];
    Json(keys)
}

async fn create_key(Json(payload): Json<CreateKeyOption>) -> (StatusCode, Json<PublicKey>) {
    let key = PublicKey {
        id: 2,
        title: payload.title,
        key: payload.key,
        fingerprint: "SHA256:new".to_string(),
    };
    (StatusCode::CREATED, Json(key))
}

async fn list_hooks(Path((_owner, _repo)): Path<(String, String)>) -> Json<Vec<Webhook>> {
    let hooks = vec![
        Webhook {
            id: 1,
            url: "http://example.com/hook".to_string(),
            events: vec!["push".to_string()],
            active: true,
        }
    ];
    Json(hooks)
}

async fn create_hook(
    Path((_owner, _repo)): Path<(String, String)>,
    Json(payload): Json<CreateHookOption>
) -> (StatusCode, Json<Webhook>) {
    let hook = Webhook {
        id: 2,
        url: payload.url,
        events: payload.events,
        active: payload.active,
    };
    (StatusCode::CREATED, Json(hook))
}

async fn list_teams(Path(_org): Path<String>) -> Json<Vec<Team>> {
    let teams = vec![
        Team {
            id: 1,
            name: "Developers".to_string(),
            description: Some("Dev Team".to_string()),
            permission: "write".to_string(),
        }
    ];
    Json(teams)
}

async fn list_projects(Path((_owner, _repo)): Path<(String, String)>) -> Json<Vec<Project>> {
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

async fn get_admin_stats() -> Json<AdminStats> {
    Json(AdminStats {
        users: 10,
        repos: 20,
        orgs: 5,
        issues: 100,
    })
}

async fn list_feeds() -> Json<Vec<Activity>> {
    let feeds = vec![
        Activity {
            id: 1,
            user_id: 1,
            user_name: "admin".to_string(),
            op_type: "push_branch".to_string(),
            content: "pushed to main".to_string(),
            created: "2023-01-01".to_string(),
        }
    ];
    Json(feeds)
}

async fn list_workflows(Path((_owner, _repo)): Path<(String, String)>) -> Json<Vec<ActionWorkflow>> {
    let wfs = vec![
        ActionWorkflow { id: 1, name: "CI".to_string(), status: "success".to_string() }
    ];
    Json(wfs)
}

async fn list_packages(Path(_owner): Path<String>) -> Json<Vec<Package>> {
    let pkgs = vec![
        Package { id: 1, name: "my-lib".to_string(), version: "1.0.0".to_string(), package_type: "cargo".to_string() }
    ];
    Json(pkgs)
}

async fn list_secrets(Path((_owner, _repo)): Path<(String, String)>) -> Json<Vec<Secret>> {
    let secrets = vec![
        Secret { name: "MY_TOKEN".to_string(), created_at: "2023-01-01".to_string() }
    ];
    Json(secrets)
}

async fn create_secret(
    Path((_owner, _repo)): Path<(String, String)>,
    Json(payload): Json<CreateSecretOption>
) -> (StatusCode, Json<Secret>) {
    let secret = Secret {
        name: payload.name,
        created_at: "2023-01-02".to_string(),
    };
    (StatusCode::CREATED, Json(secret))
}

async fn list_deploy_keys(Path((_owner, _repo)): Path<(String, String)>) -> Json<Vec<DeployKey>> {
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

async fn create_deploy_key(
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

async fn list_notices() -> Json<Vec<SystemNotice>> {
    let notices = vec![
        SystemNotice { id: 1, type_: "info".to_string(), description: "System maintenance at 00:00".to_string() }
    ];
    Json(notices)
}

async fn get_2fa() -> Json<TwoFactor> {
    Json(TwoFactor { enabled: false, method: "totp".to_string() })
}

async fn update_2fa(Json(_payload): Json<TwoFactor>) -> StatusCode {
    StatusCode::OK
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_list_repos() {
        let app = app();

        let response = app
            .oneshot(Request::builder().uri("/api/v1/repos").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let repos: Vec<Repository> = serde_json::from_slice(&body).unwrap();

        assert_eq!(repos.len(), 2);
        assert_eq!(repos[0].name, "codeza");
    }

    #[tokio::test]
    async fn test_star_repo() {
        let app = app();
        let response = app
            .oneshot(Request::builder().method("POST").uri("/api/v1/repos/admin/codeza/star").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::NO_CONTENT);
    }

    #[tokio::test]
    async fn test_fork_repo() {
        let app = app();
        let response = app
            .oneshot(Request::builder().method("POST").uri("/api/v1/repos/admin/codeza/fork").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_get_user() {
        let app = app();
        let response = app
            .oneshot(Request::builder().uri("/api/v1/users/admin").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let user: Option<User> = serde_json::from_slice(&body).unwrap();

        assert!(user.is_some());
        assert_eq!(user.unwrap().username, "admin");
    }

    #[tokio::test]
    async fn test_get_repo() {
        let app = app();
        let response = app
            .oneshot(Request::builder().uri("/api/v1/repos/admin/codeza").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let repo: Option<Repository> = serde_json::from_slice(&body).unwrap();

        assert!(repo.is_some());
        assert_eq!(repo.unwrap().name, "codeza");
    }

    #[tokio::test]
    async fn test_create_repo() {
        let app = app();
        let payload = CreateRepoOption {
            name: "new-project".to_string(),
            description: None,
            private: false,
            auto_init: true,
        };

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/user/repos")
                    .header("Content-Type", "application/json")
                    .body(Body::from(serde_json::to_string(&payload).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let repo: Repository = serde_json::from_slice(&body).unwrap();

        assert_eq!(repo.name, "new-project");
    }

    #[tokio::test]
    async fn test_list_issues() {
        let app = app();
        let response = app
            .oneshot(Request::builder().uri("/api/v1/repos/admin/codeza/issues").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let issues: Vec<Issue> = serde_json::from_slice(&body).unwrap();

        assert!(!issues.is_empty());
        assert_eq!(issues[0].title, "First Issue");
    }

    #[tokio::test]
    async fn test_create_issue() {
        let app = app();
        let payload = CreateIssueOption {
            title: "New Bug".to_string(),
            body: Some("Description".to_string()),
        };

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/repos/admin/codeza/issues")
                    .header("Content-Type", "application/json")
                    .body(Body::from(serde_json::to_string(&payload).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let issue: Issue = serde_json::from_slice(&body).unwrap();

        assert_eq!(issue.title, "New Bug");
    }

    #[tokio::test]
    async fn test_list_pulls() {
        let app = app();
        let response = app
            .oneshot(Request::builder().uri("/api/v1/repos/admin/codeza/pulls").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let pulls: Vec<PullRequest> = serde_json::from_slice(&body).unwrap();

        assert!(!pulls.is_empty());
        assert_eq!(pulls[0].title, "First PR");
    }

    #[tokio::test]
    async fn test_create_pull() {
        let app = app();
        let payload = CreatePullRequestOption {
            title: "New Feature".to_string(),
            body: None,
            head: "feature".to_string(),
            base: "main".to_string(),
        };

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/repos/admin/codeza/pulls")
                    .header("Content-Type", "application/json")
                    .body(Body::from(serde_json::to_string(&payload).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let pr: PullRequest = serde_json::from_slice(&body).unwrap();

        assert_eq!(pr.title, "New Feature");
    }

    #[tokio::test]
    async fn test_get_contents() {
        let app = app();
        let response = app
            .oneshot(Request::builder().uri("/api/v1/repos/admin/codeza/contents").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let files: Vec<FileEntry> = serde_json::from_slice(&body).unwrap();

        assert!(files.len() >= 2);
        assert_eq!(files[0].name, "src");
    }

    #[tokio::test]
    async fn test_list_commits() {
        let app = app();
        let response = app
            .oneshot(Request::builder().uri("/api/v1/repos/admin/codeza/commits").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let commits: Vec<Commit> = serde_json::from_slice(&body).unwrap();

        assert!(!commits.is_empty());
        assert_eq!(commits[0].sha, "abc123456789");
    }

    #[tokio::test]
    async fn test_list_releases() {
        let app = app();
        let response = app
            .oneshot(Request::builder().uri("/api/v1/repos/admin/codeza/releases").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let releases: Vec<Release> = serde_json::from_slice(&body).unwrap();

        assert!(!releases.is_empty());
        assert_eq!(releases[0].tag_name, "v1.0.0");
    }

    #[tokio::test]
    async fn test_create_release() {
        let app = app();
        let payload = CreateReleaseOption {
            tag_name: "v1.1.0".to_string(),
            name: "Next Release".to_string(),
            body: None,
            draft: false,
            prerelease: false,
        };

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/repos/admin/codeza/releases")
                    .header("Content-Type", "application/json")
                    .body(Body::from(serde_json::to_string(&payload).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let release: Release = serde_json::from_slice(&body).unwrap();

        assert_eq!(release.tag_name, "v1.1.0");
    }

    #[tokio::test]
    async fn test_login_success() {
        let app = app();
        let payload = LoginOption {
            username: "admin".to_string(),
            password: "password".to_string(),
        };
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/users/login")
                    .header("Content-Type", "application/json")
                    .body(Body::from(serde_json::to_string(&payload).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_get_org() {
        let app = app();
        let response = app
            .oneshot(Request::builder().uri("/api/v1/orgs/codeza-org").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let org: Option<Organization> = serde_json::from_slice(&body).unwrap();
        assert!(org.is_some());
    }

    #[tokio::test]
    async fn test_get_issue_detail() {
        let app = app();
        let response = app
            .oneshot(Request::builder().uri("/api/v1/repos/admin/codeza/issues/1").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_create_comment() {
        let app = app();
        let payload = CreateCommentOption { body: "Test comment".to_string() };
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/repos/admin/codeza/issues/1/comments")
                    .header("Content-Type", "application/json")
                    .body(Body::from(serde_json::to_string(&payload).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);
    }

    #[tokio::test]
    async fn test_list_labels() {
        let app = app();
        let response = app
            .oneshot(Request::builder().uri("/api/v1/repos/admin/codeza/labels").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let labels: Vec<Label> = serde_json::from_slice(&body).unwrap();
        assert!(!labels.is_empty());
    }

    #[tokio::test]
    async fn test_create_milestone() {
        let app = app();
        let payload = CreateMilestoneOption {
            title: "v1.0".to_string(),
            description: None,
            due_on: None,
        };
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/repos/admin/codeza/milestones")
                    .header("Content-Type", "application/json")
                    .body(Body::from(serde_json::to_string(&payload).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);
    }

    #[tokio::test]
    async fn test_list_topics() {
        let app = app();
        let response = app
            .oneshot(Request::builder().uri("/api/v1/repos/admin/codeza/topics").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let topics: Vec<Topic> = serde_json::from_slice(&body).unwrap();
        assert_eq!(topics[0].name, "rust");
    }

    #[tokio::test]
    async fn test_search_repos() {
        let app = app();
        let response = app
            .oneshot(Request::builder().uri("/api/v1/repos/search").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let repos: Vec<Repository> = serde_json::from_slice(&body).unwrap();
        assert!(!repos.is_empty());
    }

    #[tokio::test]
    async fn test_get_wiki_page() {
        let app = app();
        let response = app
            .oneshot(Request::builder().uri("/api/v1/repos/admin/codeza/wiki/pages/Home").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_create_wiki_page() {
        let app = app();
        let payload = CreateWikiPageOption {
            title: "NewPage".to_string(),
            content: "Content".to_string(),
            message: None,
        };
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/repos/admin/codeza/wiki/pages")
                    .header("Content-Type", "application/json")
                    .body(Body::from(serde_json::to_string(&payload).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);
    }

    #[tokio::test]
    async fn test_get_settings() {
        let app = app();
        let response = app
            .oneshot(Request::builder().uri("/api/v1/user/settings").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_list_notifications() {
        let app = app();
        let response = app
            .oneshot(Request::builder().uri("/api/v1/notifications").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let notifs: Vec<Notification> = serde_json::from_slice(&body).unwrap();
        assert!(!notifs.is_empty());
    }

    #[tokio::test]
    async fn test_create_key() {
        let app = app();
        let payload = CreateKeyOption {
            title: "New Key".to_string(),
            key: "ssh-rsa...".to_string(),
        };
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/user/keys")
                    .header("Content-Type", "application/json")
                    .body(Body::from(serde_json::to_string(&payload).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);
    }

    #[tokio::test]
    async fn test_create_hook() {
        let app = app();
        let payload = CreateHookOption {
            url: "http://test.com".to_string(),
            events: vec!["push".to_string()],
            active: true,
        };
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/repos/admin/codeza/hooks")
                    .header("Content-Type", "application/json")
                    .body(Body::from(serde_json::to_string(&payload).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);
    }

    #[tokio::test]
    async fn test_list_teams() {
        let app = app();
        let response = app
            .oneshot(Request::builder().uri("/api/v1/orgs/codeza-org/teams").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let teams: Vec<Team> = serde_json::from_slice(&body).unwrap();
        assert!(!teams.is_empty());
    }

    #[tokio::test]
    async fn test_list_projects() {
        let app = app();
        let response = app
            .oneshot(Request::builder().uri("/api/v1/repos/admin/codeza/projects").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let projects: Vec<Project> = serde_json::from_slice(&body).unwrap();
        assert!(!projects.is_empty());
    }

    #[tokio::test]
    async fn test_get_admin_stats() {
        let app = app();
        let response = app
            .oneshot(Request::builder().uri("/api/v1/admin/stats").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let stats: AdminStats = serde_json::from_slice(&body).unwrap();
        assert!(stats.users > 0);
    }

    #[tokio::test]
    async fn test_list_feeds() {
        let app = app();
        let response = app
            .oneshot(Request::builder().uri("/api/v1/user/feeds").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let feeds: Vec<Activity> = serde_json::from_slice(&body).unwrap();
        assert!(!feeds.is_empty());
    }

    #[tokio::test]
    async fn test_list_workflows() {
        let app = app();
        let response = app
            .oneshot(Request::builder().uri("/api/v1/repos/admin/codeza/actions/workflows").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let wfs: Vec<ActionWorkflow> = serde_json::from_slice(&body).unwrap();
        assert!(!wfs.is_empty());
    }

    #[tokio::test]
    async fn test_list_packages() {
        let app = app();
        let response = app
            .oneshot(Request::builder().uri("/api/v1/packages/admin").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let pkgs: Vec<Package> = serde_json::from_slice(&body).unwrap();
        assert!(!pkgs.is_empty());
    }

    #[tokio::test]
    async fn test_create_secret() {
        let app = app();
        let payload = CreateSecretOption { name: "TEST".to_string(), data: "val".to_string() };
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/repos/admin/codeza/secrets")
                    .header("Content-Type", "application/json")
                    .body(Body::from(serde_json::to_string(&payload).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);
    }

    #[tokio::test]
    async fn test_create_deploy_key() {
        let app = app();
        let payload = CreateKeyOption { title: "Deploy".to_string(), key: "ssh...".to_string() };
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/repos/admin/codeza/keys")
                    .header("Content-Type", "application/json")
                    .body(Body::from(serde_json::to_string(&payload).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);
    }

    #[tokio::test]
    async fn test_list_notices() {
        let app = app();
        let response = app
            .oneshot(Request::builder().uri("/api/v1/admin/notices").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let notices: Vec<SystemNotice> = serde_json::from_slice(&body).unwrap();
        assert!(!notices.is_empty());
    }

    #[tokio::test]
    async fn test_get_2fa() {
        let app = app();
        let response = app
            .oneshot(Request::builder().uri("/api/v1/user/2fa").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }
}
