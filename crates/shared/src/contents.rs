//! Source browsing: files, commits, branches, tags and commit statuses.

use crate::User;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FileEntry {
    pub name: String,
    pub path: String,
    pub kind: String, // "file" or "dir"
    pub size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateFileOption {
    pub content: String,
    pub message: String,
    pub sha: String,
    pub branch: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Commit {
    pub sha: String,
    pub repo_id: u64,
    pub message: String,
    pub author: User,
    pub date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Branch {
    pub repo_id: u64,
    pub name: String,
    pub commit: Commit,
    pub protected: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateBranchOption {
    pub name: String,
    pub base: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Tag {
    pub repo_id: u64,
    pub name: String,
    pub id: String,
    pub commit: Commit,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Collaborator {
    pub repo_id: u64,
    pub user: User,
    pub permissions: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProtectedBranch {
    pub id: u64,
    pub repo_id: u64,
    pub name: String,
    pub enable_push: bool,
    pub enable_force_push: bool,
    pub required_status_checks: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateProtectedBranchOption {
    pub name: String,
    pub enable_push: bool,
    pub enable_force_push: bool,
    pub required_status_checks: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CodeSearchResult {
    pub name: String,
    pub path: String,
    pub sha: String,
    pub url: String,
    pub content: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CommitStatus {
    pub id: u64,
    pub sha: String,
    pub state: String, // pending, success, error, failure
    pub target_url: Option<String>,
    pub description: Option<String>,
    pub context: String,
    pub created_at: String,
    pub creator: User,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateStatusOption {
    pub state: String,
    pub target_url: Option<String>,
    pub description: Option<String>,
    pub context: Option<String>,
}
