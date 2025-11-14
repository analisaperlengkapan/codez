//! Configuration management for Codeza Platform

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
    pub jwt: JwtConfig,
    pub git: GitConfig,
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
        // For now, use default_dev
        // Full implementation will use config crate
        Ok(Self::default_dev())
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
        }
    }
}
