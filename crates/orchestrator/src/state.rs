//! State management for SuperApp

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

/// App state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppState {
    pub id: Uuid,
    pub data: HashMap<String, Value>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// State store
pub struct StateStore {
    state: Arc<tokio::sync::RwLock<AppState>>,
}

impl StateStore {
    /// Create new state store
    pub fn new() -> Self {
        Self {
            state: Arc::new(tokio::sync::RwLock::new(AppState {
                id: Uuid::new_v4(),
                data: HashMap::new(),
                updated_at: chrono::Utc::now(),
            })),
        }
    }

    /// Get state value
    pub async fn get(&self, key: &str) -> Option<Value> {
        let state = self.state.read().await;
        state.data.get(key).cloned()
    }

    /// Set state value
    pub async fn set(&self, key: String, value: Value) {
        let mut state = self.state.write().await;
        state.data.insert(key, value);
        state.updated_at = chrono::Utc::now();
    }

    /// Update state value
    pub async fn update<F>(&self, key: &str, f: F)
    where
        F: FnOnce(Value) -> Value,
    {
        let mut state = self.state.write().await;
        let current = state.data.get(key).cloned().unwrap_or(Value::Null);
        state.data.insert(key.to_string(), f(current));
        state.updated_at = chrono::Utc::now();
    }

    /// Delete state value
    pub async fn delete(&self, key: &str) {
        let mut state = self.state.write().await;
        state.data.remove(key);
        state.updated_at = chrono::Utc::now();
    }

    /// Get all state
    pub async fn get_all(&self) -> AppState {
        let state = self.state.read().await;
        state.clone()
    }

    /// Clear all state
    pub async fn clear(&self) {
        let mut state = self.state.write().await;
        state.data.clear();
        state.updated_at = chrono::Utc::now();
    }
}

impl Default for StateStore {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_state_store() {
        let store = StateStore::new();

        store.set("user".to_string(), json!({"name": "John"})).await;
        let value = store.get("user").await;

        assert!(value.is_some());
        assert_eq!(value.unwrap()["name"], "John");
    }

    #[tokio::test]
    async fn test_state_update() {
        let store = StateStore::new();

        store.set("counter".to_string(), json!(0)).await;
        store
            .update("counter", |v| {
                json!(v.as_i64().unwrap_or(0) + 1)
            })
            .await;

        let value = store.get("counter").await;
        assert_eq!(value.unwrap(), 1);
    }
}
