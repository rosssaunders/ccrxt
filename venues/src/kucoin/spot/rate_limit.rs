//! KuCoin rate limiting functionality
#![allow(clippy::arithmetic_side_effects)]

use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::sync::RwLock;

/// VIP levels for KuCoin users, affecting rate limits
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum VipLevel {
    #[default]
    Vip0,
    Vip1,
    Vip2,
    Vip3,
    Vip4,
    Vip5,
    Vip6,
    Vip7,
    Vip8,
    Vip9,
    Vip10,
    Vip11,
    Vip12,
}

impl VipLevel {
    /// Get the rate limits for each resource pool based on VIP level
    pub fn limits(&self) -> ResourcePoolLimits {
        match self {
            VipLevel::Vip0 => ResourcePoolLimits {
                spot: 4000,
                futures: 2000,
                management: 2000,
                earn: 2000,
                copy_trading: 2000,
                public: 2000,
            },
            VipLevel::Vip1 => ResourcePoolLimits {
                spot: 6000,
                futures: 2000,
                management: 2000,
                earn: 2000,
                copy_trading: 2000,
                public: 2000,
            },
            VipLevel::Vip2 => ResourcePoolLimits {
                spot: 8000,
                futures: 4000,
                management: 4000,
                earn: 2000,
                copy_trading: 2000,
                public: 2000,
            },
            VipLevel::Vip3 => ResourcePoolLimits {
                spot: 10000,
                futures: 5000,
                management: 5000,
                earn: 2000,
                copy_trading: 2000,
                public: 2000,
            },
            VipLevel::Vip4 => ResourcePoolLimits {
                spot: 13000,
                futures: 6000,
                management: 6000,
                earn: 2000,
                copy_trading: 2000,
                public: 2000,
            },
            VipLevel::Vip5 => ResourcePoolLimits {
                spot: 16000,
                futures: 7000,
                management: 7000,
                earn: 2000,
                copy_trading: 2000,
                public: 2000,
            },
            VipLevel::Vip6 => ResourcePoolLimits {
                spot: 20000,
                futures: 8000,
                management: 8000,
                earn: 2000,
                copy_trading: 2000,
                public: 2000,
            },
            VipLevel::Vip7 => ResourcePoolLimits {
                spot: 23000,
                futures: 10000,
                management: 10000,
                earn: 2000,
                copy_trading: 2000,
                public: 2000,
            },
            VipLevel::Vip8 => ResourcePoolLimits {
                spot: 26000,
                futures: 12000,
                management: 12000,
                earn: 2000,
                copy_trading: 2000,
                public: 2000,
            },
            VipLevel::Vip9 => ResourcePoolLimits {
                spot: 30000,
                futures: 14000,
                management: 14000,
                earn: 2000,
                copy_trading: 2000,
                public: 2000,
            },
            VipLevel::Vip10 => ResourcePoolLimits {
                spot: 33000,
                futures: 16000,
                management: 16000,
                earn: 2000,
                copy_trading: 2000,
                public: 2000,
            },
            VipLevel::Vip11 => ResourcePoolLimits {
                spot: 36000,
                futures: 18000,
                management: 18000,
                earn: 2000,
                copy_trading: 2000,
                public: 2000,
            },
            VipLevel::Vip12 => ResourcePoolLimits {
                spot: 40000,
                futures: 20000,
                management: 20000,
                earn: 2000,
                copy_trading: 2000,
                public: 2000,
            },
        }
    }
}

/// Resource pool limits for different endpoint categories
#[derive(Debug, Clone, Copy)]
pub struct ResourcePoolLimits {
    /// Spot trading (including margin) requests per 30 seconds
    pub spot: u32,
    /// Futures trading requests per 30 seconds
    pub futures: u32,
    /// Management/account requests per 30 seconds
    pub management: u32,
    /// Earn product requests per 30 seconds
    pub earn: u32,
    /// Copy trading requests per 30 seconds
    pub copy_trading: u32,
    /// Public endpoint requests per 30 seconds (IP-based)
    pub public: u32,
}

