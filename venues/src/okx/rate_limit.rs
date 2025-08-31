use std::time::{Duration, Instant};

use async_trait::async_trait;
use thiserror::Error;
use tokio::sync::RwLock;

use super::{Errors, rate_limiter_trait::OkxRateLimiter};

/// Types of endpoints for rate limiting
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum EndpointType {
    /// Public market data endpoints
    PublicMarketData,
    /// Public trading data endpoints
    PublicTradingData,
    /// Private trading endpoints
    PrivateTrading,
    /// Private account endpoints
    PrivateAccount,
    /// Public insurance fund endpoint
    PublicInsuranceFund,
    /// Public market data history endpoints (lower rate limit)
    PublicMarketDataHistory,
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
#[non_exhaustive]
pub enum RateLimitError {
    #[error("Rate limit exceeded for endpoint type: {endpoint_type:?}")]
    Exceeded { endpoint_type: EndpointType },
}

/// Rate limiter for OKX API endpoints
#[derive(Debug, Clone)]
pub struct RateLimiter {
    /// Request timestamps for different endpoint types
    request_history: std::sync::Arc<RwLock<std::collections::HashMap<EndpointType, Vec<Instant>>>>,
}

impl RateLimiter {
    /// Create a new rate limiter
    pub fn new() -> Self {
        Self {
            request_history: std::sync::Arc::new(RwLock::new(std::collections::HashMap::new())),
        }
    }

    /// Get rate limit for endpoint type
    fn get_rate_limit(endpoint_type: &EndpointType) -> RateLimit {
        match endpoint_type {
            EndpointType::PublicMarketData => RateLimit::new(20, Duration::from_secs(2)),
            EndpointType::PublicTradingData => RateLimit::new(20, Duration::from_secs(2)),
            EndpointType::PrivateTrading => RateLimit::new(60, Duration::from_secs(2)),
            EndpointType::PrivateAccount => RateLimit::new(10, Duration::from_secs(2)),
            EndpointType::PublicInsuranceFund => RateLimit::new(10, Duration::from_secs(2)),
            EndpointType::PublicMarketDataHistory => RateLimit::new(10, Duration::from_secs(2)),
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

// Implement the OKX-specific trait
#[async_trait]
impl OkxRateLimiter for RateLimiter {
    async fn check_limits_for_endpoint(&self, endpoint_type: EndpointType) -> Result<(), Errors> {
        self.check_limits(endpoint_type).await.map_err(|e| match e {
            RateLimitError::Exceeded { endpoint_type } => {
                Errors::Error(format!("Rate limit exceeded for {:?}", endpoint_type))
            }
        })
    }

    async fn record_request_for_endpoint(&self, endpoint_type: EndpointType) {
        self.increment_request(endpoint_type).await;
    }

    async fn get_endpoint_usage_stats(
        &self,
    ) -> std::collections::HashMap<EndpointType, (u32, u32)> {
        let history = self.request_history.read().await;
        let mut stats = std::collections::HashMap::new();
        for (endpoint_type, timestamps) in history.iter() {
            let rate_limit = Self::get_rate_limit(endpoint_type);
            stats.insert(
                endpoint_type.clone(),
                (timestamps.len() as u32, rate_limit.max_requests),
            );
        }
        stats
    }
}

#[cfg(test)]
mod tests {
    use tokio::time::Duration;

    use super::*;

    #[tokio::test]
    async fn test_rate_limiter_basic() {
        let limiter = RateLimiter::new();

        // First request should succeed
        assert!(
            limiter
                .check_limits(EndpointType::PublicMarketData)
                .await
                .is_ok()
        );
        limiter
            .increment_request(EndpointType::PublicMarketData)
            .await;
    }

    #[tokio::test]
    async fn test_insurance_fund_rate_limit() {
        let limiter = RateLimiter::new();

        // Test insurance fund endpoint rate limiting
        assert!(
            limiter
                .check_limits(EndpointType::PublicInsuranceFund)
                .await
                .is_ok()
        );
        limiter
            .increment_request(EndpointType::PublicInsuranceFund)
            .await;
    }

    #[tokio::test]
    async fn test_rate_limit_config() {
        let config = RateLimit::new(100, Duration::from_secs(60));
        assert_eq!(config.max_requests, 100);
        assert_eq!(config.window, Duration::from_secs(60));
    }

    #[test]
    fn test_endpoint_types() {
        let public_data = EndpointType::PublicMarketData;
        let private_trading = EndpointType::PrivateTrading;
        let insurance_fund = EndpointType::PublicInsuranceFund;

        assert_ne!(public_data, private_trading);
        assert_ne!(public_data, insurance_fund);
        assert_eq!(public_data.clone(), EndpointType::PublicMarketData);
        assert_eq!(insurance_fund.clone(), EndpointType::PublicInsuranceFund);
    }
}
