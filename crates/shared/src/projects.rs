//! Project boards with columns and cards.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Project {
    pub id: u64,
    pub repo_id: u64,
    pub title: String,
    pub description: Option<String>,
    pub is_closed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateProjectOption {
    pub title: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProjectColumn {
    pub id: u64,
    pub project_id: u64,
    pub title: String,
    pub ordering: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateProjectColumnOption {
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProjectCard {
    pub id: u64,
    pub column_id: u64,
    pub content: Option<String>,
    pub note: Option<String>,
    pub issue_id: Option<u64>, // Linked issue
    pub ordering: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateProjectCardOption {
    pub content: Option<String>,
    pub note: Option<String>,
    pub issue_id: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MoveProjectCardOption {
    pub column_id: u64,
    pub new_index: u64,
}
