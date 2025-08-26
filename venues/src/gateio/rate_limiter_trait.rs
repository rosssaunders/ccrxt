use std::collections::HashMap;

use async_trait::async_trait;

use super::{errors::GateIoError, rate_limit::UsageInfo};

/// Gate.io-specific rate limiter trait
///
/// This trait defines the interface for Gate.io rate limiting with endpoint-specific controls.
/// It can be implemented for both native and WASM environments.
#[async_trait]
pub trait GateIoRateLimiter: Send + Sync {
    /// Get a permit for the specified endpoint
    async fn get_permit(&self, endpoint: &str) -> Result<(), GateIoError>;

    /// Update rate limit status from response headers
    async fn update_from_headers(&self, headers: &HashMap<String, String>, endpoint: &str);

    /// Get current usage statistics for all categories
    async fn get_usage_stats(&self) -> HashMap<String, UsageInfo>;

    /// Get rate limit warnings for categories approaching their limits
    async fn get_rate_limit_warnings(&self) -> Vec<String>;

    /// Reset usage statistics for a specific category
    async fn reset_category(&self, category: &str);

    /// Reset all usage statistics
    async fn reset_all(&self);

    /// Get a summary of current usage as a human-readable string
    async fn get_usage_summary(&self) -> Option<String> {
        let stats = self.get_usage_stats().await;
        if stats.is_empty() {
            None
        } else {
            let mut summary = Vec::new();
            for (category, info) in stats {
                summary.push(format!("{}: {} requests", category, info.requests_made));
            }
            Some(summary.join(", "))
        }
    }
}
