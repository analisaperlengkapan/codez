//! Dynamic module loading for MFEs

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Module loading result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadedModule {
    pub id: Uuid,
    pub name: String,
    pub url: String,
    pub loaded_at: chrono::DateTime<chrono::Utc>,
    pub cache_key: String,
}

/// Module loader trait
#[async_trait]
pub trait ModuleLoader: Send + Sync {
    /// Load module from URL
    async fn load(&self, name: &str, url: &str) -> Result<LoadedModule, String>;

    /// Unload module
    async fn unload(&self, module_id: Uuid) -> Result<(), String>;

    /// Get loaded module
    async fn get_module(&self, module_id: Uuid) -> Result<LoadedModule, String>;

    /// List loaded modules
    async fn list_modules(&self) -> Result<Vec<LoadedModule>, String>;

    /// Clear cache
    async fn clear_cache(&self) -> Result<(), String>;
}

/// In-memory module loader
pub struct InMemoryModuleLoader {
    modules: std::sync::Arc<tokio::sync::RwLock<HashMap<Uuid, LoadedModule>>>,
    cache: std::sync::Arc<tokio::sync::RwLock<HashMap<String, Vec<u8>>>>,
}

impl InMemoryModuleLoader {
    /// Create new in-memory module loader
    pub fn new() -> Self {
        Self {
            modules: std::sync::Arc::new(tokio::sync::RwLock::new(HashMap::new())),
            cache: std::sync::Arc::new(tokio::sync::RwLock::new(HashMap::new())),
        }
    }
}

impl Default for InMemoryModuleLoader {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ModuleLoader for InMemoryModuleLoader {
    async fn load(&self, name: &str, url: &str) -> Result<LoadedModule, String> {
        let module_id = Uuid::new_v4();
        let cache_key = format!("{}:{}", name, url);

        let module = LoadedModule {
            id: module_id,
            name: name.to_string(),
            url: url.to_string(),
            loaded_at: chrono::Utc::now(),
            cache_key: cache_key.clone(),
        };

        let mut modules = self.modules.write().await;
        modules.insert(module_id, module.clone());

        tracing::info!("Loaded module: {} from {}", name, url);
        Ok(module)
    }

    async fn unload(&self, module_id: Uuid) -> Result<(), String> {
        let mut modules = self.modules.write().await;
        modules.remove(&module_id);

        tracing::info!("Unloaded module: {}", module_id);
        Ok(())
    }

    async fn get_module(&self, module_id: Uuid) -> Result<LoadedModule, String> {
        let modules = self.modules.read().await;
        modules
            .get(&module_id)
            .cloned()
            .ok_or_else(|| format!("Module not found: {}", module_id))
    }

    async fn list_modules(&self) -> Result<Vec<LoadedModule>, String> {
        let modules = self.modules.read().await;
        Ok(modules.values().cloned().collect())
    }

    async fn clear_cache(&self) -> Result<(), String> {
        let mut cache = self.cache.write().await;
        cache.clear();

        tracing::info!("Cleared module cache");
        Ok(())
    }
}

/// Remote module loader (for HTTP-based loading)
pub struct RemoteModuleLoader {
    client: reqwest::Client,
}

impl RemoteModuleLoader {
    /// Create new remote module loader
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }
}

impl Default for RemoteModuleLoader {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ModuleLoader for RemoteModuleLoader {
    async fn load(&self, name: &str, url: &str) -> Result<LoadedModule, String> {
        let module_id = Uuid::new_v4();
        let cache_key = format!("{}:{}", name, url);

        // Simulate HTTP fetch
        tracing::info!("Loading module {} from remote: {}", name, url);

        let module = LoadedModule {
            id: module_id,
            name: name.to_string(),
            url: url.to_string(),
            loaded_at: chrono::Utc::now(),
            cache_key,
        };

        Ok(module)
    }

    async fn unload(&self, module_id: Uuid) -> Result<(), String> {
        tracing::info!("Unloading remote module: {}", module_id);
        Ok(())
    }

    async fn get_module(&self, module_id: Uuid) -> Result<LoadedModule, String> {
        Err(format!("Module not found: {}", module_id))
    }

    async fn list_modules(&self) -> Result<Vec<LoadedModule>, String> {
        Ok(vec![])
    }

    async fn clear_cache(&self) -> Result<(), String> {
        tracing::info!("Cleared remote module cache");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_in_memory_loader() {
        let loader = InMemoryModuleLoader::new();

        let result = loader
            .load("dashboard", "http://localhost:3001/remoteEntry.js")
            .await;
        assert!(result.is_ok());

        let module = result.unwrap();
        assert_eq!(module.name, "dashboard");

        let retrieved = loader.get_module(module.id).await;
        assert!(retrieved.is_ok());
    }

    #[tokio::test]
    async fn test_list_modules() {
        let loader = InMemoryModuleLoader::new();

        let _ = loader
            .load("dashboard", "http://localhost:3001/remoteEntry.js")
            .await;
        let _ = loader
            .load("profile", "http://localhost:3002/remoteEntry.js")
            .await;

        let modules = loader.list_modules().await;
        assert!(modules.is_ok());
        assert_eq!(modules.unwrap().len(), 2);
    }

    #[tokio::test]
    async fn test_unload_module() {
        let loader = InMemoryModuleLoader::new();

        let result = loader
            .load("dashboard", "http://localhost:3001/remoteEntry.js")
            .await;
        assert!(result.is_ok());

        let module = result.unwrap();
        let unload_result = loader.unload(module.id).await;
        assert!(unload_result.is_ok());

        let retrieved = loader.get_module(module.id).await;
        assert!(retrieved.is_err());
    }
}
