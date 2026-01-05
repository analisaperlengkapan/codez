//! Rate limiting middleware for API Gateway

use axum::{extract::ConnectInfo, middleware::Next, response::Response};
use std::net::SocketAddr;

/// Rate limiter configuration
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct RateLimiterConfig {
    pub requests_per_minute: u32,
    pub requests_per_hour: u32,
}

impl Default for RateLimiterConfig {
    fn default() -> Self {
        Self {
            requests_per_minute: 60,
            requests_per_hour: 1000,
        }
    }
}

/// Rate limiter middleware
#[allow(dead_code)]
pub async fn rate_limit_middleware(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    request: axum::extract::Request,
    next: Next,
) -> Response {
    // Get client IP
    let client_ip = addr.ip().to_string();

    // For now, just log and allow
    // Full implementation will use Redis for rate limiting
    tracing::debug!("Request from IP: {}", client_ip);

    next.run(request).await
}

/// Token bucket rate limiter
#[allow(dead_code)]
pub struct TokenBucket {
    capacity: u32,
    tokens: u32,
    refill_rate: u32, // tokens per second
    last_refill: std::time::Instant,
}

impl TokenBucket {
    #[allow(dead_code)]
    pub fn new(capacity: u32, refill_rate: u32) -> Self {
        Self {
            capacity,
            tokens: capacity,
            refill_rate,
            last_refill: std::time::Instant::now(),
        }
    }

    #[allow(dead_code)]
    pub fn try_consume(&mut self, tokens: u32) -> bool {
        self.refill();

        if self.tokens >= tokens {
            self.tokens -= tokens;
            true
        } else {
            false
        }
    }

    #[allow(dead_code)]
    fn refill(&mut self) {
        let now = std::time::Instant::now();
        let elapsed = now.duration_since(self.last_refill).as_secs_f32();
        let new_tokens = (elapsed * self.refill_rate as f32) as u32;

        self.tokens = std::cmp::min(self.capacity, self.tokens + new_tokens);
        self.last_refill = now;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_bucket() {
        let mut bucket = TokenBucket::new(10, 1);

        // Should allow first 10 requests
        for _ in 0..10 {
            assert!(bucket.try_consume(1));
        }

        // Should deny 11th request
        assert!(!bucket.try_consume(1));
    }
}
