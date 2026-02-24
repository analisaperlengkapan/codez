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
    pub page: Option<u64>,
    pub limit: Option<u64>,
    pub sort: Option<String>, // "created", "updated", "comments"
    pub direction: Option<String>, // "asc", "desc"
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GpgKey {
    pub id: u64,
    pub key_id: String,
    pub primary_key_id: String,
    pub public_key: String,
    pub emails: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateGpgKeyOption {
    pub armored_public_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RepoActionOption {
    pub action: String, // "star", "watch"
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Notification {
    pub id: u64,
    pub subject: String,
    pub unread: bool,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub id: u64,
    pub username: String,
    pub email: Option<String>,
}

impl User {
    pub fn new(id: u64, username: String, email: Option<String>) -> Self {
        Self {
            id,
            username,
            email,
        }
    }
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
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateIssueOption {
    pub title: String,
    pub body: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateIssueOption {
    pub title: Option<String>,
    pub body: Option<String>,
    pub state: Option<String>, // "open" or "closed"
    pub milestone_id: Option<u64>,
}

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
pub struct ReleaseAsset {
    pub id: u64,
    pub name: String,
    pub size: u64,
    pub download_url: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Release {
    pub id: u64,
    pub repo_id: u64,
    pub tag_name: String,
    pub name: String,
    pub body: Option<String>,
    pub draft: bool,
    pub prerelease: bool,
    pub created_at: String,
    pub author: User,
    pub assets: Vec<ReleaseAsset>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateReleaseOption {
    pub tag_name: String,
    pub name: String,
    pub body: Option<String>,
    pub draft: bool,
    pub prerelease: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateReleaseOption {
    pub tag_name: Option<String>,
    pub name: Option<String>,
    pub body: Option<String>,
    pub draft: Option<bool>,
    pub prerelease: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LoginOption {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RegisterOption {
    pub username: String,
    pub email: String,
    pub password: String,
}

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
pub struct MergePullRequestOption {
    #[serde(rename = "do")]
    pub merge_action: String, // "merge", "rebase", etc.
    pub merge_message_field: Option<String>,
    pub merge_title_field: Option<String>,
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
pub struct Topic {
    pub id: u64,
    pub repo_id: u64,
    pub name: String,
    pub created: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RepoTopicOptions {
    pub topics: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RepoSearchOptions {
    pub q: String,
    pub uid: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PaginationOptions {
    pub page: Option<u64>,
    pub limit: Option<u64>,
}

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
pub struct UserSettingsOption {
    pub full_name: Option<String>,
    pub website: Option<String>,
    pub description: Option<String>,
    pub location: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PublicKey {
    pub id: u64,
    pub title: String,
    pub key: String,
    pub fingerprint: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateKeyOption {
    pub title: String,
    pub key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PushEvent {
    pub r#ref: String,
    pub before: String,
    pub after: String,
    pub repository: Repository,
    pub pusher: User,
    pub commits: Vec<Commit>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IssueEvent {
    pub action: String,
    pub issue: Issue,
    pub repository: Repository,
    pub sender: User,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PullRequestEvent {
    pub action: String,
    pub pull_request: PullRequest,
    pub repository: Repository,
    pub sender: User,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Webhook {
    pub id: u64,
    pub repo_id: u64,
    pub url: String,
    pub events: Vec<String>,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateHookOption {
    pub url: String,
    pub events: Vec<String>,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WebhookDelivery {
    pub id: u64,
    pub hook_id: u64,
    pub event: String,
    pub status: String, // "success", "failed"
    pub request_url: String,
    pub response_status: u16,
    pub delivered_at: String,
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Collaborator {
    pub repo_id: u64,
    pub user: User,
    pub permissions: String,
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
pub struct AdminStats {
    pub users: u64,
    pub repos: u64,
    pub orgs: u64,
    pub issues: u64,
}

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
    pub name: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateWorkflowRunOption {
    pub workflow_id: u64,
    pub ref_name: String, // branch or tag
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorkflowRun {
    pub id: u64,
    pub workflow_id: u64,
    pub status: String, // "queued", "in_progress", "success", "failure"
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Package {
    pub id: u64,
    pub owner: String,
    pub name: String,
    pub version: String,
    pub package_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreatePackageOption {
    pub name: String,
    pub version: String,
    pub package_type: String, // "npm", "maven", "cargo", etc.
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Secret {
    pub name: String,
    pub repo_id: u64,
    pub created_at: String,
    pub data: String, // In real app this would be encrypted
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateSecretOption {
    pub name: String,
    pub data: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeployKey {
    pub id: u64,
    pub repo_id: u64,
    pub title: String,
    pub key: String,
    pub fingerprint: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SystemNotice {
    pub id: u64,
    pub type_: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TwoFactor {
    pub enabled: bool,
    pub method: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LfsObject {
    pub oid: String,
    pub size: u64,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OAuth2Provider {
    pub name: String,
    pub display_name: String,
    pub url: String,
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Contribution {
    pub date: String,
    pub count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OrgMember {
    pub user: User,
    pub role: String, // "owner", "member"
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LicenseTemplate {
    pub key: String,
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GitignoreTemplate {
    pub name: String,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReviewRequest {
    pub reviewer: User,
    pub status: String, // "requested", "approved", "changes_requested", "comment"
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
pub struct CreateReviewOption {
    pub body: String,
    pub event: String, // "APPROVE", "REQUEST_CHANGES", "COMMENT"
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AdminUserEditOption {
    pub email: Option<String>,
    pub password: Option<String>,
    pub active: Option<bool>,
    pub admin: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LanguageStat {
    pub language: String,
    pub percentage: u8,
    pub color: String,
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EmailAddress {
    pub email: String,
    pub verified: bool,
    pub primary: bool,
}

use std::fmt;

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct OAuth2Application {
    pub id: u64,
    pub name: String,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uris: Vec<String>,
}

impl fmt::Debug for OAuth2Application {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OAuth2Application")
            .field("id", &self.id)
            .field("name", &self.name)
            .field("client_id", &self.client_id)
            .field("client_secret", &"***REDACTED***")
            .field("redirect_uris", &self.redirect_uris)
            .finish()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateOAuth2AppOption {
    pub name: String,
    pub redirect_uris: Vec<String>,
}

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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MilestoneStats {
    pub open_issues: u64,
    pub closed_issues: u64,
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
pub struct LfsLock {
    pub id: String,
    pub repo_id: u64,
    pub path: String,
    pub owner: User,
    pub locked_at: String,
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_creation() {
        let user = User::new(1, "jules".to_string(), Some("jules@example.com".to_string()));
        assert_eq!(user.username, "jules");
        assert_eq!(user.email, Some("jules@example.com".to_string()));
    }

    #[test]
    fn test_collab_branch_tag() {
        let user = User::new(1, "u".to_string(), None);
        let commit = Commit { sha: "s".to_string(), repo_id: 1, message: "m".to_string(), author: user.clone(), date: "d".to_string() };

        let c = Collaborator { user: user.clone(), repo_id: 1, permissions: "read".to_string() };
        assert_eq!(c.permissions, "read");

        let b = Branch { name: "main".to_string(), repo_id: 1, commit: commit.clone(), protected: true };
        assert!(b.protected);

        let t = Tag { name: "v1".to_string(), repo_id: 1, id: "1".to_string(), commit: commit.clone() };
        assert_eq!(t.name, "v1");
    }

    #[test]
    fn test_system_notices_2fa() {
        let notice = SystemNotice { id: 1, type_: "alert".to_string(), description: "System update".to_string() };
        assert_eq!(notice.type_, "alert");

        let two_fa = TwoFactor { enabled: true, method: "totp".to_string() };
        assert!(two_fa.enabled);
    }

    #[test]
    fn test_lfs_oauth_reaction() {
        let lfs = LfsObject { oid: "oid".to_string(), size: 100, created_at: "now".to_string() };
        assert_eq!(lfs.size, 100);

        let oauth = OAuth2Provider { name: "github".to_string(), display_name: "GitHub".to_string(), url: "http".to_string() };
        assert_eq!(oauth.name, "github");

        let user = User::new(1, "u".to_string(), None);
        let react = Reaction { id: 1, user, content: "heart".to_string(), created_at: "now".to_string() };
        assert_eq!(react.content, "heart");

        let opt = CreateReactionOption { content: "+1".to_string() };
        assert_eq!(opt.content, "+1");
    }

    #[test]
    fn test_diff_structs() {
        let line = DiffLine {
            line_no_old: Some(1),
            line_no_new: Some(1),
            content: "code".to_string(),
            type_: "context".to_string(),
        };
        assert_eq!(line.content, "code");

        let file = DiffFile {
            name: "file.rs".to_string(),
            old_name: None,
            index: "idx".to_string(),
            additions: 1,
            deletions: 0,
            type_: "modify".to_string(),
            lines: vec![line],
        };
        assert_eq!(file.name, "file.rs");
    }

    #[test]
    fn test_contribution_org_member() {
        let c = Contribution { date: "2023-01-01".to_string(), count: 5 };
        assert_eq!(c.count, 5);

        let user = User::new(1, "u".to_string(), None);
        let m = OrgMember { user, role: "owner".to_string() };
        assert_eq!(m.role, "owner");
    }

    #[test]
    fn test_secret_deploykey() {
        let s = Secret { name: "TOKEN".to_string(), repo_id: 1, created_at: "now".to_string(), data: "d".to_string() };
        assert_eq!(s.name, "TOKEN");

        let k = DeployKey { id: 1, repo_id: 1, title: "deploy".to_string(), key: "k".to_string(), fingerprint: "f".to_string() };
        assert_eq!(k.title, "deploy");
    }

    #[test]
    fn test_actions_packages_structs() {
        let wf = ActionWorkflow { id: 1, name: "build".to_string(), status: "success".to_string() };
        assert_eq!(wf.name, "build");

        let pkg = Package { id: 1, owner: "admin".to_string(), name: "pkg".to_string(), version: "1.0".to_string(), package_type: "npm".to_string() };
        assert_eq!(pkg.package_type, "npm");
    }

    #[test]
    fn test_activity_struct() {
        let act = Activity { id: 1, repo_id: 1, user_id: 1, user_name: "u".to_string(), op_type: "push".to_string(), content: "c".to_string(), created: "d".to_string() };
        assert_eq!(act.op_type, "push");
    }

    #[test]
    fn test_admin_stats() {
        let stats = AdminStats { users: 10, repos: 20, orgs: 5, issues: 100 };
        assert_eq!(stats.users, 10);
    }

    #[test]
    fn test_team_project_structs() {
        let team = Team { id: 1, org_name: "org".to_string(), name: "dev".to_string(), description: None, permission: "write".to_string() };
        assert_eq!(team.name, "dev");

        let project = Project { id: 1, repo_id: 1, title: "v1".to_string(), description: None, is_closed: false };
        assert!(!project.is_closed);
    }

    #[test]
    fn test_keys_hooks_structs() {
        let key = PublicKey { id: 1, title: "Laptop".to_string(), key: "ssh-rsa...".to_string(), fingerprint: "sha256...".to_string() };
        assert_eq!(key.title, "Laptop");

        let hook = Webhook { id: 1, repo_id: 1, url: "http://example.com".to_string(), events: vec!["push".to_string()], active: true };
        assert!(hook.active);
    }

    #[test]
    fn test_wiki_structs() {
        let page = WikiPage {
            title: "Home".to_string(),
            content: "Welcome".to_string(),
            commit_message: None,
        };
        assert_eq!(page.title, "Home");
    }

    #[test]
    fn test_settings_structs() {
        let r_opts = RepoSettingsOption {
            description: Some("desc".to_string()),
            private: Some(true),
            website: None,
            default_branch: Some("main".to_string()),
            allow_rebase_merge: None,
            allow_squash_merge: None,
            allow_merge_commit: None,
            has_issues: None,
            has_wiki: None,
            has_projects: None,
        };
        assert_eq!(r_opts.description, Some("desc".to_string()));

        let u_opts = UserSettingsOption {
            full_name: Some("Name".to_string()),
            website: None,
            description: None,
            location: None,
        };
        assert_eq!(u_opts.full_name, Some("Name".to_string()));
    }

    #[test]
    fn test_topic_structs() {
        let topic = Topic { id: 1, repo_id: 1, name: "rust".to_string(), created: "date".to_string() };
        assert_eq!(topic.name, "rust");

        let opts = RepoTopicOptions { topics: vec!["rust".to_string(), "gitea".to_string()] };
        assert_eq!(opts.topics.len(), 2);
    }

    #[test]
    fn test_search_struct() {
        let search = RepoSearchOptions { q: "test".to_string(), uid: Some(1) };
        assert_eq!(search.q, "test");
    }

    #[test]
    fn test_label_structs() {
        let label = Label {
            id: 1,
            repo_id: 1,
            name: "bug".to_string(),
            color: "#ff0000".to_string(),
            description: None,
        };
        assert_eq!(label.name, "bug");

        let opts = CreateLabelOption {
            name: "feature".to_string(),
            color: "#00ff00".to_string(),
            description: None,
        };
        assert_eq!(opts.color, "#00ff00");
    }

    #[test]
    fn test_milestone_structs() {
        let milestone = Milestone {
            id: 1,
            repo_id: 1,
            title: "v1.0".to_string(),
            description: None,
            due_on: None,
            state: "open".to_string(),
        };
        assert_eq!(milestone.title, "v1.0");

        let opts = CreateMilestoneOption {
            title: "v2.0".to_string(),
            description: None,
            due_on: None,
        };
        assert_eq!(opts.title, "v2.0");
    }

    #[test]
    fn test_comment_structs() {
        let user = User::new(1, "u".to_string(), None);
        let comment = Comment {
            id: 1,
            issue_id: 1,
            body: "text".to_string(),
            user,
            created_at: "date".to_string(),
            reactions: vec![],
        };
        assert_eq!(comment.body, "text");

        let merge = MergePullRequestOption {
            merge_action: "merge".to_string(),
            merge_message_field: None,
            merge_title_field: None,
        };
        assert_eq!(merge.merge_action, "merge");
    }

    #[test]
    fn test_auth_structs() {
        let login = LoginOption {
            username: "u".to_string(),
            password: "p".to_string(),
        };
        assert_eq!(login.username, "u");

        let reg = RegisterOption {
            username: "u".to_string(),
            email: "e".to_string(),
            password: "p".to_string(),
        };
        assert_eq!(reg.email, "e");
    }

    #[test]
    fn test_org_struct() {
        let org = Organization {
            id: 1,
            username: "org".to_string(),
            description: None,
            avatar_url: None,
            website: None,
            location: None,
            email: None,
            visibility: None,
        };
        assert_eq!(org.username, "org");
    }

    #[test]
    fn test_release_structs() {
        let user = User::new(1, "u".to_string(), None);
        let rel = Release {
            id: 1,
            repo_id: 1,
            tag_name: "v1.0".to_string(),
            name: "Release 1.0".to_string(),
            body: None,
            draft: false,
            prerelease: false,
            created_at: "date".to_string(),
            author: user,
            assets: vec![],
        };
        assert_eq!(rel.tag_name, "v1.0");

        let opts = CreateReleaseOption {
            tag_name: "v1.1".to_string(),
            name: "Next".to_string(),
            body: None,
            draft: true,
            prerelease: false,
        };
        assert!(opts.draft);
    }

    #[test]
    fn test_commit() {
        let user = User::new(1, "committer".to_string(), None);
        let commit = Commit {
            sha: "abc1234".to_string(),
            repo_id: 1,
            message: "Initial commit".to_string(),
            author: user,
            date: "2023-01-01".to_string(),
        };
        assert_eq!(commit.sha, "abc1234");
    }

    #[test]
    fn test_file_entry() {
        let file = FileEntry {
            name: "README.md".to_string(),
            path: "README.md".to_string(),
            kind: "file".to_string(),
            size: 1024,
        };
        assert_eq!(file.name, "README.md");
        assert_eq!(file.kind, "file");
    }

    #[test]
    fn test_pull_request_structs() {
        let user = User::new(1, "user".to_string(), None);
        let pr = PullRequest {
            id: 1,
            repo_id: 1,
            number: 1,
            title: "PR Title".to_string(),
            body: None,
            state: "open".to_string(),
            user,
            merged: false,
            head_sha: "sha".to_string(),
            base: "main".to_string(),
            head: "feature".to_string(),
        };
        assert_eq!(pr.title, "PR Title");
        assert!(!pr.merged);

        let opts = CreatePullRequestOption {
            title: "New Feature".to_string(),
            body: None,
            head: "feature".to_string(),
            base: "main".to_string(),
        };
        assert_eq!(opts.head, "feature");
    }

    #[test]
    fn test_issue_structs() {
        let user = User::new(1, "user".to_string(), None);
        let issue = Issue {
            id: 1,
            repo_id: 1,
            number: 1,
            title: "Bug".to_string(),
            body: None,
            state: "open".to_string(),
            user,
            assignees: vec![],
            labels: vec![],
            milestone: None,
        };
        assert_eq!(issue.title, "Bug");

        let opts = CreateIssueOption {
            title: "New Bug".to_string(),
            body: Some("Description".to_string()),
        };
        assert_eq!(opts.title, "New Bug");

        let update = UpdateIssueOption {
            title: Some("Updated".to_string()),
            body: None,
            state: Some("closed".to_string()),
            milestone_id: None,
        };
        assert_eq!(update.title, Some("Updated".to_string()));
    }

    #[test]
    fn test_create_repo_option() {
        let opts = CreateRepoOption {
            name: "new-repo".to_string(),
            description: Some("desc".to_string()),
            private: true,
            auto_init: false,
            gitignores: Some("Rust".to_string()),
            license: Some("MIT".to_string()),
            readme: Some("Default".to_string()),
            default_branch: None,
            allow_rebase_merge: None,
            allow_squash_merge: None,
            allow_merge_commit: None,
            has_issues: None,
            has_wiki: None,
            has_projects: None,
        };
        assert_eq!(opts.name, "new-repo");
        assert!(opts.private);
        assert_eq!(opts.license, Some("MIT".to_string()));
    }

    #[test]
    fn test_repository_creation() {
        let repo = Repository::new(1, "codeza".to_string(), "jules".to_string());
        assert_eq!(repo.id, 1);
        assert_eq!(repo.name, "codeza");
        assert_eq!(repo.owner, "jules");
        assert_eq!(repo.private, false);
        assert_eq!(repo.stars_count, 0);
        assert_eq!(repo.is_mirror, false);
    }

    #[test]
    fn test_gpg_key() {
        let key = GpgKey { id: 1, key_id: "ID".to_string(), primary_key_id: "PID".to_string(), public_key: "PUB".to_string(), emails: vec![] };
        assert_eq!(key.key_id, "ID");
    }

    #[test]
    fn test_repo_action() {
        let act = RepoActionOption { action: "star".to_string() };
        assert_eq!(act.action, "star");
    }

    #[test]
    fn test_repository_serialization() {
        let repo = Repository::new(1, "codeza".to_string(), "jules".to_string());
        let json = serde_json::to_string(&repo).unwrap();
        let deserialized: Repository = serde_json::from_str(&json).unwrap();
        assert_eq!(repo, deserialized);
    }

    #[test]
    fn test_templates() {
        let l = LicenseTemplate { key: "mit".to_string(), name: "MIT".to_string(), url: "u".to_string() };
        assert_eq!(l.key, "mit");
        let g = GitignoreTemplate { name: "Rust".to_string(), source: "target/".to_string() };
        assert_eq!(g.name, "Rust");
    }

    #[test]
    fn test_review_admin_structs() {
        let u = User::new(1, "u".to_string(), None);
        let rr = ReviewRequest { reviewer: u, status: "s".to_string() };
        assert_eq!(rr.status, "s");

        let a = AdminUserEditOption { email: Some("e".to_string()), password: None, active: Some(true), admin: None };
        assert!(a.active.unwrap());
    }

    #[test]
    fn test_lang_prot_email_app() {
        let l = LanguageStat { language: "Rust".to_string(), percentage: 100, color: "#dea584".to_string() };
        assert_eq!(l.percentage, 100);

        let pb = ProtectedBranch {
            id: 1,
            repo_id: 1,
            name: "main".to_string(),
            enable_push: false,
            enable_force_push: false,
            required_status_checks: vec![],
        };
        assert!(!pb.enable_push);

        let e = EmailAddress { email: "e".to_string(), verified: true, primary: true };
        assert!(e.primary);

        let app = OAuth2Application {
            id: 1,
            name: "app".to_string(),
            client_id: "cid".to_string(),
            client_secret: "secret".to_string(),
            redirect_uris: vec![],
        };
        assert_eq!(app.client_id, "cid");
    }

    #[test]
    fn test_migrate_transfer_options() {
        let m = MigrateRepoOption {
            clone_addr: "url".to_string(),
            repo_name: "name".to_string(),
            service: "git".to_string(),
            mirror: true,
        };
        assert!(m.mirror);

        let t = TransferRepoOption { new_owner: "new".to_string() };
        assert_eq!(t.new_owner, "new");
    }

    #[test]
    fn test_milestone_stats() {
        let stats = MilestoneStats { open_issues: 10, closed_issues: 5 };
        assert_eq!(stats.open_issues, 10);
    }

    #[test]
    fn test_code_search() {
        let r = CodeSearchResult {
            name: "n".to_string(),
            path: "p".to_string(),
            sha: "s".to_string(),
            url: "u".to_string(),
            content: Some("c".to_string()),
        };
        assert_eq!(r.name, "n");
    }

    #[test]
    fn test_lfs_lock() {
        let u = User::new(1, "u".to_string(), None);
        let l = LfsLock { id: "1".to_string(), repo_id: 1, path: "p".to_string(), owner: u, locked_at: "t".to_string() };
        assert_eq!(l.path, "p");
    }
}
