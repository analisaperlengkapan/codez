//! Module Federation configuration and management

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Module Federation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FederationConfig {
    pub id: Uuid,
    pub name: String,
    pub remotes: HashMap<String, RemoteConfig>,
    pub exposes: HashMap<String, ExposeConfig>,
    pub shared: HashMap<String, SharedConfig>,
    pub filename: String,
}

/// Remote MFE configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteConfig {
    pub url: String,
    pub scope: String,
    pub module: String,
}

/// Exposed module configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExposeConfig {
    pub import: String,
    pub shared_scope: String,
}

/// Shared dependency configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedConfig {
    pub singleton: bool,
    pub strict_version: bool,
    pub eager: bool,
    pub required_version: Option<String>,
}

/// Dependency resolution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyResolution {
    pub name: String,
    pub version: String,
    pub resolved_from: String,
    pub is_shared: bool,
}

impl FederationConfig {
    /// Create new federation config
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            remotes: HashMap::new(),
            exposes: HashMap::new(),
            shared: HashMap::new(),
            filename: "remoteEntry.js".to_string(),
        }
    }

    /// Add remote MFE
    pub fn add_remote(&mut self, name: String, config: RemoteConfig) {
        self.remotes.insert(name, config);
    }

    /// Add exposed module
    pub fn add_expose(&mut self, name: String, config: ExposeConfig) {
        self.exposes.insert(name, config);
    }

    /// Add shared dependency
    pub fn add_shared(&mut self, name: String, config: SharedConfig) {
        self.shared.insert(name, config);
    }

    /// Resolve dependency
    pub fn resolve_dependency(&self, name: &str, version: &str) -> Option<DependencyResolution> {
        if let Some(shared) = self.shared.get(name) {
            Some(DependencyResolution {
                name: name.to_string(),
                version: version.to_string(),
                resolved_from: "shared".to_string(),
                is_shared: true,
            })
        } else {
            Some(DependencyResolution {
                name: name.to_string(),
                version: version.to_string(),
                resolved_from: "local".to_string(),
                is_shared: false,
            })
        }
    }

    /// Get all remotes
    pub fn get_remotes(&self) -> Vec<(&String, &RemoteConfig)> {
        self.remotes.iter().collect()
    }

    /// Get all exposes
    pub fn get_exposes(&self) -> Vec<(&String, &ExposeConfig)> {
        self.exposes.iter().collect()
    }

    /// Get all shared dependencies
    pub fn get_shared(&self) -> Vec<(&String, &SharedConfig)> {
        self.shared.iter().collect()
    }
}

impl RemoteConfig {
    /// Create new remote config
    pub fn new(url: String, scope: String, module: String) -> Self {
        Self { url, scope, module }
    }
}

impl ExposeConfig {
    /// Create new expose config
    pub fn new(import: String) -> Self {
        Self {
            import,
            shared_scope: "default".to_string(),
        }
    }
}

impl SharedConfig {
    /// Create new shared config
    pub fn new() -> Self {
        Self {
            singleton: false,
            strict_version: false,
            eager: false,
            required_version: None,
        }
    }

    /// Set as singleton
    pub fn singleton(mut self) -> Self {
        self.singleton = true;
        self
    }

    /// Set strict version
    pub fn strict_version(mut self) -> Self {
        self.strict_version = true;
        self
    }

    /// Set as eager
    pub fn eager(mut self) -> Self {
        self.eager = true;
        self
    }
}

impl Default for SharedConfig {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_federation_config_creation() {
        let config = FederationConfig::new("app-shell".to_string());
        assert_eq!(config.name, "app-shell");
        assert!(config.remotes.is_empty());
        assert!(config.exposes.is_empty());
    }

    #[test]
    fn test_add_remote() {
        let mut config = FederationConfig::new("app-shell".to_string());

        let remote = RemoteConfig::new(
            "http://localhost:3001/remoteEntry.js".to_string(),
            "@app/dashboard".to_string(),
            "dashboard".to_string(),
        );

        config.add_remote("dashboard".to_string(), remote);
        assert_eq!(config.remotes.len(), 1);
    }

    #[test]
    fn test_add_expose() {
        let mut config = FederationConfig::new("dashboard".to_string());

        let expose = ExposeConfig::new("./src/index.ts".to_string());
        config.add_expose("./Button".to_string(), expose);

        assert_eq!(config.exposes.len(), 1);
    }

    #[test]
    fn test_shared_dependency_resolution() {
        let mut config = FederationConfig::new("app-shell".to_string());

        let shared = SharedConfig::new().singleton();
        config.add_shared("react".to_string(), shared);

        let resolution = config.resolve_dependency("react", "18.0.0");
        assert!(resolution.is_some());
        assert!(resolution.unwrap().is_shared);
    }
}
