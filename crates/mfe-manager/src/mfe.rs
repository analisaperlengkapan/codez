//! Micro Frontend (MFE) models and operations
use utoipa::ToSchema;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::OnceLock;
use uuid::Uuid;
use regex::Regex;

static NAME_REGEX: OnceLock<Regex> = OnceLock::new();
static URL_REGEX: OnceLock<Regex> = OnceLock::new();
static VERSION_REGEX: OnceLock<Regex> = OnceLock::new();

/// Micro Frontend definition
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MicroFrontend {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub version: String,
    pub remote_entry: String,
    pub scope: String,
    pub dependencies: HashMap<String, String>,
    pub shared_dependencies: Vec<SharedDependency>,
    pub status: MFEStatus,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// MFE status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub enum MFEStatus {
    Active,
    Inactive,
    Deprecated,
    Maintenance,
}

/// Shared dependency
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SharedDependency {
    pub name: String,
    pub version: String,
    pub singleton: bool,
    pub strict_version: bool,
}

/// MFE manifest
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MFEManifest {
    pub name: String,
    pub version: String,
    pub remotes: HashMap<String, String>,
    pub exposes: HashMap<String, String>,
    pub shared: HashMap<String, SharedConfig>,
}

/// Shared dependency configuration
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SharedConfig {
    pub singleton: bool,
    pub strict_version: bool,
    pub eager: bool,
    pub required_version: Option<String>,
}

