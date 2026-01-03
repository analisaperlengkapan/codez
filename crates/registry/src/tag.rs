//! Image tag management

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Image tag
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageTag {
    pub id: Uuid,
    pub image_id: Uuid,
    pub name: String,
    pub version: Option<String>,
    pub is_latest: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Tag policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagPolicy {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub retention_days: u32,
    pub max_tags: Option<u32>,
    pub pattern: Option<String>,
    pub enabled: bool,
}

/// Semantic version
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct SemanticVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub prerelease: Option<String>,
}

impl SemanticVersion {
    /// Parse semantic version from string
    pub fn parse(version: &str) -> Result<Self, String> {
        let version = version.trim_start_matches('v');

        let (core, prerelease) = match version.find('-') {
            Some(idx) => (&version[..idx], Some(version[idx + 1..].to_string())),
            None => (version, None),
        };

        let parts: Vec<&str> = core.split('.').collect();
        if parts.len() != 3 {
            return Err("Invalid semantic version format".to_string());
        }

        let major = parts[0]
            .parse::<u32>()
            .map_err(|_| "Invalid major version".to_string())?;
        let minor = parts[1]
            .parse::<u32>()
            .map_err(|_| "Invalid minor version".to_string())?;
        let patch = parts[2]
            .parse::<u32>()
            .map_err(|_| "Invalid patch version".to_string())?;

        Ok(SemanticVersion {
            major,
            minor,
            patch,
            prerelease,
        })
    }

    /// Convert to string
    pub fn to_string(&self) -> String {
        let base = format!("{}.{}.{}", self.major, self.minor, self.patch);
        if let Some(prerelease) = &self.prerelease {
            format!("{}-{}", base, prerelease)
        } else {
            base
        }
    }
}

impl ImageTag {
    /// Create new tag
    pub fn new(image_id: Uuid, name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            image_id,
            name,
            version: None,
            is_latest: false,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }

    /// Set as latest
    pub fn set_latest(&mut self) {
        self.is_latest = true;
        self.updated_at = chrono::Utc::now();
    }

    /// Unset as latest
    pub fn unset_latest(&mut self) {
        self.is_latest = false;
        self.updated_at = chrono::Utc::now();
    }
}

impl TagPolicy {
    /// Create new tag policy
    pub fn new(name: String, retention_days: u32) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            description: None,
            retention_days,
            max_tags: None,
            pattern: None,
            enabled: true,
        }
    }

    /// Check if tag matches policy
    pub fn matches(&self, tag: &str) -> bool {
        if let Some(pattern) = &self.pattern {
            // Simple pattern matching (can be extended with regex)
            tag.contains(pattern)
        } else {
            true
        }
    }

    /// Check if tag should be retained
    pub fn should_retain(&self, tag: &ImageTag) -> bool {
        let now = chrono::Utc::now();
        let age_days = (now - tag.created_at).num_days() as u32;

        if age_days > self.retention_days {
            return false;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_semantic_version_parse() {
        let version = SemanticVersion::parse("1.2.3").unwrap();
        assert_eq!(version.major, 1);
        assert_eq!(version.minor, 2);
        assert_eq!(version.patch, 3);
        assert_eq!(version.prerelease, None);
    }

    #[test]
    fn test_semantic_version_with_prerelease() {
        let version = SemanticVersion::parse("1.2.3-alpha.1").unwrap();
        assert_eq!(version.major, 1);
        assert_eq!(version.minor, 2);
        assert_eq!(version.patch, 3);
        assert_eq!(version.prerelease, Some("alpha.1".to_string()));
    }

    #[test]
    fn test_semantic_version_to_string() {
        let version = SemanticVersion {
            major: 1,
            minor: 2,
            patch: 3,
            prerelease: None,
        };
        assert_eq!(version.to_string(), "1.2.3");
    }

    #[test]
    fn test_image_tag_creation() {
        let image_id = Uuid::new_v4();
        let tag = ImageTag::new(image_id, "latest".to_string());

        assert_eq!(tag.image_id, image_id);
        assert_eq!(tag.name, "latest");
        assert!(!tag.is_latest);
    }

    #[test]
    fn test_tag_policy_matching() {
        let policy = TagPolicy::new("release".to_string(), 30);
        assert!(policy.matches("release-1.0.0"));
        assert!(policy.matches("release-2.0.0"));
    }
}
