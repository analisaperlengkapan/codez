//! Organizations, teams, memberships, audit logs and instance stats.

use crate::User;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Organization {
    pub id: u64,
    pub username: String,
    pub description: Option<String>,
    pub avatar_url: Option<String>,
    pub website: Option<String>,
    pub location: Option<String>,
    pub email: Option<String>,
    pub visibility: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateOrgOption {
    pub username: String,
    pub description: Option<String>,
    pub website: Option<String>,
    pub location: Option<String>,
    pub email: Option<String>,
    pub visibility: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateTeamOption {
    pub name: String,
    pub description: Option<String>,
    pub permission: String, // "read", "write", "admin"
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AddTeamMemberOption {
    pub username: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Team {
    pub id: u64,
    pub org_name: String,
    pub name: String,
    pub description: Option<String>,
    pub permission: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OrgMember {
    pub user: User,
    pub role: String, // "owner", "member"
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateMemberRoleOption {
    pub role: String, // "owner", "maintainer", "developer", "reporter", "guest"
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AuditLog {
    pub id: u64,
    pub actor: User,
    pub action: String, // e.g. "org.member_role_update", "repo.transfer", "secret.create"
    pub target_type: String, // "org", "repo", "user", "secret"
    pub target_name: String,
    pub details: String,
    pub ip_address: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AdminStats {
    pub users: u64,
    pub repos: u64,
    pub orgs: u64,
    pub issues: u64,
}
