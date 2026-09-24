//! Codeza backend entry point.
//!
//! Serves the `/api/v1` JSON API and, when a frontend build is present, the
//! compiled WebAssembly single-page application from the configured static
//! directory (defaults to `crates/frontend/dist`).

use std::net::SocketAddr;
use std::path::{Component, Path, PathBuf};

use axum::http::{header, StatusCode, Uri};
use axum::response::{IntoResponse, Response};

mod config;
mod db;
mod handlers;
mod routes;
mod seed;
mod state;
#[cfg(test)]
mod tests;

/// Content type for the static asset extensions the frontend emits.
fn content_type(path: &str) -> &'static str {
    match path.rsplit('.').next().unwrap_or("") {
        "html" => "text/html; charset=utf-8",
        "js" => "text/javascript; charset=utf-8",
        "css" => "text/css; charset=utf-8",
        "wasm" => "application/wasm",
        "json" | "map" => "application/json",
        "svg" => "image/svg+xml",
        "png" => "image/png",
        "ico" => "image/x-icon",
        "txt" => "text/plain; charset=utf-8",
        _ => "application/octet-stream",
    }
}

/// Fallback for paths not matched by the API.
///
/// Serves the requested file from the static directory when it exists. The
/// frontend is a client-side-routed SPA, so a *navigation* path returns the
/// shell document with a `200` so the browser can resolve deep links.
///
/// Missing *asset* requests (paths under the known build-asset namespace, e.g.
/// `/*.js`, `/*.css`, `/*_bg.wasm`) return `404` rather than the shell: a
/// browser asking for an asset that does not exist must not receive `text/html`
/// with a `200`. Unknown `/api/*` paths also stay `404` so clients never mistake
/// HTML for JSON.
async fn spa_fallback(static_dir: String, uri: Uri) -> Response {
    if uri.path().starts_with("/api/") {
        return (StatusCode::NOT_FOUND, "Not Found").into_response();
    }

    let root = PathBuf::from(&static_dir);
    if let Some(file) = resolve_static_file(&root, uri.path()) {
        if let Some(bytes) = read_within_root(&root, &file).await {
            return (
                [(header::CONTENT_TYPE, content_type(&file.to_string_lossy()))],
                bytes,
            )
                .into_response();
        }
        if is_asset_request(uri.path()) {
            return (StatusCode::NOT_FOUND, "Not Found").into_response();
        }
    }

    match read_within_root(&root, &root.join("index.html")).await {
        Some(index) => {
            ([(header::CONTENT_TYPE, "text/html; charset=utf-8")], index).into_response()
        }
        None => (
            StatusCode::SERVICE_UNAVAILABLE,
            "Frontend assets are not built. Run `trunk build` in crates/frontend.",
        )
            .into_response(),
    }
}

/// Read `candidate`, but only when its canonical destination stays inside
/// `root`.
///
/// Normalizing the request path is not enough on its own: a symlink *inside*
/// the asset tree can point at a file outside it, so the resolved filesystem
/// destination must be checked too - CWE-22 (path traversal). A candidate that
/// is absent, or whose real path escapes `root`, yields `None`.
async fn read_within_root(root: &Path, candidate: &Path) -> Option<Vec<u8>> {
    let canonical_root = tokio::fs::canonicalize(root).await.ok()?;
    let canonical = tokio::fs::canonicalize(candidate).await.ok()?;
    if !canonical.starts_with(&canonical_root) {
        return None;
    }
    tokio::fs::read(&canonical).await.ok()
}

/// Resolve a request path to a file under `root`, or `None` if the path escapes
/// the static root.
///
/// The path is normalized component-by-component (`..`/`.` are resolved without
/// touching the filesystem), so a request like `/../Cargo.toml` can never read
/// outside `root` — CWE-22 (path traversal).
fn resolve_static_file(root: &Path, request_path: &str) -> Option<PathBuf> {
    let mut resolved = PathBuf::new();
    for component in Path::new(request_path.trim_start_matches('/')).components() {
        match component {
            Component::Normal(segment) => resolved.push(segment),
            Component::CurDir => {}
            // A `..` that would climb above the root is rejected outright; one
            // that stays inside is harmless but never legitimately requested.
            Component::ParentDir | Component::RootDir | Component::Prefix(_) => return None,
        }
    }
    if resolved.as_os_str().is_empty() {
        return None;
    }
    Some(root.join(resolved))
}

