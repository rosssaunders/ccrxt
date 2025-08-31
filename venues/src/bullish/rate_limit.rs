//! Rate limiting for Bullish Exchange API

use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, Instant},
};

use thiserror::Error;
use tokio::sync::RwLock;

/// Rate limit error
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum RateLimitError {
    #[error(
        "Rate limit exceeded for endpoint {endpoint:?}: {current}/{max} requests in {window:?}"
    )]
    RateLimitExceeded {
        endpoint: EndpointType,
        current: u32,
        max: u32,
        window: Duration,
    },
}

/// Represents different types of Bullish API endpoints for rate limiting
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum EndpointType {
    // Public endpoints
    PublicMarkets,
    PublicAssets,
    PublicTicker,
    PublicOrderbook,
    PublicTrades,
    PublicCandles,
    PublicTime,
    PublicOther,

    // Private endpoints
    PrivateLogin,
    PrivateTradingAccounts,
    PrivateOrders,
    PrivateTrades,
    PrivatePositions,
    PrivateAssetBalances,
    /// All custody endpoints under /v1/wallets/* share a combined limit
    PrivateCustody,
    PrivateOther,
}

impl EndpointType {
    /// Get rate limit configuration for endpoint type
    pub fn rate_limit(&self) -> RateLimit {
        match self {
            // Public endpoints - 50 requests per second
            EndpointType::PublicMarkets => RateLimit::new(50, Duration::from_secs(1)),
            EndpointType::PublicAssets => RateLimit::new(50, Duration::from_secs(1)),
            EndpointType::PublicTicker => RateLimit::new(50, Duration::from_secs(1)),
            EndpointType::PublicOrderbook => RateLimit::new(50, Duration::from_secs(1)),
            EndpointType::PublicTrades => RateLimit::new(50, Duration::from_secs(1)),
            EndpointType::PublicCandles => RateLimit::new(50, Duration::from_secs(1)),
            EndpointType::PublicTime => RateLimit::new(100, Duration::from_secs(1)),
            EndpointType::PublicOther => RateLimit::new(50, Duration::from_secs(1)),

            // Private endpoints - 50 requests per second with higher limits for order endpoints
            EndpointType::PrivateLogin => RateLimit::new(10, Duration::from_secs(1)),
            EndpointType::PrivateTradingAccounts => RateLimit::new(50, Duration::from_secs(1)),
            EndpointType::PrivateOrders => RateLimit::new(50, Duration::from_secs(1)),
            EndpointType::PrivateTrades => RateLimit::new(50, Duration::from_secs(1)),
            EndpointType::PrivatePositions => RateLimit::new(50, Duration::from_secs(1)),
            EndpointType::PrivateAssetBalances => RateLimit::new(50, Duration::from_secs(1)),
            // Custody endpoints: 40 requests per minute across all /wallets/* endpoints
            EndpointType::PrivateCustody => RateLimit::new(40, Duration::from_secs(60)),
            EndpointType::PrivateOther => RateLimit::new(50, Duration::from_secs(1)),
        }
    }

    /// Map from endpoint path to endpoint type
    pub fn from_path(path: &str) -> Self {
        match path {
            // Public endpoints
            path if path.contains("/v1/markets") => EndpointType::PublicMarkets,
            path if path.contains("/v1/assets") => EndpointType::PublicAssets,
            path if path.contains("/v1/ticker") => EndpointType::PublicTicker,
            path if path.contains("/orderbook") => EndpointType::PublicOrderbook,
            path if path.contains("/v1/time") => EndpointType::PublicTime,
            path if path.contains("/markets/") && path.contains("/trades") => {
                EndpointType::PublicTrades
            }
            path if path.contains("/v1/candles") || path.contains("/candles") => {
                EndpointType::PublicCandles
            }
            path if path.contains("/v1/nonce") => EndpointType::PublicOther,
            path if path.contains("/v1/index-prices") => EndpointType::PublicOther,

            // Private endpoints
            path if path.contains("/v1/users/login") || path.contains("/v1/users/hmac/login") => {
                EndpointType::PrivateLogin
            }
            path if path.contains("/v1/accounts/trading-accounts") => {
                EndpointType::PrivateTradingAccounts
            }
            path if path.contains("/v2/orders") => EndpointType::PrivateOrders,
            path if path.contains("/v2/history/orders") => EndpointType::PrivateOrders,
            path if path.contains("/v1/trades") => EndpointType::PrivateTrades,
            path if path.contains("/v1/history/trades") => EndpointType::PrivateTrades,
            path if path.contains("/v1/positions") => EndpointType::PrivatePositions,
            path if path.contains("/v1/derivatives-positions") => EndpointType::PrivatePositions,
            path if path.contains("/v1/history/derivatives-settlement") => {
                EndpointType::PrivatePositions
            }
            path if path.contains("/v1/accounts/asset") => EndpointType::PrivateAssetBalances,
            // Any custody path groups under the same limiter
            path if path.contains("/v1/wallets/") => EndpointType::PrivateCustody,

            // Default based on whether it's a public or private path
            path if path.starts_with("/trading-api/v1/")
                && !path.contains("/users/")
                && !path.contains("/accounts/")
                && !path.contains("/orders")
                && !path.contains("/positions")
                && !path.contains("/trades")
                && !path.contains("/wallets/") =>
            {
                EndpointType::PublicOther
            }
            _ => EndpointType::PrivateOther,
        }
    }
}

/// Rate limit configuration
#[derive(Debug, Clone)]
pub struct RateLimit {
    /// Maximum requests per window
    pub max_requests: u32,
    /// Time window duration
    pub window: Duration,
}

