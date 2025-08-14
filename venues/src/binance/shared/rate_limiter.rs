use std::{
    collections::{HashMap, VecDeque},
    sync::Arc,
};

use crate::time_compat::{Duration, Instant};

use serde::Deserialize;
use tokio::sync::RwLock;

use super::{errors::Errors, venue_trait::RateLimits};

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
pub enum IntervalUnit {
    Second,
    Minute,
    Hour,
    Day,
}

impl IntervalUnit {
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            's' | 'S' => Some(IntervalUnit::Second),
            'm' | 'M' => Some(IntervalUnit::Minute),
            'h' | 'H' => Some(IntervalUnit::Hour),
            'd' | 'D' => Some(IntervalUnit::Day),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            IntervalUnit::Second => "s",
            IntervalUnit::Minute => "m",
            IntervalUnit::Hour => "h",
            IntervalUnit::Day => "d",
        }
    }

    pub fn to_duration(&self, value: u32) -> Duration {
        match self {
            IntervalUnit::Second => Duration::from_secs(value as u64),
            IntervalUnit::Minute => Duration::from_secs(value.saturating_mul(60) as u64),
            IntervalUnit::Hour => Duration::from_secs(value.saturating_mul(3600) as u64),
            IntervalUnit::Day => Duration::from_secs(value.saturating_mul(86400) as u64),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RateLimitHeader {
    pub kind: RateLimitHeaderKind,
    pub interval_value: u32,
    pub interval_unit: IntervalUnit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RateLimitHeaderKind {
    UsedWeight,
    OrderCount,
}

impl RateLimitHeader {
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

        if rest.is_empty() {
            return None;
        }

        let unit_char = rest.chars().last()?;
        let interval_unit = IntervalUnit::from_char(unit_char)?;

        let interval_str = &rest[..rest.len().saturating_sub(1)];
        let interval_value: u32 = interval_str.parse().ok()?;

        Some(RateLimitHeader {
            kind,
            interval_value,
            interval_unit,
        })
    }
}

/// Thread-safe rate limiter usage tracking
#[derive(Debug, Default)]
pub struct RateLimitUsage {
    /// Request weight usage over time windows
    pub weight_usage: HashMap<(u32, IntervalUnit), VecDeque<(Instant, u32)>>,
    /// Raw request usage over time windows
    pub raw_usage: HashMap<(u32, IntervalUnit), VecDeque<Instant>>,
    /// Order usage over time windows
    pub order_usage: HashMap<(u32, IntervalUnit), VecDeque<Instant>>,
}

impl RateLimitUsage {
    /// Clean old entries that are outside the time window
    fn clean_old_entries(&mut self, now: Instant) {
        // Clean weight usage
        for ((interval_value, interval_unit), entries) in self.weight_usage.iter_mut() {
            let window_duration = interval_unit.to_duration(*interval_value);
            while let Some((timestamp, _)) = entries.front() {
                if now.duration_since(*timestamp) > window_duration {
                    entries.pop_front();
                } else {
                    break;
                }
            }
        }

        // Clean raw usage
        for ((interval_value, interval_unit), entries) in self.raw_usage.iter_mut() {
            let window_duration = interval_unit.to_duration(*interval_value);
            while let Some(timestamp) = entries.front() {
                if now.duration_since(*timestamp) > window_duration {
                    entries.pop_front();
                } else {
                    break;
                }
            }
        }

        // Clean order usage
        for ((interval_value, interval_unit), entries) in self.order_usage.iter_mut() {
            let window_duration = interval_unit.to_duration(*interval_value);
            while let Some(timestamp) = entries.front() {
                if now.duration_since(*timestamp) > window_duration {
                    entries.pop_front();
                } else {
                    break;
                }
            }
        }
    }

    /// Add weight usage
    pub fn add_weight_usage(
        &mut self,
        weight: u32,
        interval_value: u32,
        interval_unit: IntervalUnit,
    ) {
        let now = Instant::now();
        self.clean_old_entries(now);

        let entries = self
            .weight_usage
            .entry((interval_value, interval_unit))
            .or_default();
        entries.push_back((now, weight));
    }

    /// Add raw request usage
    pub fn add_raw_usage(&mut self, interval_value: u32, interval_unit: IntervalUnit) {
        let now = Instant::now();
        self.clean_old_entries(now);

        let entries = self
            .raw_usage
            .entry((interval_value, interval_unit))
            .or_default();
        entries.push_back(now);
    }

    /// Add order usage
    pub fn add_order_usage(&mut self, interval_value: u32, interval_unit: IntervalUnit) {
        let now = Instant::now();
        self.clean_old_entries(now);

        let entries = self
            .order_usage
            .entry((interval_value, interval_unit))
            .or_default();
        entries.push_back(now);
    }

    /// Get current weight usage for a time window
    pub fn get_weight_usage(&mut self, interval_value: u32, interval_unit: IntervalUnit) -> u32 {
        let now = Instant::now();
        self.clean_old_entries(now);

        self.weight_usage
            .get(&(interval_value, interval_unit))
            .map(|entries| entries.iter().map(|(_, weight)| weight).sum())
            .unwrap_or(0)
    }

    /// Get current raw request usage for a time window
    pub fn get_raw_usage(&mut self, interval_value: u32, interval_unit: IntervalUnit) -> u32 {
        let now = Instant::now();
        self.clean_old_entries(now);

        self.raw_usage
            .get(&(interval_value, interval_unit))
            .map(|entries| entries.len() as u32)
            .unwrap_or(0)
    }

    /// Get current order usage for a time window
    pub fn get_order_usage(&mut self, interval_value: u32, interval_unit: IntervalUnit) -> u32 {
        let now = Instant::now();
        self.clean_old_entries(now);

        self.order_usage
            .get(&(interval_value, interval_unit))
            .map(|entries| entries.len() as u32)
            .unwrap_or(0)
    }
}

/// Unified rate limiter for all Binance venues
#[derive(Debug)]
pub struct RateLimiter {
    pub limits: RateLimits,
    pub usage: Arc<RwLock<RateLimitUsage>>,
}

impl RateLimiter {
    pub fn new(limits: RateLimits) -> Self {
        Self {
            limits,
            usage: Arc::new(RwLock::new(RateLimitUsage::default())),
        }
    }

    /// Check if a request with given weight and order flag would exceed rate limits
    pub async fn check_limits(&self, weight: u32, is_order: bool) -> Result<(), Errors> {
        let mut usage = self.usage.write().await;

        // Convert time windows to interval units for checking
        let weight_window_minutes = self.limits.request_weight_window.as_secs() / 60;
        let raw_window_seconds = self.limits.raw_requests_window.as_secs();

        // Check request weight limit
        let current_weight =
            usage.get_weight_usage(weight_window_minutes as u32, IntervalUnit::Minute);
        if current_weight.saturating_add(weight) > self.limits.request_weight_limit {
            return Err(Errors::RateLimitExceeded {
                retry_after: Some(self.limits.request_weight_window),
            });
        }

        // Check raw requests limit
        let current_raw = if raw_window_seconds >= 300 {
            // 5+ minute window
            usage.get_raw_usage((raw_window_seconds / 60) as u32, IntervalUnit::Minute)
        } else {
            // Minute or less window
            usage.get_raw_usage(raw_window_seconds as u32, IntervalUnit::Second)
        };

        if current_raw.saturating_add(1) > self.limits.raw_requests_limit {
            return Err(Errors::RateLimitExceeded {
                retry_after: Some(self.limits.raw_requests_window),
            });
        }

        // Check order limits if this is an order
        if is_order {
            let current_orders_10s = usage.get_order_usage(10, IntervalUnit::Second);
            if current_orders_10s.saturating_add(1) > self.limits.orders_10s_limit {
                return Err(Errors::RateLimitExceeded {
                    retry_after: Some(Duration::from_secs(10)),
                });
            }

            let current_orders_1m = usage.get_order_usage(1, IntervalUnit::Minute);
            if current_orders_1m.saturating_add(1) > self.limits.orders_minute_limit {
                return Err(Errors::RateLimitExceeded {
                    retry_after: Some(Duration::from_secs(60)),
                });
            }

            if let Some(daily_limit) = self.limits.orders_day_limit {
                let current_orders_1d = usage.get_order_usage(1, IntervalUnit::Day);
                if current_orders_1d.saturating_add(1) > daily_limit {
                    return Err(Errors::RateLimitExceeded {
                        retry_after: Some(Duration::from_secs(86400)),
                    });
                }
            }
        }

        Ok(())
    }

    /// Record usage after a successful request
    pub async fn record_usage(&self, weight: u32, is_order: bool) {
        let mut usage = self.usage.write().await;

        let weight_window_minutes = self.limits.request_weight_window.as_secs() / 60;
        let raw_window_seconds = self.limits.raw_requests_window.as_secs();

        // Record weight usage
        usage.add_weight_usage(weight, weight_window_minutes as u32, IntervalUnit::Minute);

        // Record raw request usage
        if raw_window_seconds >= 300 {
            usage.add_raw_usage((raw_window_seconds / 60) as u32, IntervalUnit::Minute);
        } else {
            usage.add_raw_usage(raw_window_seconds as u32, IntervalUnit::Second);
        }

        // Record order usage if applicable
        if is_order {
            usage.add_order_usage(10, IntervalUnit::Second);
            usage.add_order_usage(1, IntervalUnit::Minute);
            if self.limits.orders_day_limit.is_some() {
                usage.add_order_usage(1, IntervalUnit::Day);
            }
        }
    }

    /// Update usage from response headers
    pub async fn update_from_headers(&self, headers: &HashMap<String, String>) {
        let mut usage = self.usage.write().await;

        for (header_name, header_value) in headers {
            if let Some(rate_limit_header) = RateLimitHeader::parse(header_name) {
                if let Ok(current_usage) = header_value.parse::<u32>() {
                    match rate_limit_header.kind {
                        RateLimitHeaderKind::UsedWeight => {
                            // Update our tracking to match server's view
                            let key = (
                                rate_limit_header.interval_value,
                                rate_limit_header.interval_unit,
                            );
                            let entries = usage.weight_usage.entry(key).or_default();
                            entries.clear();
                            if current_usage > 0 {
                                entries.push_back((Instant::now(), current_usage));
                            }
                        }
                        RateLimitHeaderKind::OrderCount => {
                            // Update order count tracking
                            let key = (
                                rate_limit_header.interval_value,
                                rate_limit_header.interval_unit,
                            );
                            let entries = usage.order_usage.entry(key).or_default();
                            entries.clear();
                            for _ in 0..current_usage {
                                entries.push_back(Instant::now());
                            }
                        }
                    }
                }
            }
        }
    }

    /// Get current usage statistics
    pub async fn get_usage_stats(&self) -> UsageStats {
        let mut usage = self.usage.write().await;

        let weight_window_minutes = self.limits.request_weight_window.as_secs() / 60;
        let raw_window_seconds = self.limits.raw_requests_window.as_secs();

        let current_weight =
            usage.get_weight_usage(weight_window_minutes as u32, IntervalUnit::Minute);

        let current_raw = if raw_window_seconds >= 300 {
            usage.get_raw_usage((raw_window_seconds / 60) as u32, IntervalUnit::Minute)
        } else {
            usage.get_raw_usage(raw_window_seconds as u32, IntervalUnit::Second)
        };

        let current_orders_10s = usage.get_order_usage(10, IntervalUnit::Second);
        let current_orders_1m = usage.get_order_usage(1, IntervalUnit::Minute);
        let current_orders_1d = usage.get_order_usage(1, IntervalUnit::Day);

        UsageStats {
            weight_used: current_weight,
            weight_limit: self.limits.request_weight_limit,
            raw_requests_used: current_raw,
            raw_requests_limit: self.limits.raw_requests_limit,
            orders_10s_used: current_orders_10s,
            orders_10s_limit: self.limits.orders_10s_limit,
            orders_1m_used: current_orders_1m,
            orders_1m_limit: self.limits.orders_minute_limit,
            orders_1d_used: current_orders_1d,
            orders_1d_limit: self.limits.orders_day_limit,
        }
    }
}

/// Current usage statistics
#[derive(Debug, Clone)]
pub struct UsageStats {
    pub weight_used: u32,
    pub weight_limit: u32,
    pub raw_requests_used: u32,
    pub raw_requests_limit: u32,
    pub orders_10s_used: u32,
    pub orders_10s_limit: u32,
    pub orders_1m_used: u32,
    pub orders_1m_limit: u32,
    pub orders_1d_used: u32,
    pub orders_1d_limit: Option<u32>,
}
