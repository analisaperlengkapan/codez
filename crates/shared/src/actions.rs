//! CI workflow definitions, runs and step logs, plus activity feed items.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Activity {
    pub id: u64,
    pub repo_id: u64,
    pub user_id: u64,
    pub user_name: String,
    pub op_type: String,
    pub content: String,
    pub created: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ActionWorkflow {
    pub id: u64,
    pub repo_id: u64,
    pub name: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateWorkflowRunOption {
    pub workflow_id: u64,
    pub ref_name: String, // branch or tag
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorkflowStepLog {
    pub name: String,
    pub status: String,
    pub logs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorkflowRun {
    pub id: u64,
    pub workflow_id: u64,
    pub status: String, // "queued", "in_progress", "success", "failure", "cancelled"
    pub created_at: String,
    pub step_logs: Vec<WorkflowStepLog>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateWorkflowRunOption {
    pub status: String,
}
