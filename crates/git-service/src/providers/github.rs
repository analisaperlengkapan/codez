use crate::provider::{
    GitProvider, ProviderType, RemoteOrganization, RemoteRepository, RemoteUser,
};
use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;

pub struct GitHubProvider {
    base_url: String, // e.g., "https://api.github.com"
    access_token: String,
    client: Client,
}

impl GitHubProvider {
    pub fn new(base_url: String, access_token: String) -> Self {
        // GitHub API defaults to https://api.github.com if not provided or empty
        let base_url = if base_url.is_empty() {
            "https://api.github.com".to_string()
        } else {
            base_url
        };

        Self {
            base_url,
            access_token,
            client: Client::builder()
                .user_agent("codeza-git-service")
                .build()
                .unwrap_or_default(),
        }
    }

    fn auth(&self, builder: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        builder
            .header("Authorization", format!("token {}", self.access_token))
            .header("Accept", "application/vnd.github.v3+json")
    }
}

#[derive(Debug, Deserialize)]
struct GitHubRepository {
    id: u64,
    name: String,
    full_name: String,
    description: Option<String>,
    private: bool,
    html_url: String,
    clone_url: String,
    ssh_url: String,
    created_at: String,
    updated_at: String,
}

#[derive(Debug, Deserialize)]
struct GitHubUser {
    id: u64,
    login: String,
    name: Option<String>,
    email: Option<String>,
    avatar_url: Option<String>,
    created_at: Option<String>, // Can be null in some partial responses
}

#[derive(Debug, Deserialize)]
struct GitHubOrg {
    id: u64,
    login: String,
    name: Option<String>,
    avatar_url: Option<String>,
    description: Option<String>,
    created_at: Option<String>,
}

fn map_repository(repo: GitHubRepository) -> RemoteRepository {
    RemoteRepository {
        id: repo.id,
        name: repo.name,
        full_name: repo.full_name,
        description: repo.description,
        private: repo.private,
        html_url: repo.html_url,
        clone_url: repo.clone_url,
        ssh_url: repo.ssh_url,
        created_at: repo.created_at,
        updated_at: repo.updated_at,
    }
}

fn map_user(user: GitHubUser) -> RemoteUser {
    RemoteUser {
        id: user.id,
        username: user.login,
        full_name: user.name,
        email: user.email,
        avatar_url: user.avatar_url,
        created_at: user.created_at.unwrap_or_default(),
    }
}

fn map_org(org: GitHubOrg) -> RemoteOrganization {
    RemoteOrganization {
        id: org.id,
        username: org.login,
        full_name: org.name,
        avatar_url: org.avatar_url,
        description: org.description,
        created_at: org.created_at.unwrap_or_default(),
    }
}

#[async_trait]
impl GitProvider for GitHubProvider {
    fn provider_type(&self) -> ProviderType {
        ProviderType::GitHub
    }

    async fn create_repository(
        &self,
        owner: &str, // In GitHub context, if owner != authenticated user, it implies an org
        name: &str,
        description: Option<String>,
        private: bool,
    ) -> Result<RemoteRepository, String> {
        // First, determine if we are creating in a user account or an organization
        // For simplicity, we'll try to get the authenticated user.
        // If 'owner' matches authenticated user, use /user/repos.
        // Else use /orgs/{owner}/repos.

        let user = self.get_user("").await.unwrap_or_else(|_| RemoteUser {
            id: 0,
            username: String::new(),
            full_name: None,
            email: None,
            avatar_url: None,
            created_at: String::new(),
        });

        let url = if owner.is_empty() || owner == user.username {
            format!("{}/user/repos", self.base_url)
        } else {
            format!("{}/orgs/{}/repos", self.base_url, owner)
        };

        let body = serde_json::json!({
            "name": name,
            "description": description,
            "private": private,
            "auto_init": true, // Initialize with README to allow cloning
        });

        let response = self
            .auth(self.client.post(&url).json(&body))
            .send()
            .await
            .map_err(|e| format!("Failed to create repository: {}", e))?;

        if response.status().is_success() {
            let repo = response
                .json::<GitHubRepository>()
                .await
                .map_err(|e| format!("Failed to parse response: {}", e))?;
            Ok(map_repository(repo))
        } else {
            let err_text = response.text().await.unwrap_or_default();
            Err(format!(
                "Failed to create repository: {}. Response: {}",
                "status code error", err_text
            ))
        }
    }

