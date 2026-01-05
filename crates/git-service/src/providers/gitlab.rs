use crate::provider::{
    GitProvider, ProviderType, RemoteOrganization, RemoteRepository, RemoteUser,
};
use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;

pub struct GitLabProvider {
    base_url: String,
    access_token: String,
    client: Client,
}

impl GitLabProvider {
    pub fn new(base_url: String, access_token: String) -> Self {
        Self {
            base_url,
            access_token,
            client: Client::new(),
        }
    }

    fn auth(&self, builder: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        builder.header("PRIVATE-TOKEN", &self.access_token)
    }
}

#[derive(Debug, Deserialize)]
struct GitLabProject {
    id: u64,
    name: String,
    path_with_namespace: String,
    description: Option<String>,
    visibility: String,
    web_url: String,
    http_url_to_repo: String,
    ssh_url_to_repo: String,
    created_at: Option<String>,
    last_activity_at: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GitLabUser {
    id: u64,
    username: String,
    name: Option<String>,
    email: Option<String>,
    avatar_url: Option<String>,
    created_at: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GitLabGroup {
    id: u64,
    path: String,
    name: Option<String>,
    avatar_url: Option<String>,
    description: Option<String>,
    created_at: Option<String>,
}

fn map_project(project: GitLabProject) -> RemoteRepository {
    let created = project.created_at.unwrap_or_default();
    let updated = project.last_activity_at.unwrap_or_else(|| created.clone());

    RemoteRepository {
        id: project.id,
        name: project.name,
        full_name: project.path_with_namespace,
        description: project.description,
        private: project.visibility != "public",
        html_url: project.web_url,
        clone_url: project.http_url_to_repo,
        ssh_url: project.ssh_url_to_repo,
        created_at: created,
        updated_at: updated,
    }
}

fn map_user(user: GitLabUser) -> RemoteUser {
    let created = user.created_at.unwrap_or_default();

    RemoteUser {
        id: user.id,
        username: user.username,
        full_name: user.name,
        email: user.email,
        avatar_url: user.avatar_url,
        created_at: created,
    }
}

fn map_group(group: GitLabGroup) -> RemoteOrganization {
    let created = group.created_at.unwrap_or_default();

    RemoteOrganization {
        id: group.id,
        username: group.path,
        full_name: group.name,
        avatar_url: group.avatar_url,
        description: group.description,
        created_at: created,
    }
}

#[async_trait]
impl GitProvider for GitLabProvider {
    fn provider_type(&self) -> ProviderType {
        ProviderType::GitLab
    }

    async fn create_repository(
        &self,
        _owner: &str,
        name: &str,
        description: Option<String>,
        private: bool,
    ) -> Result<RemoteRepository, String> {
        let url = format!("{}/api/v4/projects", self.base_url);

        let visibility = if private { "private" } else { "public" };

        let body = serde_json::json!({
            "name": name,
            "description": description,
            "visibility": visibility,
            "initialize_with_readme": true,
        });

        let response = self
            .auth(self.client.post(&url).json(&body))
            .send()
            .await
            .map_err(|e| format!("Failed to create repository: {}", e))?;

        if response.status().is_success() {
            let project = response
                .json::<GitLabProject>()
                .await
                .map_err(|e| format!("Failed to parse response: {}", e))?;
            Ok(map_project(project))
        } else {
            Err(format!(
                "Failed to create repository: {}",
                response.status()
            ))
        }
    }

    async fn get_repository(&self, owner: &str, repo: &str) -> Result<RemoteRepository, String> {
        let encoded = format!("{}%2F{}", owner, repo);
        let url = format!("{}/api/v4/projects/{}", self.base_url, encoded);

        let response = self
            .auth(self.client.get(&url))
            .send()
            .await
            .map_err(|e| format!("Failed to get repository: {}", e))?;

        if response.status().is_success() {
            let project = response
                .json::<GitLabProject>()
                .await
                .map_err(|e| format!("Failed to parse response: {}", e))?;
            Ok(map_project(project))
        } else {
            Err(format!("Failed to get repository: {}", response.status()))
        }
    }

    async fn list_repositories(&self, owner: &str) -> Result<Vec<RemoteRepository>, String> {
        let users_url = format!("{}/api/v4/users", self.base_url);

        let response = self
            .auth(self.client.get(&users_url).query(&[("username", owner)]))
            .send()
            .await
            .map_err(|e| format!("Failed to list repositories: {}", e))?;

        if !response.status().is_success() {
            return Err(format!(
                "Failed to list repositories: {}",
                response.status()
            ));
        }

        let users = response
            .json::<Vec<GitLabUser>>()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        let user = users
            .into_iter()
            .next()
            .ok_or_else(|| "Failed to list repositories: user not found".to_string())?;

        let projects_url = format!("{}/api/v4/users/{}/projects", self.base_url, user.id);

        let response = self
            .auth(self.client.get(&projects_url))
            .send()
            .await
            .map_err(|e| format!("Failed to list repositories: {}", e))?;

        if response.status().is_success() {
            let projects = response
                .json::<Vec<GitLabProject>>()
                .await
                .map_err(|e| format!("Failed to parse response: {}", e))?;
            Ok(projects.into_iter().map(map_project).collect())
        } else {
            Err(format!(
                "Failed to list repositories: {}",
                response.status()
            ))
        }
    }

    async fn delete_repository(&self, owner: &str, repo: &str) -> Result<(), String> {
        let encoded = format!("{}%2F{}", owner, repo);
        let url = format!("{}/api/v4/projects/{}", self.base_url, encoded);

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
        let url = format!("{}/api/v4/users", self.base_url);

        let response = self
            .auth(self.client.get(&url).query(&[("username", username)]))
            .send()
            .await
            .map_err(|e| format!("Failed to get user: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("Failed to get user: {}", response.status()));
        }

        let users = response
            .json::<Vec<GitLabUser>>()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        let user = users
            .into_iter()
            .next()
            .ok_or_else(|| "Failed to get user: user not found".to_string())?;

        Ok(map_user(user))
    }

    async fn create_user(
        &self,
        username: &str,
        email: &str,
        password: &str,
        full_name: Option<String>,
    ) -> Result<RemoteUser, String> {
        let url = format!("{}/api/v4/users", self.base_url);

        let name = full_name.clone().unwrap_or_else(|| username.to_string());

        let body = serde_json::json!({
            "username": username,
            "email": email,
            "name": name,
            "password": password,
            "skip_confirmation": true,
        });

        let response = self
            .auth(self.client.post(&url).json(&body))
            .send()
            .await
            .map_err(|e| format!("Failed to create user: {}", e))?;

        if response.status().is_success() {
            let user = response
                .json::<GitLabUser>()
                .await
                .map_err(|e| format!("Failed to parse response: {}", e))?;
            Ok(map_user(user))
        } else {
            Err(format!("Failed to create user: {}", response.status()))
        }
    }

    async fn get_organization(&self, org: &str) -> Result<RemoteOrganization, String> {
        let encoded = org.replace('/', "%2F");
        let url = format!("{}/api/v4/groups/{}", self.base_url, encoded);

        let response = self
            .auth(self.client.get(&url))
            .send()
            .await
            .map_err(|e| format!("Failed to get organization: {}", e))?;

        if response.status().is_success() {
            let group = response
                .json::<GitLabGroup>()
                .await
                .map_err(|e| format!("Failed to parse response: {}", e))?;
            Ok(map_group(group))
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
        let url = format!("{}/api/v4/groups", self.base_url);

        let name = full_name.clone().unwrap_or_else(|| username.to_string());

        let body = serde_json::json!({
            "path": username,
            "name": name,
            "description": description,
        });

        let response = self
            .auth(self.client.post(&url).json(&body))
            .send()
            .await
            .map_err(|e| format!("Failed to create organization: {}", e))?;

        if response.status().is_success() {
            let group = response
                .json::<GitLabGroup>()
                .await
                .map_err(|e| format!("Failed to parse response: {}", e))?;
            Ok(map_group(group))
        } else {
            Err(format!(
                "Failed to create organization: {}",
                response.status()
            ))
        }
    }

    async fn get_file_contents(
        &self,
        owner: &str,
        repo: &str,
        path: &str,
        r#ref: &str,
    ) -> Result<String, String> {
        let encoded_project = format!("{}%2F{}", owner, repo);
        let url = format!(
            "{}/api/v4/projects/{}/repository/files/{}/raw",
            self.base_url, encoded_project, path,
        );

        let response = self
            .auth(self.client.get(&url).query(&[("ref", r#ref)]))
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
