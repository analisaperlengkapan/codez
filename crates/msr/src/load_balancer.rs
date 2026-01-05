//! Load balancing strategies for microservices

use crate::service::ServiceInstance;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
#[allow(unused_imports)]
use uuid::Uuid;

/// Load balancing strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LoadBalancingStrategy {
    RoundRobin,
    LeastConnections,
    Random,
    IpHash,
}

/// Load balancer
pub struct LoadBalancer {
    strategy: LoadBalancingStrategy,
    current_index: Arc<AtomicUsize>,
}

impl LoadBalancer {
    /// Create new load balancer
    pub fn new(strategy: LoadBalancingStrategy) -> Self {
        Self {
            strategy,
            current_index: Arc::new(AtomicUsize::new(0)),
        }
    }

    /// Select instance using configured strategy
    pub fn select<'a>(&self, instances: &'a [ServiceInstance]) -> Option<&'a ServiceInstance> {
        if instances.is_empty() {
            return None;
        }

        match self.strategy {
            LoadBalancingStrategy::RoundRobin => self.round_robin(instances),
            LoadBalancingStrategy::LeastConnections => self.least_connections(instances),
            LoadBalancingStrategy::Random => self.random(instances),
            LoadBalancingStrategy::IpHash => self.ip_hash(instances),
        }
    }

    /// Round robin selection
    fn round_robin<'a>(&self, instances: &'a [ServiceInstance]) -> Option<&'a ServiceInstance> {
        if instances.is_empty() {
            return None;
        }
        let index = self.current_index.fetch_add(1, Ordering::SeqCst) % instances.len();
        Some(&instances[index])
    }

    /// Least connections selection
    fn least_connections<'a>(
        &self,
        instances: &'a [ServiceInstance],
    ) -> Option<&'a ServiceInstance> {
        // Simplified: just return first instance
        // In production, would track actual connections
        instances.first()
    }

    /// Random selection
    fn random<'a>(&self, instances: &'a [ServiceInstance]) -> Option<&'a ServiceInstance> {
        if instances.is_empty() {
            return None;
        }
        use std::collections::hash_map::RandomState;
        use std::hash::{BuildHasher, Hasher};

        let mut hasher = RandomState::new().build_hasher();
        hasher.write_usize(self.current_index.load(Ordering::SeqCst));
        let index = (hasher.finish() as usize) % instances.len();
        Some(&instances[index])
    }

    /// IP hash selection
    fn ip_hash<'a>(&self, instances: &'a [ServiceInstance]) -> Option<&'a ServiceInstance> {
        // Simplified: use first instance
        // In production, would hash client IP
        instances.first()
    }
}

/// Connection pool
pub struct ConnectionPool {
    max_connections: usize,
    active_connections: Arc<AtomicUsize>,
}

impl ConnectionPool {
    /// Create new connection pool
    pub fn new(max_connections: usize) -> Self {
        Self {
            max_connections,
            active_connections: Arc::new(AtomicUsize::new(0)),
        }
    }

    /// Acquire connection
    pub fn acquire(&self) -> Result<Connection, String> {
        let current = self.active_connections.load(Ordering::SeqCst);
        if current >= self.max_connections {
            return Err("Connection pool exhausted".to_string());
        }

        self.active_connections.fetch_add(1, Ordering::SeqCst);
        Ok(Connection {
            pool: self.active_connections.clone(),
        })
    }

    /// Get active connections count
    pub fn active_count(&self) -> usize {
        self.active_connections.load(Ordering::SeqCst)
    }
}

/// Connection guard
pub struct Connection {
    pool: Arc<AtomicUsize>,
}

impl Drop for Connection {
    fn drop(&mut self) {
        self.pool.fetch_sub(1, Ordering::SeqCst);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_robin() {
        let lb = LoadBalancer::new(LoadBalancingStrategy::RoundRobin);
        let instances = vec![
            ServiceInstance::new(Uuid::new_v4(), "host1".to_string(), 8001),
            ServiceInstance::new(Uuid::new_v4(), "host2".to_string(), 8002),
            ServiceInstance::new(Uuid::new_v4(), "host3".to_string(), 8003),
        ];

        let selected1 = lb.select(&instances);
        let selected2 = lb.select(&instances);

        assert!(selected1.is_some());
        assert!(selected2.is_some());
        assert_ne!(selected1.unwrap().id, selected2.unwrap().id);
    }

    #[test]
    fn test_connection_pool() {
        let pool = ConnectionPool::new(5);

        let conn1 = pool.acquire();
        assert!(conn1.is_ok());
        assert_eq!(pool.active_count(), 1);

        let conn2 = pool.acquire();
        assert!(conn2.is_ok());
        assert_eq!(pool.active_count(), 2);

        drop(conn1);
        assert_eq!(pool.active_count(), 1);
    }

    #[test]
    fn test_connection_pool_exhaustion() {
        let pool = ConnectionPool::new(2);

        let _conn1 = pool.acquire().unwrap();
        let _conn2 = pool.acquire().unwrap();

        let conn3 = pool.acquire();
        assert!(conn3.is_err());
    }
}
