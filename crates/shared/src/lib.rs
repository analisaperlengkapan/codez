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
    }

    #[test]
    fn test_repository_serialization() {
        let repo = Repository::new(1, "codeza".to_string(), "jules".to_string());
        let json = serde_json::to_string(&repo).unwrap();
        let deserialized: Repository = serde_json::from_str(&json).unwrap();
        assert_eq!(repo, deserialized);
    }
}
