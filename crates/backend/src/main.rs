use axum::{
    extract::{Json, Path},
    http::StatusCode,
    routing::{get, post},
    Router,
};
use shared::{Commit, CreateIssueOption, CreatePullRequestOption, CreateRepoOption, FileEntry, Issue, PullRequest, Repository, User};
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
}
