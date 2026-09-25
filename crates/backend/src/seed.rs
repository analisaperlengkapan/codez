//! In-memory demo data used to bootstrap the application.
//!
//! Codeza currently persists state only in memory (see the README), so a fresh
//! process starts from this deterministic demo dataset.

use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};

use shared::{
    ActionWorkflow, Activity, Comment, Commit, Issue, Label, Milestone, Notification, Organization,
    Package, PublicKey, PullRequest, Release, Repository, Team, Topic, User, Webhook,
};

use crate::state::AppState;

pub fn demo_state() -> AppState {
    let user = User::new(1, "admin".to_string(), Some("admin@codeza.com".to_string()));

    let mut file_map = HashMap::new();
    file_map.insert(
        (1, "main".to_string(), "src/main.rs".to_string()),
        "fn main() { println!(\"Welcome to codeza\"); }".to_string(),
    );
    file_map.insert(
        (1, "main".to_string(), "src/lib.rs".to_string()),
        "pub fn add(a: i32, b: i32) -> i32 { a + b }".to_string(),
    );
    file_map.insert(
        (1, "main".to_string(), "README.md".to_string()),
        "# Codeza Repository\n\nThis is a demo repository.".to_string(),
    );
    file_map.insert(
        (1, "main".to_string(), "Cargo.toml".to_string()),
        "[package]\nname = \"codeza\"\nversion = \"0.1.0\"\n".to_string(),
    );
    // Seed "feature" branch for the PR 1
    file_map.insert(
        (1, "feature".to_string(), "src/main.rs".to_string()),
        "fn main() { println!(\"Welcome to codeza feature\"); }".to_string(),
    );
    file_map.insert(
        (1, "feature".to_string(), "README.md".to_string()),
        "# Codeza Repository (Feature)\n\nThis is a demo repository.".to_string(),
    );
    // Copy unmodified files from main to feature branch
    file_map.insert(
        (1, "feature".to_string(), "src/lib.rs".to_string()),
        "pub fn add(a: i32, b: i32) -> i32 { a + b }".to_string(),
    );
    file_map.insert(
        (1, "feature".to_string(), "Cargo.toml".to_string()),
        "[package]\nname = \"codeza\"\nversion = \"0.1.0\"\n".to_string(),
    );

    // Initialize history map
    let mut history_map = HashMap::new();
    // Copy main branch entries to history
    history_map.insert(
        (1, "main".to_string(), "src/main.rs".to_string()),
        "fn main() { println!(\"Welcome to codeza\"); }".to_string(),
    );
    history_map.insert(
        (1, "main".to_string(), "src/lib.rs".to_string()),
        "pub fn add(a: i32, b: i32) -> i32 { a + b }".to_string(),
    );
    history_map.insert(
        (1, "main".to_string(), "README.md".to_string()),
        "# Codeza Repository\n\nThis is a demo repository.".to_string(),
    );
    history_map.insert(
        (1, "main".to_string(), "Cargo.toml".to_string()),
        "[package]\nname = \"codeza\"\nversion = \"0.1.0\"\n".to_string(),
    );
    // For feature branch history, use main branch content (as baseline)
    history_map.insert(
        (1, "feature".to_string(), "src/main.rs".to_string()),
        "fn main() { println!(\"Welcome to codeza\"); }".to_string(),
    );
    history_map.insert(
        (1, "feature".to_string(), "src/lib.rs".to_string()),
        "pub fn add(a: i32, b: i32) -> i32 { a + b }".to_string(),
    );
    history_map.insert(
        (1, "feature".to_string(), "README.md".to_string()),
        "# Codeza Repository\n\nThis is a demo repository.".to_string(),
    );
    history_map.insert(
        (1, "feature".to_string(), "Cargo.toml".to_string()),
        "[package]\nname = \"codeza\"\nversion = \"0.1.0\"\n".to_string(),
    );

    let mut wikis_map = HashMap::new();
    wikis_map.insert(
        (1, "Home".to_string()),
        shared::WikiPage {
            title: "Home".to_string(),
            content: "Welcome to the wiki!".to_string(),
            commit_message: None,
        },
    );
    wikis_map.insert(
        (1, "Installation".to_string()),
        shared::WikiPage {
            title: "Installation".to_string(),
            content: "How to install...".to_string(),
            commit_message: None,
        },
    );

    AppState {
        repos: Arc::new(RwLock::new(vec![
            Repository::new(1, "codeza".to_string(), "admin".to_string()),
            Repository::new(2, "gitea-clone".to_string(), "user".to_string()),
        ])),
        file_contents: Arc::new(RwLock::new(file_map)),
        file_history: Arc::new(RwLock::new(history_map)),
        issues: Arc::new(RwLock::new(vec![Issue {
            id: 1,
            repo_id: 1,
            number: 1,
            title: "First Issue".to_string(),
            body: Some("This is a bug".to_string()),
            state: "open".to_string(),
            user: user.clone(),
            assignees: vec![],
            labels: vec![],
            milestone: None,
            is_locked: false,
        }])),
        users: Arc::new(RwLock::new(vec![
            user.clone(),
            User::new(2, "user".to_string(), Some("user@example.com".to_string())),
        ])),
        pulls: Arc::new(RwLock::new(vec![PullRequest {
            id: 1,
            repo_id: 1,
            number: 1,
            title: "First PR".to_string(),
            body: Some("Description".to_string()),
            state: "open".to_string(),
            user: user.clone(),
            merged: false,
            head_sha: "mock_sha".to_string(),
            base: "main".to_string(),
            head: "feature".to_string(),
        }])),
        releases: Arc::new(RwLock::new(vec![Release {
            id: 1,
            repo_id: 1,
            tag_name: "v1.0.0".to_string(),
            name: "Initial Release".to_string(),
            body: Some("Description".to_string()),
            draft: false,
            prerelease: false,
            created_at: "2023-01-01".to_string(),
            author: user.clone(),
            assets: vec![],
        }])),
        labels: Arc::new(RwLock::new(vec![Label {
            id: 1,
            repo_id: 1,
            name: "bug".to_string(),
            color: "#ff0000".to_string(),
            description: None,
        }])),
        milestones: Arc::new(RwLock::new(vec![Milestone {
            id: 1,
            repo_id: 1,
            title: "v1.0".to_string(),
            description: None,
            due_on: None,
            state: "open".to_string(),
        }])),
        comments: Arc::new(RwLock::new(vec![Comment {
            id: 1,
            issue_id: 1,
            body: "Great idea!".to_string(),
            user: user.clone(),
            created_at: "2023-01-01".to_string(),
            reactions: vec![],
        }])),
        notifications: Arc::new(RwLock::new(vec![Notification {
            id: 1,
            subject: "Welcome to Codeza".to_string(),
            unread: true,
            updated_at: "2023-01-01".to_string(),
        }])),
        keys: Arc::new(RwLock::new(vec![PublicKey {
            id: 1,
            title: "Laptop".to_string(),
            key: "ssh-rsa AAA...".to_string(),
            fingerprint: "SHA256:...".to_string(),
        }])),
        hooks: Arc::new(RwLock::new(vec![Webhook {
            id: 1,
            repo_id: 1,
            url: "http://example.com/hook".to_string(),
            events: vec!["push".to_string()],
            active: true,
        }])),
        activities: Arc::new(RwLock::new(vec![Activity {
            id: 1,
            repo_id: 1,
            user_id: 1,
            user_name: "admin".to_string(),
            op_type: "create_repo".to_string(),
            content: "created repository codeza".to_string(),
            created: "2023-01-01".to_string(),
        }])),
        commits: Arc::new(RwLock::new(vec![Commit {
            sha: "abc123456789".to_string(),
            repo_id: 1,
            message: "Initial commit".to_string(),
            author: user.clone(),
            date: "2023-01-01T12:00:00Z".to_string(),
        }])),
        lfs_locks: Arc::new(RwLock::new(vec![])),
        topics: Arc::new(RwLock::new(vec![Topic {
            id: 1,
            repo_id: 1,
            name: "rust".to_string(),
            created: "2023-01-01".to_string(),
        }])),
        packages: Arc::new(RwLock::new(vec![Package {
            id: 1,
            owner: "admin".to_string(),
            name: "my-lib".to_string(),
            version: "1.0.0".to_string(),
            package_type: "cargo".to_string(),
        }])),
        teams: Arc::new(RwLock::new(vec![Team {
            id: 1,
            org_name: "codeza-org".to_string(),
            name: "Developers".to_string(),
            description: Some("Dev Team".to_string()),
            permission: "write".to_string(),
        }])),
        projects: Arc::new(RwLock::new(vec![])),
        project_columns: Arc::new(RwLock::new(vec![])),
        project_cards: Arc::new(RwLock::new(vec![])),
        reviews: Arc::new(RwLock::new(vec![])),
        orgs: Arc::new(RwLock::new(vec![Organization {
            id: 1,
            username: "codeza-org".to_string(),
            description: Some("Codeza Organization".to_string()),
            avatar_url: None,
            website: None,
            location: None,
            email: None,
            visibility: None,
        }])),
        org_members: Arc::new(RwLock::new(vec![
            shared::OrgMember {
                user: User::new(1, "admin".to_string(), Some("admin@codeza.com".to_string())),
                role: "owner".to_string(),
            },
            shared::OrgMember {
                user: User::new(2, "user".to_string(), Some("user@example.com".to_string())),
                role: "developer".to_string(),
            },
        ])),
        workflows: Arc::new(RwLock::new(vec![ActionWorkflow {
            id: 1,
            repo_id: 1,
            name: "CI".to_string(),
            status: "active".to_string(),
        }])),
        workflow_runs: Arc::new(RwLock::new(vec![])),
        webhook_deliveries: Arc::new(RwLock::new(vec![])),
        protected_branches: Arc::new(RwLock::new(vec![])),
        stars: Arc::new(RwLock::new(HashMap::new())),
        discussions: Arc::new(RwLock::new(vec![])),
        discussion_comments: Arc::new(RwLock::new(vec![])),
        release_assets_data: Arc::new(RwLock::new(HashMap::new())),
        gpg_keys: Arc::new(RwLock::new(vec![])),
        watchers: Arc::new(RwLock::new(HashMap::new())),
        followers: Arc::new(RwLock::new(HashMap::new())),
        following: Arc::new(RwLock::new(HashMap::new())),
        oauth2_apps: Arc::new(Mutex::new(vec![])),
        commit_statuses: Arc::new(RwLock::new(vec![])),
        wikis: Arc::new(RwLock::new(wikis_map)),
        audit_logs: Arc::new(RwLock::new(vec![shared::AuditLog {
            id: 1,
            actor: user.clone(),
            action: "org.create".to_string(),
            target_type: "org".to_string(),
            target_name: "codeza-org".to_string(),
            details: "Created organization codeza-org with Owner role".to_string(),
            ip_address: Some("127.0.0.1".to_string()),
            created_at: "2023-01-01T00:00:00Z".to_string(),
        }])),
    }
}
