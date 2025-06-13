use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

/// Represents different types of crypto.com API endpoints for rate limiting
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EndpointType {
    // Private REST endpoints
    PrivateCreateOrder,
    PrivateCancelOrder,
    PrivateCancelAllOrders,
    PrivateGetOrderDetail,
    PrivateGetTrades,
    PrivateGetOrderHistory,
    PrivateOther,
    
    // Public REST endpoints
    PublicGetAnnouncements,
    PublicGetRiskParameters,
    PublicGetInstruments,
    PublicGetBook,
    PublicGetTicker,
    PublicGetTickers,
    PublicGetTrades,
    PublicGetValuations,
    PublicGetCandlestick,
    PublicGetExpiredSettlementPrice,
    PublicGetInsurance,
    
    // Staking endpoints
    PublicStaking,
    PrivateStaking,
    
    // WebSocket
    UserApi,
    MarketData,
}

impl EndpointType {
    /// Get the rate limit for this endpoint type
    pub fn rate_limit(&self) -> RateLimit {
        match self {
            // Private REST endpoints (per API key)
            EndpointType::PrivateCreateOrder 
            | EndpointType::PrivateCancelOrder 
            | EndpointType::PrivateCancelAllOrders => {
                RateLimit::new(15, Duration::from_millis(100))
            },
            EndpointType::PrivateGetOrderDetail => {
                RateLimit::new(30, Duration::from_millis(100))
            },
            EndpointType::PrivateGetTrades 
            | EndpointType::PrivateGetOrderHistory => {
                RateLimit::new(1, Duration::from_secs(1))
            },
            EndpointType::PrivateOther => {
                RateLimit::new(3, Duration::from_millis(100))
            },
            
            // Public REST endpoints (per IP)
            EndpointType::PublicGetAnnouncements
            | EndpointType::PublicGetRiskParameters
            | EndpointType::PublicGetInstruments
            | EndpointType::PublicGetBook
            | EndpointType::PublicGetTicker
            | EndpointType::PublicGetTickers
            | EndpointType::PublicGetTrades
            | EndpointType::PublicGetValuations
            | EndpointType::PublicGetCandlestick
            | EndpointType::PublicGetExpiredSettlementPrice
            | EndpointType::PublicGetInsurance => {
                RateLimit::new(100, Duration::from_secs(1))
            },
            
            // Staking endpoints
            EndpointType::PublicStaking 
            | EndpointType::PrivateStaking => {
                RateLimit::new(50, Duration::from_secs(1))
            },
            
            // WebSocket
            EndpointType::UserApi => {
                RateLimit::new(150, Duration::from_secs(1))
            },
            EndpointType::MarketData => {
                RateLimit::new(100, Duration::from_secs(1))
            },
        }
    }

    /// Helper method to map from endpoint path to endpoint type
    /// This can be used to automatically determine the correct rate limit based on the API path
    pub fn from_path(path: &str) -> Self {
        match path {
            // Private REST endpoints
            "private/create-order" => EndpointType::PrivateCreateOrder,
            "private/cancel-order" => EndpointType::PrivateCancelOrder,
            "private/cancel-all-orders" => EndpointType::PrivateCancelAllOrders,
            "private/get-order-detail" => EndpointType::PrivateGetOrderDetail,
            "private/get-trades" => EndpointType::PrivateGetTrades,
            "private/get-order-history" => EndpointType::PrivateGetOrderHistory,
            
            // Public REST endpoints
            "public/get-announcements" => EndpointType::PublicGetAnnouncements,
            "public/get-risk-parameters" => EndpointType::PublicGetRiskParameters,
            "public/get-instruments" => EndpointType::PublicGetInstruments,
            "public/get-book" => EndpointType::PublicGetBook,
            "public/get-ticker" => EndpointType::PublicGetTicker,
            "public/get-tickers" => EndpointType::PublicGetTickers,
            "public/get-trades" => EndpointType::PublicGetTrades,
            "public/get-valuations" => EndpointType::PublicGetValuations,
            "public/get-candlestick" => EndpointType::PublicGetCandlestick,
            "public/get-expired-settlement-price" => EndpointType::PublicGetExpiredSettlementPrice,
            "public/get-insurance" => EndpointType::PublicGetInsurance,
            
            // Staking endpoints
            path if path.starts_with("public/staking/") => EndpointType::PublicStaking,
            path if path.starts_with("private/staking/") => EndpointType::PrivateStaking,
            
            // Default cases
            path if path.starts_with("private/") => EndpointType::PrivateOther,
            _ => EndpointType::PrivateOther, // Conservative default
        }
    }
}

