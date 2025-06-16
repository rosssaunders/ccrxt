use std::collections::HashMap;
use std::time::{Duration, Instant};

use thiserror::Error;
use tokio::sync::RwLock;

/// Types of endpoints for rate limiting based on Coinbase documentation
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EndpointType {
    /// Public endpoints: 10 requests/second per IP (bursts up to 15)
    Public,
    /// Private endpoints: 15 requests/second per profile (bursts up to 30)
    Private,
    /// Private /fills endpoint: 10 requests/second per profile (bursts up to 20)
    PrivateFills,
    /// Private /loans endpoint: 10 requests/second per profile
    PrivateLoans,
}

/// Rate limit configuration for different endpoint types
#[derive(Debug, Clone)]
pub struct RateLimit {
    /// Maximum requests per second
    pub max_requests_per_second: u32,
    /// Maximum burst size
    pub max_burst: u32,
    /// Time window duration
    pub window: Duration,
}

impl RateLimit {
    /// Create a new rate limit configuration
    pub fn new(max_requests_per_second: u32, max_burst: u32, window: Duration) -> Self {
        Self {
            max_requests_per_second,
            max_burst,
            window,
        }
    }
}

/// Rate limiting errors
#[derive(Error, Debug)]
pub enum RateLimitError {
    #[error("Rate limit exceeded for endpoint type: {endpoint_type:?}")]
    Exceeded { endpoint_type: EndpointType },
}

/// Request tracking for rate limiting
#[derive(Debug)]
struct RequestTracker {
    /// Timestamps of recent requests
    request_times: Vec<Instant>,
    /// Last cleanup time
    last_cleanup: Instant,
}

impl RequestTracker {
    fn new() -> Self {
        Self {
            request_times: Vec::new(),
            last_cleanup: Instant::now(),
        }
    }

    /// Clean up old request timestamps beyond the window
    fn cleanup(&mut self, window: Duration) {
        let now = Instant::now();
        let cutoff = now - window;

        self.request_times.retain(|&time| time > cutoff);
        self.last_cleanup = now;
    }

    /// Check if we can make a new request without exceeding limits
    fn can_make_request(&mut self, rate_limit: &RateLimit) -> bool {
        let now = Instant::now();

        // Cleanup old requests if needed (every 10 seconds)
        if now.duration_since(self.last_cleanup) > Duration::from_secs(10) {
            self.cleanup(rate_limit.window);
        }

        // Check burst limit
        if self.request_times.len() >= rate_limit.max_burst as usize {
            return false;
        }

        // Check rate limit (requests per second)
        let one_second_ago = now - Duration::from_secs(1);
        let recent_requests = self
            .request_times
            .iter()
            .filter(|&&time| time > one_second_ago)
            .count();

        recent_requests < rate_limit.max_requests_per_second as usize
    }

    /// Record a new request
    fn record_request(&mut self) {
        self.request_times.push(Instant::now());
    }
}

/// Rate limiter for Coinbase Exchange API endpoints
#[derive(Debug)]
pub struct RateLimiter {
    /// Rate limits for different endpoint types
    rate_limits: HashMap<EndpointType, RateLimit>,
    /// Request trackers for each endpoint type
    trackers: RwLock<HashMap<EndpointType, RequestTracker>>,
}

impl Clone for RateLimiter {
    fn clone(&self) -> Self {
        Self::new()
    }
}

impl RateLimiter {
    /// Create a new rate limiter with Coinbase-specific limits
    pub fn new() -> Self {
        let mut rate_limits = HashMap::new();

        // Configure rate limits based on Coinbase documentation
        rate_limits.insert(
            EndpointType::Public,
            RateLimit::new(10, 15, Duration::from_secs(60)),
        );
        rate_limits.insert(
            EndpointType::Private,
            RateLimit::new(15, 30, Duration::from_secs(60)),
        );
        rate_limits.insert(
            EndpointType::PrivateFills,
            RateLimit::new(10, 20, Duration::from_secs(60)),
        );
        rate_limits.insert(
            EndpointType::PrivateLoans,
            RateLimit::new(10, 10, Duration::from_secs(60)),
        );

        Self {
            rate_limits,
            trackers: RwLock::new(HashMap::new()),
        }
    }

    /// Check if a request can be made for the given endpoint type
    pub async fn check_limit(&self, endpoint_type: EndpointType) -> Result<(), RateLimitError> {
        let rate_limit = self
            .rate_limits
            .get(&endpoint_type)
            .ok_or_else(|| RateLimitError::Exceeded {
                endpoint_type: endpoint_type.clone(),
            })?;

        let mut trackers = self.trackers.write().await;
        let tracker = trackers
            .entry(endpoint_type.clone())
            .or_insert_with(RequestTracker::new);

        if !tracker.can_make_request(rate_limit) {
            return Err(RateLimitError::Exceeded { endpoint_type });
        }

        tracker.record_request();
        Ok(())
    }

    /// Wait until a request can be made for the given endpoint type
    pub async fn wait_for_capacity(&self, endpoint_type: EndpointType) {
        loop {
            if self.check_limit(endpoint_type.clone()).await.is_ok() {
                break;
            }
            // Wait a short time before checking again
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }
}

impl Default for RateLimiter {
    fn default() -> Self {
        Self::new()
    }
}
