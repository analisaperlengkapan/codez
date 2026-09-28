//! Repository discussions and their comments.

use crate::User;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Discussion {
    pub id: u64,
    pub repo_id: u64,
    pub number: u64,
    pub title: String,
    pub body: String,
    pub user: User,
    pub created_at: String,
    pub updated_at: String,
    pub is_locked: bool,
    pub category: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateDiscussionOption {
    pub title: String,
    pub body: String,
    pub category: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateDiscussionOption {
    pub title: Option<String>,
    pub body: Option<String>,
    pub category: Option<String>,
    pub is_locked: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DiscussionComment {
    pub id: u64,
    pub discussion_id: u64,
    pub body: String,
    pub user: User,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateDiscussionCommentOption {
    pub body: String,
}
