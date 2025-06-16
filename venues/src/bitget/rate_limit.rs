use std::collections::VecDeque;
use std::sync::Arc;
use std::time::{Duration, Instant};

use serde::Deserialize;
use tokio::sync::RwLock;

use crate::bitget::errors::ApiError;
use crate::bitget::{Errors, ResponseHeaders};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RateLimitType {
    RequestFrequency,
    Orders,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RateLimitInterval {
    Second,
    Minute,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
/// Represents the interval unit for Bitget rate limit tracking (e.g., 's', 'm').
pub enum IntervalUnit {
    Second,
    Minute,
}

impl IntervalUnit {
    /// Parse a single character unit (e.g., 's', 'm') into IntervalUnit.
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            's' | 'S' => Some(IntervalUnit::Second),
            'm' | 'M' => Some(IntervalUnit::Minute),
            _ => None,
        }
    }

    /// Convert IntervalUnit to its string representation (e.g., 'm').
    pub fn as_str(&self) -> &'static str {
        match self {
            IntervalUnit::Second => "s",
            IntervalUnit::Minute => "m",
        }
    }
}

/// Represents a parsed Bitget rate limit header (used for future header tracking).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RateLimitHeader {
    /// The kind of header (request frequency, order count, etc.).
    pub kind: RateLimitHeaderKind,

    /// The interval value (e.g., 1, 5, 60).
    pub interval_value: u32,

    /// The interval unit (e.g., Second, Minute).
    pub interval_unit: IntervalUnit,
}

/// The type of rate limit header.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RateLimitHeaderKind {
    RequestFrequency,
    OrderCount,
}

impl RateLimitHeader {
    /// Create a new rate limit header for request frequency tracking
    pub fn request_frequency_1s() -> Self {
        Self {
            kind: RateLimitHeaderKind::RequestFrequency,
            interval_value: 1,
            interval_unit: IntervalUnit::Second,
        }
    }

    /// Create a new rate limit header for overall IP limit tracking
    pub fn request_frequency_1m() -> Self {
        Self {
            kind: RateLimitHeaderKind::RequestFrequency,
            interval_value: 1,
            interval_unit: IntervalUnit::Minute,
        }
    }

    /// Create a new rate limit header for order count tracking
    pub fn order_count_1s() -> Self {
        Self {
            kind: RateLimitHeaderKind::OrderCount,
            interval_value: 1,
            interval_unit: IntervalUnit::Second,
        }
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
            RateLimitHeaderKind::RequestFrequency => "bitget-frequency-",
            RateLimitHeaderKind::OrderCount => "bitget-order-count-",
        };
        write!(f, "{}{}{}", prefix, self.interval_value, self.interval_unit)
    }
}

/// Tracks the current usage of rate limits for Bitget API (rolling windows)
#[derive(Debug, Default, Clone)]
pub struct RateLimitUsage {
    /// Timestamps of requests in the last second for endpoint-specific limits
    pub request_timestamps_1s: VecDeque<Instant>,

    /// Timestamps of requests in the last minute for overall IP limit (6000/IP/Min)
    pub request_timestamps_1m: VecDeque<Instant>,

    /// Timestamps of order requests for UID-specific limits
    pub order_timestamps_1s: VecDeque<Instant>,
}

