use async_trait::async_trait;

use super::{errors::BingXError, rate_limit::EndpointType};

/// BingX-specific rate limiter trait
///
/// This trait defines the interface for BingX rate limiting with endpoint-specific controls.
/// It can be implemented for both native and WASM environments.
#[async_trait]
pub trait BingXRateLimiter: Send + Sync {
    /// Check if a request can be made for the specified endpoint type
    async fn check_limits_for_endpoint(
        &self,
        endpoint_type: EndpointType,
    ) -> Result<(), BingXError>;

    /// Record a successful request for the specified endpoint type
    async fn record_request_for_endpoint(&self, endpoint_type: EndpointType);

    /// Get usage statistics for all endpoint types
    /// Returns (current_usage, limit) for each endpoint type
    async fn get_endpoint_usage_stats(&self)
    -> std::collections::HashMap<EndpointType, (u32, u32)>;

    /// Get a summary of current usage as a human-readable string
    async fn get_usage_summary(&self) -> Option<String> {
        let stats = self.get_endpoint_usage_stats().await;
        if stats.is_empty() {
            None
        } else {
            let mut summary = Vec::new();
            for (endpoint_type, (current, limit)) in stats {
                summary.push(format!("{:?}: {}/{}", endpoint_type, current, limit));
            }
            Some(summary.join(", "))
        }
    }
}
