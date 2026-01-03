//! Event bus for SuperApp

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

/// Event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: Uuid,
    pub event_type: String,
    pub source: String,
    pub payload: Value,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Event handler type
pub type EventHandler = Arc<dyn Fn(&Event) + Send + Sync>;

/// Event bus
pub struct EventBus {
    handlers: Arc<tokio::sync::RwLock<HashMap<String, Vec<EventHandler>>>>,
    history: Arc<tokio::sync::RwLock<Vec<Event>>>,
}

impl EventBus {
    /// Create new event bus
    pub fn new() -> Self {
        Self {
            handlers: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
            history: Arc::new(tokio::sync::RwLock::new(Vec::new())),
        }
    }

    /// Subscribe to event
    pub async fn subscribe(&self, event_type: String, handler: EventHandler) {
        let mut handlers = self.handlers.write().await;
        handlers.entry(event_type).or_insert_with(Vec::new).push(handler);
    }

    /// Publish event
    pub async fn publish(&self, event: Event) {
        let handlers = self.handlers.read().await;
        if let Some(event_handlers) = handlers.get(&event.event_type) {
            for handler in event_handlers {
                handler(&event);
            }
        }

        let mut history = self.history.write().await;
        history.push(event);
    }

    /// Get event history
    pub async fn history(&self) -> Vec<Event> {
        let history = self.history.read().await;
        history.clone()
    }

    /// Clear history
    pub async fn clear_history(&self) {
        let mut history = self.history.write().await;
        history.clear();
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}

impl Event {
    /// Create new event
    pub fn new(event_type: String, source: String, payload: Value) -> Self {
        Self {
            id: Uuid::new_v4(),
            event_type,
            source,
            payload,
            timestamp: chrono::Utc::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn test_event_bus() {
        let bus = EventBus::new();
        let counter = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let counter_clone = counter.clone();

        let handler: EventHandler = Arc::new(move |_event| {
            counter_clone.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        });

        bus.subscribe("test".to_string(), handler).await;

        let event = Event::new("test".to_string(), "test".to_string(), json!({}));
        bus.publish(event).await;

        assert_eq!(counter.load(std::sync::atomic::Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn test_event_history() {
        let bus = EventBus::new();

        let event1 = Event::new("event1".to_string(), "source1".to_string(), json!({}));
        let event2 = Event::new("event2".to_string(), "source2".to_string(), json!({}));

        bus.publish(event1).await;
        bus.publish(event2).await;

        let history = bus.history().await;
        assert_eq!(history.len(), 2);
    }
}
