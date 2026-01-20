use axum::{
    routing::{get, post, delete, patch},
    Router,
};
use shared::{
    Issue, PullRequest, Release, Label, Milestone, Comment, Notification, PublicKey, Webhook,
    Repository, User, Activity, Commit, LfsLock, Topic, Package, Team, Project, ProjectColumn, ProjectCard, Review,
    Organization, OrgMember, WorkflowRun, WebhookDelivery
};
use std::sync::{Arc, RwLock};
use std::collections::HashMap;
use tower_http::cors::CorsLayer;
use crate::handlers::*;

#[derive(Clone, Default)]
pub struct AppState {
    pub repos: Arc<RwLock<Vec<Repository>>>,
    pub file_contents: Arc<RwLock<HashMap<(u64, String), String>>>,
    pub issues: Arc<RwLock<Vec<Issue>>>,
    pub users: Arc<RwLock<Vec<User>>>,
    pub pulls: Arc<RwLock<Vec<PullRequest>>>,
    pub releases: Arc<RwLock<Vec<Release>>>,
    pub labels: Arc<RwLock<Vec<Label>>>,
    pub milestones: Arc<RwLock<Vec<Milestone>>>,
    pub comments: Arc<RwLock<Vec<Comment>>>,
    pub notifications: Arc<RwLock<Vec<Notification>>>,
    pub keys: Arc<RwLock<Vec<PublicKey>>>,
    pub hooks: Arc<RwLock<Vec<Webhook>>>,
    pub activities: Arc<RwLock<Vec<Activity>>>,
    pub commits: Arc<RwLock<Vec<Commit>>>,
    pub lfs_locks: Arc<RwLock<Vec<LfsLock>>>,
    pub topics: Arc<RwLock<Vec<Topic>>>,
    pub packages: Arc<RwLock<Vec<Package>>>,
    pub teams: Arc<RwLock<Vec<Team>>>,
    pub projects: Arc<RwLock<Vec<Project>>>,
    pub project_columns: Arc<RwLock<Vec<ProjectColumn>>>,
    pub project_cards: Arc<RwLock<Vec<ProjectCard>>>,
    pub reviews: Arc<RwLock<Vec<Review>>>,
    pub orgs: Arc<RwLock<Vec<Organization>>>,
    pub org_members: Arc<RwLock<Vec<OrgMember>>>,
    pub workflow_runs: Arc<RwLock<Vec<WorkflowRun>>>,
    pub webhook_deliveries: Arc<RwLock<Vec<WebhookDelivery>>>,
    pub protected_branches: Arc<RwLock<Vec<shared::ProtectedBranch>>>,
    pub stars: Arc<RwLock<HashMap<u64, Vec<u64>>>>,
}

