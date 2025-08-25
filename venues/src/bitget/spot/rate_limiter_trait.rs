use async_trait::async_trait;

use super::{Errors, ResponseHeaders};

/// BitGet-specific rate limiter trait
///
/// This trait defines the interface for BitGet rate limiting with multi-tier controls:
/// - Overall IP limit (6000/min)
/// - Endpoint-specific limits (varies per endpoint)
/// - Order-specific limits (UID-based)
///
/// It can be implemented for both native and WASM environments.
#[async_trait]
pub trait BitGetRateLimiter: Send + Sync {
    /// Check if a request can be made without exceeding rate limits
    ///
    /// # Parameters
    /// - `endpoint_limit_per_second`: The specific limit for this endpoint (e.g., 3, 10, 20)
    /// - `is_order`: Whether this is an order-related endpoint
    /// - `order_limit_per_second`: The specific order limit for this endpoint if applicable
    async fn check_limits(
        &self,
        endpoint_limit_per_second: u32,
        is_order: bool,
        order_limit_per_second: Option<u32>,
    ) -> Result<(), Errors>;

    /// Record a general request (affects both IP and endpoint limits)
    async fn record_request(&self);

    /// Record an order request (affects IP, endpoint, and order limits)
    async fn record_order_request(&self);

    /// Update rate limiter state from response headers
    async fn update_from_headers(&self, headers: &ResponseHeaders);

    /// Get current usage statistics
    async fn get_usage_stats(&self) -> BitGetUsageStats;

    /// Get a summary of current usage as a human-readable string
    async fn get_usage_summary(&self) -> Option<String> {
        let stats = self.get_usage_stats().await;
        Some(format!(
            "IP: {}/6000/min, Endpoint: {}/s, Orders: {}/s",
            stats.requests_per_minute, stats.requests_per_second, stats.orders_per_second
        ))
    }
}

/// Usage statistics for BitGet rate limiter
#[derive(Debug, Clone, Default)]
pub struct BitGetUsageStats {
    /// Current requests in the last minute (IP limit)
    pub requests_per_minute: u32,
    /// Current requests in the last second (endpoint limit)
    pub requests_per_second: u32,
    /// Current order requests in the last second
    pub orders_per_second: u32,
}
