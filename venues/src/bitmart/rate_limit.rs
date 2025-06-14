use std::collections::HashMap;
use std::fmt;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

/// Represents different types of BitMart API endpoints for rate limiting
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EndpointType {
    /// System endpoints (time, service status) - 10 times/sec per IP
    System,
    /// Funding account endpoints - varies by endpoint (2-12 times/2sec per API key)
    FundingAccount,
    /// Spot public market endpoints - varies by endpoint (8-15 times/2sec per IP)
    SpotPublicMarket,
    /// Spot trading endpoints - varies by endpoint (1-50 times per API key)
    SpotTrading,
    /// Sub-account endpoints - varies by endpoint (2-150 times/2sec per API key)
    SubAccount,
    /// Margin loan endpoints - varies by endpoint (2-150 times/2sec per API key)
    MarginLoan,
}

/// Rate limit configuration
#[derive(Debug, Clone)]
pub struct RateLimit {
    /// Maximum number of requests allowed
    pub max_requests: u32,
    /// Time window for the rate limit
    pub window: Duration,
}

impl RateLimit {
    pub fn new(max_requests: u32, window: Duration) -> Self {
        Self {
            max_requests,
            window,
        }
    }
}

/// Error type for rate limiting
#[derive(Debug, Clone)]
pub enum RateLimitError {
    /// Rate limit exceeded for the given endpoint type
    Exceeded { endpoint_type: EndpointType },
}

impl fmt::Display for RateLimitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RateLimitError::Exceeded { endpoint_type } => {
                write!(f, "Rate limit exceeded for endpoint type: {:?}", endpoint_type)
            }
        }
    }
}

impl std::error::Error for RateLimitError {}

/// Rate limiter for BitMart API
pub struct RateLimiter {
    request_history: std::sync::Arc<RwLock<HashMap<EndpointType, Vec<Instant>>>>,
}

impl RateLimiter {
    /// Create a new rate limiter
    pub fn new() -> Self {
        Self {
            request_history: std::sync::Arc::new(RwLock::new(std::collections::HashMap::new())),
        }
    }

    /// Get rate limit for endpoint type
    /// Using conservative limits based on BitMart documentation
    fn get_rate_limit(endpoint_type: &EndpointType) -> RateLimit {
        match endpoint_type {
            EndpointType::System => RateLimit::new(10, Duration::from_secs(1)),
            EndpointType::FundingAccount => RateLimit::new(12, Duration::from_secs(2)), // Most common limit
            EndpointType::SpotPublicMarket => RateLimit::new(10, Duration::from_secs(2)), // Conservative
            EndpointType::SpotTrading => RateLimit::new(12, Duration::from_secs(2)), // Conservative for query endpoints
            EndpointType::SubAccount => RateLimit::new(8, Duration::from_secs(2)), // Conservative
            EndpointType::MarginLoan => RateLimit::new(2, Duration::from_secs(2)), // Most restrictive
        }
    }

    /// Check if a request can be made
    pub async fn check_limits(&self, endpoint_type: EndpointType) -> Result<(), RateLimitError> {
        let rate_limit = Self::get_rate_limit(&endpoint_type);
        let mut history = self.request_history.write().await;
        let now = Instant::now();

        // Get or create history for this endpoint type
        let timestamps = history.entry(endpoint_type.clone()).or_default();

        // Remove old timestamps outside the window
        timestamps.retain(|&timestamp| now.duration_since(timestamp) < rate_limit.window);

        // Check if we're at the limit
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
    use tokio::time::sleep;

    #[test]
    fn test_endpoint_types() {
        let system = EndpointType::System;
        let funding = EndpointType::FundingAccount;
        assert_ne!(system, funding);
    }

    #[test]
    fn test_rate_limit_config() {
        let rate_limit = RateLimit::new(10, Duration::from_secs(60));
        assert_eq!(rate_limit.max_requests, 10);
        assert_eq!(rate_limit.window, Duration::from_secs(60));
    }

    #[test]
    fn test_funding_account_rate_limit() {
        let rate_limit = RateLimiter::get_rate_limit(&EndpointType::FundingAccount);
        assert_eq!(rate_limit.max_requests, 12);
        assert_eq!(rate_limit.window, Duration::from_secs(2));
    }

    #[tokio::test]
    async fn test_rate_limiter_basic() {
        let limiter = RateLimiter::new();
        let endpoint = EndpointType::System;

        // Should allow initial requests
        for _ in 0..10 {
            assert!(limiter.check_limits(endpoint.clone()).await.is_ok());
            limiter.increment_request(endpoint.clone()).await;
        }

        // Should exceed rate limit
        assert!(limiter.check_limits(endpoint.clone()).await.is_err());
    }

    #[tokio::test]
    async fn test_rate_limit_recovery() {
        let limiter = RateLimiter::new();
        let endpoint = EndpointType::System;

        // Fill up the rate limit
        for _ in 0..10 {
            limiter.increment_request(endpoint.clone()).await;
        }

        // Should be at limit
        assert!(limiter.check_limits(endpoint.clone()).await.is_err());

        // Wait for window to pass
        sleep(Duration::from_millis(1100)).await; // Slightly more than 1 second

        // Should be able to make requests again
        assert!(limiter.check_limits(endpoint.clone()).await.is_ok());
    }
}