/// Types of endpoint resource pools
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResourcePool {
    /// Spot trading endpoints (including margin)
    Spot,
    /// Futures trading endpoints
    Futures,
    /// Account management endpoints
    Management,
    /// Earn product endpoints
    Earn,
    /// Copy trading endpoints
    CopyTrading,
    /// Public endpoints (IP-based rate limiting)
    Public,
}

impl ResourcePool {
    /// Get the quota limit for this resource pool based on VIP level
    pub fn get_limit(&self, vip_level: VipLevel) -> u32 {
        let limits = vip_level.limits();
        match self {
            ResourcePool::Spot => limits.spot,
            ResourcePool::Futures => limits.futures,
            ResourcePool::Management => limits.management,
            ResourcePool::Earn => limits.earn,
            ResourcePool::CopyTrading => limits.copy_trading,
            ResourcePool::Public => limits.public,
        }
    }

    /// Determine the resource pool for an endpoint based on its path
    /// This helps categorize endpoints into the correct rate limit pools
    pub fn from_endpoint_path(path: &str) -> Self {
        // Spot trading endpoints (including margin)
        if path.contains("/api/v1/orders")
            || path.contains("/api/v1/stop-order")
            || path.contains("/api/v1/margin/")
            || path.contains("/api/v1/isolated/")
            || path.contains("/api/v1/hf/orders")
            || path.contains("/api/v1/oco/order")
        {
            return ResourcePool::Spot;
        }

        // Futures trading endpoints
        if path.contains("/api/v1/contracts/")
            || path.contains("/api/v1/position")
            || path.contains("/api/v1/trade-history")
            || path.contains("/api/v1/funding-history")
        {
            return ResourcePool::Futures;
        }

        // Account management endpoints
        if path.contains("/api/v1/accounts")
            || path.contains("/api/v1/sub/")
            || path.contains("/api/v1/user-info")
            || path.contains("/api/v1/deposit")
            || path.contains("/api/v1/withdrawals")
            || path.contains("/api/v1/transfer")
        {
            return ResourcePool::Management;
        }

        // Earn product endpoints
        if path.contains("/api/v1/earn/")
            || path.contains("/api/v1/lending/")
            || path.contains("/api/v1/margin/lend/")
        {
            return ResourcePool::Earn;
        }

        // Copy trading endpoints
        if path.contains("/api/v1/copytrading/") {
            return ResourcePool::CopyTrading;
        }

        // Default to public for unmatched endpoints (most market data endpoints)
        ResourcePool::Public
    }
}

/// Rate limiting errors
#[derive(Error, Debug)]
pub enum RateLimitError {
    #[error("Rate limit exceeded for resource pool: {pool:?}. Used: {used}, Limit: {limit}")]
    Exceeded {
        pool: ResourcePool,
        used: u32,
        limit: u32,
    },

    #[error("Server overload rate limit (error code 429000)")]
    ServerOverload,
}

/// Resource pool usage tracker
#[derive(Debug, Clone)]
struct ResourcePoolTracker {
    /// Current weight used in the current 30-second window
    used_weight: u32,
    /// Maximum weight allowed in 30 seconds
    max_weight: u32,
    /// When the current window started
    window_start: Instant,
    /// Duration of the rate limit window (30 seconds)
    window_duration: Duration,
}

impl ResourcePoolTracker {
    fn new(max_weight: u32) -> Self {
        Self {
            used_weight: 0,
            max_weight,
            window_start: Instant::now(),
            window_duration: Duration::from_secs(30),
        }
    }

    /// Check if we can consume the given weight
    fn can_consume(&mut self, weight: u32) -> bool {
        self.reset_if_needed();
        self.used_weight + weight <= self.max_weight
    }

