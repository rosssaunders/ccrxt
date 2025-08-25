use async_trait::async_trait;

use super::errors::Errors;

/// Coinbase Exchange-specific rate limiter trait
///
/// This trait defines the interface for Coinbase Exchange rate limiting.
/// Coinbase uses tier-based rate limiting with different limits for different API tiers.
/// It can be implemented for both native and WASM environments.
#[async_trait]
pub trait CoinbaseRateLimiter: Send + Sync {
    /// Check if a request can be made without exceeding rate limits
    ///
    /// # Parameters
    /// - `is_private`: Whether this is a private (authenticated) endpoint
    async fn check_limits(&self, is_private: bool) -> Result<(), Errors>;

    /// Record a successful request
    async fn record_request(&self, is_private: bool);

    /// Update rate limiter state from response headers
    async fn update_from_headers(&self, headers: &std::collections::HashMap<String, String>);

    /// Get current usage statistics
    /// Returns (public_requests, private_requests, tier_limit)
    async fn get_usage_stats(&self) -> (u32, u32, u32);

    /// Get a summary of current usage as a human-readable string
    async fn get_usage_summary(&self) -> Option<String> {
        let (public, private, limit) = self.get_usage_stats().await;
        Some(format!(
            "Public: {}, Private: {}, Limit: {}/s",
            public, private, limit
        ))
    }
}
