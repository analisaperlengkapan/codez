//! Runtime configuration, sourced from environment variables.
//!
//! Every value has a sensible default so the demo works with `cargo run -p backend`
//! and no extra setup.

use std::net::{IpAddr, Ipv4Addr};

/// Absolute path to the compiled frontend assets, anchored to the crate
/// manifest directory so it does not depend on the process working directory.
fn default_static_dir() -> String {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../frontend/dist")
        .to_string_lossy()
        .into_owned()
}

#[derive(Debug, Clone)]
pub struct Config {
    pub host: IpAddr,
    pub port: u16,
    /// Directory containing the compiled frontend assets.
    pub static_dir: String,
    /// Optional PostgreSQL connection string. When `None`, the server runs
    /// entirely in memory.
    pub database_url: Option<String>,
}

impl Config {
    pub fn from_env() -> Self {
        let host = std::env::var("CODEZA_HOST")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(IpAddr::V4(Ipv4Addr::LOCALHOST));

        let port = std::env::var("CODEZA_PORT")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(3000);

        // Resolve the default against the crate manifest directory rather than
        // the process working directory, so running the binary from anywhere
        // finds the built assets without an explicit `CODEZA_STATIC_DIR`.
        let static_dir =
            std::env::var("CODEZA_STATIC_DIR").unwrap_or_else(|_| default_static_dir());

        Self {
            host,
            port,
            static_dir,
            database_url: std::env::var("DATABASE_URL").ok(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::default_static_dir;

    #[test]
    fn default_static_dir_is_absolute_and_manifest_relative() {
        let dir = default_static_dir();
        assert!(
            std::path::Path::new(&dir).is_absolute(),
            "default static dir must not depend on the launch directory: {dir}"
        );
        assert!(
            dir.ends_with("frontend/dist"),
            "unexpected static dir: {dir}"
        );
    }
}