    /// Consume the given weight
    fn consume(&mut self, weight: u32) -> Result<(), RateLimitError> {
        self.reset_if_needed();

        if self.used_weight + weight > self.max_weight {
            return Err(RateLimitError::Exceeded {
                pool: ResourcePool::Spot, // Will be overridden by caller
                used: self.used_weight + weight,
                limit: self.max_weight,
            });
        }

        self.used_weight += weight;
        Ok(())
    }

    /// Reset the window if 30 seconds have passed
    fn reset_if_needed(&mut self) {
        if self.window_start.elapsed() >= self.window_duration {
            self.used_weight = 0;
            self.window_start = Instant::now();
        }
    }

    /// Get remaining quota
    fn remaining(&mut self) -> u32 {
        self.reset_if_needed();
        self.max_weight.saturating_sub(self.used_weight)
    }

    /// Get time until reset in milliseconds
    fn reset_time_ms(&self) -> u64 {
        let elapsed = self.window_start.elapsed();
        if elapsed >= self.window_duration {
            0
        } else {
            (self.window_duration - elapsed).as_millis() as u64
        }
    }
}

/// Rate limiter for KuCoin API
#[derive(Debug)]
pub struct RateLimiter {
    /// VIP level of the user
    vip_level: VipLevel,
    /// Resource pool trackers
    trackers: RwLock<HashMap<ResourcePool, ResourcePoolTracker>>,
}

impl RateLimiter {
    /// Create a new rate limiter with default VIP level (VIP0)
    pub fn new() -> Self {
        Self::new_with_vip(VipLevel::Vip0)
    }

    /// Create a new rate limiter for the given VIP level
    pub fn new_with_vip(vip_level: VipLevel) -> Self {
        let limits = vip_level.limits();
        let mut trackers = HashMap::new();

        trackers.insert(ResourcePool::Spot, ResourcePoolTracker::new(limits.spot));
        trackers.insert(
            ResourcePool::Futures,
            ResourcePoolTracker::new(limits.futures),
        );
        trackers.insert(
            ResourcePool::Management,
            ResourcePoolTracker::new(limits.management),
        );
        trackers.insert(ResourcePool::Earn, ResourcePoolTracker::new(limits.earn));
        trackers.insert(
            ResourcePool::CopyTrading,
            ResourcePoolTracker::new(limits.copy_trading),
        );
        trackers.insert(
            ResourcePool::Public,
            ResourcePoolTracker::new(limits.public),
        );

        Self {
            vip_level,
            trackers: RwLock::new(trackers),
        }
    }

    /// Check if a request can be made for the given resource pool with the specified weight
    pub async fn check_limits(
        &self,
        pool: ResourcePool,
        weight: u32,
    ) -> Result<(), RateLimitError> {
        let mut trackers = self.trackers.write().await;

        if let Some(tracker) = trackers.get_mut(&pool) {
            tracker.consume(weight).map_err(|mut e| {
                // Override the pool in the error to be correct
                if let RateLimitError::Exceeded {
                    pool: ref mut p, ..
                } = e
                {
                    *p = pool;
                }
                e
            })
        } else {
            // This shouldn't happen if we initialize all pools correctly
            Ok(())
        }
    }

    /// Record a successful request for the given resource pool with the specified weight
    pub async fn record_request(&self, pool: ResourcePool, weight: u32) {
        // The weight is already consumed in check_limits, so this is a no-op
        // But we keep this method for consistency with other exchanges
        let _ = (pool, weight);
    }

    /// Update VIP level and recreate trackers with new limits
    pub async fn update_vip_level(&mut self, new_vip_level: VipLevel) {
        if new_vip_level == self.vip_level {
            return;
        }

        self.vip_level = new_vip_level;
        let mut trackers = self.trackers.write().await;
        let limits = new_vip_level.limits();

        // Update max weights for existing trackers while preserving current usage
        if let Some(tracker) = trackers.get_mut(&ResourcePool::Spot) {
            tracker.max_weight = limits.spot;
        }
        if let Some(tracker) = trackers.get_mut(&ResourcePool::Futures) {
            tracker.max_weight = limits.futures;
        }
        if let Some(tracker) = trackers.get_mut(&ResourcePool::Management) {
            tracker.max_weight = limits.management;
        }
        if let Some(tracker) = trackers.get_mut(&ResourcePool::Earn) {
            tracker.max_weight = limits.earn;
        }
        if let Some(tracker) = trackers.get_mut(&ResourcePool::CopyTrading) {
            tracker.max_weight = limits.copy_trading;
        }
        if let Some(tracker) = trackers.get_mut(&ResourcePool::Public) {
            tracker.max_weight = limits.public;
        }
    }

