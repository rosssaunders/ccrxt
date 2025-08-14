use std::{
    collections::VecDeque,
    sync::Arc,
};

use crate::time_compat::{Duration, Instant};

use serde::Deserialize;
use tokio::sync::RwLock;

use crate::binance::spot::{Errors, ResponseHeaders, errors::ApiError};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RateLimitType {
    RequestWeight,
    Orders,
    RawRequests,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RateLimitInterval {
    Second,
    Minute,
    Day,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
/// Represents the interval unit for Binance rate limit headers (e.g., '1m', '1h').
pub enum IntervalUnit {
    Second,
    Minute,
    Hour,
    Day,
}

impl IntervalUnit {
    /// Parse a single character unit (e.g., 's', 'm', 'h', 'd') into IntervalUnit.
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            's' | 'S' => Some(IntervalUnit::Second),
            'm' | 'M' => Some(IntervalUnit::Minute),
            'h' | 'H' => Some(IntervalUnit::Hour),
            'd' | 'D' => Some(IntervalUnit::Day),
            _ => None,
        }
    }

    /// Convert IntervalUnit to its string representation (e.g., 'm').
    pub fn as_str(&self) -> &'static str {
        match self {
            IntervalUnit::Second => "s",
            IntervalUnit::Minute => "m",
            IntervalUnit::Hour => "h",
            IntervalUnit::Day => "d",
        }
    }
}

/// Represents a parsed Binance rate limit/order count header (e.g., "x-mbx-used-weight-1m").
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RateLimitHeader {
    /// The kind of header (used weight, order count, etc.).
    pub kind: RateLimitHeaderKind,

    /// The interval value (e.g., 1, 5, 60).
    pub interval_value: u32,

    /// The interval unit (e.g., Minute, Hour, Day).
    pub interval_unit: IntervalUnit,
}

/// The type of rate limit header (used weight, order count, etc.).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RateLimitHeaderKind {
    UsedWeight,
    OrderCount,
}

impl RateLimitHeader {
    /// Attempt to parse a Binance rate limit/order count header name into a RateLimitHeader struct.
    /// E.g., "x-mbx-used-weight-1m" or "x-mbx-order-count-1d".
    pub fn parse(header: &str) -> Option<Self> {
        fn ascii_starts_with(haystack: &str, needle: &str) -> bool {
            haystack.len() >= needle.len()
                && haystack
                    .chars()
                    .zip(needle.chars())
                    .all(|(a, b)| a.eq_ignore_ascii_case(&b))
        }
        let (kind, rest) = if ascii_starts_with(header, "x-mbx-used-weight-") {
            (
                RateLimitHeaderKind::UsedWeight,
                &header["x-mbx-used-weight-".len()..],
            )
        } else if ascii_starts_with(header, "x-mbx-order-count-") {
            (
                RateLimitHeaderKind::OrderCount,
                &header["x-mbx-order-count-".len()..],
            )
        } else {
            return None;
        };
        if rest.len() < 2 {
            return None;
        }
        let (num, unit) = rest.split_at(rest.len().saturating_sub(1));
        let interval_value = num.parse::<u32>().ok()?;
        let interval_unit = IntervalUnit::from_char(unit.chars().next()?)?;
        Some(RateLimitHeader {
            kind,
            interval_value,
            interval_unit,
        })
    }
}

impl std::fmt::Display for IntervalUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::fmt::Display for RateLimitHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let prefix = match self.kind {
            RateLimitHeaderKind::UsedWeight => "x-mbx-used-weight-",
            RateLimitHeaderKind::OrderCount => "x-mbx-order-count-",
        };
        write!(f, "{}{}{}", prefix, self.interval_value, self.interval_unit)
    }
}

/// Tracks the current usage of rate limits for Binance Spot API (rolling windows)
#[derive(Debug, Default, Clone)]
pub struct RateLimitUsage {
    /// Timestamps of raw requests in the last 5 minutes
    pub raw_request_timestamps: VecDeque<Instant>,

