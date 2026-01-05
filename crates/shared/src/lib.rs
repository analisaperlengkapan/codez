use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Repository {
    pub id: u64,
    pub name: String,
    pub description: Option<String>,
    pub private: bool,
    pub owner: String,
}

impl Repository {
    pub fn new(id: u64, name: String, owner: String) -> Self {
        Self {
            id,
            name,
            description: None,
            private: false,
            owner,
        }
    }
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
    }

    #[test]
    fn test_repository_serialization() {
        let repo = Repository::new(1, "codeza".to_string(), "jules".to_string());
        let json = serde_json::to_string(&repo).unwrap();
        let deserialized: Repository = serde_json::from_str(&json).unwrap();
        assert_eq!(repo, deserialized);
    }
}