    /// Get current rate limit status for a resource pool
    pub async fn get_status(&self, pool: ResourcePool) -> Option<RateLimitStatus> {
        let mut trackers = self.trackers.write().await;

        trackers.get_mut(&pool).map(|tracker| RateLimitStatus {
            limit: tracker.max_weight,
            remaining: tracker.remaining(),
            reset_time_ms: tracker.reset_time_ms(),
            used: tracker.used_weight,
        })
    }

    /// Get all resource pool statuses
    pub async fn get_all_statuses(&self) -> HashMap<ResourcePool, RateLimitStatus> {
        let mut trackers = self.trackers.write().await;
        let mut statuses = HashMap::new();

        for (pool, tracker) in trackers.iter_mut() {
            statuses.insert(
                *pool,
                RateLimitStatus {
                    limit: tracker.max_weight,
                    remaining: tracker.remaining(),
                    reset_time_ms: tracker.reset_time_ms(),
                    used: tracker.used_weight,
                },
            );
        }

        statuses
    }

    /// Check if a request can be made without consuming quota
    pub async fn check_can_proceed(&self, pool: ResourcePool, weight: u32) -> bool {
        let mut trackers = self.trackers.write().await;

        if let Some(tracker) = trackers.get_mut(&pool) {
            tracker.can_consume(weight)
        } else {
            false
        }
    }

    /// Wait until the rate limit allows the request (async backoff)
    pub async fn wait_if_needed(
        &self,
        pool: ResourcePool,
        weight: u32,
    ) -> Result<(), RateLimitError> {
        // First check if we can proceed immediately
        if self.check_can_proceed(pool, weight).await {
            return self.check_limits(pool, weight).await;
        }

        // If not, wait for the next reset window
        if let Some(status) = self.get_status(pool).await
            && status.reset_time_ms > 0
        {
            tokio::time::sleep(Duration::from_millis(status.reset_time_ms + 100)).await; // Add small buffer
        }

        // Try again after waiting
        self.check_limits(pool, weight).await
    }

    /// Get the current VIP level
    pub fn vip_level(&self) -> VipLevel {
        self.vip_level
    }

    /// Backward compatibility: Check if a request can proceed (uses default weight 1 for Public pool)
    pub async fn can_proceed(&self) -> bool {
        // For backward compatibility, assume public endpoint with weight 1
        self.check_limits(ResourcePool::Public, 1).await.is_ok()
    }

    /// Backward compatibility: Update from headers (extracts rate limit info but doesn't change limits)
    pub fn update_from_headers(&mut self, headers: &HashMap<String, String>) {
        // For backward compatibility, we extract the headers but don't modify the rate limiter state
        // This is because KuCoin's rate limiting is client-side based on known limits
        let _rate_limit_header = RateLimitHeader::from_headers(headers);
    }
}

impl Clone for RateLimiter {
    fn clone(&self) -> Self {
        Self::new_with_vip(self.vip_level)
    }
}

impl Default for RateLimiter {
    fn default() -> Self {
        Self::new()
    }
}

/// Current status of a resource pool's rate limits
#[derive(Debug, Clone)]
pub struct RateLimitStatus {
    /// Total quota limit for this resource pool
    pub limit: u32,
    /// Remaining quota in current window
    pub remaining: u32,
    /// Time until quota reset in milliseconds
    pub reset_time_ms: u64,
    /// Currently used quota
    pub used: u32,
}

