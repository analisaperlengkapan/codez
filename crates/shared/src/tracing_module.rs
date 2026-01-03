//! Distributed tracing for observability

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Trace span
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Span {
    pub id: Uuid,
    pub trace_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub name: String,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
    pub attributes: HashMap<String, String>,
    pub events: Vec<SpanEvent>,
}

/// Span event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpanEvent {
    pub name: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub attributes: HashMap<String, String>,
}

/// Trace
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trace {
    pub id: Uuid,
    pub spans: Vec<Span>,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
}

/// Tracer
pub struct Tracer {
    traces: std::sync::Arc<tokio::sync::RwLock<Vec<Trace>>>,
}

impl Tracer {
    /// Create new tracer
    pub fn new() -> Self {
        Self {
            traces: std::sync::Arc::new(tokio::sync::RwLock::new(Vec::new())),
        }
    }

    /// Start span
    pub fn start_span(&self, trace_id: Uuid, name: String) -> Span {
        Span {
            id: Uuid::new_v4(),
            trace_id,
            parent_id: None,
            name,
            start_time: chrono::Utc::now(),
            end_time: None,
            attributes: HashMap::new(),
            events: Vec::new(),
        }
    }

    /// End span
    pub fn end_span(&self, mut span: Span) {
        span.end_time = Some(chrono::Utc::now());
    }

    /// Add event to span
    pub fn add_event(&self, span: &mut Span, event_name: String) {
        span.events.push(SpanEvent {
            name: event_name,
            timestamp: chrono::Utc::now(),
            attributes: HashMap::new(),
        });
    }

    /// Get all traces
    pub async fn traces(&self) -> Vec<Trace> {
        self.traces.read().await.clone()
    }
}

impl Default for Tracer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_span_creation() {
        let tracer = Tracer::new();
        let trace_id = Uuid::new_v4();
        let span = tracer.start_span(trace_id, "test_span".to_string());

        assert_eq!(span.trace_id, trace_id);
        assert_eq!(span.name, "test_span");
    }
}
