use std::collections::HashMap;

use async_trait::async_trait;

use super::{errors::Errors, rate_limiter::UsageStats};

/// Binance-specific rate limiter trait
///
/// This trait defines the interface for Binance rate limiting with weight-based controls.
/// It can be implemented for both native and WASM environments.
#[async_trait]
pub trait BinanceRateLimiter: Send + Sync {
    /// Check if a request with given weight and order flag would exceed rate limits
    async fn check_limits_with_weight(&self, weight: u32, is_order: bool) -> Result<(), Errors>;

    /// Record a successful request with given weight and order flag
    async fn record_request_with_weight(&self, weight: u32, is_order: bool);

    /// Update rate limiter state from response headers
    async fn update_from_headers(&self, headers: &HashMap<String, String>);

    /// Get detailed usage statistics
    async fn get_detailed_usage_stats(&self) -> UsageStats;

    /// Get a summary of current usage as a human-readable string
    async fn get_usage_summary(&self) -> Option<String> {
        let stats = self.get_detailed_usage_stats().await;
        Some(format!(
            "Weight: {}/{}, Raw: {}/{}, Orders 10s: {}/{}, Orders 1m: {}/{}",
            stats.weight_used,
            stats.weight_limit,
            stats.raw_requests_used,
            stats.raw_requests_limit,
            stats.orders_10s_used,
            stats.orders_10s_limit,
            stats.orders_1m_used,
            stats.orders_1m_limit
        ))
    }
}