/// Rate limit header information from KuCoin responses
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RateLimitHeader {
    /// Total resource pool quota (gw-ratelimit-limit)
    pub limit: Option<u32>,
    /// Resource pool remaining quota (gw-ratelimit-remaining)
    pub remaining: Option<u32>,
    /// Resource pool quota reset countdown in milliseconds (gw-ratelimit-reset)
    pub reset: Option<u64>,
}

impl RateLimitHeader {
    /// Extract rate limit headers from response headers
    pub fn from_headers(headers: &HashMap<String, String>) -> Self {
        let limit = headers
            .get("gw-ratelimit-limit")
            .and_then(|s| s.parse().ok());

        let remaining = headers
            .get("gw-ratelimit-remaining")
            .and_then(|s| s.parse().ok());

        let reset = headers
            .get("gw-ratelimit-reset")
            .and_then(|s| s.parse().ok());

        Self {
            limit,
            remaining,
            reset,
        }
    }

    /// Convert to HashMap for easier access
    pub fn to_hashmap(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();

        if let Some(limit) = self.limit {
            map.insert("gw-ratelimit-limit".to_string(), limit.to_string());
        }
        if let Some(remaining) = self.remaining {
            map.insert("gw-ratelimit-remaining".to_string(), remaining.to_string());
        }
        if let Some(reset) = self.reset {
            map.insert("gw-ratelimit-reset".to_string(), reset.to_string());
        }

        map
    }
}

#[cfg(test)]
mod tests {
    use tokio::time::Duration;

    use super::*;

    #[test]
    fn test_vip_level_limits() {
        let vip0_limits = VipLevel::Vip0.limits();
        assert_eq!(vip0_limits.spot, 4000);
        assert_eq!(vip0_limits.futures, 2000);

        let vip12_limits = VipLevel::Vip12.limits();
        assert_eq!(vip12_limits.spot, 40000);
        assert_eq!(vip12_limits.futures, 20000);
    }

    #[tokio::test]
    async fn test_rate_limiter_basic() {
        let limiter = RateLimiter::new_with_vip(VipLevel::Vip0);

        // Should be able to make a request with weight 1
        assert!(limiter.check_limits(ResourcePool::Spot, 1).await.is_ok());

        // Check status
        let status = limiter.get_status(ResourcePool::Spot).await.unwrap();
        assert_eq!(status.limit, 4000);
        assert_eq!(status.used, 1);
        assert_eq!(status.remaining, 3999);
    }

    #[tokio::test]
    async fn test_rate_limiter_exceeded() {
        let limiter = RateLimiter::new_with_vip(VipLevel::Vip0);

        // Try to exceed the spot limit (4000)
        let result = limiter.check_limits(ResourcePool::Spot, 4001).await;
        assert!(result.is_err());

        if let Err(RateLimitError::Exceeded { pool, used, limit }) = result {
            assert_eq!(pool, ResourcePool::Spot);
            assert_eq!(used, 4001);
            assert_eq!(limit, 4000);
        }
    }

    #[tokio::test]
    async fn test_resource_pool_tracker_reset() {
        let mut tracker = ResourcePoolTracker::new(100);

        // Consume some weight
        assert!(tracker.consume(50).is_ok());
        assert_eq!(tracker.used_weight, 50);

        // Fast-forward time by simulating window expiry
        tracker.window_start = Instant::now() - Duration::from_secs(31);
        tracker.reset_if_needed();

        assert_eq!(tracker.used_weight, 0);
    }

    #[tokio::test]
    async fn test_vip_level_update() {
        let mut limiter = RateLimiter::new_with_vip(VipLevel::Vip0);

        // Initial limit should be 4000 for spot
        let status = limiter.get_status(ResourcePool::Spot).await.unwrap();
        assert_eq!(status.limit, 4000);

        // Update to VIP12
        limiter.update_vip_level(VipLevel::Vip12).await;

        let status = limiter.get_status(ResourcePool::Spot).await.unwrap();
        assert_eq!(status.limit, 40000);
    }

