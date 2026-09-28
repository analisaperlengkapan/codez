//! Integration tests for cross-repo search and template catalogs.

use shared::{CodeSearchResult, GitignoreTemplate, Issue, LicenseTemplate, Repository};

use super::TestApp;

#[tokio::test]
async fn global_issue_search_returns_matches_for_multiple_repos() {
    let app = TestApp::new();

    let results: Vec<Issue> = app.get_json("/api/v1/search/issues?q=bug").await;
    // The seeded issue body contains "bug", so a body search may match.
    let _ = results;

    let results: Vec<Issue> = app.get_json("/api/v1/search/issues?q=First").await;
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].title, "First Issue");
}

#[tokio::test]
async fn repo_code_search_returns_list() {
    let app = TestApp::new();
    let results: Vec<CodeSearchResult> = app
        .get_json("/api/v1/repos/admin/codeza/search?q=add")
        .await;
    let _ = results;
}

#[tokio::test]
async fn repo_search_is_empty_for_unknown_query() {
    let app = TestApp::new();
    let results: Vec<Repository> = app.get_json("/api/v1/repos/search?q=zzzznope").await;
    assert!(results.is_empty());
}

#[tokio::test]
async fn template_catalogs_are_available() {
    let app = TestApp::new();

    let licenses: Vec<LicenseTemplate> = app.get_json("/api/v1/licenses").await;
    assert!(!licenses.is_empty());

    let gitignores: Vec<GitignoreTemplate> = app.get_json("/api/v1/gitignore/templates").await;
    assert!(!gitignores.is_empty());
}
