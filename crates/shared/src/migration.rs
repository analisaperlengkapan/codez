//! Repository migration and transfer options.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MigrateRepoOption {
    pub clone_addr: String,
    pub repo_name: String,
    pub service: String, // "git", "github", "gitlab", "gitea"
    pub mirror: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TransferRepoOption {
    pub new_owner: String,
}
