use std::time::{Duration, Instant};
use thiserror::Error;
use tokio::sync::RwLock;

/// Account tiers for Deribit matching engine rate limits based on 7-day trading volume
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AccountTier {
    /// Tier 1: Over USD 25 million - 30 req/sec sustained, 100 burst
    Tier1,
    /// Tier 2: Over USD 5 million - 20 req/sec sustained, 50 burst
    Tier2,
    /// Tier 3: Over USD 1 million - 10 req/sec sustained, 30 burst
    Tier3,
    /// Tier 4: Up to USD 1 million - 5 req/sec sustained, 20 burst
    Tier4,
}

impl AccountTier {
    /// Get the sustained rate limit (requests per second) for this tier
    pub fn sustained_rate(&self) -> u32 {
        match self {
            AccountTier::Tier1 => 30,
            AccountTier::Tier2 => 20,
            AccountTier::Tier3 => 10,
            AccountTier::Tier4 => 5,
        }
    }

    /// Get the burst limit (number of requests) for this tier
    pub fn burst_limit(&self) -> u32 {
        match self {
            AccountTier::Tier1 => 100,
            AccountTier::Tier2 => 50,
            AccountTier::Tier3 => 30,
            AccountTier::Tier4 => 20,
        }
    }
}

/// Types of endpoints for Deribit rate limiting
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EndpointType {
    /// Non-matching engine requests (500 credits each)
    NonMatchingEngine,
    /// Matching engine requests (trading operations)
    MatchingEngine,
    /// Special case: public/get_instruments (1 req per 10s, burst of 5)
    PublicGetInstruments,
}

impl EndpointType {
    /// Get the credit cost for this endpoint type
    pub fn credit_cost(&self) -> u32 {
        match self {
            EndpointType::NonMatchingEngine => 500,
            EndpointType::MatchingEngine => 0, // Uses tier-based limits, not credits
            EndpointType::PublicGetInstruments => 0, // Special time-based limit
        }
    }

    /// Determine endpoint type from API path
    pub fn from_path(path: &str) -> Self {
        if path == "public/get_instruments" {
            return EndpointType::PublicGetInstruments;
        }

        // Matching engine endpoints as per Deribit documentation
        match path {
            "private/buy" | "private/sell" | "private/edit" | "private/edit_by_label"
            | "private/cancel" | "private/cancel_by_label" | "private/cancel_all"
            | "private/cancel_all_by_instrument" | "private/cancel_all_by_currency"
            | "private/cancel_all_by_kind_or_type" | "private/close_position"
            | "private/verify_block_trade" | "private/execute_block_trade"
            | "private/move_positions" | "private/mass_quote" | "private/cancel_quotes"
            | "private/add_block_rfq_quote" | "private/edit_block_rfq_quote"
            | "private/cancel_block_rfq_quote" | "private/cancel_all_block_rfq_quotes" => {
                EndpointType::MatchingEngine
            }
            _ => EndpointType::NonMatchingEngine,
        }
    }
}

/// Rate limiting errors for Deribit
#[derive(Error, Debug)]
pub enum RateLimitError {
    #[error("Credit limit exceeded: {available} credits available, {required} credits required")]
    CreditLimitExceeded { available: u32, required: u32 },
    
    #[error("Matching engine rate limit exceeded for tier {tier:?}: {requests_in_window} requests in current window")]
    MatchingEngineRateExceeded {
        tier: AccountTier,
        requests_in_window: usize,
    },
    
    #[error("Special endpoint rate limit exceeded for {endpoint}: {requests_in_window} requests in current window")]
    SpecialEndpointRateExceeded {
        endpoint: String,
        requests_in_window: usize,
    },
}

/// Credit pool state for non-matching engine requests
#[derive(Debug, Clone)]
struct CreditPool {
    /// Current available credits
    available_credits: u32,
    /// Maximum credits that can be accumulated
    max_credits: u32,
    /// Credits refilled per second
    refill_rate: u32,
    /// Last time credits were refilled
    last_refill: Instant,
}

impl CreditPool {
    fn new(max_credits: u32, refill_rate: u32) -> Self {
        Self {
            available_credits: max_credits,
            max_credits,
            refill_rate,
            last_refill: Instant::now(),
        }
    }

    /// Refill credits based on elapsed time
    #[allow(clippy::arithmetic_side_effects, clippy::float_arithmetic)]
    fn refill(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_refill);
        let credits_to_add = (elapsed.as_secs_f64() * self.refill_rate as f64) as u32;
        
        if credits_to_add > 0 {
            self.available_credits = (self.available_credits + credits_to_add).min(self.max_credits);
            self.last_refill = now;
        }
    }

    /// Check if enough credits are available and consume them
    fn consume_credits(&mut self, required: u32) -> Result<(), RateLimitError> {
        self.refill();
        
        if self.available_credits < required {
            return Err(RateLimitError::CreditLimitExceeded {
                available: self.available_credits,
                required,
            });
        }
        
        self.available_credits = self.available_credits.saturating_sub(required);
        Ok(())
    }
}

