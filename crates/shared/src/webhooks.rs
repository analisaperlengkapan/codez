//! Webhook events and deliveries plus repository secrets and deploy keys.

use crate::Commit;
use crate::Issue;
use crate::PullRequest;
use crate::Repository;
use crate::User;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PushEvent {
    pub r#ref: String,
    pub before: String,
    pub after: String,
    pub repository: Repository,
    pub pusher: User,
    pub commits: Vec<Commit>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IssueEvent {
    pub action: String,
    pub issue: Issue,
    pub repository: Repository,
    pub sender: User,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PullRequestEvent {
    pub action: String,
    pub pull_request: PullRequest,
    pub repository: Repository,
    pub sender: User,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Webhook {
    pub id: u64,
    pub repo_id: u64,
    pub url: String,
    pub events: Vec<String>,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateHookOption {
    pub url: String,
    pub events: Vec<String>,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WebhookDelivery {
    pub id: u64,
    pub hook_id: u64,
    pub event: String,
    pub status: String, // "success", "failed"
    pub request_url: String,
    pub response_status: u16,
    pub delivered_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Secret {
    pub name: String,
    pub repo_id: u64,
    pub created_at: String,
    pub data: String, // In real app this would be encrypted
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateSecretOption {
    pub name: String,
    pub data: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeployKey {
    pub id: u64,
    pub repo_id: u64,
    pub title: String,
    pub key: String,
    pub fingerprint: String,
}
