//! Binance Options API (EAPI) module
//!
//! This module provides rate limiting and error handling for Binance Options API endpoints.
//! The Options API uses /eapi/v1/ endpoints and has its own rate limiting rules.

use std::time::Duration;

pub mod errors;
pub mod rate_limit;

// Private API module
mod private;
pub use private::RestClient as PrivateRestClient;

pub use errors::*;
pub use rate_limit::{
    IntervalUnit, RateLimitHeader, RateLimitHeaderKind, RateLimitUsage, RateLimiter,
    ResponseHeaders,
};

mod enums;

// Re-export enums for public use
pub use enums::*;

// Re-export compatible enums from coinm where appropriate
pub use crate::binance::coinm::{KlineInterval, OrderResponseType, OrderSide, TimeInForce};

/// Response structure for Binance Options API requests
#[derive(Debug, Clone)]
pub struct RestResponse<T> {
    pub data: T,
    pub request_duration: Duration,
    pub headers: ResponseHeaders,
}

/// Type alias for results returned by Binance Options API operations
pub type RestResult<T> = Result<RestResponse<T>, Errors>;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_rate_limiter_creation() {
        let limiter = RateLimiter::new();
        assert!(limiter.check_limits(1, false).await.is_ok());
    }

    #[tokio::test]
    async fn test_raw_request_limit() {
        let limiter = RateLimiter::new();

        // Should be fine under limit
        assert!(limiter.check_limits(1, false).await.is_ok());

        // Simulate hitting the raw request limit
        limiter.test_set_raw_requests(61000).await;

        // Should now fail
        assert!(limiter.check_limits(1, false).await.is_err());
    }

    #[tokio::test]
    async fn test_request_weight_limit() {
        let limiter = RateLimiter::new();

        // Should be fine under limit
        assert!(limiter.check_limits(1000, false).await.is_ok());

        // Simulate approaching the weight limit
        limiter.test_set_weight(5500).await; // Set close to 6000 limit

        // Should still be ok with small weight
        assert!(limiter.check_limits(400, false).await.is_ok());

        // Should fail with weight that would exceed limit
        assert!(limiter.check_limits(600, false).await.is_err());
    }

    #[tokio::test]
    async fn test_order_limit_10s() {
        let limiter = RateLimiter::new();

        // Should be fine under limit
        assert!(limiter.check_limits(1, true).await.is_ok());

        // Simulate hitting the 10s order limit
        limiter.test_set_orders_10s(100).await;

        // Should now fail for order requests
        assert!(limiter.check_limits(1, true).await.is_err());

        // Should still be ok for non-order requests
        assert!(limiter.check_limits(1, false).await.is_ok());
    }

    #[tokio::test]
    async fn test_order_limit_1m() {
        let limiter = RateLimiter::new();

        // Should be fine under limit
        assert!(limiter.check_limits(1, true).await.is_ok());

        // Simulate hitting the 1m order limit
        limiter.test_set_orders_1m(1200).await;

        // Should now fail for order requests
        assert!(limiter.check_limits(1, true).await.is_err());

        // Should still be ok for non-order requests
        assert!(limiter.check_limits(1, false).await.is_ok());
    }

    #[tokio::test]
    async fn test_header_parsing() {
        // Test valid headers
        assert!(RateLimitHeader::parse("x-mbx-used-weight-1m").is_some());
        assert!(RateLimitHeader::parse("x-mbx-order-count-10s").is_some());
        assert!(RateLimitHeader::parse("X-MBX-USED-WEIGHT-1H").is_some());

        // Test invalid headers
        assert!(RateLimitHeader::parse("invalid-header").is_none());
        assert!(RateLimitHeader::parse("x-mbx-used-weight-").is_none());
        assert!(RateLimitHeader::parse("x-mbx-used-weight-1").is_none());
    }

    #[tokio::test]
    async fn test_increment_operations() {
        let limiter = RateLimiter::new();

        // Test increment_raw_request
        limiter.increment_raw_request().await;
        let (raw_count, _, _, _) = limiter.test_get_stats().await;
        assert_eq!(raw_count, 1);

        // Test increment_order
        limiter.increment_order().await;
        let (_, orders_10s, orders_1m, _) = limiter.test_get_stats().await;
        assert_eq!(orders_10s, 1);
        assert_eq!(orders_1m, 1);
    }

    #[tokio::test]
    async fn test_update_from_headers() {
        let limiter = RateLimiter::new();
        let mut headers = ResponseHeaders::default();

        // Create a header for 1-minute weight
        let header = RateLimitHeader {
            kind: RateLimitHeaderKind::UsedWeight,
            interval_value: 1,
            interval_unit: IntervalUnit::Minute,
        };
        headers.values.insert(header, 2500);

        // Update from headers
        limiter.update_from_headers(&headers).await;

        // Check that the weight was updated
        let (_, _, _, weight) = limiter.test_get_stats().await;
        assert_eq!(weight, 2500);
    }
}