impl RateLimit {
    /// Create a new rate limit configuration
    pub fn new(max_requests: u32, window: Duration) -> Self {
        Self {
            max_requests,
            window,
        }
    }
}

/// Rate limiter for Bullish API
#[derive(Debug, Clone, Default)]
pub struct RateLimiter {
    /// Request timestamps for different endpoint types
    request_history: Arc<RwLock<HashMap<EndpointType, Vec<Instant>>>>,
}

impl RateLimiter {
    /// Create a new rate limiter
    pub fn new() -> Self {
        Self::default()
    }

    /// Check if a request can be made for the given endpoint type
    pub async fn check_limits(&self, endpoint_type: EndpointType) -> Result<(), RateLimitError> {
        let rate_limit = endpoint_type.rate_limit();
        let mut history = self.request_history.write().await;

        let timestamps = history.entry(endpoint_type).or_default();
        let now = Instant::now();

        // Remove old timestamps outside the window
        timestamps.retain(|&timestamp| now.duration_since(timestamp) < rate_limit.window);

        // Check if adding this request would exceed the limit
        if timestamps.len() as u32 >= rate_limit.max_requests {
            return Err(RateLimitError::RateLimitExceeded {
                endpoint: endpoint_type,
                current: timestamps.len() as u32,
                max: rate_limit.max_requests,
                window: rate_limit.window,
            });
        }

        Ok(())
    }

    /// Record a request for the given endpoint type
    pub async fn increment_request(&self, endpoint_type: EndpointType) {
        let mut history = self.request_history.write().await;
        let timestamps = history.entry(endpoint_type).or_default();
        timestamps.push(Instant::now());
    }

    /// Get current usage statistics for an endpoint type
    pub async fn get_usage(&self, endpoint_type: EndpointType) -> (u32, u32) {
        let rate_limit = endpoint_type.rate_limit();
        let history = self.request_history.read().await;

        if let Some(timestamps) = history.get(&endpoint_type) {
            let now = Instant::now();
            let recent_count = timestamps
                .iter()
                .filter(|&&timestamp| now.duration_since(timestamp) < rate_limit.window)
                .count() as u32;
            (recent_count, rate_limit.max_requests)
        } else {
            (0, rate_limit.max_requests)
        }
    }
}

#[cfg(test)]
mod tests {
    use tokio::time::{Duration, sleep};

    use super::*;

    #[test]
    fn test_rate_limit_config() {
        let rate_limit = RateLimit::new(100, Duration::from_secs(60));
        assert_eq!(rate_limit.max_requests, 100);
        assert_eq!(rate_limit.window, Duration::from_secs(60));
    }

    #[test]
    fn test_endpoint_type_from_path() {
        assert_eq!(
            EndpointType::from_path("/v1/markets"),
            EndpointType::PublicMarkets
        );
        assert_eq!(
            EndpointType::from_path("/v1/assets/BTC"),
            EndpointType::PublicAssets
        );
        assert_eq!(
            EndpointType::from_path("/v1/accounts/trading-accounts"),
            EndpointType::PrivateTradingAccounts
        );
        assert_eq!(
            EndpointType::from_path("/v2/orders"),
            EndpointType::PrivateOrders
        );
        assert_eq!(
            EndpointType::from_path("/v1/users/login"),
            EndpointType::PrivateLogin
        );
    }

    #[test]
    fn test_endpoint_rate_limits() {
        let public_markets_limit = EndpointType::PublicMarkets.rate_limit();
        assert_eq!(public_markets_limit.max_requests, 50);
        assert_eq!(public_markets_limit.window, Duration::from_secs(1));

        let private_orders_limit = EndpointType::PrivateOrders.rate_limit();
        assert_eq!(private_orders_limit.max_requests, 50);
        assert_eq!(private_orders_limit.window, Duration::from_secs(1));

        let login_limit = EndpointType::PrivateLogin.rate_limit();
        assert_eq!(login_limit.max_requests, 10);
        assert_eq!(login_limit.window, Duration::from_secs(1));
    }

    #[tokio::test]
    async fn test_rate_limiter_basic() {
        let rate_limiter = RateLimiter::new();

        // Should allow first request
        assert!(
            rate_limiter
                .check_limits(EndpointType::PublicMarkets)
                .await
                .is_ok()
        );
        rate_limiter
            .increment_request(EndpointType::PublicMarkets)
            .await;

        let (current, max) = rate_limiter.get_usage(EndpointType::PublicMarkets).await;
        assert_eq!(current, 1);
        assert_eq!(max, 50);
    }

    #[tokio::test]
    async fn test_rate_limiter_limit_exceeded() {
        let rate_limiter = RateLimiter::new();
        let endpoint = EndpointType::PrivateLogin; // Has limit of 10 per second

        // Make 10 requests (at the limit)
        for _ in 0..10 {
            assert!(rate_limiter.check_limits(endpoint).await.is_ok());
            rate_limiter.increment_request(endpoint).await;
        }

        // 11th request should fail
        assert!(rate_limiter.check_limits(endpoint).await.is_err());
    }

    #[tokio::test]
    async fn test_rate_limiter_window_expiry() {
        let rate_limiter = RateLimiter::new();
        let endpoint = EndpointType::PrivateLogin;

        // Fill up the rate limit
        for _ in 0..10 {
            rate_limiter.increment_request(endpoint).await;
        }

        // Should be at limit
        assert!(rate_limiter.check_limits(endpoint).await.is_err());

        // Wait for window to expire (1 second + buffer)
        sleep(Duration::from_millis(1100)).await;

        // Should be able to make requests again
        assert!(rate_limiter.check_limits(endpoint).await.is_ok());
    }
}
