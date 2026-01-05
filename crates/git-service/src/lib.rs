//! Codeza Git Service
//! Handles Git repository operations and webhook events
//! Supports multiple Git providers (Gitea, GitLab, GitHub, etc.)

use std::sync::Arc;

pub mod provider;
pub mod providers;
pub mod repository_service;
pub mod webhook;

pub use provider::{
    GitProvider, ProviderConfig, ProviderType, RemoteOrganization, RemoteRepository, RemoteUser,
};
pub use providers::{GitHubProvider, GitLabProvider, GiteaProvider};
pub use repository_service::{CreateRepositoryRequest, Repository, RepositoryService};
pub use webhook::{IssueEvent, PullRequestEvent, PushEvent, WebhookEventType, WebhookValidator};

pub fn create_git_provider(config: provider::ProviderConfig) -> Arc<dyn provider::GitProvider> {
    let provider_type = config.provider_type;
    let base_url = config.base_url;
    let access_token = config.access_token;

    match provider_type {
        provider::ProviderType::Gitea => {
            Arc::new(providers::GiteaProvider::new(base_url, access_token))
        }
        provider::ProviderType::GitLab => {
            Arc::new(providers::GitLabProvider::new(base_url, access_token))
        }
        provider::ProviderType::GitHub => {
            Arc::new(providers::GitHubProvider::new(base_url, access_token))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_gitlab_provider_uses_gitlab_type() {
        let config = provider::ProviderConfig::new(
            provider::ProviderType::GitLab,
            "http://example.com".to_string(),
            "token".to_string(),
        );

        let provider = create_git_provider(config);
        assert_eq!(provider.provider_type(), provider::ProviderType::GitLab);
    }

    #[test]
    fn create_gitea_provider_uses_gitea_type() {
        let config = provider::ProviderConfig::new(
            provider::ProviderType::Gitea,
            "http://example.com".to_string(),
            "token".to_string(),
        );

        let provider = create_git_provider(config);
        assert_eq!(provider.provider_type(), provider::ProviderType::Gitea);
    }

    #[test]
    fn create_github_provider_uses_github_type() {
        let config = provider::ProviderConfig::new(
            provider::ProviderType::GitHub,
            "https://api.github.com".to_string(),
            "token".to_string(),
        );

        let provider = create_git_provider(config);
        assert_eq!(provider.provider_type(), provider::ProviderType::GitHub);
    }
}
