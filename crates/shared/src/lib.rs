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
