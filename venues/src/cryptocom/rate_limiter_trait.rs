use async_trait::async_trait;

use super::errors::Errors;

/// Crypto.com-specific rate limiter trait
///
/// This trait defines the interface for Crypto.com rate limiting.
/// Crypto.com uses different rate limits for public vs private endpoints with IP and user-based limits.
/// It can be implemented for both native and WASM environments.
#[async_trait]
pub trait CryptoComRateLimiter: Send + Sync {
    /// Check if a request can be made without exceeding rate limits
    ///
    /// # Parameters
    /// - `endpoint_limit`: The specific limit for this endpoint
    /// - `is_private`: Whether this is a private (authenticated) endpoint
    async fn check_limits(&self, endpoint_limit: u32, is_private: bool) -> Result<(), Errors>;

    /// Record a successful request
    async fn record_request(&self, is_private: bool);

    /// Get current usage statistics
    /// Returns (public_requests, private_requests, window_seconds)
    async fn get_usage_stats(&self) -> (u32, u32, u32);

    /// Get a summary of current usage as a human-readable string
    async fn get_usage_summary(&self) -> Option<String> {
        let (public, private, window) = self.get_usage_stats().await;
        Some(format!(
            "Public: {}, Private: {} ({}s window)",
            public, private, window
        ))
    }
}
