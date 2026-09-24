//! Integration tests for the webhooks endpoints.
use crate::routes::api_router;
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use shared::CreateIssueOption;
use tower::ServiceExt; // for `oneshot`

#[tokio::test]
async fn test_webhook_delivery_flow() {
    let app = api_router();

    // 1. Create Webhook
    let hook_payload = shared::CreateHookOption {
        url: "http://example.com/webhook".to_string(),
        events: vec!["issues".to_string()],
        active: true,
    };
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/repos/admin/codeza/hooks")
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_string(&hook_payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let hook: shared::Webhook = serde_json::from_slice(&body).unwrap();
    let hook_id = hook.id;

    // 2. Trigger Event (Create Issue)
    let issue_payload = CreateIssueOption {
        title: "Webhook Test Issue".to_string(),
        body: None,
        milestone: None,
    };
    let _ = app
        .clone()
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

    // 3. Verify Delivery
    let mut found = false;
    for _ in 0..50 {
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri(format!(
                        "/api/v1/repos/admin/codeza/hooks/{}/deliveries",
                        hook_id
                    ))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let deliveries: Vec<shared::WebhookDelivery> = serde_json::from_slice(&body).unwrap();

        // Should have 1 delivery for "issues" event. Status might be "failed" because example.com is unreachable.
        if deliveries.iter().any(|d| d.event == "issues") {
            found = true;
            break;
        }
    }
    assert!(found, "Webhook delivery should be recorded");
}
#[tokio::test]
async fn test_webhook_ssrf_prevention() {
    let app = api_router();

    // 1. Create Webhook with restricted URL (localhost)
    let hook_payload = shared::CreateHookOption {
        url: "http://127.0.0.1:3000/api/v1/test_hook_receiver".to_string(),
        events: vec!["issues".to_string()],
        active: true,
    };
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/repos/admin/codeza/hooks")
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_string(&hook_payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let hook: shared::Webhook = serde_json::from_slice(&body).unwrap();
    let hook_id = hook.id;

    // 2. Trigger Event (Create Issue)
    let issue_payload = CreateIssueOption {
        title: "SSRF Test Issue".to_string(),
        body: None,
        milestone: None,
    };
    let _ = app
        .clone()
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

    // 3. Verify Delivery was Blocked
    let mut blocked = false;
    for _ in 0..50 {
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri(format!(
                        "/api/v1/repos/admin/codeza/hooks/{}/deliveries",
                        hook_id
                    ))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let deliveries: Vec<shared::WebhookDelivery> = serde_json::from_slice(&body).unwrap();

        if let Some(d) = deliveries.iter().find(|d| d.event == "issues") {
            // Check that it was blocked (status contains "blocked")
            if d.status.contains("blocked") {
                blocked = true;
                break;
            }
        }
    }
    assert!(
        blocked,
        "Webhook delivery to localhost should be blocked by SSRF protection"
    );
}
