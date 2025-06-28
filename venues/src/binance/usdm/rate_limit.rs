use std::collections::VecDeque;
use std::sync::Arc;
use std::time::{Duration, Instant};

use serde::Deserialize;
use tokio::sync::RwLock;

use crate::binance::usdm::errors::ApiError;
use crate::binance::usdm::{Errors, ResponseHeaders};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RateLimitType {
    RequestWeight,
    Orders,
    RawRequests,
    RequestSize,
    OrdersPerSecond,
    OrdersPerDay,
    OrdersPerCountdown,
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

/// Tracks the current usage of rate limits (rolling windows)
#[derive(Debug, Default, Clone)]
pub struct RateLimitUsage {
    /// Timestamps of raw requests in the last 1 minute (for USDM)
    pub raw_request_timestamps: VecDeque<Instant>,

    /// Timestamps of order requests in the last 10s, 1m, 1d
    pub order_timestamps_10s: VecDeque<Instant>,

    pub order_timestamps_1m: VecDeque<Instant>,

    pub order_timestamps_1d: VecDeque<Instant>,

    /// Last known request weight (from header)
    pub used_weight_1m: u32,
}

/// Manages rate limiting for Binance USD-M Futures API
///
/// This rate limiter implements the rate limiting requirements for Binance USD-M Futures (USDM) API
/// as described in the official documentation. It tracks three types of limits:
///
/// 1. **Raw Requests**: Total number of API calls regardless of weight (1,200 per minute)
/// 2. **Request Weight**: Weighted requests based on endpoint complexity (2,400 per minute)  
/// 3. **Orders**: Order-related operations (100 per 10s, 1,200 per minute)
///
/// # Usage
///
/// ```rust
/// use venues::binance::usdm::RateLimiter;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let rate_limiter = RateLimiter::new();
///     
///     // Check if we can make a request with weight 10
///     rate_limiter.check_limits(10, false).await?;
///     
///     // Increment counters after making the request
///     rate_limiter.increment_raw_request().await;
///     
///     // For order endpoints, also increment order counter
///     let is_order_endpoint = true; // This would be determined by your endpoint logic
///     if is_order_endpoint {
///         rate_limiter.increment_order().await;
///     }
///     
///     Ok(())
/// }
/// ```
///
/// # Rate Limits (USDM)
///
/// - **Raw Requests**: 1,200 requests per minute
/// - **Request Weight**: 2,400 weight units per minute  
/// - **Orders**: 100 orders per 10 seconds, 1,200 orders per minute
///
/// These limits are based on IP address and are enforced by Binance. When limits are exceeded,
/// the API returns HTTP 429 (Too Many Requests) or HTTP 418 (IP banned for repeated violations).
///
/// # Headers
///
/// The rate limiter automatically parses response headers:
/// - `X-MBX-USED-WEIGHT-1M`: Current weight usage in the last minute
/// - `X-MBX-ORDER-COUNT-10S`: Order count in the last 10 seconds
/// - `X-MBX-ORDER-COUNT-1M`: Order count in the last minute
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

    /// Call this after every REST call to increment raw_request (1min window, 1,200 cap for USDM)
    #[allow(clippy::arithmetic_side_effects)]
    pub async fn increment_raw_request(&self) {
        let mut usage = self.usage.write().await;
        let now = Instant::now();
        usage.raw_request_timestamps.push_back(now);
        // Remove timestamps older than 1 minute for USDM
        Self::trim_older_than(
            &mut usage.raw_request_timestamps,
            now - Duration::from_secs(60),
        );
    }

    /// Call this after every order-related REST call to increment order counters
    #[allow(clippy::arithmetic_side_effects)]
    pub async fn increment_order(&self) {
        let mut usage = self.usage.write().await;
        let now = Instant::now();
        usage.order_timestamps_10s.push_back(now);
        usage.order_timestamps_1m.push_back(now);
        usage.order_timestamps_1d.push_back(now);
        // Remove timestamps older than 10s, 1m, 1d
        Self::trim_older_than(
            &mut usage.order_timestamps_10s,
            now - Duration::from_secs(10),
        );
        Self::trim_older_than(
            &mut usage.order_timestamps_1m,
            now - Duration::from_secs(60),
        );
        Self::trim_older_than(
            &mut usage.order_timestamps_1d,
            now - Duration::from_secs(86400),
        );
    }

    /// Call this after every response to update counters from headers (authoritative)
    pub async fn update_from_headers(&self, headers: &ResponseHeaders) {
        let mut usage = self.usage.write().await;
        // Use a strongly-typed RateLimitHeader as the key
        use crate::binance::usdm::rate_limit::{
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

    /// Convenience method for order endpoints: check limits and increment if successful
    pub async fn acquire_order(&self) -> Result<(), Errors> {
        self.check_limits(1, true).await?;
        self.increment_order().await;
        Ok(())
    }

    /// Convenience method for regular endpoints: check limits and increment if successful
    pub async fn acquire_request(&self, weight: u32) -> Result<(), Errors> {
        self.check_limits(weight, false).await?;
        self.increment_raw_request().await;
        Ok(())
    }

    /// Checks if a new request can be made without exceeding any bucket
    /// - weight: request weight for this endpoint
    /// - is_order: whether this is an order-related endpoint
    #[allow(clippy::arithmetic_side_effects)]
    pub async fn check_limits(&self, weight: u32, is_order: bool) -> Result<(), Errors> {
        let usage = self.usage.read().await;

        // Raw requests: 1,200 per 1 min for USDM (different from COINM)
        if usage.raw_request_timestamps.len() >= 1200 {
            return Err(Errors::ApiError(ApiError::TooManyRequests {
                msg: "Raw request cap (1,200/1min) exceeded".to_string(),
            }));
        }

        // Request weight: 2,400 per 1 min for USDM (different from COINM)
        if usage.used_weight_1m + weight > 2400 {
            return Err(Errors::ApiError(ApiError::TooManyRequests {
                msg: format!(
                    "Request weight {} would exceed limit of 2,400",
                    usage.used_weight_1m + weight
                ),
            }));
        }

        // Orders: 100 per 10s, 1,200 per 1m (same as COINM)
        if is_order {
            if usage.order_timestamps_10s.len() >= 100 {
                return Err(Errors::ApiError(ApiError::TooManyOrders {
                    msg: "Order cap (100/10s) exceeded".to_string(),
                }));
            }
            if usage.order_timestamps_1m.len() >= 1200 {
                return Err(Errors::ApiError(ApiError::TooManyOrders {
                    msg: "Order cap (1,200/1min) exceeded".to_string(),
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
    async fn test_rate_limiter_raw_requests() {
        let rate_limiter = RateLimiter::new();

        // Test that we can make requests up to the limit
        for _ in 0..10 {
            rate_limiter.increment_raw_request().await;
            assert!(rate_limiter.check_limits(1, false).await.is_ok());
        }
    }

    #[tokio::test]
    async fn test_rate_limiter_request_weight() {
        let rate_limiter = RateLimiter::new();

        // Test request weight limits
        assert!(rate_limiter.check_limits(1000, false).await.is_ok());
        assert!(rate_limiter.check_limits(2400, false).await.is_ok());

        // This should fail as it exceeds the 2400 limit
        assert!(rate_limiter.check_limits(2401, false).await.is_err());
    }

    #[tokio::test]
    async fn test_rate_limiter_orders() {
        let rate_limiter = RateLimiter::new();

        // Test order limits - should be able to place some orders
        for _ in 0..10 {
            rate_limiter.increment_order().await;
            assert!(rate_limiter.check_limits(1, true).await.is_ok());
        }
    }

    #[tokio::test]
    async fn test_rate_limit_header_parsing() {
        let header = RateLimitHeader::parse("x-mbx-used-weight-1m");
        assert!(header.is_some());
        let header = header.unwrap();
        assert_eq!(header.kind, RateLimitHeaderKind::UsedWeight);
        assert_eq!(header.interval_value, 1);
        assert_eq!(header.interval_unit, IntervalUnit::Minute);

        let header = RateLimitHeader::parse("x-mbx-order-count-10s");
        assert!(header.is_some());
        let header = header.unwrap();
        assert_eq!(header.kind, RateLimitHeaderKind::OrderCount);
        assert_eq!(header.interval_value, 10);
        assert_eq!(header.interval_unit, IntervalUnit::Second);

        // Test invalid headers
        assert!(RateLimitHeader::parse("invalid-header").is_none());
        assert!(RateLimitHeader::parse("x-mbx-invalid-1m").is_none());
    }
}
