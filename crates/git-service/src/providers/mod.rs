//! Git provider implementations

pub mod gitea;
pub mod github;
pub mod gitlab;

pub use gitea::GiteaProvider;
pub use github::GitHubProvider;
pub use gitlab::GitLabProvider;
