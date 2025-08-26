mod enums;
mod errors;
mod private_client;
mod public_client;
mod rate_limit;

// Private module with re-exports
mod private {
    pub mod rest;
}

// Public module with re-exports
mod public {
    pub mod rest;
}

// Re-export the RestClients at the spot level
// Re-export key components
pub use enums::*;
pub use errors::{ApiError, Errors};
// Re-export for backward compatibility
pub use private::PrivateRestClient;
pub use private_client::RestClient as PrivateRestClient;
pub use public::PublicRestClient;
pub use public_client::RestClient as PublicRestClient;
// Re-export all public request types for integration tests
pub use public::rest::{
    AggTradesRequest, AvgPriceRequest, DepthRequest, ExchangeInfoRequest, HistoricalTradesRequest,
    KlinesRequest, Ticker24hrRequest, TickerBookRequest, TickerPriceRequest, TickerRequest,
    TickerTradingDayRequest, TradesRequest, UiKlinesRequest,
};
pub use rate_limit::{RateLimitHeader, RateLimiter};

pub use crate::binance::spot::errors::ErrorResponse;

/// Represents the relevant response headers returned by the Binance Spot API for rate limiting and order tracking.
///
/// Each field corresponds to a specific header returned by the API, such as used weights or order counts for various intervals.
/// This structure is now strongly typed for high performance and correctness.
#[derive(Debug, Clone, Default)]
pub struct ResponseHeaders {
    pub values: std::collections::HashMap<rate_limit::RateLimitHeader, u32>,
}

impl ResponseHeaders {}

#[derive(Debug, Clone)]
pub struct RestResponse<T> {
    pub data: T,
    pub headers: ResponseHeaders,
}

/// Type alias for results returned by Binance Spot API operations
pub type RestResult<T> = Result<RestResponse<T>, Errors>;

pub type SpotClient = crate::binance::shared::client::PrivateBinanceClient;

use std::time::Duration;

use crate::binance::shared::venue_trait::{RateLimits, VenueConfig};

/// Spot trading venue configuration
pub struct SpotConfig;

impl VenueConfig for SpotConfig {
    fn base_url(&self) -> &str {
        "https://api.binance.com"
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
}
