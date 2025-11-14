//! Git provider implementations

pub mod gitea;
pub mod gitlab;

pub use gitea::GiteaProvider;
pub use gitlab::GitLabProvider;
