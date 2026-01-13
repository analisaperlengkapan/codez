#[cfg(test)]
mod tests {
    use crate::router::api_router;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt; // for `oneshot`
    use shared::{
        CreateRepoOption, Repository, Activity, CreateIssueOption, Issue, UpdateFileOption, FileEntry, UpdateIssueOption,
        CreateCommentOption, Comment, UpdateCommentOption, CreatePullRequestOption, UpdatePullRequestOption, PullRequest,
        CreateProjectOption, Project, CreateProjectColumnOption, ProjectColumn, CreateProjectCardOption, ProjectCard, MoveProjectCardOption
    };

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
    async fn test_issue_pagination_sort_flow() {
        let app = api_router();

        // Create 2 issues
        for i in 1..=2 {
            let payload = CreateIssueOption {
                title: format!("Issue {}", i),
                body: None,
            };
            let _ = app.clone()
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
        }

        // Test Pagination: Limit 1
        let response = app.clone()
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/api/v1/repos/admin/codeza/issues?limit=1&page=1")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let issues: Vec<Issue> = serde_json::from_slice(&body).unwrap();
        assert_eq!(issues.len(), 1);

        // Test Sort Desc (Default) - Issue 2 should be first
        let response = app.clone()
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/api/v1/repos/admin/codeza/issues?sort=created&direction=desc")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let issues: Vec<Issue> = serde_json::from_slice(&body).unwrap();
        // Since we created 2 new issues, plus the initial 1, we have 3 total.
        // Order desc by ID: Issue 3 (created 2nd here), Issue 2 (created 1st here), Issue 1 (initial).
        // Let's verify the first one is the latest created.
        assert!(issues[0].id > issues[1].id);
    }

