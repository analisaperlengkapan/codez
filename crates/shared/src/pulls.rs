//! Pull requests, reviews and diff representations.

use crate::User;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PullRequest {
    pub id: u64,
    pub repo_id: u64,
    pub number: u64,
    pub title: String,
    pub body: Option<String>,
    pub state: String,
    pub user: User,
    pub merged: bool,
    pub head_sha: String,
    pub base: String,
    pub head: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreatePullRequestOption {
    pub title: String,
    pub body: Option<String>,
    pub head: String,
    pub base: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdatePullRequestOption {
    pub title: Option<String>,
    pub body: Option<String>,
    pub state: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MergePullRequestOption {
    #[serde(rename = "do")]
    pub merge_action: String, // "merge", "rebase", etc.
    pub merge_message_field: Option<String>,
    pub merge_title_field: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Review {
    pub id: u64,
    pub pull_request_id: u64,
    pub user: User,
    pub body: String,
    pub state: String, // "APPROVED", "CHANGES_REQUESTED", "COMMENTED", "PENDING"
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReviewRequest {
    pub reviewer: User,
    pub status: String, // "requested", "approved", "changes_requested", "comment"
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateReviewOption {
    pub body: String,
    pub event: String, // "APPROVE", "REQUEST_CHANGES", "COMMENT"
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DiffLine {
    pub line_no_old: Option<u64>,
    pub line_no_new: Option<u64>,
    pub content: String,
    pub type_: String, // "add", "delete", "context"
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DiffFile {
    pub name: String,
    pub old_name: Option<String>,
    pub index: String,
    pub additions: u64,
    pub deletions: u64,
    pub type_: String, // "add", "modify", "delete", "rename"
    pub lines: Vec<DiffLine>,
}
