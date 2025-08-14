use crate::time_compat::{Duration, Instant};

use thiserror::Error;
use tokio::sync::RwLock;

/// Types of endpoints for rate limiting in ByBit V5 API
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EndpointType {
    /// Account related endpoints (wallet balance, etc.)
    Account,
    /// Trading related endpoints
    Trade,
    /// Position related endpoints
    Position,
    /// Asset related endpoints
    Asset,
    /// User management endpoints
    User,
    /// Spot margin trade endpoints
    SpotMargin,
    /// Broker related endpoints
    Broker,
    /// Crypto loan endpoints
    CryptoLoan,
    /// Earn endpoints
    Earn,
    /// INS loan endpoints
    InsLoan,
    /// Market data endpoints (public)
    Market,
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

    #[error("IP rate limit exceeded (600 requests per 5 seconds)")]
    IpLimitExceeded,
}

/// Rate limiter for ByBit V5 API endpoints
#[derive(Debug, Clone)]
pub struct RateLimiter {
    /// Request timestamps for different endpoint types
    request_history: std::sync::Arc<RwLock<std::collections::HashMap<EndpointType, Vec<Instant>>>>,
    /// IP-level request history (600 requests per 5 seconds)
    ip_request_history: std::sync::Arc<RwLock<Vec<Instant>>>,
}

impl RateLimiter {
    /// Create a new rate limiter
    pub fn new() -> Self {
        Self {
            request_history: std::sync::Arc::new(RwLock::new(std::collections::HashMap::new())),
            ip_request_history: std::sync::Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Get rate limit for endpoint type based on ByBit V5 API documentation
    fn get_rate_limit(endpoint_type: &EndpointType) -> RateLimit {
        match endpoint_type {
            // Account endpoints like wallet balance: varies by account type (10-50/s)
            // Using conservative 10/s for general account endpoints
            EndpointType::Account => RateLimit::new(10, Duration::from_secs(1)),
            // Trade endpoints: varies (10-20/s)
            EndpointType::Trade => RateLimit::new(10, Duration::from_secs(1)),
            // Position endpoints: varies (10-50/s)
            EndpointType::Position => RateLimit::new(10, Duration::from_secs(1)),
            // Asset endpoints: varies (5 req/s to 300 req/min)
            EndpointType::Asset => RateLimit::new(5, Duration::from_secs(1)),
            // User endpoints: 5-10 req/s
            EndpointType::User => RateLimit::new(5, Duration::from_secs(1)),
            // Spot margin trade endpoints
            EndpointType::SpotMargin => RateLimit::new(10, Duration::from_secs(1)),
            // Broker endpoints
            EndpointType::Broker => RateLimit::new(5, Duration::from_secs(1)),
            // Crypto loan endpoints
            EndpointType::CryptoLoan => RateLimit::new(5, Duration::from_secs(1)),
            // Earn endpoints
            EndpointType::Earn => RateLimit::new(5, Duration::from_secs(1)),
            // INS loan endpoints
            EndpointType::InsLoan => RateLimit::new(5, Duration::from_secs(1)),
            // Market data endpoints (public)
            EndpointType::Market => RateLimit::new(10, Duration::from_secs(1)),
        }
    }

    /// Check if a request can be made (both endpoint-specific and IP limits)
    pub async fn check_limits(&self, endpoint_type: EndpointType) -> Result<(), RateLimitError> {
        // Check IP-level limit first (600 requests per 5 seconds)
        self.check_ip_limits().await?;

        // Check endpoint-specific limit
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

    /// Check IP-level rate limits (600 requests per 5 seconds)
    async fn check_ip_limits(&self) -> Result<(), RateLimitError> {
        let mut ip_history = self.ip_request_history.write().await;
        let now = Instant::now();
        let window = Duration::from_secs(5);

        // Remove old timestamps outside the 5-second window
        ip_history.retain(|&timestamp| now.duration_since(timestamp) < window);

        // Check if we're at the IP limit (600 requests per 5 seconds)
        if ip_history.len() >= 600 {
            return Err(RateLimitError::IpLimitExceeded);
        }

        Ok(())
    }

    /// Record a request
    pub async fn increment_request(&self, endpoint_type: EndpointType) {
        let now = Instant::now();

        // Record endpoint-specific request
        let mut history = self.request_history.write().await;
        let timestamps = history.entry(endpoint_type).or_default();
        timestamps.push(now);

        // Record IP-level request
        let mut ip_history = self.ip_request_history.write().await;
        ip_history.push(now);
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
    async fn test_rate_limiter_basic() {
        let limiter = RateLimiter::new();

        // First request should succeed
        assert!(limiter.check_limits(EndpointType::Account).await.is_ok());
        limiter.increment_request(EndpointType::Account).await;
    }

    #[tokio::test]
    async fn test_endpoint_specific_limits() {
        let limiter = RateLimiter::new();

        // Test account endpoint rate limiting
        assert!(limiter.check_limits(EndpointType::Account).await.is_ok());
        limiter.increment_request(EndpointType::Account).await;

        // Test trading endpoint rate limiting
        assert!(limiter.check_limits(EndpointType::Trade).await.is_ok());
        limiter.increment_request(EndpointType::Trade).await;
    }

    #[tokio::test]
    async fn test_rate_limit_config() {
        let config = RateLimit::new(100, Duration::from_secs(60));
        assert_eq!(config.max_requests, 100);
        assert_eq!(config.window, Duration::from_secs(60));
    }

    #[test]
    fn test_endpoint_types() {
        let account = EndpointType::Account;
        let trading = EndpointType::Trade;
        let asset = EndpointType::Asset;
        let user = EndpointType::User;

        assert_ne!(account, trading);
        assert_ne!(account, asset);
        assert_ne!(trading, user);
        assert_eq!(account.clone(), EndpointType::Account);
    }

    #[tokio::test]
    async fn test_ip_rate_limit_check() {
        let limiter = RateLimiter::new();

        // Should start with no issues
        assert!(limiter.check_ip_limits().await.is_ok());
    }
}
