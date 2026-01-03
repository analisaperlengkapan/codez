//! Codeza MicroService Registry (MSR)
//! Handles service registration, discovery, health checking, and load balancing

pub mod service;
pub mod health;
pub mod load_balancer;

pub use service::{Microservice, ServiceStatus, ServiceEndpoint, ServiceInstance};
pub use health::{HealthChecker, HttpHealthChecker, TcpHealthChecker, HealthCheckResult};
pub use load_balancer::{LoadBalancer, LoadBalancingStrategy, ConnectionPool};
