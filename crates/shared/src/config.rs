//! Configuration management for Codeza Platform

use config::{Config as ConfigLoader, Environment, File};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
    pub jwt: JwtConfig,
    pub git: GitConfig,
    #[serde(default)]
    pub cors_origins: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub env: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedisConfig {
    pub url: String,
    pub db: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtConfig {
    pub secret: String,
    pub expiration_hours: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitConfig {
    pub provider: String,
    pub base_url: String,
    pub access_token: String,
    pub webhook_secret: String,
}

impl Config {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let run_mode = std::env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let s = ConfigLoader::builder()
            // Start with default values
            .set_default("server.host", "127.0.0.1")?
            .set_default("server.port", 3000)?
            .set_default("server.env", "development")?
            .set_default(
                "database.url",
                "postgres://codeza:codeza@localhost:5432/codeza_dev",
            )?
            .set_default("database.max_connections", 10)?
            .set_default("database.min_connections", 2)?
            .set_default("redis.url", "redis://localhost:6379")?
            .set_default("redis.db", 0)?
            .set_default("jwt.secret", "dev-secret-key-change-in-production")?
            .set_default("jwt.expiration_hours", 24)?
            .set_default("git.provider", "gitea")?
            .set_default("git.base_url", "http://localhost:3001")?
            .set_default("git.access_token", "dev-git-token-change-in-production")?
            .set_default(
                "git.webhook_secret",
                "dev-git-webhook-secret-change-in-production",
            )?
            .set_default("cors_origins", Vec::<String>::new())?
            // Add configuration from files (optional)
            .add_source(File::with_name("config/default").required(false))
            .add_source(File::with_name(&format!("config/{}", run_mode)).required(false))
            // Add Environment Variables
            // e.g., APP_SERVER__PORT=8080 maps to server.port
            .add_source(Environment::with_prefix("APP").separator("__"))
            .build()?;

        s.try_deserialize().map_err(|e| e.into())
    }

    pub fn default_dev() -> Self {
        Self {
            server: ServerConfig {
                host: "127.0.0.1".to_string(),
                port: 3000,
                env: "development".to_string(),
            },
            database: DatabaseConfig {
                url: "postgres://codeza:codeza@localhost:5432/codeza_dev".to_string(),
                max_connections: 10,
                min_connections: 2,
            },
            redis: RedisConfig {
                url: "redis://localhost:6379".to_string(),
                db: 0,
            },
            jwt: JwtConfig {
                secret: "dev-secret-key-change-in-production".to_string(),
                expiration_hours: 24,
            },
            git: GitConfig {
                provider: "gitea".to_string(),
                base_url: "http://localhost:3001".to_string(),
                access_token: "dev-git-token-change-in-production".to_string(),
                webhook_secret: "dev-git-webhook-secret-change-in-production".to_string(),
            },
            cors_origins: vec!["*".to_string()],
        }
    }
}
