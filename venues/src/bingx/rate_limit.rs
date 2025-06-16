use std::collections::HashMap;
use std::time::{Duration, Instant};

use thiserror::Error;
use tokio::sync::RwLock;

/// Types of endpoints for rate limiting based on BingX API documentation
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EndpointType {
    /// Account-related endpoints (like balance queries) - 5/s
    Account,
    /// General trading endpoints - varies by endpoint
    Trading,
    /// General endpoints - varies by endpoint
    General,
}

/// Rate limit configuration for different endpoint types
#[derive(Debug, Clone)]
pub struct RateLimit {
    /// Maximum requests per window
    pub max_requests: u32,
    /// Time window duration
    pub window: Duration,
}

impl RateLimit {
    /// Create a new rate limit configuration
    pub fn new(max_requests: u32, window: Duration) -> Self {
        Self {
            max_requests,
            window,
        }
    }
}

/// Rate limiting errors
#[derive(Error, Debug)]
pub enum RateLimitError {
    #[error("Rate limit exceeded for endpoint type: {endpoint_type:?}")]
    Exceeded { endpoint_type: EndpointType },
}

/// Rate limiter for BingX API endpoints
#[derive(Debug, Clone)]
pub struct RateLimiter {
    /// Request timestamps for different endpoint types
    request_history: std::sync::Arc<RwLock<HashMap<EndpointType, Vec<Instant>>>>,
}

impl RateLimiter {
    /// Create a new rate limiter
    pub fn new() -> Self {
        Self {
            request_history: std::sync::Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Get rate limit for endpoint type based on BingX documentation
    fn get_rate_limit(endpoint_type: &EndpointType) -> RateLimit {
        match endpoint_type {
            EndpointType::Account => RateLimit::new(5, Duration::from_secs(1)), // 5/s for account endpoints
            EndpointType::Trading => RateLimit::new(10, Duration::from_secs(1)), // Conservative default
            EndpointType::General => RateLimit::new(20, Duration::from_secs(1)), // Conservative default
        }
    }

    /// Check if a request can be made
    pub async fn check_limits(&self, endpoint_type: EndpointType) -> Result<(), RateLimitError> {
        let rate_limit = Self::get_rate_limit(&endpoint_type);
        let now = Instant::now();
        let cutoff = now.checked_sub(rate_limit.window).unwrap_or(now);

        let mut history = self.request_history.write().await;
        let timestamps = history.entry(endpoint_type.clone()).or_default();

        // Remove old timestamps
        timestamps.retain(|&timestamp| timestamp > cutoff);

        // Check if we can make a new request
        if timestamps.len() >= rate_limit.max_requests as usize {
            return Err(RateLimitError::Exceeded { endpoint_type });
        }

        Ok(())
    }

    /// Record a request
    pub async fn increment_request(&self, endpoint_type: EndpointType) {
        let mut history = self.request_history.write().await;
        let timestamps = history.entry(endpoint_type).or_default();
        timestamps.push(Instant::now());
    }
}

impl Default for RateLimiter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_rate_limiter_allows_requests_within_limit() {
        let rate_limiter = RateLimiter::new();

        // Should allow first request
        assert!(
            rate_limiter
                .check_limits(EndpointType::Account)
                .await
                .is_ok()
        );
        rate_limiter.increment_request(EndpointType::Account).await;

        // Should allow second request
        assert!(
            rate_limiter
                .check_limits(EndpointType::Account)
                .await
                .is_ok()
        );
        rate_limiter.increment_request(EndpointType::Account).await;
    }

    #[tokio::test]
    async fn test_rate_limiter_blocks_requests_over_limit() {
        let rate_limiter = RateLimiter::new();

        // Make 5 requests (the limit for Account endpoints)
        for _ in 0..5 {
            assert!(
                rate_limiter
                    .check_limits(EndpointType::Account)
                    .await
                    .is_ok()
            );
            rate_limiter.increment_request(EndpointType::Account).await;
        }

        // The 6th request should be blocked
        assert!(
            rate_limiter
                .check_limits(EndpointType::Account)
                .await
                .is_err()
        );
    }

    #[tokio::test]
    async fn test_rate_limiter_different_endpoint_types() {
        let rate_limiter = RateLimiter::new();

        // Account and Trading endpoints have different limits
        assert!(
            rate_limiter
                .check_limits(EndpointType::Account)
                .await
                .is_ok()
        );
        rate_limiter.increment_request(EndpointType::Account).await;

        assert!(
            rate_limiter
                .check_limits(EndpointType::Trading)
                .await
                .is_ok()
        );
        rate_limiter.increment_request(EndpointType::Trading).await;
    }
}
