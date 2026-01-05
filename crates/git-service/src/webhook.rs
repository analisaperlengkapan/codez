//! Webhook handling for Git events

use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::fmt;

type HmacSha256 = Hmac<Sha256>;

/// Webhook event type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WebhookEventType {
    Push,
    PullRequest,
    Issue,
    Release,
    Repository,
}

impl fmt::Display for WebhookEventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WebhookEventType::Push => write!(f, "push"),
            WebhookEventType::PullRequest => write!(f, "pull_request"),
            WebhookEventType::Issue => write!(f, "issue"),
            WebhookEventType::Release => write!(f, "release"),
            WebhookEventType::Repository => write!(f, "repository"),
        }
    }
}

/// Push event payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PushEvent {
    pub ref_: String,
    pub before: String,
    pub after: String,
    pub created: bool,
    pub deleted: bool,
    pub forced: bool,
    pub compare: String,
    pub commits: Vec<Commit>,
    pub head_commit: Option<Commit>,
    pub repository: Repository,
    pub pusher: User,
}

/// Commit information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Commit {
    pub id: String,
    pub message: String,
    pub timestamp: String,
    pub url: String,
    pub author: User,
}

/// Repository information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repository {
    pub id: u64,
    pub name: String,
    pub full_name: String,
    pub html_url: String,
    pub clone_url: String,
    pub ssh_url: String,
    pub private: bool,
    pub description: Option<String>,
}

/// User information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: u64,
    pub username: String,
    pub full_name: Option<String>,
    pub email: Option<String>,
    pub avatar_url: Option<String>,
}

/// Pull request event payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PullRequestEvent {
    pub action: String,
    pub pull_request: PullRequest,
    pub repository: Repository,
}

/// Pull request information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PullRequest {
    pub id: u64,
    pub number: u64,
    pub title: String,
    pub body: Option<String>,
    pub state: String,
    pub html_url: String,
    pub head: BranchRef,
    pub base: BranchRef,
    pub user: User,
    pub created_at: String,
    pub updated_at: String,
}

/// Branch reference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchRef {
    pub ref_: String,
    pub sha: String,
    pub repo: Option<Repository>,
}

/// Issue event payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueEvent {
    pub action: String,
    pub issue: Issue,
    pub repository: Repository,
}

/// Issue information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Issue {
    pub id: u64,
    pub number: u64,
    pub title: String,
    pub body: Option<String>,
    pub state: String,
    pub html_url: String,
    pub user: User,
    pub created_at: String,
    pub updated_at: String,
}

/// Webhook validator
pub struct WebhookValidator {
    secret: String,
}

impl WebhookValidator {
    /// Create new webhook validator
    pub fn new(secret: String) -> Self {
        Self { secret }
    }

    /// Validate webhook signature
    pub fn validate(&self, payload: &[u8], signature: &str) -> bool {
        let mut mac = HmacSha256::new_from_slice(self.secret.as_bytes())
            .expect("HMAC can take key of any size");
        mac.update(payload);

        let computed_signature = format!("sha256={}", hex::encode(mac.finalize().into_bytes()));
        computed_signature == signature
    }
}

/// Parse webhook event type from header
pub fn parse_event_type(event_header: &str) -> Option<WebhookEventType> {
    match event_header {
        "push" => Some(WebhookEventType::Push),
        "pull_request" => Some(WebhookEventType::PullRequest),
        "issues" => Some(WebhookEventType::Issue),
        "release" => Some(WebhookEventType::Release),
        "repository" => Some(WebhookEventType::Repository),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_webhook_validator() {
        let secret = "test_secret".to_string();
        let validator = WebhookValidator::new(secret);

        let payload = b"test payload";
        let mut mac = HmacSha256::new_from_slice(b"test_secret").unwrap();
        mac.update(payload);
        let signature = format!("sha256={}", hex::encode(mac.finalize().into_bytes()));

        assert!(validator.validate(payload, &signature));
    }

    #[test]
    fn test_parse_event_type() {
        assert_eq!(parse_event_type("push"), Some(WebhookEventType::Push));
        assert_eq!(
            parse_event_type("pull_request"),
            Some(WebhookEventType::PullRequest)
        );
        assert_eq!(parse_event_type("issues"), Some(WebhookEventType::Issue));
        assert_eq!(parse_event_type("release"), Some(WebhookEventType::Release));
        assert_eq!(
            parse_event_type("repository"),
            Some(WebhookEventType::Repository)
        );
        assert_eq!(parse_event_type("unknown"), None);
    }
}
