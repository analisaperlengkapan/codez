//! Integration tests for the repository wiki.

use axum::http::StatusCode;
use shared::WikiPage;

use super::TestApp;

#[tokio::test]
async fn list_and_get_seeded_wiki_pages() {
    let app = TestApp::new();

    let pages: Vec<WikiPage> = app.get_json("/api/v1/repos/admin/codeza/wiki/pages").await;
    assert_eq!(pages.len(), 2);

    let page: Option<WikiPage> = app
        .get_json("/api/v1/repos/admin/codeza/wiki/pages/Home")
        .await;
    assert_eq!(page.unwrap().content, "Welcome to the wiki!");
}

#[tokio::test]
async fn unknown_wiki_page_is_null() {
    let app = TestApp::new();
    let page: Option<WikiPage> = app
        .get_json("/api/v1/repos/admin/codeza/wiki/pages/DoesNotExist")
        .await;
    assert!(page.is_none());
}

#[tokio::test]
async fn wiki_page_can_be_created_and_updated() {
    let app = TestApp::new();

    let page: WikiPage = app
        .post_created(
            "/api/v1/repos/admin/codeza/wiki/pages",
            serde_json::json!({
                "title": "FAQ",
                "content": "Q&A",
                "message": "add faq"
            }),
        )
        .await;
    assert_eq!(page.title, "FAQ");

    app.put_json(
        "/api/v1/repos/admin/codeza/wiki/pages/FAQ",
        serde_json::json!({
            "title": "FAQ",
            "content": "Updated Q&A",
            "message": "edit faq"
        }),
    )
    .await
    .assert_status(StatusCode::OK);

    let page: Option<WikiPage> = app
        .get_json("/api/v1/repos/admin/codeza/wiki/pages/FAQ")
        .await;
    assert_eq!(page.unwrap().content, "Updated Q&A");
}

#[tokio::test]
async fn update_unknown_wiki_page_is_not_found() {
    let app = TestApp::new();
    app.put_json(
        "/api/v1/repos/admin/codeza/wiki/pages/Ghost",
        serde_json::json!({ "title": "Ghost", "content": "x" }),
    )
    .await
    .assert_status(StatusCode::NOT_FOUND);
}
