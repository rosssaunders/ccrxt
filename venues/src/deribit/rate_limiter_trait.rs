use async_trait::async_trait;

use super::errors::Errors;

/// Deribit-specific rate limiter trait
///
/// This trait defines the interface for Deribit rate limiting.
/// Deribit uses a sophisticated multi-tier rate limiting system with different limits
/// for different endpoint categories and user tiers.
/// It can be implemented for both native and WASM environments.
#[async_trait]
pub trait DeribitRateLimiter: Send + Sync {
    /// Check if a request can be made without exceeding rate limits
    ///
    /// # Parameters
    /// - `tier`: The rate limit tier for this endpoint (1-5)
    /// - `is_private`: Whether this is a private (authenticated) endpoint
    async fn check_limits(&self, tier: u8, is_private: bool) -> Result<(), Errors>;

    /// Record a successful request
    async fn record_request(&self, tier: u8, is_private: bool);

    /// Update rate limiter state from response headers
    async fn update_from_headers(&self, headers: &std::collections::HashMap<String, String>);

    /// Get current usage statistics for all tiers
    /// Returns a map of tier -> (current_usage, max_limit)
    async fn get_tier_usage_stats(&self) -> std::collections::HashMap<u8, (u32, u32)>;

    /// Get a summary of current usage as a human-readable string
    async fn get_usage_summary(&self) -> Option<String> {
        let stats = self.get_tier_usage_stats().await;
        if stats.is_empty() {
            None
        } else {
            let mut summary = Vec::new();
            for (tier, (current, limit)) in stats {
                summary.push(format!("T{}: {}/{}", tier, current, limit));
            }
            Some(summary.join(", "))
        }
    }
}
