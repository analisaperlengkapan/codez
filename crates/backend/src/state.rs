//! Shared application state.
//!
//! Codeza stores all data in memory behind `Arc<RwLock<..>>` guards. The
//! granular fields keep contention low and make each feature area independently
//! testable. Handlers lock fields directly; poisoning is tolerated rather than
//! propagated, because a panic in one request should not permanently break the
//! whole server.

use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};

use shared::{
    ActionWorkflow, Activity, Comment, Commit, CommitStatus, Discussion, DiscussionComment, GpgKey,
    Issue, Label, LfsLock, Milestone, Notification, OAuth2Application, OrgMember, Organization,
    Package, Project, ProjectCard, ProjectColumn, PublicKey, PullRequest, Release, Repository,
    Review, Team, Topic, User, Webhook, WebhookDelivery, WorkflowRun,
};

#[derive(Clone, Default)]
pub struct AppState {
    pub repos: Arc<RwLock<Vec<Repository>>>,
    #[allow(clippy::type_complexity)]
    pub file_contents: Arc<RwLock<HashMap<(u64, String, String), String>>>,
    #[allow(clippy::type_complexity)]
    pub file_history: Arc<RwLock<HashMap<(u64, String, String), String>>>,
    pub issues: Arc<RwLock<Vec<Issue>>>,
    pub users: Arc<RwLock<Vec<User>>>,
    pub pulls: Arc<RwLock<Vec<PullRequest>>>,
    pub releases: Arc<RwLock<Vec<Release>>>,
    pub labels: Arc<RwLock<Vec<Label>>>,
    pub milestones: Arc<RwLock<Vec<Milestone>>>,
    pub comments: Arc<RwLock<Vec<Comment>>>,
    pub notifications: Arc<RwLock<Vec<Notification>>>,
    pub keys: Arc<RwLock<Vec<PublicKey>>>,
    pub hooks: Arc<RwLock<Vec<Webhook>>>,
    pub activities: Arc<RwLock<Vec<Activity>>>,
    pub commits: Arc<RwLock<Vec<Commit>>>,
    pub lfs_locks: Arc<RwLock<Vec<LfsLock>>>,
    pub topics: Arc<RwLock<Vec<Topic>>>,
    pub packages: Arc<RwLock<Vec<Package>>>,
    pub teams: Arc<RwLock<Vec<Team>>>,
    pub projects: Arc<RwLock<Vec<Project>>>,
    pub project_columns: Arc<RwLock<Vec<ProjectColumn>>>,
    pub project_cards: Arc<RwLock<Vec<ProjectCard>>>,
    pub reviews: Arc<RwLock<Vec<Review>>>,
    pub orgs: Arc<RwLock<Vec<Organization>>>,
    pub org_members: Arc<RwLock<Vec<OrgMember>>>,
    pub workflows: Arc<RwLock<Vec<ActionWorkflow>>>,
    pub workflow_runs: Arc<RwLock<Vec<WorkflowRun>>>,
    pub webhook_deliveries: Arc<RwLock<Vec<WebhookDelivery>>>,
    pub protected_branches: Arc<RwLock<Vec<shared::ProtectedBranch>>>,
    pub stars: Arc<RwLock<HashMap<u64, Vec<u64>>>>,
    pub discussions: Arc<RwLock<Vec<Discussion>>>,
    pub discussion_comments: Arc<RwLock<Vec<DiscussionComment>>>,
    #[allow(clippy::type_complexity)]
    pub release_assets_data: Arc<RwLock<HashMap<(u64, u64), Vec<u8>>>>,
    pub gpg_keys: Arc<RwLock<Vec<GpgKey>>>,
    pub watchers: Arc<RwLock<HashMap<u64, Vec<u64>>>>, // repo_id -> user_ids
    pub followers: Arc<RwLock<HashMap<u64, Vec<u64>>>>, // user_id -> follower_ids
    pub following: Arc<RwLock<HashMap<u64, Vec<u64>>>>, // user_id -> following_ids
    pub oauth2_apps: Arc<Mutex<Vec<OAuth2Application>>>,
    pub commit_statuses: Arc<RwLock<Vec<CommitStatus>>>,
    pub wikis: Arc<RwLock<HashMap<(u64, String), shared::WikiPage>>>,
    pub audit_logs: Arc<RwLock<Vec<shared::AuditLog>>>,
}