/// MFE registry entry
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MFERegistry {
    pub mfes: Vec<MicroFrontend>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl MicroFrontend {
    /// Create new MFE
    pub fn new(name: String, version: String, remote_entry: String, scope: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            description: None,
            version,
            remote_entry,
            scope,
            dependencies: HashMap::new(),
            shared_dependencies: Vec::new(),
            status: MFEStatus::Active,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }

    /// Add dependency
    pub fn add_dependency(&mut self, name: String, version: String) {
        self.dependencies.insert(name, version);
        self.updated_at = chrono::Utc::now();
    }

    /// Add shared dependency
    pub fn add_shared_dependency(&mut self, dep: SharedDependency) {
        self.shared_dependencies.push(dep);
        self.updated_at = chrono::Utc::now();
    }

    /// Set status
    pub fn set_status(&mut self, status: MFEStatus) {
        self.status = status;
        self.updated_at = chrono::Utc::now();
    }

    /// Validate MFE configuration
    pub fn validate(&self) -> Result<(), String> {
        // Validate name (alphanumeric, kebab-case)
        let name_regex = NAME_REGEX.get_or_init(|| Regex::new(r"^[a-z0-9]+(?:-[a-z0-9]+)*$").unwrap());
        if !name_regex.is_match(&self.name) {
            return Err("name must be kebab-case (e.g., my-app)".to_string());
        }

        // Validate remote_entry URL
        let url_regex = URL_REGEX.get_or_init(|| {
            Regex::new(r"^https?://[a-zA-Z0-9.-]+(?::\d+)?(?:/[a-zA-Z0-9._-]+)*$").unwrap()
        });
        if !url_regex.is_match(&self.remote_entry) {
            return Err("remote_entry must be a valid HTTP/HTTPS URL".to_string());
        }

        // Basic check for empty fields
        if self.name.is_empty() {
            return Err("name cannot be empty".to_string());
        }
        if self.version.is_empty() {
            return Err("version cannot be empty".to_string());
        }

        // Semantic version check (x.y.z)
        let version_regex = VERSION_REGEX.get_or_init(|| {
            Regex::new(r"^\d+\.\d+\.\d+(?:-[0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*)?(?:\+[0-9A-Za-z-]+)?$")
                .unwrap()
        });
        if !version_regex.is_match(&self.version) {
            return Err("version must follow semantic versioning (e.g. 1.0.0)".to_string());
        }

        if self.scope.is_empty() {
            return Err("scope cannot be empty".to_string());
        }

        Ok(())
    }
}

impl MFERegistry {
    /// Create new registry
    pub fn new() -> Self {
        Self {
            mfes: Vec::new(),
            updated_at: chrono::Utc::now(),
        }
    }

    /// Register MFE
    pub fn register(&mut self, mfe: MicroFrontend) {
        self.mfes.push(mfe);
        self.updated_at = chrono::Utc::now();
    }

    /// Unregister MFE
    pub fn unregister(&mut self, mfe_id: Uuid) {
        self.mfes.retain(|m| m.id != mfe_id);
        self.updated_at = chrono::Utc::now();
    }

    /// Get MFE by ID
    pub fn get_mfe(&self, mfe_id: Uuid) -> Option<&MicroFrontend> {
        self.mfes.iter().find(|m| m.id == mfe_id)
    }

    /// Get MFE by name
    pub fn get_mfe_by_name(&self, name: &str) -> Option<&MicroFrontend> {
        self.mfes.iter().find(|m| m.name == name)
    }

    /// List active MFEs
    pub fn list_active(&self) -> Vec<&MicroFrontend> {
        self.mfes
            .iter()
            .filter(|m| m.status == MFEStatus::Active)
            .collect()
    }
}

impl Default for MFERegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mfe_creation() {
        let mfe = MicroFrontend::new(
            "dashboard".to_string(),
            "1.0.0".to_string(),
            "http://localhost:3001/remoteEntry.js".to_string(),
            "@app/dashboard".to_string(),
        );

        assert_eq!(mfe.name, "dashboard");
        assert_eq!(mfe.version, "1.0.0");
        assert_eq!(mfe.status, MFEStatus::Active);
    }

    #[test]
    fn test_mfe_dependencies() {
        let mut mfe = MicroFrontend::new(
            "dashboard".to_string(),
            "1.0.0".to_string(),
            "http://localhost:3001/remoteEntry.js".to_string(),
            "@app/dashboard".to_string(),
        );

        mfe.add_dependency("react".to_string(), "^18.0.0".to_string());
        mfe.add_dependency("react-dom".to_string(), "^18.0.0".to_string());

        assert_eq!(mfe.dependencies.len(), 2);
    }

    #[test]
    fn test_mfe_registry() {
        let mut registry = MFERegistry::new();

        let mfe1 = MicroFrontend::new(
            "dashboard".to_string(),
            "1.0.0".to_string(),
            "http://localhost:3001/remoteEntry.js".to_string(),
            "@app/dashboard".to_string(),
        );

        let mfe2 = MicroFrontend::new(
            "profile".to_string(),
            "1.0.0".to_string(),
            "http://localhost:3002/remoteEntry.js".to_string(),
            "@app/profile".to_string(),
        );

        registry.register(mfe1.clone());
        registry.register(mfe2.clone());

        assert_eq!(registry.mfes.len(), 2);
        assert_eq!(registry.list_active().len(), 2);

        registry.unregister(mfe1.id);
        assert_eq!(registry.mfes.len(), 1);
    }

    #[test]
    fn test_mfe_validation() {
        // Valid MFE
        let mfe = MicroFrontend::new(
            "my-app".to_string(),
            "1.0.0".to_string(),
            "http://localhost:3000/remoteEntry.js".to_string(),
            "@app/my-app".to_string(),
        );
        assert!(mfe.validate().is_ok());

        // Invalid Name (spaces)
        let mfe = MicroFrontend::new(
            "my app".to_string(),
            "1.0.0".to_string(),
            "http://localhost:3000/remoteEntry.js".to_string(),
            "@app/my-app".to_string(),
        );
        assert!(mfe.validate().is_err());

        // Invalid Name (uppercase)
        let mfe = MicroFrontend::new(
            "MyApp".to_string(),
            "1.0.0".to_string(),
            "http://localhost:3000/remoteEntry.js".to_string(),
            "@app/my-app".to_string(),
        );
        assert!(mfe.validate().is_err());

        // Invalid Version
        let mfe = MicroFrontend::new(
            "my-app".to_string(),
            "v1.0".to_string(),
            "http://localhost:3000/remoteEntry.js".to_string(),
            "@app/my-app".to_string(),
        );
        assert!(mfe.validate().is_err());

        // Invalid URL
        let mfe = MicroFrontend::new(
            "my-app".to_string(),
            "1.0.0".to_string(),
            "not-a-url".to_string(),
            "@app/my-app".to_string(),
        );
        assert!(mfe.validate().is_err());
    }
}