/// Request history for time-based rate limiting (matching engine and special endpoints)
#[derive(Debug, Clone)]
struct RequestHistory {
    /// Timestamps of recent requests
    timestamps: Vec<Instant>,
    /// Window duration for rate limiting
    window: Duration,
    /// Maximum requests allowed in the window
    max_requests: u32,
}

impl RequestHistory {
    fn new(window: Duration, max_requests: u32) -> Self {
        Self {
            timestamps: Vec::new(),
            window,
            max_requests,
        }
    }

    /// Clean old timestamps outside the window
    #[allow(clippy::arithmetic_side_effects)]
    fn clean_old_timestamps(&mut self) {
        let cutoff = Instant::now() - self.window;
        self.timestamps.retain(|&timestamp| timestamp > cutoff);
    }

    /// Check if a new request can be made
    fn check_limit(&mut self) -> Result<(), usize> {
        self.clean_old_timestamps();
        
        if self.timestamps.len() >= self.max_requests as usize {
            return Err(self.timestamps.len());
        }
        
        Ok(())
    }

    /// Record a new request
    fn record_request(&mut self) {
        self.timestamps.push(Instant::now());
    }
}

/// Deribit rate limiter implementing credit-based system
#[derive(Debug)]
pub struct RateLimiter {
    /// Credit pool for non-matching engine requests
    credit_pool: RwLock<CreditPool>,
    /// Account tier for matching engine limits
    account_tier: AccountTier,
    /// Request history for matching engine endpoints
    matching_engine_history: RwLock<RequestHistory>,
    /// Request history for public/get_instruments endpoint
    get_instruments_history: RwLock<RequestHistory>,
}

impl RateLimiter {
    /// Create a new rate limiter with default settings for non-matching engine requests
    /// and specified account tier for matching engine requests
    pub fn new(account_tier: AccountTier) -> Self {
        // Default settings for non-matching engine requests
        let credit_pool = CreditPool::new(50_000, 10_000); // 50k max credits, 10k credits/sec refill
        
        // Matching engine limits based on account tier
        let matching_engine_history = RequestHistory::new(
            Duration::from_secs(1),
            account_tier.sustained_rate(),
        );
        
        // Special limit for public/get_instruments: 1 req per 10s, burst of 5
        let get_instruments_history = RequestHistory::new(
            Duration::from_secs(10),
            5, // burst allowance
        );

        Self {
            credit_pool: RwLock::new(credit_pool),
            account_tier,
            matching_engine_history: RwLock::new(matching_engine_history),
            get_instruments_history: RwLock::new(get_instruments_history),
        }
    }

    /// Create a rate limiter with custom credit pool settings
    pub fn with_custom_credits(
        account_tier: AccountTier,
        max_credits: u32,
        refill_rate: u32,
    ) -> Self {
        let credit_pool = CreditPool::new(max_credits, refill_rate);
        
        let matching_engine_history = RequestHistory::new(
            Duration::from_secs(1),
            account_tier.sustained_rate(),
        );
        
        let get_instruments_history = RequestHistory::new(
            Duration::from_secs(10),
            5,
        );

        Self {
            credit_pool: RwLock::new(credit_pool),
            account_tier,
            matching_engine_history: RwLock::new(matching_engine_history),
            get_instruments_history: RwLock::new(get_instruments_history),
        }
    }

    /// Check if a request can be made for the given endpoint type
    pub async fn check_limits(&self, endpoint_type: EndpointType) -> Result<(), RateLimitError> {
        match endpoint_type {
            EndpointType::NonMatchingEngine => {
                let mut pool = self.credit_pool.write().await;
                pool.consume_credits(endpoint_type.credit_cost())
            }
            EndpointType::MatchingEngine => {
                let mut history = self.matching_engine_history.write().await;
                history.check_limit().map_err(|requests_in_window| {
                    RateLimitError::MatchingEngineRateExceeded {
                        tier: self.account_tier,
                        requests_in_window,
                    }
                })
            }
            EndpointType::PublicGetInstruments => {
                let mut history = self.get_instruments_history.write().await;
                history.check_limit().map_err(|requests_in_window| {
                    RateLimitError::SpecialEndpointRateExceeded {
                        endpoint: "public/get_instruments".to_string(),
                        requests_in_window,
                    }
                })
            }
        }
    }

