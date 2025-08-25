use async_trait::async_trait;

use super::errors::KucoinError;

/// KuCoin-specific rate limiter trait
///
/// This trait defines the interface for KuCoin rate limiting.
/// KuCoin uses endpoint-specific rate limits with different windows and burst allowances.
/// It can be implemented for both native and WASM environments.
#[async_trait]
pub trait KuCoinRateLimiter: Send + Sync {
    /// Check if a request can be made without exceeding rate limits
    ///
    /// # Parameters
    /// - `requests_per_window`: The limit for this specific endpoint
    /// - `window_seconds`: The time window for the limit
    /// - `is_private`: Whether this is a private (authenticated) endpoint
    async fn check_limits(
        &self,
        requests_per_window: u32,
        window_seconds: u32,
        is_private: bool,
    ) -> Result<(), KucoinError>;

    /// Record a successful request
    async fn record_request(&self, is_private: bool);

    /// Update rate limiter state from response headers
    async fn update_from_headers(&self, headers: &std::collections::HashMap<String, String>);

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
