//! Runtime configuration, sourced from environment variables.
//!
//! Every value has a sensible default so the demo works with `cargo run -p backend`
//! and no extra setup.

use std::net::{IpAddr, Ipv4Addr};

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

        let static_dir = std::env::var("CODEZA_STATIC_DIR")
            .unwrap_or_else(|_| "crates/frontend/dist".to_string());

        Self {
            host,
            port,
            static_dir,
            database_url: std::env::var("DATABASE_URL").ok(),
        }
    }
}