    #[tokio::test]
    async fn test_comment_update_delete_flow() {
        let app = api_router();

        // Create comment first
        let payload = CreateCommentOption { body: "Initial comment".to_string() };
        let response = app.clone()
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

        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let comment: Comment = serde_json::from_slice(&body).unwrap();
        let comment_id = comment.id;

        // Update comment
        let update_payload = UpdateCommentOption { body: "Updated comment".to_string() };
        let response = app.clone()
            .oneshot(
                Request::builder()
                    .method("PATCH")
                    .uri(&format!("/api/v1/repos/admin/codeza/issues/comments/{}", comment_id))
                    .header("Content-Type", "application/json")
                    .body(Body::from(serde_json::to_string(&update_payload).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let updated_comment: Option<Comment> = serde_json::from_slice(&body).unwrap();
        assert_eq!(updated_comment.unwrap().body, "Updated comment");

        // Delete comment
        let response = app.clone()
            .oneshot(
                Request::builder()
                    .method("DELETE")
                    .uri(&format!("/api/v1/repos/admin/codeza/issues/comments/{}", comment_id))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NO_CONTENT);
    }

    #[tokio::test]
    async fn test_update_pull_request_flow() {
        let app = api_router();

        // Create a PR first
        let payload = CreatePullRequestOption {
            title: "Test PR".to_string(),
            body: Some("Body".to_string()),
            head: "feature".to_string(),
            base: "main".to_string(),
        };
        let _ = app.clone()
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

        // Update the PR
        let update_payload = UpdatePullRequestOption {
            title: Some("Updated PR Title".to_string()),
            body: None,
            state: Some("closed".to_string()),
        };
        let response = app.clone()
            .oneshot(
                Request::builder()
                    .method("PATCH")
                    .uri("/api/v1/repos/admin/codeza/pulls/1") // PR created gets ID 2 because ID 1 is in mock init? No, init has 1. Created will be 2.
                    // Wait, init state has PR 1. So we can just update PR 1.
                    .header("Content-Type", "application/json")
                    .body(Body::from(serde_json::to_string(&update_payload).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let pr: Option<PullRequest> = serde_json::from_slice(&body).unwrap();
        let pr = pr.unwrap();
        assert_eq!(pr.title, "Updated PR Title");
        assert_eq!(pr.state, "closed");
    }

    #[tokio::test]
    async fn test_comment_reaction_flow() {
        let app = api_router();

        // Add reaction to comment 1 (mock init state)
        let payload = shared::CreateReactionOption { content: "+1".to_string() };
        let response = app.clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/repos/admin/codeza/issues/comments/1/reactions")
                    .header("Content-Type", "application/json")
                    .body(Body::from(serde_json::to_string(&payload).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let reaction: shared::Reaction = serde_json::from_slice(&body).unwrap();
        assert_eq!(reaction.content, "+1");
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

    #[tokio::test]
    async fn test_user_dashboard_flow() {
        let app = api_router();

        // 1. Create an issue assigned to current user (admin, id=1)
        // Note: create_issue currently doesn't assign automatically. We need to assign it.
        // Step 1.1 Create issue
        let payload = CreateIssueOption {
            title: "Assigned Task".to_string(),
            body: None,
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
        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let issue: Issue = serde_json::from_slice(&body).unwrap();
        let issue_id = issue.id;

        // Step 1.2 Assign to admin (username "admin")
        let user_payload = shared::User::new(1, "admin".to_string(), None);
        let _ = app.clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(&format!("/api/v1/repos/admin/codeza/issues/{}/assignees", issue_id))
                    .header("Content-Type", "application/json")
                    .body(Body::from(serde_json::to_string(&user_payload).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        // 2. Create a PR by current user (admin is default creator in mock)
        let pr_payload = CreatePullRequestOption {
            title: "My PR".to_string(),
            body: None,
            head: "f".to_string(),
            base: "m".to_string(),
        };
        let _ = app.clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/repos/admin/codeza/pulls")
                    .header("Content-Type", "application/json")
                    .body(Body::from(serde_json::to_string(&pr_payload).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        // 3. Verify list_user_issues
        let response = app.clone()
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/api/v1/user/issues?state=open")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let my_issues: Vec<Issue> = serde_json::from_slice(&body).unwrap();
        // Should find "Assigned Task"
        assert!(my_issues.iter().any(|i| i.title == "Assigned Task"));

        // 4. Verify list_user_pulls
        let response = app.clone()
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/api/v1/user/pulls?state=open")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let my_pulls: Vec<PullRequest> = serde_json::from_slice(&body).unwrap();
        // Should find "My PR" (or "First PR" from init)
        assert!(my_pulls.iter().any(|p| p.title == "My PR"));
    }

    #[tokio::test]
    async fn test_project_flow() {
        let app = api_router();

        // 1. Create Project
        let payload = CreateProjectOption {
            title: "My Project".to_string(),
            description: Some("Kanban Board".to_string()),
        };
        let response = app.clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/repos/admin/codeza/projects")
                    .header("Content-Type", "application/json")
                    .body(Body::from(serde_json::to_string(&payload).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let project: Project = serde_json::from_slice(&body).unwrap();
        let project_id = project.id;
        assert_eq!(project.title, "My Project");

        // 2. Create Column
        let payload = CreateProjectColumnOption { title: "To Do".to_string() };
        let response = app.clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(&format!("/api/v1/repos/admin/codeza/projects/{}/columns", project_id))
                    .header("Content-Type", "application/json")
                    .body(Body::from(serde_json::to_string(&payload).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let column: ProjectColumn = serde_json::from_slice(&body).unwrap();
        let column_id = column.id;
        assert_eq!(column.title, "To Do");

        // 3. Create Card
        let payload = CreateProjectCardOption {
            content: Some("Task 1".to_string()),
            note: None,
            issue_id: None,
        };
        let response = app.clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(&format!("/api/v1/repos/admin/codeza/projects/columns/{}/cards", column_id))
                    .header("Content-Type", "application/json")
                    .body(Body::from(serde_json::to_string(&payload).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let card: ProjectCard = serde_json::from_slice(&body).unwrap();
        let card_id = card.id;
        assert_eq!(card.content, Some("Task 1".to_string()));

        // 4. Move Card (to same column just index change, or assume 2nd column exists)
        // Let's just create a second column to be sure
        let payload = CreateProjectColumnOption { title: "Done".to_string() };
        let response = app.clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(&format!("/api/v1/repos/admin/codeza/projects/{}/columns", project_id))
                    .header("Content-Type", "application/json")
                    .body(Body::from(serde_json::to_string(&payload).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();
        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let col2: ProjectColumn = serde_json::from_slice(&body).unwrap();

        let payload = MoveProjectCardOption { column_id: col2.id, new_index: 0 };
        let response = app.clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(&format!("/api/v1/repos/admin/codeza/projects/cards/{}/move", card_id))
                    .header("Content-Type", "application/json")
                    .body(Body::from(serde_json::to_string(&payload).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        // 5. Close Project
        let response = app.clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(&format!("/api/v1/repos/admin/codeza/projects/{}/close", project_id))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        // Verify closed
        let response = app.clone()
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri(&format!("/api/v1/repos/admin/codeza/projects/{}", project_id))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let p: Option<Project> = serde_json::from_slice(&body).unwrap();
        assert!(p.unwrap().is_closed);

        // 6. Create Card Linked to Issue
        // Create Issue first to check if we can link it
        let issue_payload = CreateIssueOption { title: "Issue for card".to_string(), body: None };
        let _ = app.clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/repos/admin/codeza/issues")
                    .header("Content-Type", "application/json")
                    .body(Body::from(serde_json::to_string(&issue_payload).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        let payload = CreateProjectCardOption {
            content: None,
            note: None,
            issue_id: Some(1), // Assuming ID 1 or we should fetch it. Since tests run in parallel or sequence,
                               // ID prediction is brittle. Let's assume ID is > 0.
        };
        let response = app.clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(&format!("/api/v1/repos/admin/codeza/projects/columns/{}/cards", column_id))
                    .header("Content-Type", "application/json")
                    .body(Body::from(serde_json::to_string(&payload).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);

        // Verify Activity Log for card creation
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
        assert!(activities.iter().any(|a| a.op_type == "create_project_card"));
        assert!(activities.iter().any(|a| a.op_type == "move_project_card"));
    }

    #[tokio::test]
    async fn test_pull_request_review_flow() {
        let app = api_router();

        // Create PR first
        let payload = CreatePullRequestOption {
            title: "PR for Review".to_string(),
            body: None,
            head: "f".to_string(),
            base: "m".to_string(),
        };
        let response = app.clone()
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
        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let pr: PullRequest = serde_json::from_slice(&body).unwrap();
        let pr_number = pr.number;

        // Submit Review
        let review_payload = shared::CreateReviewOption {
            body: "Looks good to me".to_string(),
            event: "APPROVE".to_string(),
        };
        let response = app.clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(&format!("/api/v1/repos/admin/codeza/pulls/{}/reviews", pr_number))
                    .header("Content-Type", "application/json")
                    .body(Body::from(serde_json::to_string(&review_payload).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let review: shared::Review = serde_json::from_slice(&body).unwrap();
        assert_eq!(review.state, "APPROVED");
        assert_eq!(review.body, "Looks good to me");

        // List Reviews
        let response = app.clone()
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri(&format!("/api/v1/repos/admin/codeza/pulls/{}/reviews", pr_number))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let reviews: Vec<shared::Review> = serde_json::from_slice(&body).unwrap();
        assert_eq!(reviews.len(), 1);
        assert_eq!(reviews[0].state, "APPROVED");
    }

    #[tokio::test]
    async fn test_org_management_flow() {
        let app = api_router();

        // 1. Create Organization
        let payload = shared::CreateOrgOption {
            username: "new-org".to_string(),
            description: Some("New Org".to_string()),
        };
        let response = app.clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/orgs")
                    .header("Content-Type", "application/json")
                    .body(Body::from(serde_json::to_string(&payload).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);

        // 2. Create Team
        let team_payload = shared::CreateTeamOption {
            name: "Devs".to_string(),
            description: None,
            permission: "write".to_string(),
        };
        let response = app.clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/orgs/new-org/teams")
                    .header("Content-Type", "application/json")
                    .body(Body::from(serde_json::to_string(&team_payload).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);

        // 3. List Teams
        let response = app.clone()
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/api/v1/orgs/new-org/teams")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let teams: Vec<shared::Team> = serde_json::from_slice(&body).unwrap();
        assert_eq!(teams.len(), 1);
        assert_eq!(teams[0].name, "Devs");
    }
}