/// Represents a rate limit with max requests and time window
#[derive(Debug, Clone, Copy)]
pub struct RateLimit {
    pub max_requests: u32,
    pub window: Duration,
}

impl RateLimit {
    pub fn new(max_requests: u32, window: Duration) -> Self {
        Self { max_requests, window }
    }
}

/// Tracks request timestamps for a specific endpoint type
#[derive(Debug, Default, Clone)]
struct EndpointUsage {
    timestamps: VecDeque<Instant>,
}

impl EndpointUsage {
    /// Add a new request timestamp and clean up old ones
    fn add_request(&mut self, now: Instant, window: Duration) {
        self.timestamps.push_back(now);
        self.trim_older_than(now - window);
    }
    
    /// Remove timestamps older than the cutoff
    fn trim_older_than(&mut self, cutoff: Instant) {
        while self.timestamps.front().is_some_and(|&ts| ts < cutoff) {
            self.timestamps.pop_front();
        }
    }
    
    /// Get current request count in the window
    fn current_count(&self) -> usize {
        self.timestamps.len()
    }
}

/// Tracks usage for all endpoint types
#[derive(Debug, Default)]
struct RateLimitUsage {
    /// Per-endpoint usage tracking
    endpoints: HashMap<EndpointType, EndpointUsage>,
}

impl RateLimitUsage {
    /// Get or create usage tracking for an endpoint
    fn get_or_create_endpoint(&mut self, endpoint_type: EndpointType) -> &mut EndpointUsage {
        self.endpoints.entry(endpoint_type).or_default()
    }
}

/// Rate limiter for crypto.com API
#[derive(Debug, Clone, Default)]
pub struct RateLimiter {
    usage: Arc<RwLock<RateLimitUsage>>,
}

/// Errors that can occur during rate limiting
#[derive(Debug, thiserror::Error)]
pub enum RateLimitError {
    #[error("Rate limit exceeded for {endpoint:?}: {current}/{max} requests in {window:?}")]
    RateLimitExceeded {
        endpoint: EndpointType,
        current: u32,
        max: u32,
        window: Duration,
    },
}

impl RateLimiter {
    /// Create a new rate limiter
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Check if a request can be made for the given endpoint type
    pub async fn check_limits(&self, endpoint_type: EndpointType) -> Result<(), RateLimitError> {
        let rate_limit = endpoint_type.rate_limit();
        let usage = self.usage.read().await;
        
        // Check if this endpoint type has usage tracking
        if let Some(endpoint_usage) = usage.endpoints.get(&endpoint_type) {
            let current_count = endpoint_usage.current_count() as u32;
            
            if current_count >= rate_limit.max_requests {
                return Err(RateLimitError::RateLimitExceeded {
                    endpoint: endpoint_type,
                    current: current_count,
                    max: rate_limit.max_requests,
                    window: rate_limit.window,
                });
            }
        }
        
        Ok(())
    }
    
    /// Increment the request counter for the given endpoint type
    pub async fn increment_request(&self, endpoint_type: EndpointType) {
        let rate_limit = endpoint_type.rate_limit();
        let mut usage = self.usage.write().await;
        let now = Instant::now();
        
        let endpoint_usage = usage.get_or_create_endpoint(endpoint_type);
        endpoint_usage.add_request(now, rate_limit.window);
    }
    
    /// Get current usage statistics for an endpoint type
    pub async fn get_usage(&self, endpoint_type: EndpointType) -> (u32, u32) {
        let rate_limit = endpoint_type.rate_limit();
        let usage = self.usage.read().await;
        
        if let Some(endpoint_usage) = usage.endpoints.get(&endpoint_type) {
            (endpoint_usage.current_count() as u32, rate_limit.max_requests)
        } else {
            (0, rate_limit.max_requests)
        }
    }
    
