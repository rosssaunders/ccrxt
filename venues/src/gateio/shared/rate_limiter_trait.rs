use std::collections::HashMap;

use async_trait::async_trait;

use super::rate_limit::{RateLimitHeader, RateLimitStatus, UsageInfo};

/// Gate.io-specific rate limiter trait
///
/// This trait defines the interface for Gate.io rate limiting with endpoint categorization.
/// It can be implemented for both native and WASM environments.
#[async_trait]
pub trait GateioRateLimiter: Send + Sync {
    /// Get a permit for the specified endpoint, respecting rate limits
    async fn get_permit(
        &self,
        endpoint: &str,
    ) -> Result<tokio::sync::SemaphorePermit<'_>, tokio::sync::AcquireError>;

    /// Update rate limit status from response headers
    fn update_from_headers(
        &self,
        headers: &RateLimitHeader,
        endpoint: &str,
    ) -> Option<RateLimitStatus>;

    /// Get usage statistics for all categories
    async fn get_category_usage_stats(&self) -> HashMap<String, UsageInfo>;

    /// Get rate limit warnings for categories approaching limits
    async fn get_rate_limit_warnings(&self) -> Vec<String>;

    /// Reset usage for a specific category (useful for testing)
    async fn reset_category(&self, category: &str);

    /// Reset all usage statistics
    async fn reset_all(&self);

    /// Get a summary of current usage as a human-readable string
    async fn get_usage_summary(&self) -> Option<String> {
        let stats = self.get_category_usage_stats().await;
        if stats.is_empty() {
            None
        } else {
            let mut result = Vec::new();
            for (category, info) in stats {
                result.push(format!("{}: {} requests", category, info.requests_made));
            }
            Some(result.join(", "))
        }
    }
}