    /// Record a successful request for the given endpoint type
    pub async fn record_request(&self, endpoint_type: EndpointType) {
        match endpoint_type {
            EndpointType::NonMatchingEngine => {
                // Credits are already consumed in check_limits
            }
            EndpointType::MatchingEngine => {
                let mut history = self.matching_engine_history.write().await;
                history.record_request();
            }
            EndpointType::PublicGetInstruments => {
                let mut history = self.get_instruments_history.write().await;
                history.record_request();
            }
        }
    }

    /// Get current rate limit status for debugging/monitoring
    pub async fn get_status(&self) -> RateLimitStatus {
        let pool = self.credit_pool.read().await;
        let mut pool_clone = pool.clone();
        pool_clone.refill(); // Get current state after refill
        
        let matching_history = self.matching_engine_history.read().await;
        let instruments_history = self.get_instruments_history.read().await;

        RateLimitStatus {
            available_credits: pool_clone.available_credits,
            max_credits: pool_clone.max_credits,
            credit_refill_rate: pool_clone.refill_rate,
            account_tier: self.account_tier,
            matching_engine_requests_in_window: matching_history.timestamps.len() as u32,
            instruments_requests_in_window: instruments_history.timestamps.len() as u32,
        }
    }

    /// Update account tier (e.g., when trading volume changes)
    pub async fn update_account_tier(&self, new_tier: AccountTier) {
        if new_tier != self.account_tier {
            let mut history = self.matching_engine_history.write().await;
            *history = RequestHistory::new(
                Duration::from_secs(1),
                new_tier.sustained_rate(),
            );
        }
    }
}

/// Status information for rate limits
#[derive(Debug, Clone)]
pub struct RateLimitStatus {
    /// Currently available credits for non-matching engine requests
    pub available_credits: u32,
    /// Maximum credits in the pool
    pub max_credits: u32,
    /// Credits refilled per second
    pub credit_refill_rate: u32,
    /// Current account tier
    pub account_tier: AccountTier,
    /// Number of matching engine requests in current 1-second window
    pub matching_engine_requests_in_window: u32,
    /// Number of public/get_instruments requests in current 10-second window
    pub instruments_requests_in_window: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::{sleep, Duration};

    #[tokio::test]
    async fn test_account_tier_limits() {
        assert_eq!(AccountTier::Tier1.sustained_rate(), 30);
        assert_eq!(AccountTier::Tier1.burst_limit(), 100);
        
        assert_eq!(AccountTier::Tier4.sustained_rate(), 5);
        assert_eq!(AccountTier::Tier4.burst_limit(), 20);
    }

    #[tokio::test]
    async fn test_endpoint_type_from_path() {
        assert_eq!(
            EndpointType::from_path("public/get_instruments"),
            EndpointType::PublicGetInstruments
        );
        
        assert_eq!(
            EndpointType::from_path("private/buy"),
            EndpointType::MatchingEngine
        );
        
        assert_eq!(
            EndpointType::from_path("private/get_account_summary"),
            EndpointType::NonMatchingEngine
        );
    }

    #[tokio::test]
    async fn test_credit_pool_refill() {
        let mut pool = CreditPool::new(1000, 100); // 1000 max, 100 per second
        pool.available_credits = 500;
        
        // Simulate 1 second passing
        pool.last_refill = Instant::now() - Duration::from_secs(1);
        pool.refill();
        
        assert_eq!(pool.available_credits, 600); // 500 + 100
        
        // Test max limit
        pool.available_credits = 950;
        pool.last_refill = Instant::now() - Duration::from_secs(1);
        pool.refill();
        
        assert_eq!(pool.available_credits, 1000); // Capped at max
    }

    #[tokio::test]
    async fn test_credit_consumption() {
        let mut pool = CreditPool::new(1000, 100);
        
        // Should succeed
        assert!(pool.consume_credits(500).is_ok());
        assert_eq!(pool.available_credits, 500);
        
        // Should fail - not enough credits
        assert!(pool.consume_credits(600).is_err());
        assert_eq!(pool.available_credits, 500); // Unchanged after failure
    }

    #[tokio::test]
    async fn test_rate_limiter_non_matching_engine() {
        let limiter = RateLimiter::new(AccountTier::Tier4);
        
        // Should be able to make requests within credit limit
        assert!(limiter.check_limits(EndpointType::NonMatchingEngine).await.is_ok());
        limiter.record_request(EndpointType::NonMatchingEngine).await;
        
        let status = limiter.get_status().await;
        assert_eq!(status.available_credits, 50_000 - 500); // 500 credits consumed
    }

    #[tokio::test]
    async fn test_rate_limiter_matching_engine() {
        let limiter = RateLimiter::new(AccountTier::Tier4);
        
        // Should be able to make requests within tier limit (5 per second)
        for _ in 0..5 {
            assert!(limiter.check_limits(EndpointType::MatchingEngine).await.is_ok());
            limiter.record_request(EndpointType::MatchingEngine).await;
        }
        
        // 6th request should fail
        assert!(limiter.check_limits(EndpointType::MatchingEngine).await.is_err());
    }