    /// Clean up old timestamps for all endpoints
    pub async fn cleanup_old_timestamps(&self) {
        let mut usage = self.usage.write().await;
        let now = Instant::now();
        
        for (endpoint_type, endpoint_usage) in usage.endpoints.iter_mut() {
            let rate_limit = endpoint_type.rate_limit();
            endpoint_usage.trim_older_than(now - rate_limit.window);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::{sleep, Duration};

    #[tokio::test]
    async fn test_rate_limiter_creation() {
        let limiter = RateLimiter::new();
        
        // Should be able to check limits for any endpoint initially
        let result = limiter.check_limits(EndpointType::PrivateCreateOrder).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_endpoint_rate_limits() {
        // Test that endpoint types return correct rate limits
        let create_order_limit = EndpointType::PrivateCreateOrder.rate_limit();
        assert_eq!(create_order_limit.max_requests, 15);
        assert_eq!(create_order_limit.window, Duration::from_millis(100));

        let order_detail_limit = EndpointType::PrivateGetOrderDetail.rate_limit();
        assert_eq!(order_detail_limit.max_requests, 30);
        assert_eq!(order_detail_limit.window, Duration::from_millis(100));

        let get_trades_limit = EndpointType::PrivateGetTrades.rate_limit();
        assert_eq!(get_trades_limit.max_requests, 1);
        assert_eq!(get_trades_limit.window, Duration::from_secs(1));

        let public_book_limit = EndpointType::PublicGetBook.rate_limit();
        assert_eq!(public_book_limit.max_requests, 100);
        assert_eq!(public_book_limit.window, Duration::from_secs(1));
    }

    #[tokio::test]
    async fn test_rate_limit_enforcement() {
        let limiter = RateLimiter::new();
        let endpoint = EndpointType::PrivateCreateOrder; // 15 requests per 100ms

        // Should be able to make 15 requests
        for _ in 0..15 {
            limiter.increment_request(endpoint).await;
            let result = limiter.check_limits(endpoint).await;
            if result.is_err() {
                // If we hit the limit early, that's expected due to timing
                break;
            }
        }

        // The 16th request should fail
        limiter.increment_request(endpoint).await;
        let result = limiter.check_limits(endpoint).await;
        assert!(result.is_err());

        if let Err(RateLimitError::RateLimitExceeded { current, max, .. }) = result {
            assert!(current >= max);
        } else {
            panic!("Expected RateLimitExceeded error");
        }
    }

    #[tokio::test]
    async fn test_rate_limit_recovery() {
        let limiter = RateLimiter::new();
        let endpoint = EndpointType::PrivateGetTrades; // 1 request per second

        // Make a request
        limiter.increment_request(endpoint).await;
        
        // Second request should fail
        limiter.increment_request(endpoint).await;
        let result = limiter.check_limits(endpoint).await;
        assert!(result.is_err());

        // Wait for the rate limit window to pass
        sleep(Duration::from_millis(1100)).await;

        // Clean up old timestamps
        limiter.cleanup_old_timestamps().await;

        // Should be able to make requests again
        let result = limiter.check_limits(endpoint).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_usage_statistics() {
        let limiter = RateLimiter::new();
        let endpoint = EndpointType::PrivateOther; // 3 requests per 100ms

        // Initially should have 0 usage
        let (current, max) = limiter.get_usage(endpoint).await;
        assert_eq!(current, 0);
        assert_eq!(max, 3);

        // Add some requests
        limiter.increment_request(endpoint).await;
        limiter.increment_request(endpoint).await;

        let (current, max) = limiter.get_usage(endpoint).await;
        assert_eq!(current, 2);
        assert_eq!(max, 3);
    }

    #[tokio::test]
    async fn test_multiple_endpoint_types() {
        let limiter = RateLimiter::new();
        
        // Different endpoints should have independent rate limits
        let endpoint1 = EndpointType::PrivateCreateOrder; // 15 per 100ms
        let endpoint2 = EndpointType::PublicGetBook; // 100 per second

        // Fill up endpoint1
        for _ in 0..15 {
            limiter.increment_request(endpoint1).await;
        }
        limiter.increment_request(endpoint1).await;
        
        // endpoint1 should be rate limited
        let result1 = limiter.check_limits(endpoint1).await;
        assert!(result1.is_err());

        // endpoint2 should still be available
        let result2 = limiter.check_limits(endpoint2).await;
        assert!(result2.is_ok());
    }

    #[tokio::test]
    async fn test_websocket_rate_limits() {
        let user_api_limit = EndpointType::UserApi.rate_limit();
        assert_eq!(user_api_limit.max_requests, 150);
        assert_eq!(user_api_limit.window, Duration::from_secs(1));

        let market_data_limit = EndpointType::MarketData.rate_limit();
        assert_eq!(market_data_limit.max_requests, 100);
        assert_eq!(market_data_limit.window, Duration::from_secs(1));
    }

    #[tokio::test]
    async fn test_staking_rate_limits() {
        let public_staking_limit = EndpointType::PublicStaking.rate_limit();
        assert_eq!(public_staking_limit.max_requests, 50);
        assert_eq!(public_staking_limit.window, Duration::from_secs(1));

        let private_staking_limit = EndpointType::PrivateStaking.rate_limit();
        assert_eq!(private_staking_limit.max_requests, 50);
        assert_eq!(private_staking_limit.window, Duration::from_secs(1));
    }

    #[tokio::test]
    async fn test_endpoint_from_path() {
        // Test private endpoints
        assert_eq!(EndpointType::from_path("private/create-order"), EndpointType::PrivateCreateOrder);
        assert_eq!(EndpointType::from_path("private/cancel-order"), EndpointType::PrivateCancelOrder);
        assert_eq!(EndpointType::from_path("private/cancel-all-orders"), EndpointType::PrivateCancelAllOrders);
        assert_eq!(EndpointType::from_path("private/get-order-detail"), EndpointType::PrivateGetOrderDetail);
        assert_eq!(EndpointType::from_path("private/get-trades"), EndpointType::PrivateGetTrades);
        assert_eq!(EndpointType::from_path("private/get-order-history"), EndpointType::PrivateGetOrderHistory);

        // Test public endpoints
        assert_eq!(EndpointType::from_path("public/get-announcements"), EndpointType::PublicGetAnnouncements);
        assert_eq!(EndpointType::from_path("public/get-risk-parameters"), EndpointType::PublicGetRiskParameters);
        assert_eq!(EndpointType::from_path("public/get-instruments"), EndpointType::PublicGetInstruments);
        assert_eq!(EndpointType::from_path("public/get-book"), EndpointType::PublicGetBook);
        assert_eq!(EndpointType::from_path("public/get-ticker"), EndpointType::PublicGetTicker);
        assert_eq!(EndpointType::from_path("public/get-tickers"), EndpointType::PublicGetTickers);
        assert_eq!(EndpointType::from_path("public/get-trades"), EndpointType::PublicGetTrades);
        assert_eq!(EndpointType::from_path("public/get-valuations"), EndpointType::PublicGetValuations);
        assert_eq!(EndpointType::from_path("public/get-candlestick"), EndpointType::PublicGetCandlestick);
        assert_eq!(EndpointType::from_path("public/get-expired-settlement-price"), EndpointType::PublicGetExpiredSettlementPrice);
        assert_eq!(EndpointType::from_path("public/get-insurance"), EndpointType::PublicGetInsurance);

        // Test staking endpoints
        assert_eq!(EndpointType::from_path("public/staking/get-products"), EndpointType::PublicStaking);
        assert_eq!(EndpointType::from_path("private/staking/get-stakes"), EndpointType::PrivateStaking);

        // Test default cases
        assert_eq!(EndpointType::from_path("private/some-other-endpoint"), EndpointType::PrivateOther);
        assert_eq!(EndpointType::from_path("unknown/endpoint"), EndpointType::PrivateOther);
    }
}