/// Manages rate limiting for Bitget API
///
/// Bitget API has the following rate limits:
/// - Overall limit: 6000 requests per IP per minute
/// - Endpoint-specific limits: varies per endpoint (e.g., 3 times/1s, 10 times/1s, 20 times/1s)
/// - Order limits: varies per endpoint and is UID-based
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

    /// Call this after every REST call to increment request counters
    pub async fn increment_request(&self) {
        let mut usage = self.usage.write().await;
        let now = Instant::now();

        // Track for 1-second window (endpoint-specific limits)
        usage.request_timestamps_1s.push_back(now);
        Self::trim_older_than(
            &mut usage.request_timestamps_1s,
            now - Duration::from_secs(1),
        );

        // Track for 1-minute window (overall IP limit)
        usage.request_timestamps_1m.push_back(now);
        Self::trim_older_than(
            &mut usage.request_timestamps_1m,
            now - Duration::from_secs(60),
        );
    }

    /// Call this after every order-related REST call to increment order counters
    pub async fn increment_order(&self) {
        let mut usage = self.usage.write().await;
        let now = Instant::now();
        usage.order_timestamps_1s.push_back(now);
        // Remove timestamps older than 1 second
        Self::trim_older_than(&mut usage.order_timestamps_1s, now - Duration::from_secs(1));
    }

    /// Call this after every response to update counters from headers (for future implementation)
    pub async fn update_from_headers(&self, _headers: &ResponseHeaders) {
        // Bitget doesn't currently expose rate limit usage in headers
        // This is kept for future compatibility if they add such headers
    }

    /// Checks if a new request can be made without exceeding any bucket for Bitget API
    /// - endpoint_limit_per_second: the specific limit for this endpoint (e.g., 3, 10, 20)
    /// - is_order: whether this is an order-related endpoint
    /// - order_limit_per_second: the specific order limit for this endpoint if applicable
    pub async fn check_limits(&self, endpoint_limit_per_second: u32, is_order: bool, order_limit_per_second: Option<u32>) -> Result<(), Errors> {
        let usage = self.usage.read().await;

        // Overall IP limit: 6,000 per minute
        if usage.request_timestamps_1m.len() >= 6000 {
            return Err(Errors::ApiError(ApiError::TooManyRequests {
                msg: "Overall IP rate limit (6,000/min) exceeded".to_string(),
            }));
        }

        // Endpoint-specific limit per second
        if usage.request_timestamps_1s.len() >= endpoint_limit_per_second as usize {
            return Err(Errors::ApiError(ApiError::TooManyRequests {
                msg: format!(
                    "Endpoint rate limit ({}/1s) exceeded",
                    endpoint_limit_per_second
                ),
            }));
        }

        // Order-specific limits (UID-based)
        if is_order {
            if let Some(order_limit) = order_limit_per_second {
                if usage.order_timestamps_1s.len() >= order_limit as usize {
                    return Err(Errors::ApiError(ApiError::TooManyRequests {
                        msg: format!("Order rate limit ({}/1s) exceeded", order_limit),
                    }));
                }
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
        assert!(limiter.check_limits(10, false, None).await.is_ok());
    }

    #[tokio::test]
    async fn test_overall_ip_limit() {
        let limiter = RateLimiter::new();

        // Add requests up to the overall IP limit
        for _ in 0..6000 {
            limiter.increment_request().await;
        }

        // Should reject additional request
        let result = limiter.check_limits(10, false, None).await;
        assert!(result.is_err());

        if let Err(Errors::ApiError(ApiError::TooManyRequests { msg })) = result {
            assert!(msg.contains("6,000/min"));
        } else {
            panic!("Expected TooManyRequests error for overall IP limit");
        }
    }

    #[tokio::test]
    async fn test_endpoint_specific_limit() {
        let limiter = RateLimiter::new();

        // Add requests up to the endpoint limit (e.g., 3/s)
        for _ in 0..3 {
            limiter.increment_request().await;
        }

        // Should reject additional request for this endpoint
        let result = limiter.check_limits(3, false, None).await;
        assert!(result.is_err());

        if let Err(Errors::ApiError(ApiError::TooManyRequests { msg })) = result {
            assert!(msg.contains("3/1s"));
        } else {
            panic!("Expected TooManyRequests error for endpoint limit");
        }
    }

    #[tokio::test]
    async fn test_order_rate_limit() {
        let limiter = RateLimiter::new();

        // Add order requests up to the limit (e.g., 5/s)
        for _ in 0..5 {
            limiter.increment_order().await;
        }

        // Should reject additional order request
        let result = limiter.check_limits(10, true, Some(5)).await;
        assert!(result.is_err());

        if let Err(Errors::ApiError(ApiError::TooManyRequests { msg })) = result {
            assert!(msg.contains("5/1s"));
        } else {
            panic!("Expected TooManyRequests error for order limit");
        }
    }

    #[tokio::test]
    async fn test_rate_limit_header_creation() {
        let freq_1s = RateLimitHeader::request_frequency_1s();
        assert_eq!(freq_1s.kind, RateLimitHeaderKind::RequestFrequency);
        assert_eq!(freq_1s.interval_value, 1);
        assert_eq!(freq_1s.interval_unit, IntervalUnit::Second);

        let freq_1m = RateLimitHeader::request_frequency_1m();
        assert_eq!(freq_1m.kind, RateLimitHeaderKind::RequestFrequency);
        assert_eq!(freq_1m.interval_value, 1);
        assert_eq!(freq_1m.interval_unit, IntervalUnit::Minute);

        let order_1s = RateLimitHeader::order_count_1s();
        assert_eq!(order_1s.kind, RateLimitHeaderKind::OrderCount);
        assert_eq!(order_1s.interval_value, 1);
        assert_eq!(order_1s.interval_unit, IntervalUnit::Second);
    }
}
