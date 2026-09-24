//! HTTP router construction.
//!
//! Routes are grouped into small, feature-focused sub-routers that are merged
//! into a single application router. The public API is versioned under
//! `/api/v1`; all responses are JSON unless noted otherwise.

use axum::{
    routing::{delete, get, patch, post, put},
    Router,
};
use tower_http::cors::CorsLayer;

use crate::handlers::*;
use crate::state::AppState;

/// Repository, source browsing, branches, tags and topics.
fn repo_routes() -> Router<AppState> {
    Router::new()
        .route("/api/v1/repos", get(list_repos))
        .route("/api/v1/repos/:owner/:repo", get(get_repo))
        .route("/api/v1/user/repos", post(create_repo))
        .route(
            "/api/v1/repos/:owner/:repo/contents/*path",
            get(get_contents).put(update_file),
        )
        .route(
            "/api/v1/repos/:owner/:repo/contents",
            get(get_root_contents),
        )
        .route("/api/v1/repos/:owner/:repo/commits", get(list_commits))
        .route(
            "/api/v1/repos/:owner/:repo/topics",
            get(list_topics).put(update_topics),
        )
        .route("/api/v1/repos/:owner/:repo/star", post(star_repo))
        .route(
            "/api/v1/repos/:owner/:repo/user_status",
            get(get_user_repo_status),
        )
        .route("/api/v1/repos/:owner/:repo/watch", post(watch_repo))
        .route("/api/v1/repos/:owner/:repo/fork", post(fork_repo))
        .route("/api/v1/repos/search", get(search_repos))
        .route(
            "/api/v1/repos/:owner/:repo/settings",
            get(get_repo_settings).patch(update_repo_settings),
        )
        .route(
            "/api/v1/repos/:owner/:repo/hooks",
            get(list_hooks).post(create_hook),
        )
        .route(
            "/api/v1/repos/:owner/:repo/hooks/:id/deliveries",
            get(list_hook_deliveries),
        )
        .route(
            "/api/v1/repos/:owner/:repo/secrets",
            get(list_secrets).post(create_secret),
        )
        .route(
            "/api/v1/repos/:owner/:repo/keys",
            get(list_deploy_keys).post(create_deploy_key),
        )
        .route("/api/v1/repos/:owner/:repo/mirror-sync", post(mirror_sync))
        .route(
            "/api/v1/repos/:owner/:repo/collaborators",
            get(list_collaborators),
        )
        .route(
            "/api/v1/repos/:owner/:repo/collaborators/:collaborator",
            get(get_collaborator).put(add_collaborator),
        )
        .route(
            "/api/v1/repos/:owner/:repo/branches",
            get(list_branches).post(create_branch),
        )
        .route("/api/v1/repos/:owner/:repo/tags", get(list_tags))
        .route("/api/v1/repos/:owner/:repo/media", post(upload_media))
        .route(
            "/api/v1/repos/:owner/:repo/commits/:sha/diff",
            get(get_commit_diff),
        )
        .route("/api/v1/repos/:owner/:repo/raw/*path", get(get_raw_file))
        .route(
            "/api/v1/repos/:owner/:repo/languages",
            get(get_repo_languages),
        )
        .route(
            "/api/v1/repos/:owner/:repo/branch_protections",
            get(list_branch_protections).post(create_branch_protection),
        )
        .route(
            "/api/v1/repos/:owner/:repo/branch_protections/:name",
            delete(delete_branch_protection),
        )
        .route(
            "/api/v1/repos/:owner/:repo/statuses/:sha",
            post(create_commit_status),
        )
        .route(
            "/api/v1/repos/:owner/:repo/commits/:ref/statuses",
            get(list_commit_statuses),
        )
        .route("/api/v1/repos/migrate", post(migrate_repo))
        .route("/api/v1/repos/:owner/:repo/transfer", post(transfer_repo))
        .route("/api/v1/repos/:owner/:repo/search", get(search_repo_code))
        .route("/api/v1/repos/:owner/:repo/pulse", get(get_repo_pulse))
        .route(
            "/api/v1/repos/:owner/:repo/security/scan",
            post(run_security_scan).get(run_security_scan),
        )
        .route(
            "/api/v1/repos/:owner/:repo/wiki/pages",
            get(list_wiki_pages).post(create_wiki_page),
        )
        .route(
            "/api/v1/repos/:owner/:repo/wiki/pages/:page_name",
            get(get_wiki_page).put(update_wiki_page),
        )
        .route(
            "/api/v1/repos/:owner/:repo/discussions",
            get(list_discussions).post(create_discussion),
        )
        .route(
            "/api/v1/repos/:owner/:repo/discussions/:id",
            get(get_discussion)
                .patch(update_discussion)
                .delete(delete_discussion),
        )
        .route(
            "/api/v1/repos/:owner/:repo/discussions/:id/comments",
            get(list_discussion_comments).post(create_discussion_comment),
        )
        .route(
            "/api/v1/repos/:owner/:repo/git/lfs/locks",
            get(list_lfs_locks).post(create_lfs_lock),
        )
        .route(
            "/api/v1/repos/:owner/:repo/git/lfs/locks/:id/unlock",
            post(delete_lfs_lock),
        )
}

