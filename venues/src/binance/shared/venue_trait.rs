use std::time::Duration;

/// Configuration trait for Binance venues
pub trait VenueConfig: Send + Sync {
    /// Base URL for the venue's REST API
    fn base_url(&self) -> &str;

    /// Name of the venue (e.g., "spot", "usdm", "coinm")
    fn venue_name(&self) -> &str;

    /// Rate limit configuration for this venue
    fn rate_limits(&self) -> RateLimits;

    /// Whether this venue supports futures trading features
    fn supports_futures(&self) -> bool;

    /// Whether this venue supports options trading
    fn supports_options(&self) -> bool;

    /// Whether this venue supports margin trading
    fn supports_margin(&self) -> bool;
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

/// Predefined venue configurations
pub mod configs {
    use super::*;

    /// Spot trading venue configuration
    pub struct SpotConfig;

    impl VenueConfig for SpotConfig {
        fn base_url(&self) -> &str {
            "https://api.binance.com"
        }

        fn venue_name(&self) -> &str {
            "spot"
        }

        fn rate_limits(&self) -> RateLimits {
            RateLimits {
                request_weight_limit: 1200,
                request_weight_window: Duration::from_secs(60),
                raw_requests_limit: 6000,
                raw_requests_window: Duration::from_secs(300), // 5 minutes
                orders_10s_limit: 100,
                orders_minute_limit: 1000,
                orders_day_limit: Some(1000),
            }
        }

        fn supports_futures(&self) -> bool {
            false
        }

        fn supports_options(&self) -> bool {
            false
        }

        fn supports_margin(&self) -> bool {
            true
        }
    }

    /// USD-Margined Futures venue configuration
    pub struct UsdmConfig;

    impl VenueConfig for UsdmConfig {
        fn base_url(&self) -> &str {
            "https://fapi.binance.com"
        }

        fn venue_name(&self) -> &str {
            "usdm"
        }

        fn rate_limits(&self) -> RateLimits {
            RateLimits {
                request_weight_limit: 2400,
                request_weight_window: Duration::from_secs(60),
                raw_requests_limit: 1200,
                raw_requests_window: Duration::from_secs(60),
                orders_10s_limit: 100,
                orders_minute_limit: 1200,
                orders_day_limit: None,
            }
        }

        fn supports_futures(&self) -> bool {
            true
        }

        fn supports_options(&self) -> bool {
            false
        }

        fn supports_margin(&self) -> bool {
            false
        }
    }

    /// Coin-Margined Futures venue configuration
    pub struct CoinmConfig;

    impl VenueConfig for CoinmConfig {
        fn base_url(&self) -> &str {
            "https://dapi.binance.com"
        }

        fn venue_name(&self) -> &str {
            "coinm"
        }

        fn rate_limits(&self) -> RateLimits {
            RateLimits {
                request_weight_limit: 6000,
                request_weight_window: Duration::from_secs(60),
                raw_requests_limit: 61000,
                raw_requests_window: Duration::from_secs(300), // 5 minutes
                orders_10s_limit: 100,
                orders_minute_limit: 1200,
                orders_day_limit: None,
            }
        }

        fn supports_futures(&self) -> bool {
            true
        }

        fn supports_options(&self) -> bool {
            false
        }

        fn supports_margin(&self) -> bool {
            false
        }
    }

    /// Options venue configuration
    pub struct OptionsConfig;

    impl VenueConfig for OptionsConfig {
        fn base_url(&self) -> &str {
            "https://eapi.binance.com"
        }

        fn venue_name(&self) -> &str {
            "options"
        }

        fn rate_limits(&self) -> RateLimits {
            // Same as COINM
            RateLimits {
                request_weight_limit: 6000,
                request_weight_window: Duration::from_secs(60),
                raw_requests_limit: 61000,
                raw_requests_window: Duration::from_secs(300), // 5 minutes
                orders_10s_limit: 100,
                orders_minute_limit: 1200,
                orders_day_limit: None,
            }
        }

        fn supports_futures(&self) -> bool {
            false
        }

        fn supports_options(&self) -> bool {
            true
        }

        fn supports_margin(&self) -> bool {
            false
        }
    }

    /// Portfolio Margin venue configuration
    pub struct PortfolioConfig;

    impl VenueConfig for PortfolioConfig {
        fn base_url(&self) -> &str {
            "https://papi.binance.com"
        }

        fn venue_name(&self) -> &str {
            "portfolio"
        }

        fn rate_limits(&self) -> RateLimits {
            // Same as COINM
            RateLimits {
                request_weight_limit: 6000,
                request_weight_window: Duration::from_secs(60),
                raw_requests_limit: 61000,
                raw_requests_window: Duration::from_secs(300), // 5 minutes
                orders_10s_limit: 100,
                orders_minute_limit: 1200,
                orders_day_limit: None,
            }
        }

        fn supports_futures(&self) -> bool {
            true
        }

        fn supports_options(&self) -> bool {
            false
        }

        fn supports_margin(&self) -> bool {
            true
        }
    }
}