    /// Timestamps of order requests in the last 10s and 24h for Spot API
    pub order_timestamps_10s: VecDeque<Instant>,
    pub order_timestamps_1d: VecDeque<Instant>,

    /// Last known request weight (from header)
    pub used_weight_1m: u32,
}

/// Manages rate limiting for Binance Spot API
///
/// Binance Spot API has different rate limits compared to COIN-M Futures:
/// - Request Weight: 1200 per minute (vs 6000 for COIN-M)
/// - Raw Requests: 6000 per 5 minutes (vs 61000 for COIN-M)  
/// - Orders: 100 per 10 seconds, 1000 per 24 hours (vs 100/10s, 1200/1m for COIN-M)
#[derive(Debug, Clone, Default)]
pub struct RateLimiter {
    usage: Arc<RwLock<RateLimitUsage>>,
}

impl RateLimiter {
    pub fn new() -> Self {
        Self::default()
    }

    /// Helper to trim timestamps older than a cutoff from a VecDeque
    fn trim_older_than(buf: &mut VecDeque<Instant>, cutoff: Instant) {
        while buf.front().is_some_and(|&ts| ts < cutoff) {
            buf.pop_front();
        }
    }

    /// Call this after every REST call to increment raw_request (5min window, 6,000 cap for Spot)
    pub async fn increment_raw_request(&self) {
        let mut usage = self.usage.write().await;
        let now = Instant::now();
        usage.raw_request_timestamps.push_back(now);
        // Remove timestamps older than 5 minutes
        #[allow(clippy::arithmetic_side_effects)]
        Self::trim_older_than(
            &mut usage.raw_request_timestamps,
            now - Duration::from_secs(300),
        );
    }

    /// Call this after every order-related REST call to increment order counters
    pub async fn increment_order(&self) {
        let mut usage = self.usage.write().await;
        let now = Instant::now();
        usage.order_timestamps_10s.push_back(now);
        usage.order_timestamps_1d.push_back(now);
        // Remove timestamps older than 10s and 24h
        #[allow(clippy::arithmetic_side_effects)]
        Self::trim_older_than(
            &mut usage.order_timestamps_10s,
            now - Duration::from_secs(10),
        );
        #[allow(clippy::arithmetic_side_effects)]
        Self::trim_older_than(
            &mut usage.order_timestamps_1d,
            now - Duration::from_secs(86400),
        );
    }

    /// Call this after every response to update counters from headers (authoritative)
    pub async fn update_from_headers(&self, headers: &ResponseHeaders) {
        let mut usage = self.usage.write().await;
        // Use a strongly-typed RateLimitHeader as the key
        use crate::binance::spot::rate_limit::{
            IntervalUnit, RateLimitHeader, RateLimitHeaderKind,
        };
        let key = RateLimitHeader {
            kind: RateLimitHeaderKind::UsedWeight,
            interval_value: 1,
            interval_unit: IntervalUnit::Minute,
        };
        if let Some(weight) = headers.values.get(&key) {
            usage.used_weight_1m = *weight;
        }
        // Orders are tracked by rolling window, but you could optionally sync to header counts if needed
    }

