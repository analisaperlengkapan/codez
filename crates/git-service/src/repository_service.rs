//! Repository service for managing Git repositories

use crate::provider::GitProvider;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

/// Repository model (Codeza-internal)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repository {
    pub id: Uuid,
    pub name: String,
    pub owner: String,
    pub description: Option<String>,
    pub private: bool,
    pub url: String,
    pub ssh_url: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Create repository request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRepositoryRequest {
    pub name: String,
    pub owner: String,
    pub description: Option<String>,
    pub private: bool,
}

/// Repository service (provider-agnostic)
pub struct RepositoryService {
    provider: Arc<dyn GitProvider>,
}

impl RepositoryService {
    /// Create new repository service
    pub fn new(provider: Arc<dyn GitProvider>) -> Self {
        Self { provider }
    }

    /// Create a new repository
    pub async fn create_repository(
        &self,
        req: CreateRepositoryRequest,
    ) -> Result<Repository, String> {
        let remote_repo = self
            .provider
            .create_repository(&req.owner, &req.name, req.description.clone(), req.private)
            .await?;

        Ok(Repository {
            id: Uuid::new_v4(),
            name: remote_repo.name,
            owner: req.owner,
            description: remote_repo.description,
            private: remote_repo.private,
            url: remote_repo.html_url,
            ssh_url: remote_repo.ssh_url,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        })
    }

    /// Get repository
    pub async fn get_repository(&self, owner: &str, name: &str) -> Result<Repository, String> {
        let remote_repo = self.provider.get_repository(owner, name).await?;

        Ok(Repository {
            id: Uuid::new_v4(),
            name: remote_repo.name,
            owner: owner.to_string(),
            description: remote_repo.description,
            private: remote_repo.private,
            url: remote_repo.html_url,
            ssh_url: remote_repo.ssh_url,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        })
    }

    /// List repositories
    pub async fn list_repositories(&self, owner: &str) -> Result<Vec<Repository>, String> {
        let remote_repos = self.provider.list_repositories(owner).await?;

        Ok(remote_repos
            .into_iter()
            .map(|repo| Repository {
                id: Uuid::new_v4(),
                name: repo.name,
                owner: owner.to_string(),
                description: repo.description,
                private: repo.private,
                url: repo.html_url,
                ssh_url: repo.ssh_url,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            })
            .collect())
    }

    /// Delete repository
    pub async fn delete_repository(&self, owner: &str, name: &str) -> Result<(), String> {
        self.provider.delete_repository(owner, name).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_repository_request() {
        let req = CreateRepositoryRequest {
            name: "test-repo".to_string(),
            owner: "test-user".to_string(),
            description: Some("Test repository".to_string()),
            private: false,
        };

        assert_eq!(req.name, "test-repo");
        assert_eq!(req.owner, "test-user");
        assert!(!req.private);
    }

    #[test]
    fn test_repository_model() {
        let repo = Repository {
            id: Uuid::new_v4(),
            name: "test-repo".to_string(),
            owner: "test-user".to_string(),
            description: Some("Test repository".to_string()),
            private: false,
            url: "https://example.com/test-user/test-repo".to_string(),
            ssh_url: "git@example.com:test-user/test-repo.git".to_string(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        assert_eq!(repo.name, "test-repo");
        assert_eq!(repo.owner, "test-user");
        assert!(!repo.private);
    }
}