/// Whether a request targets a missing build asset rather than an SPA route.
///
/// Asset classification keys off the *first* path segment only. Build assets
/// live at the root (`/frontend-<hash>.js`, `/style-<hash>.css`) whereas SPA
/// deep links begin with a route segment (`/repos/...`), so a repository named
/// `library.js` still resolves as a navigation and never gets a spurious `404`.
fn is_asset_request(path: &str) -> bool {
    let mut segments = path.trim_start_matches('/').split('/');
    let Some(first) = segments.next() else {
        return false;
    };
    // Only a bare asset at the root counts; a deeper path is a client route.
    if segments.next().is_some() {
        return false;
    }
    is_asset_filename(first)
}

/// Whether a single (top-level) filename carries a known build-asset extension.
fn is_asset_filename(name: &str) -> bool {
    matches!(
        name.rsplit('.').next().unwrap_or(""),
        "js" | "css" | "wasm" | "map" | "json" | "svg" | "png" | "ico" | "txt"
    )
}

#[tokio::main]
async fn main() {
    let config = config::Config::from_env();
    let state = seed::demo_state();
    let api_router = routes::build_router(state.clone());

    let fallback_dir = config.static_dir.clone();
    let app = api_router.fallback(move |uri: Uri| spa_fallback(fallback_dir.clone(), uri));

    if config.database_url.is_some() {
        tokio::spawn(async move {
            db::init_postgres_db(&state).await;
        });
    }

    let addr = SocketAddr::from((config.host, config.port));
    println!("Codeza listening on http://{addr}");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[cfg(test)]
mod static_asset_tests {
    use super::{is_asset_request, read_within_root, resolve_static_file};
    use std::path::{Path, PathBuf};

    /// Create a unique scratch directory for filesystem-backed tests.
    fn scratch_dir(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("codeza-{}-{}", name, std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn top_level_assets_are_assets() {
        assert!(is_asset_request("frontend-abc123.js"));
        assert!(is_asset_request("/frontend-abc123.js"));
        assert!(is_asset_request("style-abc.css"));
        assert!(is_asset_request("frontend-abc_bg.wasm"));
        assert!(is_asset_request("favicon.ico"));
    }

    #[test]
    fn spa_routes_are_not_assets() {
        assert!(!is_asset_request("repos/admin/codeza"));
        assert!(!is_asset_request("search"));
        // A dotted repository name is a route, not a missing asset (finding 2).
        assert!(!is_asset_request("repos/admin/library.js"));
        assert!(!is_asset_request("/repos/admin/library.js"));
        assert!(!is_asset_request("users/admin.json"));
    }

    #[test]
    fn traversal_paths_are_rejected() {
        let root = PathBuf::from("/srv/dist");
        assert_eq!(
            resolve_static_file(&root, "/index.html"),
            Some(PathBuf::from("/srv/dist/index.html"))
        );
        assert_eq!(
            resolve_static_file(&root, "/sub/./x.js"),
            Some(PathBuf::from("/srv/dist/sub/x.js"))
        );
        for evil in [
            "/../Cargo.toml",
            "/sub/../../etc/passwd",
            "/..",
            "/a/../../b",
        ] {
            assert_eq!(
                resolve_static_file(&root, evil),
                None,
                "{evil} must not resolve outside the static root"
            );
        }
        assert_eq!(resolve_static_file(Path::new("/srv/dist"), "/"), None);
    }

    #[tokio::test]
    async fn regular_files_inside_the_root_are_served() {
        let root = scratch_dir("static-ok");
        std::fs::write(root.join("index.html"), b"shell").unwrap();
        std::fs::write(root.join("app.js"), b"asset").unwrap();

        assert_eq!(
            read_within_root(&root, &root.join("app.js")).await,
            Some(b"asset".to_vec())
        );
        assert_eq!(
            read_within_root(&root, &root.join("index.html")).await,
            Some(b"shell".to_vec())
        );
        assert_eq!(
            read_within_root(&root, &root.join("missing.js")).await,
            None
        );
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn symlinks_pointing_outside_the_root_are_not_served() {
        let root = scratch_dir("static-symlink-root");
        let outside = scratch_dir("static-symlink-outside");
        std::fs::write(outside.join("secret.txt"), b"top secret").unwrap();
        std::fs::write(root.join("app.js"), b"asset").unwrap();

        // A file symlink inside the asset tree aimed at an outside file.
        std::os::unix::fs::symlink(outside.join("secret.txt"), root.join("escape.txt")).unwrap();
        // A directory symlink escaping the root, reachable via a deeper path.
        std::os::unix::fs::symlink(&outside, root.join("escape-dir")).unwrap();

        assert_eq!(
            read_within_root(&root, &root.join("escape.txt")).await,
            None
        );
        assert_eq!(
            read_within_root(&root, &root.join("escape-dir").join("secret.txt")).await,
            None
        );
        // The legitimate asset still resolves.
        assert_eq!(
            read_within_root(&root, &root.join("app.js")).await,
            Some(b"asset".to_vec())
        );
    }
}
