//! Health checking for microservices

use crate::service::{ServiceInstance, ServiceStatus};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Health check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckResult {
    pub instance_id: Uuid,
    pub status: ServiceStatus,
    pub response_time: u64, // milliseconds
    pub checked_at: chrono::DateTime<chrono::Utc>,
    pub error: Option<String>,
}

/// Health checker trait
#[async_trait]
pub trait HealthChecker: Send + Sync {
    /// Check health of instance
    async fn check(&self, instance: &ServiceInstance) -> HealthCheckResult;

    /// Check multiple instances
    async fn check_batch(&self, instances: &[ServiceInstance]) -> Vec<HealthCheckResult>;
}

/// HTTP health checker
pub struct HttpHealthChecker {
    client: reqwest::Client,
    timeout_ms: u64,
}

impl HttpHealthChecker {
    /// Create new HTTP health checker
    pub fn new(timeout_ms: u64) -> Self {
        Self {
            client: reqwest::Client::new(),
            timeout_ms,
        }
    }
}

#[async_trait]
impl HealthChecker for HttpHealthChecker {
    async fn check(&self, instance: &ServiceInstance) -> HealthCheckResult {
        let start = std::time::Instant::now();
        let health_url = format!("{}/health", instance.url());

        let status = match tokio::time::timeout(
            tokio::time::Duration::from_millis(self.timeout_ms),
            self.client.get(&health_url).send(),
        )
        .await
        {
            Ok(Ok(response)) if response.status().is_success() => ServiceStatus::Healthy,
            Ok(Ok(_)) => ServiceStatus::Unhealthy,
            Ok(Err(e)) => {
                return HealthCheckResult {
                    instance_id: instance.id,
                    status: ServiceStatus::Unhealthy,
                    response_time: start.elapsed().as_millis() as u64,
                    checked_at: chrono::Utc::now(),
                    error: Some(e.to_string()),
                };
            }
            Err(_) => ServiceStatus::Unhealthy,
        };

        HealthCheckResult {
            instance_id: instance.id,
            status,
            response_time: start.elapsed().as_millis() as u64,
            checked_at: chrono::Utc::now(),
            error: None,
        }
    }

    async fn check_batch(&self, instances: &[ServiceInstance]) -> Vec<HealthCheckResult> {
        let futures: Vec<_> = instances.iter().map(|i| self.check(i)).collect();
        futures::future::join_all(futures).await
    }
}

/// TCP health checker
pub struct TcpHealthChecker {
    timeout_ms: u64,
}

impl TcpHealthChecker {
    /// Create new TCP health checker
    pub fn new(timeout_ms: u64) -> Self {
        Self { timeout_ms }
    }
}

#[async_trait]
impl HealthChecker for TcpHealthChecker {
    async fn check(&self, instance: &ServiceInstance) -> HealthCheckResult {
        let start = std::time::Instant::now();
        let addr = format!("{}:{}", instance.host, instance.port);

        let status = match tokio::time::timeout(
            tokio::time::Duration::from_millis(self.timeout_ms),
            tokio::net::TcpStream::connect(&addr),
        )
        .await
        {
            Ok(Ok(_)) => ServiceStatus::Healthy,
            Ok(Err(e)) => {
                return HealthCheckResult {
                    instance_id: instance.id,
                    status: ServiceStatus::Unhealthy,
                    response_time: start.elapsed().as_millis() as u64,
                    checked_at: chrono::Utc::now(),
                    error: Some(e.to_string()),
                };
            }
            Err(_) => ServiceStatus::Unhealthy,
        };

        HealthCheckResult {
            instance_id: instance.id,
            status,
            response_time: start.elapsed().as_millis() as u64,
            checked_at: chrono::Utc::now(),
            error: None,
        }
    }

    async fn check_batch(&self, instances: &[ServiceInstance]) -> Vec<HealthCheckResult> {
        let futures: Vec<_> = instances.iter().map(|i| self.check(i)).collect();
        futures::future::join_all(futures).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_tcp_health_checker() {
        let checker = TcpHealthChecker::new(1000);
        let instance = ServiceInstance::new(
            Uuid::new_v4(),
            "localhost".to_string(),
            9999, // Non-existent port
        );

        let result = checker.check(&instance).await;
        assert_eq!(result.status, ServiceStatus::Unhealthy);
    }

    #[test]
    fn test_health_check_result() {
        let result = HealthCheckResult {
            instance_id: Uuid::new_v4(),
            status: ServiceStatus::Healthy,
            response_time: 50,
            checked_at: chrono::Utc::now(),
            error: None,
        };

        assert_eq!(result.status, ServiceStatus::Healthy);
        assert_eq!(result.response_time, 50);
        assert!(result.error.is_none());
    }
}
