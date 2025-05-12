use std::time::Duration;
use async_trait::async_trait;
use crate::error::RestError;

/// Common trait for rate limiters
#[async_trait]
pub trait RateLimiter: Send + Sync {
    /// Check if the request can be made based on rate limits
    async fn check_rate_limit(&self, endpoint: &str, weight: u32) -> Result<(), RestError>;
    
    /// Get the current rate limit status
    fn get_rate_limit_status(&self) -> RateLimitStatus;
}

/// Status of rate limits
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RateLimitStatus {
    /// Number of requests remaining in the current window
    pub remaining: u32,
    
    /// Total number of requests allowed in the window
    pub limit: u32,
    
    /// Time until the rate limit window resets
    pub reset_in: Duration,
} 