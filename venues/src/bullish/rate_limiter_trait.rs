use async_trait::async_trait;

use super::errors::Errors;

/// Bullish-specific rate limiter trait
///
/// This trait defines the interface for Bullish rate limiting.
/// Bullish uses a simple request-per-second model with burst capacity.
/// It can be implemented for both native and WASM environments.
#[async_trait]
pub trait BullishRateLimiter: Send + Sync {
    /// Check if a request can be made without exceeding rate limits
    async fn check_limits(&self) -> Result<(), Errors>;

    /// Record a successful request
    async fn record_request(&self);

    /// Get current usage statistics
    /// Returns (current_requests, max_requests, window_seconds)
    async fn get_usage_stats(&self) -> (u32, u32, u32);

    /// Get a summary of current usage as a human-readable string
    async fn get_usage_summary(&self) -> Option<String> {
        let (current, max, window) = self.get_usage_stats().await;
        Some(format!("Requests: {}/{}/{}s", current, max, window))
    }
}
