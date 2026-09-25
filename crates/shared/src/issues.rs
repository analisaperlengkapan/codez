//! Issues, labels, milestones and issue comments.

use crate::User;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Issue {
    pub id: u64,
    pub repo_id: u64,
    pub number: u64,
    pub title: String,
    pub body: Option<String>,
    pub state: String,
    pub user: User,
    pub assignees: Vec<User>,
    pub labels: Vec<Label>,
    pub milestone: Option<Milestone>,
    pub is_locked: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateIssueOption {
    pub title: String,
    pub body: Option<String>,
    pub milestone: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateIssueOption {
    pub title: Option<String>,
    pub body: Option<String>,
    pub state: Option<String>, // "open" or "closed"
    pub milestone_id: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Label {
    pub id: u64,
    pub repo_id: u64,
    pub name: String,
    pub color: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateLabelOption {
    pub name: String,
    pub color: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateLabelOption {
    pub name: Option<String>,
    pub color: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Milestone {
    pub id: u64,
    pub repo_id: u64,
    pub title: String,
    pub description: Option<String>,
    pub due_on: Option<String>,
    pub state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateMilestoneOption {
    pub title: String,
    pub description: Option<String>,
    pub due_on: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateMilestoneOption {
    pub title: Option<String>,
    pub description: Option<String>,
    pub due_on: Option<String>,
    pub state: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MilestoneStats {
    pub open_issues: u64,
    pub closed_issues: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Comment {
    pub id: u64,
    pub issue_id: u64,
    pub body: String,
    pub user: User,
    pub created_at: String,
    pub reactions: Vec<Reaction>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateCommentOption {
    pub body: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateCommentOption {
    pub body: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Reaction {
    pub id: u64,
    pub user: User,
    pub content: String, // e.g., "+1", "-1", "laugh", "confused", "heart", "hooray", "eyes", "rocket"
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateReactionOption {
    pub content: String,
}
