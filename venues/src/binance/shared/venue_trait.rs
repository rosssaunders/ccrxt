use std::time::Duration;

/// Configuration trait for Binance venues
pub trait VenueConfig: Send + Sync {
    /// Base URL for the venue's REST API
    fn base_url(&self) -> &str;

    /// Rate limit configuration for this venue
    fn rate_limits(&self) -> RateLimits;
}

/// Rate limit configuration for a Binance venue
#[derive(Debug, Clone)]
pub struct RateLimits {
    /// Request weight limit per interval
    pub request_weight_limit: u32,

    /// Request weight interval (usually 1 minute)
    pub request_weight_window: Duration,

    /// Raw requests limit per interval
    pub raw_requests_limit: u32,

    /// Raw requests interval (varies by venue)
    pub raw_requests_window: Duration,

    /// Order limit per 10 seconds
    pub orders_10s_limit: u32,

    /// Order limit per minute
    pub orders_minute_limit: u32,

    /// Order limit per day (optional, only for some venues)
    pub orders_day_limit: Option<u32>,
}
