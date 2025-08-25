use async_trait::async_trait;

use super::errors::Errors;

/// ByBit-specific rate limiter trait
///
/// This trait defines the interface for ByBit rate limiting.
/// ByBit uses different rate limits per endpoint category with IP-based and UID-based limits.
/// It can be implemented for both native and WASM environments.
#[async_trait]
pub trait BybitRateLimiter: Send + Sync {
    /// Check if a request can be made without exceeding rate limits
    ///
    /// # Parameters
    /// - `requests_per_second`: The limit for this specific endpoint
    /// - `is_private`: Whether this is a private (authenticated) endpoint
    async fn check_limits(&self, requests_per_second: u32, is_private: bool) -> Result<(), Errors>;

    /// Record a successful request
    async fn record_request(&self, is_private: bool);

    /// Get current usage statistics
    /// Returns (public_requests, private_requests, max_per_second)
    async fn get_usage_stats(&self) -> (u32, u32, u32);

    /// Get a summary of current usage as a human-readable string
    async fn get_usage_summary(&self) -> Option<String> {
        let (public, private, max) = self.get_usage_stats().await;
        Some(format!(
            "Public: {}/{}/s, Private: {}/{}/s",
            public, max, private, max
        ))
    }
}