    /// Checks if a new request can be made without exceeding any bucket for Binance Spot API
    /// - weight: request weight for this endpoint
    /// - is_order: whether this is an order-related endpoint
    pub async fn check_limits(&self, weight: u32, is_order: bool) -> Result<(), Errors> {
        let usage = self.usage.read().await;

        // Raw requests: 6,000 per 5 min (Spot limit)
        if usage.raw_request_timestamps.len() >= 6000 {
            return Err(Errors::ApiError(ApiError::TooManyRequests {
                msg: "Raw request cap (6,000/5min) exceeded".to_string(),
            }));
        }

        // Request weight: 1,200 per 1 min (Spot limit)
        if usage.used_weight_1m.saturating_add(weight) > 1200 {
            return Err(Errors::ApiError(ApiError::TooManyRequests {
                msg: format!(
                    "Request weight {} would exceed limit of 1,200",
                    usage.used_weight_1m.saturating_add(weight)
                ),
            }));
        }

        // Orders: 100 per 10s, 1,000 per 24h (Spot limits)
        if is_order {
            if usage.order_timestamps_10s.len() >= 100 {
                return Err(Errors::ApiError(ApiError::TooManyOrders {
                    msg: "Order cap (100/10s) exceeded".to_string(),
                }));
            }
            if usage.order_timestamps_1d.len() >= 1000 {
                return Err(Errors::ApiError(ApiError::TooManyOrders {
                    msg: "Order cap (1,000/24h) exceeded".to_string(),
                }));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_rate_limiter_new() {
        let limiter = RateLimiter::new();
        // Basic check - should allow initial requests
        assert!(limiter.check_limits(1, false).await.is_ok());
    }

    #[tokio::test]
    async fn test_request_weight_limit() {
        let limiter = RateLimiter::new();

        // Should reject request that would exceed weight limit
        let result = limiter.check_limits(1201, false).await;
        assert!(result.is_err());

        if let Err(Errors::ApiError(ApiError::TooManyRequests { msg })) = result {
            assert!(msg.contains("1,200"));
        } else {
            assert_eq!(true, false, "Expected TooManyRequests error");
        }
    }

    #[tokio::test]
    async fn test_order_rate_limit_10s() {
        let limiter = RateLimiter::new();

        // Add 100 orders (should be at the limit)
        for _ in 0..100 {
            limiter.increment_order().await;
        }

        // Should reject 101st order
        let result = limiter.check_limits(1, true).await;
        assert!(result.is_err());

        if let Err(Errors::ApiError(ApiError::TooManyOrders { msg })) = result {
            assert!(msg.contains("100/10s"));
        } else {
            assert_eq!(true, false, "Expected TooManyOrders error for 10s limit");
        }
    }

    #[tokio::test]
    async fn test_raw_request_limit() {
        let limiter = RateLimiter::new();

        // Add raw requests up to the limit
        for _ in 0..6000 {
            limiter.increment_raw_request().await;
        }

        // Should reject additional raw request
        let result = limiter.check_limits(1, false).await;
        assert!(result.is_err());

        if let Err(Errors::ApiError(ApiError::TooManyRequests { msg })) = result {
            assert!(msg.contains("6,000/5min"));
        } else {
            unreachable!("Expected TooManyRequests error for raw request limit");
        }
    }

    #[tokio::test]
    async fn test_header_parsing() {
        // Test parsing of various header formats
        assert!(RateLimitHeader::parse("x-mbx-used-weight-1m").is_some());
        assert!(RateLimitHeader::parse("x-mbx-order-count-10s").is_some());
        assert!(RateLimitHeader::parse("x-mbx-order-count-1d").is_some());
        assert!(RateLimitHeader::parse("invalid-header").is_none());

        let header = RateLimitHeader::parse("x-mbx-used-weight-1m").unwrap();
        assert_eq!(header.kind, RateLimitHeaderKind::UsedWeight);
        assert_eq!(header.interval_value, 1);
        assert_eq!(header.interval_unit, IntervalUnit::Minute);
    }

    #[tokio::test]
    async fn test_update_from_headers() {
        let limiter = RateLimiter::new();
        let mut headers = ResponseHeaders::default();

        // Create header key for 1-minute weight
        let key = RateLimitHeader {
            kind: RateLimitHeaderKind::UsedWeight,
            interval_value: 1,
            interval_unit: IntervalUnit::Minute,
        };
        headers.values.insert(key, 500);

        limiter.update_from_headers(&headers).await;

        // Check that internal state was updated
        let usage = limiter.usage.read().await;
        assert_eq!(usage.used_weight_1m, 500);
    }
}
