//! Microservice definition and models

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Microservice definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Microservice {
    pub id: Uuid,
    pub name: String,
    pub version: String,
    pub host: String,
    pub port: u16,
    pub protocol: String, // http, grpc, etc
    pub status: ServiceStatus,
    pub metadata: HashMap<String, String>,
    pub tags: Vec<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Service status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ServiceStatus {
    Healthy,
    Unhealthy,
    Maintenance,
    Unknown,
}

/// Service endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceEndpoint {
    pub service_id: Uuid,
    pub host: String,
    pub port: u16,
    pub protocol: String,
    pub weight: u32,
}

/// Service instance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceInstance {
    pub id: Uuid,
    pub service_id: Uuid,
    pub host: String,
    pub port: u16,
    pub status: ServiceStatus,
    pub metadata: HashMap<String, String>,
    pub registered_at: chrono::DateTime<chrono::Utc>,
}

impl Microservice {
    /// Create new microservice
    pub fn new(name: String, version: String, host: String, port: u16) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            version,
            host,
            port,
            protocol: "http".to_string(),
            status: ServiceStatus::Unknown,
            metadata: HashMap::new(),
            tags: Vec::new(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }

    /// Get service URL
    pub fn url(&self) -> String {
        format!("{}://{}:{}", self.protocol, self.host, self.port)
    }

    /// Add tag
    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
        }
        self.updated_at = chrono::Utc::now();
    }

    /// Add metadata
    pub fn add_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
        self.updated_at = chrono::Utc::now();
    }

    /// Set status
    pub fn set_status(&mut self, status: ServiceStatus) {
        self.status = status;
        self.updated_at = chrono::Utc::now();
    }
}

impl ServiceEndpoint {
    /// Create new service endpoint
    pub fn new(service_id: Uuid, host: String, port: u16) -> Self {
        Self {
            service_id,
            host,
            port,
            protocol: "http".to_string(),
            weight: 1,
        }
    }

    /// Get endpoint URL
    pub fn url(&self) -> String {
        format!("{}://{}:{}", self.protocol, self.host, self.port)
    }
}

impl ServiceInstance {
    /// Create new service instance
    pub fn new(service_id: Uuid, host: String, port: u16) -> Self {
        Self {
            id: Uuid::new_v4(),
            service_id,
            host,
            port,
            status: ServiceStatus::Unknown,
            metadata: HashMap::new(),
            registered_at: chrono::Utc::now(),
        }
    }

    /// Get instance URL
    pub fn url(&self) -> String {
        format!("http://{}:{}", self.host, self.port)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_microservice_creation() {
        let service = Microservice::new(
            "user-service".to_string(),
            "1.0.0".to_string(),
            "localhost".to_string(),
            8001,
        );

        assert_eq!(service.name, "user-service");
        assert_eq!(service.version, "1.0.0");
        assert_eq!(service.url(), "http://localhost:8001");
    }

    #[test]
    fn test_service_tagging() {
        let mut service = Microservice::new(
            "user-service".to_string(),
            "1.0.0".to_string(),
            "localhost".to_string(),
            8001,
        );

        service.add_tag("api".to_string());
        service.add_tag("v1".to_string());

        assert_eq!(service.tags.len(), 2);
    }

    #[test]
    fn test_service_endpoint() {
        let service_id = Uuid::new_v4();
        let endpoint = ServiceEndpoint::new(service_id, "localhost".to_string(), 8001);

        assert_eq!(endpoint.service_id, service_id);
        assert_eq!(endpoint.url(), "http://localhost:8001");
    }
}
