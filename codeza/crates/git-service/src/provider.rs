//! Git provider abstraction layer
//! Supports multiple Git providers (Gitea, GitLab, GitHub, etc.)

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// Git provider type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProviderType {
    Gitea,
    GitLab,
    GitHub,
}

/// Repository information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteRepository {
    pub id: u64,
    pub name: String,
    pub full_name: String,
    pub description: Option<String>,
    pub private: bool,
    pub html_url: String,
    pub clone_url: String,
    pub ssh_url: String,
    pub created_at: String,
    pub updated_at: String,
}

/// User information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteUser {
    pub id: u64,
    pub username: String,
    pub full_name: Option<String>,
    pub email: Option<String>,
    pub avatar_url: Option<String>,
    pub created_at: String,
}

/// Organization information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteOrganization {
    pub id: u64,
    pub username: String,
    pub full_name: Option<String>,
    pub avatar_url: Option<String>,
    pub description: Option<String>,
    pub created_at: String,
}

/// Git provider trait
#[async_trait]
pub trait GitProvider: Send + Sync {
    /// Get provider type
    fn provider_type(&self) -> ProviderType;

    /// Create a new repository
    async fn create_repository(
        &self,
        owner: &str,
        name: &str,
        description: Option<String>,
        private: bool,
    ) -> Result<RemoteRepository, String>;

    /// Get repository
    async fn get_repository(&self, owner: &str, repo: &str) -> Result<RemoteRepository, String>;

    /// List repositories
    async fn list_repositories(&self, owner: &str) -> Result<Vec<RemoteRepository>, String>;

    /// Delete repository
    async fn delete_repository(&self, owner: &str, repo: &str) -> Result<(), String>;

    /// Get user
    async fn get_user(&self, username: &str) -> Result<RemoteUser, String>;

    /// Create user
    async fn create_user(
        &self,
        username: &str,
        email: &str,
        password: &str,
        full_name: Option<String>,
    ) -> Result<RemoteUser, String>;

    /// Get organization
    async fn get_organization(&self, org: &str) -> Result<RemoteOrganization, String>;

    /// Create organization
    async fn create_organization(
        &self,
        username: &str,
        full_name: Option<String>,
        description: Option<String>,
    ) -> Result<RemoteOrganization, String>;

    async fn get_file_contents(
        &self,
        owner: &str,
        repo: &str,
        path: &str,
        r#ref: &str,
    ) -> Result<String, String>;
}

/// Provider configuration
#[derive(Debug, Clone)]
pub struct ProviderConfig {
    pub provider_type: ProviderType,
    pub base_url: String,
    pub access_token: String,
}

impl ProviderConfig {
    /// Create new provider config
    pub fn new(provider_type: ProviderType, base_url: String, access_token: String) -> Self {
        Self {
            provider_type,
            base_url,
            access_token,
        }
    }
}
