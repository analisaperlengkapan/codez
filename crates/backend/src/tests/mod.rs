//! Backend integration tests.
//!
//! Each submodule exercises one API area end-to-end through the real Axum router
//! (`routes::api_router`) backed by a fresh demo state.
//!
//! To keep the flows readable, requests go through the small [`TestApp`] wrapper
//! below rather than hand-building `Request`s with `oneshot` in every test.

use axum::{
    body::Body,
    http::{Request, StatusCode},
    Router,
};
use serde::de::DeserializeOwned;
use tower::ServiceExt;

#[cfg(test)]
mod actions;
#[cfg(test)]
mod admin;
#[cfg(test)]
mod comments;
#[cfg(test)]
mod contents;
#[cfg(test)]
mod discussions;
#[cfg(test)]
mod issues;
#[cfg(test)]
mod labels;
#[cfg(test)]
mod milestones;
#[cfg(test)]
mod orgs;
#[cfg(test)]
mod packages;
#[cfg(test)]
mod projects;
#[cfg(test)]
mod pulls;
#[cfg(test)]
mod releases;
#[cfg(test)]
mod repos;
#[cfg(test)]
mod search;
#[cfg(test)]
mod settings;
#[cfg(test)]
mod users;
#[cfg(test)]
mod webhooks;
#[cfg(test)]
mod wiki;

/// Thin ergonomic wrapper over the demo router.
///
/// Each call clones the router (cheap — it is a `Send` service of `ServiceFn`s)
/// and dispatches one request, so tests read as a sequence of API calls without
/// repeating the `Request::builder().oneshot()` dance.
#[derive(Clone)]
pub struct TestApp {
    router: Router,
}

/// A response with its status and raw body already buffered.
pub struct Res {
    pub status: StatusCode,
    body: Vec<u8>,
}

impl Res {
    /// Decode the buffered body as JSON. Panics (with the status) on failure so a
    /// decode error points at the offending call rather than a line number.
    pub fn json<T: DeserializeOwned>(&self) -> T {
        serde_json::from_slice(&self.body).unwrap_or_else(|e| {
            panic!(
                "failed to decode {} response as JSON: {e}\nbody: {}",
                self.status,
                String::from_utf8_lossy(&self.body)
            )
        })
    }

    pub fn text(&self) -> String {
        String::from_utf8_lossy(&self.body).into_owned()
    }

    /// Assert a specific status, printing the body on mismatch.
    pub fn assert_status(&self, expected: StatusCode) -> &Self {
        assert_eq!(
            self.status,
            expected,
            "unexpected status; body: {}",
            self.text()
        );
        self
    }
}

impl TestApp {
    /// Fresh router backed by a fresh demo state.
    pub fn new() -> Self {
        Self {
            router: crate::routes::api_router(),
        }
    }

    async fn send(&self, req: Request<Body>) -> Res {
        let response = self
            .router
            .clone()
            .oneshot(req)
            .await
            .expect("router is infallible");
        let status = response.status();
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap()
            .to_vec();
        Res { status, body }
    }

    pub async fn get(&self, uri: &str) -> Res {
        self.send(
            Request::builder()
                .method("GET")
                .uri(uri)
                .body(Body::empty())
                .unwrap(),
        )
        .await
    }

    /// GET expecting `200 OK`. Returns the body so callers can decode it.
    pub async fn get_ok(&self, uri: &str) -> Res {
        let res = self.get(uri).await;
        res.assert_status(StatusCode::OK);
        res
    }

    pub async fn get_json<T: DeserializeOwned>(&self, uri: &str) -> T {
        self.get_ok(uri).await.json()
    }

    async fn send_json(&self, method: &str, uri: &str, body: serde_json::Value) -> Res {
        self.send(
            Request::builder()
                .method(method)
                .uri(uri)
                .header("Content-Type", "application/json")
                .body(Body::from(body.to_string()))
                .unwrap(),
        )
        .await
    }

    pub async fn post(&self, uri: &str) -> Res {
        self.send(
            Request::builder()
                .method("POST")
                .uri(uri)
                .body(Body::empty())
                .unwrap(),
        )
        .await
    }

    pub async fn post_json(&self, uri: &str, body: serde_json::Value) -> Res {
        self.send_json("POST", uri, body).await
    }

    /// POST a raw body (used by endpoints that take `Bytes`, e.g. asset upload).
    pub async fn post_bytes(&self, uri: &str, bytes: Vec<u8>) -> Res {
        self.send(
            Request::builder()
                .method("POST")
                .uri(uri)
                .header("Content-Type", "application/octet-stream")
                .body(Body::from(bytes))
                .unwrap(),
        )
        .await
    }

    /// POST a JSON body expecting `201 Created`, decoding the response.
    pub async fn post_created<T: DeserializeOwned>(&self, uri: &str, body: serde_json::Value) -> T {
        let res = self.post_json(uri, body).await;
        res.assert_status(StatusCode::CREATED);
        res.json()
    }

    pub async fn patch_json(&self, uri: &str, body: serde_json::Value) -> Res {
        self.send_json("PATCH", uri, body).await
    }

    pub async fn put_json(&self, uri: &str, body: serde_json::Value) -> Res {
        self.send_json("PUT", uri, body).await
    }

    pub async fn delete(&self, uri: &str) -> Res {
        self.send(
            Request::builder()
                .method("DELETE")
                .uri(uri)
                .body(Body::empty())
                .unwrap(),
        )
        .await
    }
}

impl Default for TestApp {
    fn default() -> Self {
        Self::new()
    }
}
