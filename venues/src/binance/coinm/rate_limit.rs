use std::collections::VecDeque;
use std::num::NonZeroU32;
use std::sync::Arc;
use std::time::{Duration, Instant};
use serde::Deserialize;
use tokio::sync::RwLock;

use crate::binance::coinm::errors::ApiError;
use crate::binance::coinm::{ResponseHeaders, Errors};

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

#[derive(Copy, PartialEq, Eq, Hash)]
/// Represents the interval unit for Binance rate limit headers (e.g., '1m', '1h').
pub enum IntervalUnit {
    Second,
    Minute,
    Hour,
    Day,
}

impl std::fmt::Debug for IntervalUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IntervalUnit::Second => write!(f, "Second"),
            IntervalUnit::Minute => write!(f, "Minute"),
            IntervalUnit::Hour => write!(f, "Hour"),
            IntervalUnit::Day => write!(f, "Day"),
        }
    }
}

impl Clone for IntervalUnit {
    fn clone(&self) -> Self {
        *self
    }
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
        let header = header.to_ascii_lowercase();
        if let Some(rest) = header.strip_prefix("x-mbx-used-weight-") {
            if let Some((num, unit)) = rest.split_at(rest.len().saturating_sub(1)).into() {
                if let (Ok(interval_value), Some(interval_unit)) = (num.parse::<u32>(), IntervalUnit::from_char(unit.chars().next()?)) {
                    return Some(RateLimitHeader {
                        kind: RateLimitHeaderKind::UsedWeight,
                        interval_value,
                        interval_unit,
                    });
                }
            }
        } else if let Some(rest) = header.strip_prefix("x-mbx-order-count-") {
            if let Some((num, unit)) = rest.split_at(rest.len().saturating_sub(1)).into() {
                if let (Ok(interval_value), Some(interval_unit)) = (num.parse::<u32>(), IntervalUnit::from_char(unit.chars().next()?)) {
                    return Some(RateLimitHeader {
                        kind: RateLimitHeaderKind::OrderCount,
                        interval_value,
                        interval_unit,
                    });
                }
            }
        }
        None
    }

    /// Convert the RateLimitHeader back to its string representation (e.g., "x-mbx-used-weight-1m").
    pub fn to_string(&self) -> String {
        let prefix = match self.kind {
            RateLimitHeaderKind::UsedWeight => "x-mbx-used-weight-",
            RateLimitHeaderKind::OrderCount => "x-mbx-order-count-",
        };
        format!("{}{}{}", prefix, self.interval_value, self.interval_unit.as_str())
    }
}

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
pub struct RateLimiter {
    usage: Arc<RwLock<RateLimitUsage>>,
}

impl RateLimiter {
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
    pub async fn update_from_headers(&self, headers: &ResponseHeaders) {
        let mut usage = self.usage.write().await;
        // Use a strongly-typed RateLimitHeader as the key
        use crate::binance::coinm::rate_limit::{RateLimitHeader, RateLimitHeaderKind, IntervalUnit};
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

    /// Checks if a new request can be made without exceeding any bucket
    /// - weight: request weight for this endpoint
    /// - is_order: whether this is an order-related endpoint
    pub async fn check_limits(&self, weight: u32, is_order: bool) -> Result<(), Errors> {
        let usage = self.usage.read().await;
        // Raw requests: 61,000 per 5 min
        if usage.raw_request_timestamps.len() >= 61000 {
            return Err(Errors::ApiError(
                ApiError::TooManyRequests {
                    msg: "Raw request cap (61,000/5min) exceeded".to_string(),
                },
            ));
        }
        // Request weight: 6,000 per 1 min
        if usage.used_weight_1m + weight > 6000 {
            return Err(Errors::ApiError(
                ApiError::TooManyRequests {
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
                return Err(Errors::ApiError(
                    ApiError::TooManyOrders {
                        msg: "Order cap (100/10s) exceeded".to_string(),
                    },
                ));
            }
            if usage.order_timestamps_1m.len() >= 1200 {
                return Err(Errors::ApiError(
                    ApiError::TooManyOrders {
                        msg: "Order cap (1,200/1min) exceeded".to_string(),
                    },
                ));
            }
        }
        Ok(())
    }
}

impl Default for RateLimiter {
    fn default() -> Self {
        Self::default()
    }
}
