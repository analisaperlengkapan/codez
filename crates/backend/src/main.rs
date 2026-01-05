use axum::{
    extract::{Json, Path},
    http::StatusCode,
    routing::{get, post},
    Router,
};
use shared::{CreateRepoOption, Repository, User};
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
}