    async fn get_repository(&self, owner: &str, repo: &str) -> Result<RemoteRepository, String> {
        let url = format!("{}/repos/{}/{}", self.base_url, owner, repo);

        let response = self
            .auth(self.client.get(&url))
            .send()
            .await
            .map_err(|e| format!("Failed to get repository: {}", e))?;

        if response.status().is_success() {
            let repo = response
                .json::<GitHubRepository>()
                .await
                .map_err(|e| format!("Failed to parse response: {}", e))?;
            Ok(map_repository(repo))
        } else {
            Err(format!("Failed to get repository: {}", response.status()))
        }
    }

    async fn list_repositories(&self, owner: &str) -> Result<Vec<RemoteRepository>, String> {
        // If owner is empty, list authenticated user's repos.
        // If owner is provided, list that user/org's repos.
        let url = if owner.is_empty() {
            format!("{}/user/repos", self.base_url)
        } else {
            format!("{}/users/{}/repos", self.base_url, owner)
        };

        let response = self
            .auth(self.client.get(&url))
            .send()
            .await
            .map_err(|e| format!("Failed to list repositories: {}", e))?;

        if response.status().is_success() {
            let repos = response
                .json::<Vec<GitHubRepository>>()
                .await
                .map_err(|e| format!("Failed to parse response: {}", e))?;
            Ok(repos.into_iter().map(map_repository).collect())
        } else {
            Err(format!(
                "Failed to list repositories: {}",
                response.status()
            ))
        }
    }

    async fn delete_repository(&self, owner: &str, repo: &str) -> Result<(), String> {
        let url = format!("{}/repos/{}/{}", self.base_url, owner, repo);

        let response = self
            .auth(self.client.delete(&url))
            .send()
            .await
            .map_err(|e| format!("Failed to delete repository: {}", e))?;

        if response.status().is_success() || response.status().as_u16() == 404 {
            Ok(())
        } else {
            Err(format!(
                "Failed to delete repository: {}",
                response.status()
            ))
        }
    }

    async fn get_user(&self, username: &str) -> Result<RemoteUser, String> {
        let url = if username.is_empty() {
            format!("{}/user", self.base_url)
        } else {
            format!("{}/users/{}", self.base_url, username)
        };

        let response = self
            .auth(self.client.get(&url))
            .send()
            .await
            .map_err(|e| format!("Failed to get user: {}", e))?;

        if response.status().is_success() {
            let user = response
                .json::<GitHubUser>()
                .await
                .map_err(|e| format!("Failed to parse response: {}", e))?;
            Ok(map_user(user))
        } else {
            Err(format!("Failed to get user: {}", response.status()))
        }
    }

    async fn create_user(
        &self,
        _username: &str,
        _email: &str,
        _password: &str,
        _full_name: Option<String>,
    ) -> Result<RemoteUser, String> {
        Err("GitHub does not support creating users via API".to_string())
    }

    async fn get_organization(&self, org: &str) -> Result<RemoteOrganization, String> {
        let url = format!("{}/orgs/{}", self.base_url, org);

        let response = self
            .auth(self.client.get(&url))
            .send()
            .await
            .map_err(|e| format!("Failed to get organization: {}", e))?;

        if response.status().is_success() {
            let org_data = response
                .json::<GitHubOrg>()
                .await
                .map_err(|e| format!("Failed to parse response: {}", e))?;
            Ok(map_org(org_data))
        } else {
            Err(format!("Failed to get organization: {}", response.status()))
        }
    }

    async fn create_organization(
        &self,
        _username: &str,
        _full_name: Option<String>,
        _description: Option<String>,
    ) -> Result<RemoteOrganization, String> {
        Err("GitHub does not support creating organizations via API".to_string())
    }

    async fn get_file_contents(
        &self,
        owner: &str,
        repo: &str,
        path: &str,
        r#ref: &str,
    ) -> Result<String, String> {
        let url = format!(
            "{}/repos/{}/{}/contents/{}",
            self.base_url, owner, repo, path
        );

        let response = self
            .auth(self.client.get(&url).query(&[("ref", r#ref)]))
            .header("Accept", "application/vnd.github.v3.raw") // Request raw content
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