    #[test]
    fn test_rate_limit_header() {
        let mut headers = HashMap::new();
        headers.insert("gw-ratelimit-limit".to_string(), "500".to_string());
        headers.insert("gw-ratelimit-remaining".to_string(), "300".to_string());
        headers.insert("gw-ratelimit-reset".to_string(), "1489".to_string());

        let rate_limit_header = RateLimitHeader::from_headers(&headers);
        assert_eq!(rate_limit_header.limit, Some(500));
        assert_eq!(rate_limit_header.remaining, Some(300));
        assert_eq!(rate_limit_header.reset, Some(1489));

        let hashmap = rate_limit_header.to_hashmap();
        assert_eq!(hashmap.get("gw-ratelimit-limit"), Some(&"500".to_string()));
        assert_eq!(
            hashmap.get("gw-ratelimit-remaining"),
            Some(&"300".to_string())
        );
        assert_eq!(hashmap.get("gw-ratelimit-reset"), Some(&"1489".to_string()));
    }

    #[test]
    fn test_resource_pool_from_endpoint_path() {
        // Test spot trading endpoints
        assert_eq!(
            ResourcePool::from_endpoint_path("/api/v1/orders"),
            ResourcePool::Spot
        );
        assert_eq!(
            ResourcePool::from_endpoint_path("/api/v1/margin/borrow"),
            ResourcePool::Spot
        );

        // Test futures endpoints
        assert_eq!(
            ResourcePool::from_endpoint_path("/api/v1/contracts/XBTUSDM"),
            ResourcePool::Futures
        );
        assert_eq!(
            ResourcePool::from_endpoint_path("/api/v1/position"),
            ResourcePool::Futures
        );

        // Test management endpoints
        assert_eq!(
            ResourcePool::from_endpoint_path("/api/v1/accounts"),
            ResourcePool::Management
        );
        assert_eq!(
            ResourcePool::from_endpoint_path("/api/v1/user-info"),
            ResourcePool::Management
        );

        // Test earn endpoints
        assert_eq!(
            ResourcePool::from_endpoint_path("/api/v1/earn/products"),
            ResourcePool::Earn
        );

        // Test copy trading endpoints
        assert_eq!(
            ResourcePool::from_endpoint_path("/api/v1/copytrading/positions"),
            ResourcePool::CopyTrading
        );

        // Test default (public) endpoints
        assert_eq!(
            ResourcePool::from_endpoint_path("/api/v1/market/orderbook/level1"),
            ResourcePool::Public
        );
        assert_eq!(
            ResourcePool::from_endpoint_path("/api/v1/symbols"),
            ResourcePool::Public
        );
    }

    #[tokio::test]
    async fn test_check_can_proceed() {
        let limiter = RateLimiter::new_with_vip(VipLevel::Vip0);

        // Should be able to proceed with small weight
        assert!(limiter.check_can_proceed(ResourcePool::Spot, 100).await);

        // Should not be able to proceed with weight exceeding limit
        assert!(!limiter.check_can_proceed(ResourcePool::Spot, 5000).await);
    }

    #[tokio::test]
    async fn test_get_all_statuses() {
        let limiter = RateLimiter::new_with_vip(VipLevel::Vip1);

        // Consume some quota
        limiter.check_limits(ResourcePool::Spot, 100).await.unwrap();
        limiter
            .check_limits(ResourcePool::Futures, 50)
            .await
            .unwrap();

        let statuses = limiter.get_all_statuses().await;

        // Check that all resource pools are present
        assert_eq!(statuses.len(), 6); // All 6 resource pools

        // Check specific values
        let spot_status = &statuses[&ResourcePool::Spot];
        assert_eq!(spot_status.limit, 6000); // VIP1 limit
        assert_eq!(spot_status.used, 100);
        assert_eq!(spot_status.remaining, 5900);

        let futures_status = &statuses[&ResourcePool::Futures];
        assert_eq!(futures_status.used, 50);
        assert_eq!(futures_status.remaining, 1950);
    }
}
