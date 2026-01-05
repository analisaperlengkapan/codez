//! Codeza MicroService Registry (MSR)
//! Handles service registration, discovery, health checking, and load balancing

pub mod db;
pub mod health;
pub mod load_balancer;
pub mod service;

pub use db::MicroserviceRepository;
pub use health::{HealthCheckResult, HealthChecker, HttpHealthChecker, TcpHealthChecker};
pub use load_balancer::{ConnectionPool, LoadBalancer, LoadBalancingStrategy};
pub use service::{Microservice, ServiceEndpoint, ServiceInstance, ServiceStatus};
