use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use reqwest::header::HeaderMap;
use tokio::sync::{Mutex, Semaphore};

/// Rate limit header information from Gate.io API responses
#[derive(Debug, Clone, Default)]
pub struct RateLimitHeader {
    /// Remaining requests in the current window
    pub requests_remain: Option<u32>,
    /// Total limit for the current window
    pub limit: Option<u32>,
    /// Reset timestamp (Unix timestamp in seconds)
    pub reset_timestamp: Option<u64>,
}

impl RateLimitHeader {
    /// Parse rate limit headers from response
    pub fn from_headers(headers: &HeaderMap) -> Self {
        Self {
            requests_remain: headers
                .get("X-Gate-RateLimit-Requests-Remain")
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.parse().ok()),
            limit: headers
                .get("X-Gate-RateLimit-Limit")
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.parse().ok()),
            reset_timestamp: headers
                .get("X-Gate-RateLimit-Reset-Timestamp")
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.parse().ok()),
        }
    }

    /// Check if rate limit is about to be exceeded
    pub fn is_near_limit(&self) -> bool {
        match (self.requests_remain, self.limit) {
            (Some(remain), Some(limit)) => {
                #[allow(clippy::float_arithmetic)]
                let threshold = (limit as f32 * 0.1) as u32; // 10% threshold
                remain <= threshold
            }
            _ => false,
        }
    }
}

/// Rate limit status
#[derive(Debug, Clone)]
pub struct RateLimitStatus {
    pub endpoint: String,
    pub requests_remaining: u32,
    pub limit: u32,
    pub reset_at: Instant,
}

/// Usage tracking information
#[derive(Debug, Clone)]
pub struct UsageInfo {
    pub requests_made: u32,
    pub last_request: Instant,
    pub reset_time: Instant,
}

/// Rate limiter for Gate.io API endpoints
pub struct RateLimiter {
    /// Semaphores for different endpoint categories
    spot_order_placement: Arc<Semaphore>,
    spot_order_cancellation: Arc<Semaphore>,
    spot_other: Arc<Semaphore>,
    public_endpoints: Arc<Semaphore>,
    unified_borrow_repay: Arc<Semaphore>,
    unified_other: Arc<Semaphore>,
    futures_endpoints: Arc<Semaphore>,
    delivery_endpoints: Arc<Semaphore>,
    options_endpoints: Arc<Semaphore>,
    wallet_endpoints: Arc<Semaphore>,
    margin_endpoints: Arc<Semaphore>,

    /// Last reset times for different categories
    #[allow(dead_code)]
    last_reset: Arc<Mutex<std::collections::HashMap<String, Instant>>>,

    /// Current usage tracking
    usage_tracker: Arc<Mutex<std::collections::HashMap<String, UsageInfo>>>,
}

impl RateLimiter {
    pub fn new() -> Self {
        Self {
            // Spot trading limits (per second or per 10 seconds)
            spot_order_placement: Arc::new(Semaphore::new(10)), // 10 req/s
            spot_order_cancellation: Arc::new(Semaphore::new(200)), // 200 req/s
            spot_other: Arc::new(Semaphore::new(200)),          // 200 req/10s

            // Public endpoints
            public_endpoints: Arc::new(Semaphore::new(1000)), // 1000 req/10s

            // Unified account limits
            unified_borrow_repay: Arc::new(Semaphore::new(15)), // 15 req/10s
            unified_other: Arc::new(Semaphore::new(150)),       // 150 req/10s

            // Futures limits
            futures_endpoints: Arc::new(Semaphore::new(200)), // 200 req/10s

            // Delivery limits
            delivery_endpoints: Arc::new(Semaphore::new(200)), // 200 req/10s

            // Options limits
            options_endpoints: Arc::new(Semaphore::new(200)), // 200 req/10s

            // Wallet limits
            wallet_endpoints: Arc::new(Semaphore::new(100)), // 100 req/10s

            // Margin limits
            margin_endpoints: Arc::new(Semaphore::new(200)), // 200 req/10s

            last_reset: Arc::new(Mutex::new(std::collections::HashMap::new())),
            usage_tracker: Arc::new(Mutex::new(std::collections::HashMap::new())),
        }
    }

    /// Get appropriate semaphore for the endpoint
    pub async fn get_permit(
        &self,
        endpoint: &str,
    ) -> Result<tokio::sync::SemaphorePermit<'_>, tokio::sync::AcquireError> {
        let category = self.categorize_endpoint(endpoint);

        // Track usage
        {
            let mut usage = self.usage_tracker.lock().await;
            let info = usage.entry(category.clone()).or_insert(UsageInfo {
                requests_made: 0,
                last_request: Instant::now(),
                #[allow(clippy::arithmetic_side_effects)]
                reset_time: Instant::now() + Duration::from_secs(10),
            });

            // Reset if needed
            if info.reset_time <= Instant::now() {
                info.requests_made = 0;
                #[allow(clippy::arithmetic_side_effects)]
                {
                    info.reset_time = Instant::now() + Duration::from_secs(10);
                }
            }

            #[allow(clippy::arithmetic_side_effects)]
            {
                info.requests_made += 1;
            }
            info.last_request = Instant::now();
        }

