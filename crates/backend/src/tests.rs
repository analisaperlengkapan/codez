#[cfg(test)]
mod tests {
    use crate::router::api_router;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt; // for `oneshot`
    use shared::{CreateRepoOption, Repository, Activity, CreateIssueOption, Issue, UpdateFileOption, FileEntry, UpdateIssueOption};

    #[tokio::test]
    async fn test_create_repo_flow() {
        let app = api_router();

        let payload = CreateRepoOption {
            name: "test-repo".to_string(),
            description: None,
            private: false,
            auto_init: true,
            gitignores: None,
            license: None,
            readme: None,
        };

        let response = app.clone()
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
        assert_eq!(repo.name, "test-repo");

        // Verify Activity Log
        let response = app.clone()
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/api/v1/user/feeds")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let activities: Vec<Activity> = serde_json::from_slice(&body).unwrap();

        let found = activities.iter().any(|a| a.content.contains("created repository test-repo"));
        assert!(found, "Should find creation activity in feed");
    }

    #[tokio::test]
    async fn test_create_issue_flow() {
        let app = api_router();

        // Create Issue
        let payload = CreateIssueOption {
            title: "Test Bug".to_string(),
            body: Some("Description".to_string()),
        };

        let response = app.clone()
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
        assert_eq!(issue.title, "Test Bug");

        // Verify Activity
        let response = app.clone()
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/api/v1/user/feeds")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let activities: Vec<Activity> = serde_json::from_slice(&body).unwrap();
        let found = activities.iter().any(|a| a.op_type == "create_issue" && a.content.contains("opened issue"));
        assert!(found);
    }

    #[tokio::test]
    async fn test_update_issue_flow() {
        let app = api_router();

        // Update Issue 1 (default mock issue)
        let payload = UpdateIssueOption {
            title: Some("Updated Title".to_string()),
            body: Some("Updated Body".to_string()),
            state: Some("closed".to_string()),
            milestone_id: None,
        };

        let response = app.clone()
            .oneshot(
                Request::builder()
                    .method("PATCH")
                    .uri("/api/v1/repos/admin/codeza/issues/1")
                    .header("Content-Type", "application/json")
                    .body(Body::from(serde_json::to_string(&payload).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let issue: Option<Issue> = serde_json::from_slice(&body).unwrap();
        let issue = issue.unwrap();
        assert_eq!(issue.title, "Updated Title");
        assert_eq!(issue.state, "closed");
    }

    #[tokio::test]
    async fn test_remove_issue_assignee_flow() {
        let app = api_router();

        // Add assignee first (mocked user is already in Assignees? No, init is empty)
        // Add User 2
        let payload = shared::User::new(2, "user".to_string(), None);
        let _ = app.clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/repos/admin/codeza/issues/1/assignees")
                    .header("Content-Type", "application/json")
                    .body(Body::from(serde_json::to_string(&payload).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        // Remove User 2
        let response = app.clone()
            .oneshot(
                Request::builder()
                    .method("DELETE")
                    .uri("/api/v1/repos/admin/codeza/issues/1/assignees/user")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NO_CONTENT);
    }

    #[tokio::test]
    async fn test_issue_filter_flow() {
        let app = api_router();

        // Ensure issue 1 has label 1 and assignee "admin" (default mock state needs setup?)
        // Actually mock state init has no labels/assignees on issue 1.
        // Let's add them via API first or assume test starts fresh.
        // We'll add a label to issue 1
        let payload = shared::CreateLabelOption { name: "bug".to_string(), color: "#f00".to_string(), description: None };
        let _ = app.clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/repos/admin/codeza/issues/1/labels")
                    .header("Content-Type", "application/json")
                    .body(Body::from(serde_json::to_string(&payload).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        // Filter by label (mock label id 100 from handler)
        let response = app.clone()
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/api/v1/repos/admin/codeza/issues?label_id=100")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let issues: Vec<Issue> = serde_json::from_slice(&body).unwrap();
        assert_eq!(issues.len(), 1);
        assert_eq!(issues[0].number, 1);

        // Filter by wrong label
        let response = app.clone()
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/api/v1/repos/admin/codeza/issues?label_id=999")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let issues: Vec<Issue> = serde_json::from_slice(&body).unwrap();
        assert_eq!(issues.len(), 0);
    }

    #[tokio::test]
    async fn test_update_file_flow() {
        let app = api_router();

        let payload = UpdateFileOption {
            content: "fn main() {}".to_string(),
            message: "Update main.rs".to_string(),
            sha: "old_sha".to_string(),
            branch: Some("main".to_string()),
        };

        let response = app.clone()
            .oneshot(
                Request::builder()
                    .method("PUT")
                    .uri("/api/v1/repos/admin/codeza/contents/src/main.rs")
                    .header("Content-Type", "application/json")
                    .body(Body::from(serde_json::to_string(&payload).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let file: FileEntry = serde_json::from_slice(&body).unwrap();
        assert_eq!(file.path, "src/main.rs");

        // Verify Commit (via Activity or Commit List)
        let response = app.clone()
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/api/v1/user/feeds")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let activities: Vec<Activity> = serde_json::from_slice(&body).unwrap();
        let found = activities.iter().any(|a| a.op_type == "update_file" && a.content.contains("updated file src/main.rs"));
        assert!(found);
    }
}
