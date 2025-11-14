//! Circuit breaker pattern implementation

use std::sync::Arc;
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

/// Circuit breaker state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CircuitState {
    Closed,      // Normal operation
    Open,        // Failing, reject requests
    HalfOpen,    // Testing if service recovered
}

/// Circuit breaker configuration
#[derive(Debug, Clone)]
pub struct CircuitBreakerConfig {
    pub failure_threshold: u32,      // Failures before opening
    pub success_threshold: u32,      // Successes before closing from half-open
    pub timeout_seconds: u64,        // Time before trying half-open
}

impl Default for CircuitBreakerConfig {
    fn default() -> Self {
        Self {
            failure_threshold: 5,
            success_threshold: 2,
            timeout_seconds: 60,
        }
    }
}

/// Circuit breaker
pub struct CircuitBreaker {
    state: Arc<std::sync::Mutex<CircuitState>>,
    failure_count: Arc<AtomicU32>,
    success_count: Arc<AtomicU32>,
    last_failure_time: Arc<AtomicU64>,
    config: CircuitBreakerConfig,
}

impl CircuitBreaker {
    pub fn new(config: CircuitBreakerConfig) -> Self {
        Self {
            state: Arc::new(std::sync::Mutex::new(CircuitState::Closed)),
            failure_count: Arc::new(AtomicU32::new(0)),
            success_count: Arc::new(AtomicU32::new(0)),
            last_failure_time: Arc::new(AtomicU64::new(0)),
            config,
        }
    }

    pub fn call<F, T>(&self, f: F) -> Result<T, String>
    where
        F: FnOnce() -> Result<T, String>,
    {
        let mut state = self.state.lock().unwrap();

        match *state {
            CircuitState::Closed => {
                match f() {
                    Ok(result) => {
                        self.failure_count.store(0, Ordering::SeqCst);
                        Ok(result)
                    }
                    Err(e) => {
                        let failures = self.failure_count.fetch_add(1, Ordering::SeqCst) + 1;
                        let now = SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap()
                            .as_secs();
                        self.last_failure_time.store(now, Ordering::SeqCst);

                        if failures >= self.config.failure_threshold {
                            *state = CircuitState::Open;
                            tracing::warn!("Circuit breaker opened after {} failures", failures);
                        }

                        Err(e)
                    }
                }
            }
            CircuitState::Open => {
                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                let last_failure = self.last_failure_time.load(Ordering::SeqCst);

                if now - last_failure >= self.config.timeout_seconds {
                    *state = CircuitState::HalfOpen;
                    self.success_count.store(0, Ordering::SeqCst);
                    tracing::info!("Circuit breaker half-open, testing recovery");
                    drop(state);
                    self.call(f)
                } else {
                    Err("Circuit breaker is open".to_string())
                }
            }
            CircuitState::HalfOpen => {
                match f() {
                    Ok(result) => {
                        let successes = self.success_count.fetch_add(1, Ordering::SeqCst) + 1;

                        if successes >= self.config.success_threshold {
                            *state = CircuitState::Closed;
                            self.failure_count.store(0, Ordering::SeqCst);
                            tracing::info!("Circuit breaker closed, service recovered");
                        }

                        Ok(result)
                    }
                    Err(e) => {
                        *state = CircuitState::Open;
                        let now = SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap()
                            .as_secs();
                        self.last_failure_time.store(now, Ordering::SeqCst);
                        tracing::warn!("Circuit breaker re-opened during half-open state");
                        Err(e)
                    }
                }
            }
        }
    }

    pub fn state(&self) -> CircuitState {
        *self.state.lock().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circuit_breaker_closed() {
        let cb = CircuitBreaker::new(CircuitBreakerConfig::default());
        
        let result = cb.call(|| Ok::<_, String>(42));
        assert!(result.is_ok());
        assert_eq!(cb.state(), CircuitState::Closed);
    }

    #[test]
    fn test_circuit_breaker_opens() {
        let cb = CircuitBreaker::new(CircuitBreakerConfig {
            failure_threshold: 2,
            success_threshold: 1,
            timeout_seconds: 1,
        });

        // Fail twice to open circuit
        for _ in 0..2 {
            let _ = cb.call(|| Err::<i32, _>("error".to_string()));
        }

        assert_eq!(cb.state(), CircuitState::Open);
    }
}