        match category.as_str() {
            "spot_order_placement" => self.spot_order_placement.acquire().await,
            "spot_order_cancellation" => self.spot_order_cancellation.acquire().await,
            "spot_other" => self.spot_other.acquire().await,
            "unified_borrow_repay" => self.unified_borrow_repay.acquire().await,
            "unified_other" => self.unified_other.acquire().await,
            "futures" => self.futures_endpoints.acquire().await,
            "delivery" => self.delivery_endpoints.acquire().await,
            "options" => self.options_endpoints.acquire().await,
            "wallet" => self.wallet_endpoints.acquire().await,
            "margin" => self.margin_endpoints.acquire().await,
            _ => self.public_endpoints.acquire().await,
        }
    }

    /// Categorize endpoint for rate limiting
    fn categorize_endpoint(&self, endpoint: &str) -> String {
        match endpoint {
            // Spot order placement/amendment
            "/spot/orders" | "/spot/batch_orders" | "/spot/amend_batch_orders" => {
                "spot_order_placement".to_string()
            }
            // Spot order cancellation
            endpoint
                if endpoint.starts_with("/spot/")
                    && (endpoint.contains("cancel") || endpoint.contains("DELETE")) =>
            {
                "spot_order_cancellation".to_string()
            }
            // Other spot endpoints
            endpoint if endpoint.starts_with("/spot/") => "spot_other".to_string(),
            // Unified borrow/repay
            "/unified/loans" | "/unified/borrow_or_repay" => "unified_borrow_repay".to_string(),
            // Other unified endpoints
            endpoint if endpoint.starts_with("/unified/") => "unified_other".to_string(),
            // Futures endpoints
            endpoint if endpoint.contains("/futures/") => "futures".to_string(),
            // Delivery endpoints
            endpoint if endpoint.contains("/delivery/") => "delivery".to_string(),
            // Options endpoints
            endpoint if endpoint.contains("/options/") => "options".to_string(),
            // Wallet endpoints
            endpoint if endpoint.starts_with("/wallet/") => "wallet".to_string(),
            // Margin endpoints
            endpoint if endpoint.starts_with("/margin/") => "margin".to_string(),
            // Public endpoints (default)
            _ => "public".to_string(),
        }
    }

    /// Update rate limit status from response headers
    pub fn update_from_headers(
        &self,
        headers: &RateLimitHeader,
        endpoint: &str,
    ) -> Option<RateLimitStatus> {
        if let (Some(remain), Some(limit), Some(reset)) = (
            headers.requests_remain,
            headers.limit,
            headers.reset_timestamp,
        ) {
            #[allow(clippy::unwrap_used)]
            #[allow(clippy::arithmetic_side_effects)]
            let current_timestamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();

            // Handle case where reset timestamp is in the past by using a default duration
            let reset_duration = if reset > current_timestamp {
                Duration::from_secs(reset - current_timestamp)
            } else {
                // If reset time is in the past, assume a 10-second window
                Duration::from_secs(10)
            };

            #[allow(clippy::arithmetic_side_effects)]
            let reset_at = Instant::now() + reset_duration;

            Some(RateLimitStatus {
                endpoint: endpoint.to_string(),
                requests_remaining: remain,
                limit,
                reset_at,
            })
        } else {
            None
        }
    }

    /// Get current usage statistics
    pub async fn get_usage_stats(&self) -> std::collections::HashMap<String, UsageInfo> {
        self.usage_tracker.lock().await.clone()
    }

    /// Check if we're approaching rate limits for any category
    pub async fn get_rate_limit_warnings(&self) -> Vec<String> {
        let usage = self.usage_tracker.lock().await;
        let mut warnings = Vec::new();

        for (category, info) in usage.iter() {
            let limit = match category.as_str() {
                "spot_order_placement" => 10,
                "spot_order_cancellation" => 200,
                "spot_other" => 200,
                "unified_borrow_repay" => 15,
                "unified_other" => 150,
                "futures" | "delivery" | "options" | "margin" => 200,
                "wallet" => 100,
                "public" => 1000,
                _ => 1000,
            };

            #[allow(clippy::float_arithmetic)]
            let usage_percentage = (info.requests_made as f32 / limit as f32) * 100.0;
            if usage_percentage > 80.0 {
                warnings.push(format!(
                    "Category '{}' at {:.1}% usage ({}/{})",
                    category, usage_percentage, info.requests_made, limit
                ));
            }
        }

        warnings
    }

    /// Reset usage for a specific category (useful for testing)
    pub async fn reset_category(&self, category: &str) {
        let mut usage = self.usage_tracker.lock().await;
        usage.remove(category);
    }

    /// Reset all usage statistics
    pub async fn reset_all(&self) {
        let mut usage = self.usage_tracker.lock().await;
        usage.clear();
    }
}

impl Default for RateLimiter {
    fn default() -> Self {
        Self::new()
    }
}
