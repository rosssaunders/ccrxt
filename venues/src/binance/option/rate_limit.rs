use std::collections::VecDeque;
use std::sync::Arc;
use std::time::{Duration, Instant};
use serde::Deserialize;
use tokio::sync::RwLock;

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
/// Represents the interval unit for Binance Options rate limit headers (e.g., '1m', '1h').
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
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
/// Represents the kind of rate limiting being tracked.
pub enum RateLimitKind {
    UsedWeight,
    OrderCount,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
/// Strongly-typed descriptor for a rate limit header.
/// This allows efficient parsing and storage of rate limit information.
pub struct RateLimitHeader {
    pub kind: RateLimitKind,
    pub interval_value: u32,
    pub interval_unit: IntervalUnit,
}

/// A basic rate limiter for the Binance Options API
#[derive(Debug, Clone)]
pub struct RateLimiter {
    requests: Arc<RwLock<VecDeque<Instant>>>,
    window: Duration,
    max_requests: u32,
}

impl RateLimiter {
    pub fn new(max_requests: u32, window: Duration) -> Self {
        Self {
            requests: Arc::new(RwLock::new(VecDeque::new())),
            window,
            max_requests,
        }
    }

    /// Check if a request can be made without exceeding rate limits
    pub async fn can_proceed(&self) -> bool {
        let mut requests = self.requests.write().await;
        let now = Instant::now();
        
        // Remove old requests outside the window
        while let Some(&front) = requests.front() {
            if now.duration_since(front) > self.window {
                requests.pop_front();
            } else {
                break;
            }
        }
        
        requests.len() < self.max_requests as usize
    }

    /// Record a new request
    pub async fn record_request(&self) {
        let mut requests = self.requests.write().await;
        requests.push_back(Instant::now());
    }
}