pub fn api_router() -> Router {
    let user = User::new(1, "admin".to_string(), Some("admin@codeza.com".to_string()));

    let mut file_map = HashMap::new();
    file_map.insert((1, "src/main.rs".to_string()), "fn main() { println!(\"Welcome to codeza\"); }".to_string());
    file_map.insert((1, "src/lib.rs".to_string()), "pub fn add(a: i32, b: i32) -> i32 { a + b }".to_string());
    file_map.insert((1, "README.md".to_string()), "# Codeza Repository\n\nThis is a demo repository.".to_string());
    file_map.insert((1, "Cargo.toml".to_string()), "[package]\nname = \"codeza\"\nversion = \"0.1.0\"\n".to_string());

    let state = AppState {
        repos: Arc::new(RwLock::new(vec![
            Repository::new(1, "codeza".to_string(), "admin".to_string()),
            Repository::new(2, "gitea-clone".to_string(), "user".to_string()),
        ])),
        file_contents: Arc::new(RwLock::new(file_map)),
        issues: Arc::new(RwLock::new(vec![
             Issue {
                id: 1,
                repo_id: 1,
                number: 1,
                title: "First Issue".to_string(),
                body: Some("This is a bug".to_string()),
                state: "open".to_string(),
                user: user.clone(),
                assignees: vec![],
                labels: vec![],
                milestone: None,
            }
        ])),
        users: Arc::new(RwLock::new(vec![
            user.clone(),
            User::new(2, "user".to_string(), Some("user@example.com".to_string())),
        ])),
        pulls: Arc::new(RwLock::new(vec![
            PullRequest {
                id: 1,
                repo_id: 1,
                number: 1,
                title: "First PR".to_string(),
                body: Some("Description".to_string()),
                state: "open".to_string(),
                user: user.clone(),
                merged: false,
            }
        ])),
        releases: Arc::new(RwLock::new(vec![
            Release {
                id: 1,
                repo_id: 1,
                tag_name: "v1.0.0".to_string(),
                name: "Initial Release".to_string(),
                body: Some("Description".to_string()),
                draft: false,
                prerelease: false,
                created_at: "2023-01-01".to_string(),
                author: user.clone(),
                assets: vec![],
            }
        ])),
        labels: Arc::new(RwLock::new(vec![
            Label {
                id: 1,
                repo_id: 1,
                name: "bug".to_string(),
                color: "#ff0000".to_string(),
                description: None,
            }
        ])),
        milestones: Arc::new(RwLock::new(vec![
            Milestone {
                id: 1,
                repo_id: 1,
                title: "v1.0".to_string(),
                description: None,
                due_on: None,
                state: "open".to_string(),
            }
        ])),
        comments: Arc::new(RwLock::new(vec![
            Comment {
                id: 1,
                issue_id: 1,
                body: "Great idea!".to_string(),
                user: user.clone(),
                created_at: "2023-01-01".to_string(),
                reactions: vec![],
            }
        ])),
        notifications: Arc::new(RwLock::new(vec![
            Notification {
                id: 1,
                subject: "Welcome to Codeza".to_string(),
                unread: true,
                updated_at: "2023-01-01".to_string(),
            }
        ])),
        keys: Arc::new(RwLock::new(vec![
            PublicKey {
                id: 1,
                title: "Laptop".to_string(),
                key: "ssh-rsa AAA...".to_string(),
                fingerprint: "SHA256:...".to_string(),
            }
        ])),
        hooks: Arc::new(RwLock::new(vec![
            Webhook {
                id: 1,
                repo_id: 1,
                url: "http://example.com/hook".to_string(),
                events: vec!["push".to_string()],
                active: true,
            }
        ])),
        activities: Arc::new(RwLock::new(vec![
            Activity {
                id: 1,
                repo_id: 1,
                user_id: 1,
                user_name: "admin".to_string(),
                op_type: "create_repo".to_string(),
                content: "created repository codeza".to_string(),
                created: "2023-01-01".to_string(),
            }
        ])),
        commits: Arc::new(RwLock::new(vec![
            Commit {
                sha: "abc123456789".to_string(),
                repo_id: 1,
                message: "Initial commit".to_string(),
                author: user.clone(),
                date: "2023-01-01T12:00:00Z".to_string(),
            }
        ])),
        lfs_locks: Arc::new(RwLock::new(vec![])),
        topics: Arc::new(RwLock::new(vec![
            Topic {
                id: 1,
                repo_id: 1,
                name: "rust".to_string(),
                created: "2023-01-01".to_string(),
            }
        ])),
        packages: Arc::new(RwLock::new(vec![
            Package {
                id: 1,
                owner: "admin".to_string(),
                name: "my-lib".to_string(),
                version: "1.0.0".to_string(),
                package_type: "cargo".to_string(),
            }
        ])),
        teams: Arc::new(RwLock::new(vec![
            Team {
                id: 1,
                org_name: "codeza-org".to_string(),
                name: "Developers".to_string(),
                description: Some("Dev Team".to_string()),
                permission: "write".to_string(),
            }
        ])),
        projects: Arc::new(RwLock::new(vec![])),
        project_columns: Arc::new(RwLock::new(vec![])),
        project_cards: Arc::new(RwLock::new(vec![])),
        reviews: Arc::new(RwLock::new(vec![])),
        orgs: Arc::new(RwLock::new(vec![
            Organization {
                id: 1,
                username: "codeza-org".to_string(),
                description: Some("Codeza Organization".to_string()),
                avatar_url: None,
            }
        ])),
        org_members: Arc::new(RwLock::new(vec![])),
        workflow_runs: Arc::new(RwLock::new(vec![])),
        webhook_deliveries: Arc::new(RwLock::new(vec![])),
        protected_branches: Arc::new(RwLock::new(vec![])),
        stars: Arc::new(RwLock::new(HashMap::new())),
    };

    Router::new()
        .route("/api/v1/repos", get(list_repos))
        .route("/api/v1/users/:username", get(get_user))
        .route("/api/v1/repos/:owner/:repo", get(get_repo))
        .route("/api/v1/user/repos", post(create_repo))
        .route("/api/v1/repos/:owner/:repo/issues", get(list_issues).post(create_issue))
        .route("/api/v1/repos/:owner/:repo/pulls", get(list_pulls).post(create_pull))
        .route("/api/v1/repos/:owner/:repo/pulls/:index", patch(update_pull))
        .route("/api/v1/repos/:owner/:repo/pulls/:index/reviews", get(list_reviews).post(create_review))
        .route("/api/v1/user/issues", get(list_user_issues))
        .route("/api/v1/user/pulls", get(list_user_pulls))
        .route("/api/v1/repos/:owner/:repo/contents/*path", get(get_contents).put(update_file))
        .route("/api/v1/repos/:owner/:repo/contents", get(get_root_contents))
        .route("/api/v1/repos/:owner/:repo/commits", get(list_commits))
        .route("/api/v1/repos/:owner/:repo/releases", get(list_releases).post(create_release))
        .route("/api/v1/repos/:owner/:repo/releases/:id", get(get_release).patch(update_release).delete(delete_release))
        .route("/api/v1/repos/:owner/:repo/releases/:id/assets", post(upload_release_asset))
        .route("/api/v1/repos/:owner/:repo/releases/:id/assets/:asset_id", get(download_release_asset))
        .route("/api/v1/users/login", post(login_user))
        .route("/api/v1/users/register", post(register_user))
        .route("/api/v1/orgs", post(create_org))
        .route("/api/v1/orgs/:org", get(get_org))
        .route("/api/v1/orgs/:org/repos", get(list_org_repos))
        .route("/api/v1/repos/:owner/:repo/issues/:index", get(get_issue).patch(update_issue))
        .route("/api/v1/repos/:owner/:repo/issues/:index/comments", get(list_comments).post(create_comment))
        .route("/api/v1/repos/:owner/:repo/issues/comments/:id", patch(update_comment).delete(delete_comment))
        .route("/api/v1/repos/:owner/:repo/pulls/:index/merge", post(merge_pull))
        .route("/api/v1/repos/:owner/:repo/labels", get(list_labels).post(create_label))
        .route("/api/v1/repos/:owner/:repo/milestones", get(list_milestones).post(create_milestone))
        .route("/api/v1/repos/:owner/:repo/milestones/:id", get(get_milestone))
        .route("/api/v1/repos/:owner/:repo/topics", get(list_topics).put(update_topics))
        .route("/api/v1/repos/:owner/:repo/issues/comments/:id/reactions", post(add_reaction))
        .route("/api/v1/repos/:owner/:repo/star", post(star_repo))
        .route("/api/v1/repos/:owner/:repo/user_status", get(get_user_repo_status))
        .route("/api/v1/user/starred", get(list_starred_repos))
        .route("/api/v1/repos/:owner/:repo/watch", post(watch_repo))
        .route("/api/v1/repos/:owner/:repo/fork", post(fork_repo))
        .route("/api/v1/repos/search", get(search_repos))
        .route("/api/v1/repos/:owner/:repo/settings", get(get_repo_settings).patch(update_repo_settings))
        .route("/api/v1/user/settings", get(get_user_settings).patch(update_user_settings))
        .route("/api/v1/notifications", get(list_notifications))
        .route("/api/v1/user/keys", get(list_keys).post(create_key))
        .route("/api/v1/repos/:owner/:repo/hooks", get(list_hooks).post(create_hook))
        .route("/api/v1/repos/:owner/:repo/hooks/:id/deliveries", get(list_hook_deliveries))
        .route("/api/v1/orgs/:org/teams", get(list_teams).post(create_team))
        .route("/api/v1/repos/:owner/:repo/projects", get(list_projects).post(create_project))
        .route("/api/v1/repos/:owner/:repo/projects/:id", get(get_project))
        .route("/api/v1/repos/:owner/:repo/projects/:id/close", post(close_project))
        .route("/api/v1/repos/:owner/:repo/projects/:id/reopen", post(reopen_project))
        .route("/api/v1/repos/:owner/:repo/projects/:id/columns", get(list_project_columns).post(create_project_column))
        .route("/api/v1/repos/:owner/:repo/projects/columns/:id/cards", get(list_project_cards).post(create_project_card))
        .route("/api/v1/repos/:owner/:repo/projects/cards/:id/move", post(move_project_card))
        .route("/api/v1/admin/stats", get(get_admin_stats))
        .route("/api/v1/user/feeds", get(list_feeds))
        .route("/api/v1/repos/:owner/:repo/actions/workflows", get(list_workflows))
        .route("/api/v1/repos/:owner/:repo/actions/workflows/:id/runs", get(list_workflow_runs).post(trigger_workflow))
        .route("/api/v1/packages/:owner", get(list_packages).post(upload_package))
        .route("/api/v1/repos/:owner/:repo/secrets", get(list_secrets).post(create_secret))
        .route("/api/v1/repos/:owner/:repo/keys", get(list_deploy_keys).post(create_deploy_key))
        .route("/api/v1/admin/notices", get(list_notices))
        .route("/api/v1/user/2fa", get(get_2fa).post(update_2fa))
        .route("/api/v1/user/gpg_keys", get(list_gpg_keys).post(create_gpg_key))
        .route("/api/v1/repos/:owner/:repo/mirror-sync", post(mirror_sync))
        .route("/api/v1/repos/:owner/:repo/collaborators", get(list_collaborators))
        .route("/api/v1/repos/:owner/:repo/collaborators/:collaborator", get(get_collaborator).put(add_collaborator))
        .route("/api/v1/repos/:owner/:repo/branches", get(list_branches).post(create_branch))
        .route("/api/v1/repos/:owner/:repo/tags", get(list_tags))
        .route("/api/v1/repos/:owner/:repo/media", post(upload_media))
        .route("/api/v1/user/oauth2", get(list_oauth2_providers))
        .route("/api/v1/repos/:owner/:repo/commits/:sha/diff", get(get_commit_diff))
        .route("/api/v1/repos/:owner/:repo/raw/*path", get(get_raw_file))
        .route("/api/v1/users/:username/followers", get(list_followers))
        .route("/api/v1/users/:username/following", get(list_following))
        .route("/api/v1/users/:username/heatmap", get(get_user_heatmap))
        .route("/api/v1/orgs/:org/members", get(list_org_members))
        .route("/api/v1/orgs/:org/members/:username", post(add_org_member).delete(remove_org_member))
        .route("/api/v1/licenses", get(list_licenses))
        .route("/api/v1/gitignore/templates", get(list_gitignores))
        .route("/api/v1/repos/:owner/:repo/issues/:index/assignees", post(add_issue_assignee))
        .route("/api/v1/repos/:owner/:repo/issues/:index/assignees/:username", delete(remove_issue_assignee))
        .route("/api/v1/repos/:owner/:repo/pulls/:index/requested_reviewers", post(request_review))
        .route("/api/v1/admin/users", get(admin_list_users).post(admin_create_user))
        .route("/api/v1/admin/users/:username", post(admin_edit_user).delete(admin_delete_user))
        .route("/api/v1/repos/:owner/:repo/languages", get(get_repo_languages))
        .route("/api/v1/repos/:owner/:repo/branch_protections", get(list_branch_protections).post(create_branch_protection))
        .route("/api/v1/repos/:owner/:repo/branch_protections/:name", delete(delete_branch_protection))
        .route("/api/v1/search/issues", get(search_issues_global))
        .route("/api/v1/user/emails", get(list_emails))
        .route("/api/v1/user/applications/oauth2", get(list_oauth2_apps))
        .route("/api/v1/repos/migrate", post(migrate_repo))
        .route("/api/v1/repos/:owner/:repo/transfer", post(transfer_repo))
        .route("/api/v1/user/keys/:id", delete(delete_ssh_key))
        .route("/api/v1/user/gpg_keys/:id", delete(delete_gpg_key))
        .route("/api/v1/repos/:owner/:repo/milestones/:id/stats", get(get_milestone_stats))
        .route("/api/v1/repos/:owner/:repo/pulls/:index/files", get(get_pr_files))
        .route("/api/v1/repos/:owner/:repo/issues/:index/labels", post(add_issue_label))
        .route("/api/v1/repos/:owner/:repo/issues/:index/labels/:id", delete(remove_issue_label))
        .route("/api/v1/repos/:owner/:repo/search", get(search_repo_code))
        .route("/api/v1/packages/:owner/:type/:name/:version", get(get_package_detail))
        .route("/api/v1/repos/:owner/:repo/wiki/pages", get(list_wiki_pages).post(create_wiki_page))
        .route("/api/v1/repos/:owner/:repo/wiki/pages/:page_name", get(get_wiki_page).put(update_wiki_page))
        .route("/api/v1/repos/:owner/:repo/git/lfs/locks", get(list_lfs_locks).post(create_lfs_lock))
        .route("/api/v1/user/gpg_keys/:id/verify", post(verify_gpg_key))
        .route("/api/v1/notifications/threads/:id", patch(mark_notification_read))
        .layer(CorsLayer::permissive())
        .with_state(state)
}
