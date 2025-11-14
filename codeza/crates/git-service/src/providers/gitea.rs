//! Gitea provider implementation

use crate::provider::{GitProvider, ProviderType, RemoteRepository, RemoteUser, RemoteOrganization};
use async_trait::async_trait;
use reqwest::Client;

/// Gitea provider
pub struct GiteaProvider {
    base_url: String,
    access_token: String,
    client: Client,
}

impl GiteaProvider {
    /// Create new Gitea provider
    pub fn new(base_url: String, access_token: String) -> Self {
        Self {
            base_url,
            access_token,
            client: Client::new(),
        }
    }
}

#[async_trait]
impl GitProvider for GiteaProvider {
    fn provider_type(&self) -> ProviderType {
        ProviderType::Gitea
    }

    async fn create_repository(
        &self,
        _owner: &str,
        name: &str,
        description: Option<String>,
        private: bool,
    ) -> Result<RemoteRepository, String> {
        let url = format!("{}/api/v1/user/repos", self.base_url);

        let body = serde_json::json!({
            "name": name,
            "description": description,
            "private": private,
            "auto_init": true,
        });

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("token {}", self.access_token))
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("Failed to create repository: {}", e))?;

        if response.status().is_success() {
            response
                .json::<RemoteRepository>()
                .await
                .map_err(|e| format!("Failed to parse response: {}", e))
        } else {
            Err(format!("Failed to create repository: {}", response.status()))
        }
    }

    async fn get_repository(&self, owner: &str, repo: &str) -> Result<RemoteRepository, String> {
        let url = format!("{}/api/v1/repos/{}/{}", self.base_url, owner, repo);

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("token {}", self.access_token))
            .send()
            .await
            .map_err(|e| format!("Failed to get repository: {}", e))?;

        if response.status().is_success() {
            response
                .json::<RemoteRepository>()
                .await
                .map_err(|e| format!("Failed to parse response: {}", e))
        } else {
            Err(format!("Failed to get repository: {}", response.status()))
        }
    }

    async fn list_repositories(&self, owner: &str) -> Result<Vec<RemoteRepository>, String> {
        let url = format!("{}/api/v1/users/{}/repos", self.base_url, owner);

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("token {}", self.access_token))
            .send()
            .await
            .map_err(|e| format!("Failed to list repositories: {}", e))?;

        if response.status().is_success() {
            response
                .json::<Vec<RemoteRepository>>()
                .await
                .map_err(|e| format!("Failed to parse response: {}", e))
        } else {
            Err(format!("Failed to list repositories: {}", response.status()))
        }
    }

    async fn delete_repository(&self, owner: &str, repo: &str) -> Result<(), String> {
        let url = format!("{}/api/v1/repos/{}/{}", self.base_url, owner, repo);

        let response = self
            .client
            .delete(&url)
            .header("Authorization", format!("token {}", self.access_token))
            .send()
            .await
            .map_err(|e| format!("Failed to delete repository: {}", e))?;

        if response.status().is_success() || response.status().as_u16() == 404 {
            Ok(())
        } else {
            Err(format!("Failed to delete repository: {}", response.status()))
        }
    }

    async fn get_user(&self, username: &str) -> Result<RemoteUser, String> {
        let url = format!("{}/api/v1/users/{}", self.base_url, username);

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("token {}", self.access_token))
            .send()
            .await
            .map_err(|e| format!("Failed to get user: {}", e))?;

        if response.status().is_success() {
            response
                .json::<RemoteUser>()
                .await
                .map_err(|e| format!("Failed to parse response: {}", e))
        } else {
            Err(format!("Failed to get user: {}", response.status()))
        }
    }

    async fn create_user(
        &self,
        username: &str,
        email: &str,
        password: &str,
        full_name: Option<String>,
    ) -> Result<RemoteUser, String> {
        let url = format!("{}/api/v1/admin/users", self.base_url);

        let body = serde_json::json!({
            "username": username,
            "email": email,
            "password": password,
            "full_name": full_name,
        });

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("token {}", self.access_token))
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("Failed to create user: {}", e))?;

        if response.status().is_success() {
            response
                .json::<RemoteUser>()
                .await
                .map_err(|e| format!("Failed to parse response: {}", e))
        } else {
            Err(format!("Failed to create user: {}", response.status()))
        }
    }

    async fn get_organization(&self, org: &str) -> Result<RemoteOrganization, String> {
        let url = format!("{}/api/v1/orgs/{}", self.base_url, org);

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("token {}", self.access_token))
            .send()
            .await
            .map_err(|e| format!("Failed to get organization: {}", e))?;

        if response.status().is_success() {
            response
                .json::<RemoteOrganization>()
                .await
                .map_err(|e| format!("Failed to parse response: {}", e))
        } else {
            Err(format!("Failed to get organization: {}", response.status()))
        }
    }

    async fn create_organization(
        &self,
        username: &str,
        full_name: Option<String>,
        description: Option<String>,
    ) -> Result<RemoteOrganization, String> {
        let url = format!("{}/api/v1/orgs", self.base_url);

        let body = serde_json::json!({
            "username": username,
            "full_name": full_name,
            "description": description,
        });

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("token {}", self.access_token))
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("Failed to create organization: {}", e))?;

        if response.status().is_success() {
            response
                .json::<RemoteOrganization>()
                .await
                .map_err(|e| format!("Failed to parse response: {}", e))
        } else {
            Err(format!("Failed to create organization: {}", response.status()))
        }
    }

    async fn get_file_contents(
        &self,
        owner: &str,
        repo: &str,
        path: &str,
        r#ref: &str,
    ) -> Result<String, String> {
        let url = format!(
            "{}/api/v1/repos/{}/{}/raw/{}?ref={}",
            self.base_url, owner, repo, path, r#ref,
        );

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("token {}", self.access_token))
            .send()
            .await
            .map_err(|e| format!("Failed to get file contents: {}", e))?;

        if response.status().is_success() {
            response
                .text()
                .await
                .map_err(|e| format!("Failed to read file contents: {}", e))
        } else {
            Err(format!(
                "Failed to get file contents: {}",
                response.status()
            ))
        }
    }
}
