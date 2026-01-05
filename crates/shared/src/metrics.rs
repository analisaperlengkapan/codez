//! Metrics collection for monitoring

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

/// Metric type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MetricType {
    Counter,
    Gauge,
    Histogram,
    Summary,
}

/// Metric value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricValue {
    pub name: String,
    pub metric_type: MetricType,
    pub value: f64,
    pub labels: HashMap<String, String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Counter metric
#[derive(Clone)]
pub struct Counter {
    value: Arc<AtomicU64>,
    #[allow(dead_code)]
    name: String,
    #[allow(dead_code)]
    labels: HashMap<String, String>,
}

/// Gauge metric
#[derive(Clone)]
pub struct Gauge {
    value: Arc<std::sync::RwLock<f64>>,
    #[allow(dead_code)]
    name: String,
    #[allow(dead_code)]
    labels: HashMap<String, String>,
}

/// Histogram metric
#[derive(Clone)]
pub struct Histogram {
    buckets: Arc<std::sync::RwLock<Vec<u64>>>,
    #[allow(dead_code)]
    name: String,
    #[allow(dead_code)]
    labels: HashMap<String, String>,
}

/// Metrics registry
pub struct MetricsRegistry {
    counters: Arc<std::sync::RwLock<HashMap<String, Counter>>>,
    gauges: Arc<std::sync::RwLock<HashMap<String, Gauge>>>,
    histograms: Arc<std::sync::RwLock<HashMap<String, Histogram>>>,
}

impl Clone for MetricsRegistry {
    fn clone(&self) -> Self {
        Self {
            counters: Arc::clone(&self.counters),
            gauges: Arc::clone(&self.gauges),
            histograms: Arc::clone(&self.histograms),
        }
    }
}

impl Counter {
    /// Create new counter
    pub fn new(name: String) -> Self {
        Self {
            value: Arc::new(AtomicU64::new(0)),
            name,
            labels: HashMap::new(),
        }
    }

    /// Increment counter
    pub fn inc(&self) {
        self.value.fetch_add(1, Ordering::SeqCst);
    }

    /// Increment by value
    pub fn add(&self, value: u64) {
        self.value.fetch_add(value, Ordering::SeqCst);
    }

    /// Get current value
    pub fn value(&self) -> u64 {
        self.value.load(Ordering::SeqCst)
    }
}

impl Gauge {
    /// Create new gauge
    pub fn new(name: String) -> Self {
        Self {
            value: Arc::new(std::sync::RwLock::new(0.0)),
            name,
            labels: HashMap::new(),
        }
    }

    /// Set gauge value
    pub fn set(&self, value: f64) {
        let mut v = self.value.write().unwrap();
        *v = value;
    }

    /// Get gauge value
    pub fn value(&self) -> f64 {
        *self.value.read().unwrap()
    }

    /// Increment gauge
    pub fn inc(&self) {
        let mut v = self.value.write().unwrap();
        *v += 1.0;
    }

    /// Decrement gauge
    pub fn dec(&self) {
        let mut v = self.value.write().unwrap();
        *v -= 1.0;
    }
}

impl Histogram {
    /// Create new histogram
    pub fn new(name: String) -> Self {
        Self {
            buckets: Arc::new(std::sync::RwLock::new(vec![0; 10])),
            name,
            labels: HashMap::new(),
        }
    }

    /// Observe value
    pub fn observe(&self, value: u64) {
        let mut buckets = self.buckets.write().unwrap();
        if (value as usize) < buckets.len() {
            buckets[value as usize] += 1;
        }
    }

    /// Get bucket values
    pub fn buckets(&self) -> Vec<u64> {
        self.buckets.read().unwrap().clone()
    }
}

impl MetricsRegistry {
    /// Create new metrics registry
    pub fn new() -> Self {
        Self {
            counters: Arc::new(std::sync::RwLock::new(HashMap::new())),
            gauges: Arc::new(std::sync::RwLock::new(HashMap::new())),
            histograms: Arc::new(std::sync::RwLock::new(HashMap::new())),
        }
    }

    /// Register counter
    pub fn register_counter(&self, name: String) -> Counter {
        let mut counters = self.counters.write().unwrap();
        if let Some(existing) = counters.get(&name) {
            return existing.clone();
        }

        let counter = Counter::new(name.clone());
        counters.insert(name, counter.clone());
        counter
    }

    /// Register gauge
    pub fn register_gauge(&self, name: String) -> Gauge {
        let mut gauges = self.gauges.write().unwrap();
        if let Some(existing) = gauges.get(&name) {
            return existing.clone();
        }

        let gauge = Gauge::new(name.clone());
        gauges.insert(name, gauge.clone());
        gauge
    }

    /// Get all metrics
    pub fn collect(&self) -> Vec<MetricValue> {
        let mut metrics = Vec::new();

        let counters = self.counters.read().unwrap();
        for (name, counter) in counters.iter() {
            metrics.push(MetricValue {
                name: name.clone(),
                metric_type: MetricType::Counter,
                value: counter.value() as f64,
                labels: HashMap::new(),
                timestamp: chrono::Utc::now(),
            });
        }

        let gauges = self.gauges.read().unwrap();
        for (name, gauge) in gauges.iter() {
            metrics.push(MetricValue {
                name: name.clone(),
                metric_type: MetricType::Gauge,
                value: gauge.value(),
                labels: HashMap::new(),
                timestamp: chrono::Utc::now(),
            });
        }

        metrics
    }
}

impl Default for MetricsRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter() {
        let counter = Counter::new("test_counter".to_string());
        counter.inc();
        counter.add(5);
        assert_eq!(counter.value(), 6);
    }

    #[test]
    fn test_gauge() {
        let gauge = Gauge::new("test_gauge".to_string());
        gauge.set(10.0);
        assert_eq!(gauge.value(), 10.0);
        gauge.inc();
        assert_eq!(gauge.value(), 11.0);
    }

    #[test]
    fn test_histogram() {
        let histogram = Histogram::new("test_histogram".to_string());
        histogram.observe(5);
        histogram.observe(5);
        let buckets = histogram.buckets();
        assert_eq!(buckets[5], 2);
    }
}
