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
        }
    }
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
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Issue {
    pub id: u64,
    pub number: u64,
    pub title: String,
    pub body: Option<String>,
    pub state: String,
    pub user: User,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateIssueOption {
    pub title: String,
    pub body: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PullRequest {
    pub id: u64,
    pub number: u64,
    pub title: String,
    pub body: Option<String>,
    pub state: String,
    pub user: User,
    pub merged: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreatePullRequestOption {
    pub title: String,
    pub body: Option<String>,
    pub head: String,
    pub base: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FileEntry {
    pub name: String,
    pub path: String,
    pub kind: String, // "file" or "dir"
    pub size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Commit {
    pub sha: String,
    pub message: String,
    pub author: User,
    pub date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Release {
    pub id: u64,
    pub tag_name: String,
    pub name: String,
    pub body: Option<String>,
    pub draft: bool,
    pub prerelease: bool,
    pub created_at: String,
    pub author: User,
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
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Comment {
    pub id: u64,
    pub body: String,
    pub user: User,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateCommentOption {
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
pub struct Milestone {
    pub id: u64,
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
pub struct Topic {
    pub id: u64,
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
pub struct Webhook {
    pub id: u64,
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
pub struct Team {
    pub id: u64,
    pub name: String,
    pub description: Option<String>,
    pub permission: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Project {
    pub id: u64,
    pub title: String,
    pub description: Option<String>,
    pub is_closed: bool,
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
    pub user_id: u64,
    pub user_name: String,
    pub op_type: String,
    pub content: String,
    pub created: String,
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
    fn test_activity_struct() {
        let act = Activity { id: 1, user_id: 1, user_name: "u".to_string(), op_type: "push".to_string(), content: "c".to_string(), created: "d".to_string() };
        assert_eq!(act.op_type, "push");
    }

    #[test]
    fn test_admin_stats() {
        let stats = AdminStats { users: 10, repos: 20, orgs: 5, issues: 100 };
        assert_eq!(stats.users, 10);
    }

    #[test]
    fn test_team_project_structs() {
        let team = Team { id: 1, name: "dev".to_string(), description: None, permission: "write".to_string() };
        assert_eq!(team.name, "dev");

        let project = Project { id: 1, title: "v1".to_string(), description: None, is_closed: false };
        assert!(!project.is_closed);
    }

    #[test]
    fn test_keys_hooks_structs() {
        let key = PublicKey { id: 1, title: "Laptop".to_string(), key: "ssh-rsa...".to_string(), fingerprint: "sha256...".to_string() };
        assert_eq!(key.title, "Laptop");

        let hook = Webhook { id: 1, url: "http://example.com".to_string(), events: vec!["push".to_string()], active: true };
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
        let topic = Topic { id: 1, name: "rust".to_string(), created: "date".to_string() };
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
            body: "text".to_string(),
            user,
            created_at: "date".to_string(),
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
        };
        assert_eq!(org.username, "org");
    }

    #[test]
    fn test_release_structs() {
        let user = User::new(1, "u".to_string(), None);
        let rel = Release {
            id: 1,
            tag_name: "v1.0".to_string(),
            name: "Release 1.0".to_string(),
            body: None,
            draft: false,
            prerelease: false,
            created_at: "date".to_string(),
            author: user,
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
            number: 1,
            title: "PR Title".to_string(),
            body: None,
            state: "open".to_string(),
            user,
            merged: false,
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
            number: 1,
            title: "Bug".to_string(),
            body: None,
            state: "open".to_string(),
            user,
        };
        assert_eq!(issue.title, "Bug");

        let opts = CreateIssueOption {
            title: "New Bug".to_string(),
            body: Some("Description".to_string()),
        };
        assert_eq!(opts.title, "New Bug");
    }

    #[test]
    fn test_create_repo_option() {
        let opts = CreateRepoOption {
            name: "new-repo".to_string(),
            description: Some("desc".to_string()),
            private: true,
            auto_init: false,
        };
        assert_eq!(opts.name, "new-repo");
        assert!(opts.private);
    }

    #[test]
    fn test_repository_creation() {
        let repo = Repository::new(1, "codeza".to_string(), "jules".to_string());
        assert_eq!(repo.id, 1);
        assert_eq!(repo.name, "codeza");
        assert_eq!(repo.owner, "jules");
        assert_eq!(repo.private, false);
        assert_eq!(repo.stars_count, 0);
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
}
