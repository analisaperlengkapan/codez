//! Repository records, search/listing options and repository settings.

use crate::User;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Repository {
    pub id: u64,
    pub name: String,
    pub description: Option<String>,
    pub private: bool,
    pub owner: String,
    pub stars_count: u64,
    pub forks_count: u64,
    pub watchers_count: u64,
    pub is_mirror: bool,
    pub parent_id: Option<u64>,
    pub website: Option<String>,
    pub default_branch: Option<String>,
    pub allow_rebase_merge: bool,
    pub allow_squash_merge: bool,
    pub allow_merge_commit: bool,
    pub has_issues: bool,
    pub has_wiki: bool,
    pub has_projects: bool,
}

impl Repository {
    pub fn new(id: u64, name: String, owner: String) -> Self {
        Self {
            id,
            name,
            description: None,
            private: false,
            owner,
            stars_count: 0,
            forks_count: 0,
            watchers_count: 0,
            is_mirror: false,
            parent_id: None,
            website: None,
            default_branch: Some("main".to_string()),
            allow_rebase_merge: true,
            allow_squash_merge: true,
            allow_merge_commit: true,
            has_issues: true,
            has_wiki: true,
            has_projects: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RepoUserStatus {
    pub starred: bool,
    pub watching: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IssueFilterOptions {
    pub state: Option<String>, // "open", "closed", "all"
    pub q: Option<String>,     // search query
    pub label_id: Option<u64>,
    pub assignee_username: Option<String>,
    pub milestone_id: Option<u64>,
    pub page: Option<u64>,
    pub limit: Option<u64>,
    pub sort: Option<String>,      // "created", "updated", "comments"
    pub direction: Option<String>, // "asc", "desc"
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RepoActionOption {
    pub action: String, // "star", "watch"
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RepoSearchOptions {
    pub q: String,
    pub uid: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RepoTopicOptions {
    pub topics: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Topic {
    pub id: u64,
    pub repo_id: u64,
    pub name: String,
    pub created: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PaginationOptions {
    pub page: Option<u64>,
    pub limit: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RepoSettingsOption {
    pub description: Option<String>,
    pub private: Option<bool>,
    pub website: Option<String>,
    pub default_branch: Option<String>,
    pub allow_rebase_merge: Option<bool>,
    pub allow_squash_merge: Option<bool>,
    pub allow_merge_commit: Option<bool>,
    pub has_issues: Option<bool>,
    pub has_wiki: Option<bool>,
    pub has_projects: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RepoPulseStats {
    pub period: String,
    pub active_issues: u64,
    pub closed_issues: u64,
    pub opened_prs: u64,
    pub merged_prs: u64,
    pub new_commits: u64,
    pub active_authors: Vec<User>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateRepoOption {
    pub name: String,
    pub description: Option<String>,
    pub private: bool,
    pub auto_init: bool,
    pub gitignores: Option<String>,
    pub license: Option<String>,
    pub readme: Option<String>,
    pub default_branch: Option<String>,
    pub allow_rebase_merge: Option<bool>,
    pub allow_squash_merge: Option<bool>,
    pub allow_merge_commit: Option<bool>,
    pub has_issues: Option<bool>,
    pub has_wiki: Option<bool>,
    pub has_projects: Option<bool>,
}
