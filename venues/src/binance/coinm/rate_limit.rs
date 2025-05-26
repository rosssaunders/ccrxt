use super::api_errors::BinanceCoinMAPIError;
use super::enums::{RateLimitInterval, RateLimitType};
use super::types::BinanceCoinMError;
use std::collections::VecDeque;
use std::num::NonZeroU32;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

/// Configuration for a rate limit
#[derive(Debug, Clone)]
pub struct RateLimitConfig {
    /// The type of rate limit (e.g., RequestWeight, Orders, RawRequests)
    pub limit_type: RateLimitType,

    /// The interval for the rate limit (e.g., Second, Minute, Day)
    pub interval: RateLimitInterval,

    /// The maximum number of requests allowed in the interval
    pub limit: NonZeroU32,
}

/// Tracks the current usage of rate limits (rolling windows)
#[derive(Debug, Default, Clone)]
pub struct RateLimitUsage {
    /// Timestamps of raw requests in the last 5 minutes
    pub raw_request_timestamps: VecDeque<Instant>,
    /// Timestamps of order requests in the last 10s, 1m, 1d
    pub order_timestamps_10s: VecDeque<Instant>,
    pub order_timestamps_1m: VecDeque<Instant>,
    pub order_timestamps_1d: VecDeque<Instant>,
    /// Last known request weight (from header)
    pub used_weight_1m: u32,
}

/// Manages rate limiting for Binance Coin-M Futures API
#[derive(Debug, Clone)]
pub struct BinanceCoinMRateLimiter {
    usage: Arc<RwLock<RateLimitUsage>>,
}

impl BinanceCoinMRateLimiter {
    pub fn new() -> Self {
        Self {
            usage: Arc::new(RwLock::new(RateLimitUsage::default())),
        }
    }

    pub fn default() -> Self {
        Self::new()
    }

    /// Call this after every REST call to increment raw_request (5min window, 61,000 cap)
    pub async fn increment_raw_request(&self) {
        let mut usage = self.usage.write().await;
        let now = Instant::now();
        usage.raw_request_timestamps.push_back(now);
        // Remove timestamps older than 5 minutes
        let five_min_ago = now - Duration::from_secs(300);
        while let Some(&front) = usage.raw_request_timestamps.front() {
            if front < five_min_ago {
                usage.raw_request_timestamps.pop_front();
            } else {
                break;
            }
        }
    }

    /// Call this after every order-related REST call to increment order counters
    pub async fn increment_order(&self) {
        let mut usage = self.usage.write().await;
        let now = Instant::now();
        usage.order_timestamps_10s.push_back(now);
        usage.order_timestamps_1m.push_back(now);
        usage.order_timestamps_1d.push_back(now);
        // Remove timestamps older than 10s, 1m, 1d
        let ten_sec_ago = now - Duration::from_secs(10);
        let one_min_ago = now - Duration::from_secs(60);
        let one_day_ago = now - Duration::from_secs(86400);
        while let Some(&front) = usage.order_timestamps_10s.front() {
            if front < ten_sec_ago {
                usage.order_timestamps_10s.pop_front();
            } else {
                break;
            }
        }
        while let Some(&front) = usage.order_timestamps_1m.front() {
            if front < one_min_ago {
                usage.order_timestamps_1m.pop_front();
            } else {
                break;
            }
        }
        while let Some(&front) = usage.order_timestamps_1d.front() {
            if front < one_day_ago {
                usage.order_timestamps_1d.pop_front();
            } else {
                break;
            }
        }
    }

    /// Call this after every response to update counters from headers (authoritative)
    pub async fn update_from_headers(&self, headers: &super::types::BinanceHeaders) {
        let mut usage = self.usage.write().await;
        if let Some(weight) = headers.used_weight_1m {
            usage.used_weight_1m = weight;
        }
        // Orders are tracked by rolling window, but you could optionally sync to header counts if needed
    }

    /// Checks if a new request can be made without exceeding any bucket
    /// - weight: request weight for this endpoint
    /// - is_order: whether this is an order-related endpoint
    pub async fn check_limits(&self, weight: u32, is_order: bool) -> Result<(), BinanceCoinMError> {
        let usage = self.usage.read().await;
        // Raw requests: 61,000 per 5 min
        if usage.raw_request_timestamps.len() >= 61000 {
            return Err(BinanceCoinMError::ApiError(
                BinanceCoinMAPIError::TooManyRequests {
                    msg: "Raw request cap (61,000/5min) exceeded".to_string(),
                },
            ));
        }
        // Request weight: 6,000 per 1 min
        if usage.used_weight_1m + weight > 6000 {
            return Err(BinanceCoinMError::ApiError(
                BinanceCoinMAPIError::TooManyRequests {
                    msg: format!(
                        "Request weight {} would exceed limit of 6,000",
                        usage.used_weight_1m + weight
                    ),
                },
            ));
        }
        // Orders: 100 per 10s, 1,200 per 1m
        if is_order {
            if usage.order_timestamps_10s.len() >= 100 {
                return Err(BinanceCoinMError::ApiError(
                    BinanceCoinMAPIError::TooManyOrders {
                        msg: "Order cap (100/10s) exceeded".to_string(),
                    },
                ));
            }
            if usage.order_timestamps_1m.len() >= 1200 {
                return Err(BinanceCoinMError::ApiError(
                    BinanceCoinMAPIError::TooManyOrders {
                        msg: "Order cap (1,200/1min) exceeded".to_string(),
                    },
                ));
            }
        }
        Ok(())
    }
}

impl Default for BinanceCoinMRateLimiter {
    fn default() -> Self {
        Self::default()
    }
}
