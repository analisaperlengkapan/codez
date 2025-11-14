//! SuperApp definition and composition

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// SuperApp definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuperApp {
    pub id: Uuid,
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub modules: Vec<AppModule>,
    pub config: AppConfig,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// App module
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppModule {
    pub id: Uuid,
    pub name: String,
    pub version: String,
    pub remote_entry: String,
    pub scope: String,
    pub dependencies: HashMap<String, String>,
}

/// App configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub theme: String,
    pub locale: String,
    pub features: HashMap<String, bool>,
    pub settings: HashMap<String, serde_json::Value>,
}

impl SuperApp {
    /// Create new SuperApp
    pub fn new(name: String, version: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            version,
            description: None,
            modules: Vec::new(),
            config: AppConfig {
                theme: "light".to_string(),
                locale: "en-US".to_string(),
                features: HashMap::new(),
                settings: HashMap::new(),
            },
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }

    /// Add module
    pub fn add_module(&mut self, module: AppModule) {
        self.modules.push(module);
        self.updated_at = chrono::Utc::now();
    }

    /// Remove module
    pub fn remove_module(&mut self, module_id: Uuid) {
        self.modules.retain(|m| m.id != module_id);
        self.updated_at = chrono::Utc::now();
    }

    /// Get module by name
    pub fn get_module(&self, name: &str) -> Option<&AppModule> {
        self.modules.iter().find(|m| m.name == name)
    }
}

impl AppModule {
    /// Create new app module
    pub fn new(name: String, version: String, remote_entry: String, scope: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            version,
            remote_entry,
            scope,
            dependencies: HashMap::new(),
        }
    }

    /// Add dependency
    pub fn add_dependency(&mut self, name: String, version: String) {
        self.dependencies.insert(name, version);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_superapp_creation() {
        let app = SuperApp::new("MyApp".to_string(), "1.0.0".to_string());
        assert_eq!(app.name, "MyApp");
        assert_eq!(app.version, "1.0.0");
        assert!(app.modules.is_empty());
    }

    #[test]
    fn test_add_module() {
        let mut app = SuperApp::new("MyApp".to_string(), "1.0.0".to_string());
        let module = AppModule::new(
            "dashboard".to_string(),
            "1.0.0".to_string(),
            "http://localhost:3001/remoteEntry.js".to_string(),
            "@app/dashboard".to_string(),
        );

        app.add_module(module);
        assert_eq!(app.modules.len(), 1);
    }
}