    #[tokio::test]
    async fn test_rate_limiter_special_endpoint() {
        let limiter = RateLimiter::new(AccountTier::Tier4);
        
        // Should be able to make up to 5 requests (burst limit)
        for _ in 0..5 {
            assert!(limiter.check_limits(EndpointType::PublicGetInstruments).await.is_ok());
            limiter.record_request(EndpointType::PublicGetInstruments).await;
        }
        
        // 6th request should fail
        assert!(limiter.check_limits(EndpointType::PublicGetInstruments).await.is_err());
    }

    #[tokio::test]
    async fn test_request_history_cleanup() {
        let mut history = RequestHistory::new(Duration::from_millis(100), 2);
        
        // Add some requests
        history.record_request();
        assert!(history.check_limit().is_ok());
        
        history.record_request();
        assert!(history.check_limit().is_err()); // At limit
        
        // Wait for window to expire
        sleep(Duration::from_millis(150)).await;
        
        // Should be able to make requests again
        assert!(history.check_limit().is_ok());
    }

    #[tokio::test]
    async fn test_rate_limit_status() {
        let limiter = RateLimiter::new(AccountTier::Tier2);
        
        let status = limiter.get_status().await;
        assert_eq!(status.max_credits, 50_000);
        assert_eq!(status.credit_refill_rate, 10_000);
        assert_eq!(status.account_tier, AccountTier::Tier2);
        assert_eq!(status.matching_engine_requests_in_window, 0);
        assert_eq!(status.instruments_requests_in_window, 0);
    }

    #[tokio::test]
    async fn test_update_account_tier() {
        let limiter = RateLimiter::new(AccountTier::Tier4);
        
        // Make some requests at Tier4 (5 per second limit)
        for _ in 0..5 {
            assert!(limiter.check_limits(EndpointType::MatchingEngine).await.is_ok());
            limiter.record_request(EndpointType::MatchingEngine).await;
        }
        
        // Should be at limit
        assert!(limiter.check_limits(EndpointType::MatchingEngine).await.is_err());
        
        // Update to Tier1 (30 per second limit)
        limiter.update_account_tier(AccountTier::Tier1).await;
        
        // Should be able to make more requests now
        assert!(limiter.check_limits(EndpointType::MatchingEngine).await.is_ok());
    }

    #[test]
    fn test_error_display() {
        let error = RateLimitError::CreditLimitExceeded {
            available: 100,
            required: 500,
        };
        assert!(format!("{}", error).contains("100 credits available"));
        assert!(format!("{}", error).contains("500 credits required"));
    }

    #[tokio::test]
    async fn test_realistic_usage_scenario() {
        // Test a realistic scenario with mixed endpoint types
        let limiter = RateLimiter::new(AccountTier::Tier3);
        
        // Make some non-matching engine requests
        for i in 0..10 {
            assert!(limiter.check_limits(EndpointType::NonMatchingEngine).await.is_ok(),
                "Non-matching engine request {} should succeed", i);
            limiter.record_request(EndpointType::NonMatchingEngine).await;
        }
        
        // Make some matching engine requests (Tier3 allows 10 per second)
        for i in 0..10 {
            assert!(limiter.check_limits(EndpointType::MatchingEngine).await.is_ok(),
                "Matching engine request {} should succeed", i);
            limiter.record_request(EndpointType::MatchingEngine).await;
        }
        
        // This should fail as we're at the tier limit
        assert!(limiter.check_limits(EndpointType::MatchingEngine).await.is_err());
        
        // Check status
        let status = limiter.get_status().await;
        assert_eq!(status.account_tier, AccountTier::Tier3);
        assert_eq!(status.available_credits, 50_000 - (10 * 500)); // 10 non-matching requests used
        assert_eq!(status.matching_engine_requests_in_window, 10);
    }

    #[tokio::test]
    async fn test_credit_refill_over_time() {
        // Test that credits refill correctly over time
        let limiter = RateLimiter::with_custom_credits(AccountTier::Tier4, 1000, 100); // 100 credits/sec
        
        // Consume all credits
        for _ in 0..2 {
            assert!(limiter.check_limits(EndpointType::NonMatchingEngine).await.is_ok());
            limiter.record_request(EndpointType::NonMatchingEngine).await;
        }
        
        let status = limiter.get_status().await;
        assert_eq!(status.available_credits, 0); // All credits consumed
        
        // Wait for some credits to refill
        tokio::time::sleep(Duration::from_millis(50)).await; // 0.05 seconds should give us ~5 credits
        
        let status = limiter.get_status().await;
        assert!(status.available_credits > 0, "Credits should have refilled");
        assert!(status.available_credits < 100, "Should not have refilled completely yet");
    }
}