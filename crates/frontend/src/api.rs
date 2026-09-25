//! Typed HTTP helpers for talking to the Codeza API.
//!
//! Every page fetches through these helpers instead of hand-rolling
//! `Request::get(..).send().await.unwrap().json()` chains. Read helpers are
//! infallible from the caller's point of view: a transport or decode failure
//! yields `T::default()` (or `None` / a supplied fallback), which keeps Leptos
//! resources simple and keeps the UI rendering an empty state rather than
//! panicking. Write helpers fire-and-forget; callers only follow them with a
//! refresh, so the response body is deliberately dropped.
//!
//! Paths passed to these helpers are absolute API paths (`/api/v1/repos/owner/name`),
//! not API-relative ones; the helpers do not rewrite the URL. Build paths with
//! [`api_url`] (or format them explicitly) so the versioned base stays in one
//! place rather than being hand-written at every call site.

use gloo_net::http::Request;
use serde::{de::DeserializeOwned, Serialize};

pub const BASE_URL: &str = "/api/v1";

/// Build an absolute API URL from a path relative to `/api/v1`.
pub fn api_url(path: &str) -> String {
    format!("{BASE_URL}{path}")
}

/// Percent-encode a string for use as a query-parameter value (RFC 3986
/// unreserved set). Keeps spaces, `&`, `#`, and other reserved characters from
/// breaking out of the query string.
pub fn encode_query(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for byte in value.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(byte as char)
            }
            _ => out.push_str(&format!("%{byte:02X}")),
        }
    }
    out
}

/// `GET url`, decoding `T`. Falls back to `T::default()` on error.
pub async fn get<T: DeserializeOwned + Default>(url: &str) -> T {
    match Request::get(url).send().await {
        Ok(resp) => resp.json::<T>().await.unwrap_or_default(),
        Err(_) => T::default(),
    }
}

/// `GET url`, decoding `T`. Returns `None` on error.
pub async fn get_opt<T: DeserializeOwned>(url: &str) -> Option<T> {
    match Request::get(url).send().await {
        Ok(resp) => resp.json::<T>().await.ok(),
        Err(_) => None,
    }
}

/// `GET url`, decoding `T`. Falls back to `fallback` on error.
pub async fn get_or<T: DeserializeOwned>(url: &str, fallback: T) -> T {
    match Request::get(url).send().await {
        Ok(resp) => resp.json::<T>().await.unwrap_or(fallback),
        Err(_) => fallback,
    }
}

/// `GET url` as raw text. Falls back to an empty string on error.
pub async fn get_text(url: &str) -> String {
    match Request::get(url).send().await {
        Ok(resp) => resp.text().await.unwrap_or_default(),
        Err(_) => String::new(),
    }
}

/// `GET url`, invoking `on_ok` with the decoded value when the request succeeds
/// and the body decodes. Used by screens that populate signals instead of
/// returning a value (e.g. form pre-fill).
pub async fn get_then<T, F>(url: &str, on_ok: F)
where
    T: DeserializeOwned,
    F: FnOnce(T),
{
    if let Ok(resp) = Request::get(url).send().await {
        if let Ok(value) = resp.json::<T>().await {
            on_ok(value);
        }
    }
}

/// `POST url` with no body. The response is discarded.
pub async fn post(url: &str) {
    let _ = Request::post(url).send().await;
}

/// `PATCH url` with no body. The response is discarded.
pub async fn patch(url: &str) {
    let _ = Request::patch(url).send().await;
}

/// `PUT url` with no body. The response is discarded.
pub async fn put(url: &str) {
    let _ = Request::put(url).send().await;
}

/// `DELETE url`. The response is discarded.
pub async fn delete(url: &str) {
    let _ = Request::delete(url).send().await;
}

/// `POST url` with a JSON body. The response is discarded.
pub async fn post_json<B: Serialize + ?Sized>(url: &str, body: &B) {
    if let Ok(req) = Request::post(url).json(body) {
        let _ = req.send().await;
    }
}

/// `PATCH url` with a JSON body. The response is discarded.
pub async fn patch_json<B: Serialize + ?Sized>(url: &str, body: &B) {
    if let Ok(req) = Request::patch(url).json(body) {
        let _ = req.send().await;
    }
}

/// `PUT url` with a JSON body. The response is discarded.
pub async fn put_json<B: Serialize + ?Sized>(url: &str, body: &B) {
    if let Ok(req) = Request::put(url).json(body) {
        let _ = req.send().await;
    }
}

/// `POST url` with a JSON body, returning whether the server replied `2xx`.
pub async fn post_json_ok<B: Serialize + ?Sized>(url: &str, body: &B) -> bool {
    match Request::post(url).json(body) {
        Ok(req) => req.send().await.map(|r| r.ok()).unwrap_or(false),
        Err(_) => false,
    }
}

/// `POST url` with a JSON body, decoding the response into `T`. `None` when the
/// request fails *or* the server replies with a non-2xx status — a rejected
/// write must not be mistaken for success, since error responses carry a
/// placeholder body that decodes fine.
pub async fn post_json_resp<B, T>(url: &str, body: &B) -> Option<T>
where
    B: Serialize + ?Sized,
    T: DeserializeOwned,
{
    let req = Request::post(url).json(body).ok()?;
    let resp = req.send().await.ok()?;
    if !resp.ok() {
        return None;
    }
    resp.json::<T>().await.ok()
}
