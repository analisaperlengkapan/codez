//! Repository wiki pages and Git LFS lock/object records.

use crate::User;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WikiPage {
    pub title: String,
    pub content: String,
    pub commit_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateWikiPageOption {
    pub title: String,
    pub content: String,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LfsLock {
    pub id: String,
    pub repo_id: u64,
    pub path: String,
    pub owner: User,
    pub locked_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LfsObject {
    pub oid: String,
    pub size: u64,
    pub created_at: String,
}