/// Issues, comments, labels and milestones.
fn issue_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/api/v1/repos/:owner/:repo/issues",
            get(list_issues).post(create_issue),
        )
        .route("/api/v1/user/issues", get(list_user_issues))
        .route(
            "/api/v1/repos/:owner/:repo/issues/:index",
            get(get_issue).patch(update_issue),
        )
        .route(
            "/api/v1/repos/:owner/:repo/issues/:index/lock",
            put(lock_issue).delete(unlock_issue),
        )
        .route(
            "/api/v1/repos/:owner/:repo/issues/:index/comments",
            get(list_comments).post(create_comment),
        )
        .route(
            "/api/v1/repos/:owner/:repo/issues/comments/:id",
            patch(update_comment).delete(delete_comment),
        )
        .route(
            "/api/v1/repos/:owner/:repo/labels",
            get(list_labels).post(create_label),
        )
        .route(
            "/api/v1/repos/:owner/:repo/labels/:id",
            patch(update_label).delete(delete_label),
        )
        .route(
            "/api/v1/repos/:owner/:repo/milestones",
            get(list_milestones).post(create_milestone),
        )
        .route(
            "/api/v1/repos/:owner/:repo/milestones/:id",
            get(get_milestone)
                .patch(update_milestone)
                .delete(delete_milestone),
        )
        .route(
            "/api/v1/repos/:owner/:repo/issues/comments/:id/reactions",
            post(add_reaction),
        )
        .route(
            "/api/v1/repos/:owner/:repo/issues/:index/assignees",
            post(add_issue_assignee),
        )
        .route(
            "/api/v1/repos/:owner/:repo/issues/:index/assignees/:username",
            delete(remove_issue_assignee),
        )
        .route("/api/v1/search/issues", get(search_issues_global))
        .route(
            "/api/v1/repos/:owner/:repo/milestones/:id/stats",
            get(get_milestone_stats),
        )
        .route(
            "/api/v1/repos/:owner/:repo/issues/:index/labels",
            post(add_issue_label),
        )
        .route(
            "/api/v1/repos/:owner/:repo/issues/:index/labels/:id",
            delete(remove_issue_label),
        )
}

/// Pull requests, reviews and merges.
fn pull_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/api/v1/repos/:owner/:repo/pulls",
            get(list_pulls).post(create_pull),
        )
        .route(
            "/api/v1/repos/:owner/:repo/pulls/:index",
            patch(update_pull),
        )
        .route(
            "/api/v1/repos/:owner/:repo/pulls/:index/reviews",
            get(list_reviews).post(create_review),
        )
        .route("/api/v1/user/pulls", get(list_user_pulls))
        .route(
            "/api/v1/repos/:owner/:repo/pulls/:index/merge",
            post(merge_pull),
        )
        .route(
            "/api/v1/repos/:owner/:repo/pulls/:index/requested_reviewers",
            post(request_review),
        )
        .route(
            "/api/v1/repos/:owner/:repo/pulls/:index/files",
            get(get_pr_files),
        )
}

/// Releases and release assets.
fn release_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/api/v1/repos/:owner/:repo/releases",
            get(list_releases).post(create_release),
        )
        .route(
            "/api/v1/repos/:owner/:repo/releases/:id",
            get(get_release)
                .patch(update_release)
                .delete(delete_release),
        )
        .route(
            "/api/v1/repos/:owner/:repo/releases/:id/assets",
            post(upload_release_asset),
        )
        .route(
            "/api/v1/repos/:owner/:repo/releases/:id/assets/:asset_id",
            get(download_release_asset),
        )
}

/// Actions workflows and runs.
fn action_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/api/v1/repos/:owner/:repo/actions/workflows",
            get(list_workflows),
        )
        .route(
            "/api/v1/repos/:owner/:repo/actions/workflows/:id/runs",
            get(list_workflow_runs).post(trigger_workflow),
        )
        .route(
            "/api/v1/repos/:owner/:repo/actions/runs/:run_id",
            patch(update_workflow_run).delete(delete_workflow_run),
        )
        .route(
            "/api/v1/repos/:owner/:repo/actions/runs/:run_id/logs",
            get(get_workflow_run_logs),
        )
        .route(
            "/api/v1/repos/:owner/:repo/actions/runs/:run_id/rerun",
            post(rerun_workflow_run),
        )
}

/// Package registry endpoints.
fn package_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/api/v1/packages/:owner",
            get(list_packages).post(upload_package),
        )
        .route(
            "/api/v1/packages/:owner/:type/:name/:version",
            get(get_package_detail).delete(delete_package),
        )
}

/// Projects, columns and cards.
fn project_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/api/v1/repos/:owner/:repo/projects",
            get(list_projects).post(create_project),
        )
        .route("/api/v1/repos/:owner/:repo/projects/:id", get(get_project))
        .route(
            "/api/v1/repos/:owner/:repo/projects/:id/close",
            post(close_project),
        )
        .route(
            "/api/v1/repos/:owner/:repo/projects/:id/reopen",
            post(reopen_project),
        )
        .route(
            "/api/v1/repos/:owner/:repo/projects/:id/columns",
            get(list_project_columns).post(create_project_column),
        )
        .route(
            "/api/v1/repos/:owner/:repo/projects/columns/:id/cards",
            get(list_project_cards).post(create_project_card),
        )
        .route(
            "/api/v1/repos/:owner/:repo/projects/cards/:id/move",
            post(move_project_card),
        )
}

/// Organizations, teams, members and audit logs.
fn org_routes() -> Router<AppState> {
    Router::new()
        .route("/api/v1/orgs", post(create_org))
        .route("/api/v1/orgs/:org", get(get_org))
        .route("/api/v1/orgs/:org/repos", get(list_org_repos))
        .route("/api/v1/orgs/:org/teams", get(list_teams).post(create_team))
        .route("/api/v1/orgs/:org/members", get(list_org_members))
        .route(
            "/api/v1/orgs/:org/members/:username",
            post(add_org_member)
                .delete(remove_org_member)
                .put(update_org_member_role),
        )
        .route("/api/v1/orgs/:org/audit-logs", get(list_org_audit_logs))
}

/// Authentication, profile, keys, follows and settings.
fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/api/v1/users/:username", get(get_user))
        .route("/api/v1/users/login", post(login_user))
        .route("/api/v1/users/register", post(register_user))
        .route("/api/v1/user/starred", get(list_starred_repos))
        .route(
            "/api/v1/user/settings",
            get(get_user_settings).patch(update_user_settings),
        )
        .route("/api/v1/user/keys", get(list_keys).post(create_key))
        .route("/api/v1/user/feeds", get(list_feeds))
        .route("/api/v1/user/2fa", get(get_2fa).post(update_2fa))
        .route(
            "/api/v1/user/gpg_keys",
            get(list_gpg_keys).post(create_gpg_key),
        )
        .route("/api/v1/user/gpg_keys/:id", delete(delete_gpg_key))
        .route("/api/v1/user/oauth2", get(list_oauth2_providers))
        .route("/api/v1/users/:username/followers", get(list_followers))
        .route("/api/v1/users/:username/following", get(list_following))
        .route(
            "/api/v1/users/:username/follow",
            post(follow_user).delete(unfollow_user),
        )
        .route("/api/v1/users/:username/heatmap", get(get_user_heatmap))
        .route("/api/v1/user/emails", get(list_emails))
        .route(
            "/api/v1/user/applications/oauth2",
            get(list_oauth2_apps).post(create_oauth2_app),
        )
        .route(
            "/api/v1/user/applications/oauth2/:id",
            delete(delete_oauth2_app),
        )
        .route("/api/v1/user/keys/:id", delete(delete_ssh_key))
        .route("/api/v1/user/gpg_keys/:id/verify", post(verify_gpg_key))
}

/// Instance administration.
fn admin_routes() -> Router<AppState> {
    Router::new()
        .route("/api/v1/admin/stats", get(get_admin_stats))
        .route("/api/v1/admin/notices", get(list_notices))
        .route(
            "/api/v1/admin/users",
            get(admin_list_users).post(admin_create_user),
        )
        .route(
            "/api/v1/admin/users/:username",
            post(admin_edit_user).delete(admin_delete_user),
        )
}

/// Search, notifications and template helpers.
fn misc_routes() -> Router<AppState> {
    Router::new()
        .route("/api/v1/notifications", get(list_notifications))
        .route("/api/v1/licenses", get(list_licenses))
        .route("/api/v1/gitignore/templates", get(list_gitignores))
        .route(
            "/api/v1/notifications/threads/:id",
            patch(mark_notification_read),
        )
}

/// Build the complete application router with state attached.
pub fn build_router(state: AppState) -> Router {
    Router::new()
        .merge(repo_routes())
        .merge(issue_routes())
        .merge(pull_routes())
        .merge(release_routes())
        .merge(action_routes())
        .merge(package_routes())
        .merge(project_routes())
        .merge(org_routes())
        .merge(user_routes())
        .merge(admin_routes())
        .merge(misc_routes())
        .layer(CorsLayer::permissive())
        .with_state(state)
}

/// Router backed by a fresh demo state. Test-only convenience used by the
/// integration test suite.
#[cfg(test)]
pub fn api_router() -> Router {
    build_router(crate::seed::demo_state())
}
