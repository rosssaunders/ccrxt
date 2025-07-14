mod enums;
mod errors;
mod rate_limit;
mod request;
pub(crate) mod rest;

// Private module with re-exports
pub mod private {
    pub mod rest;
    // Re-export RestClient so it can be re-exported by the parent
    pub use self::rest::RestClient as PrivateRestClient;
}

// Public module with re-exports
pub mod public {
    pub mod rest;
    // Re-export RestClient so it can be re-exported by the parent
    pub use self::rest::RestClient as PublicRestClient;
}

// Re-export the RestClients at the spot level
// Re-export key components
pub use enums::*;
pub use errors::{ApiError, Errors};
// Re-export for backward compatibility
pub use private::PrivateRestClient;
pub use public::PublicRestClient;
// Re-export all public request types for integration tests
pub use public::rest::{
    AggTradesRequest, AvgPriceRequest, DepthRequest, ExchangeInfoRequest, HistoricalTradesRequest,
    KlinesRequest, Ticker24hrRequest, TickerBookRequest, TickerPriceRequest, TickerRequest,
    TickerTradingDayRequest, TradesRequest, UiKlinesRequest,
};

// Re-export all private request types for integration tests
pub use private::rest::{
    AccountCommissionRequest, AccountCommissionResponse, AccountRequest, AccountResponse,
    AllOrdersRequest, CancelOrderRequest, MyTradesRequest, NewOrderRequest, OpenOrdersRequest,
    QueryOrderRequest, TestNewOrderRequest,
};
pub use rate_limit::{RateLimitHeader, RateLimiter};

pub use crate::binance::spot::errors::ErrorResponse;
// Internal re-export for private client usage
pub(crate) use crate::binance::spot::request::execute_request;

/// Represents the relevant response headers returned by the Binance Spot API for rate limiting and order tracking.
///
/// Each field corresponds to a specific header returned by the API, such as used weights or order counts for various intervals.
/// This structure is now strongly typed for high performance and correctness.
#[derive(Debug, Clone, Default)]
pub struct ResponseHeaders {
    pub values: std::collections::HashMap<rate_limit::RateLimitHeader, u32>,
}

impl ResponseHeaders {
    /// Create ResponseHeaders from reqwest::HeaderMap by parsing rate limit headers
    pub fn from_reqwest_headers(headers: &reqwest::header::HeaderMap) -> Self {
        let values = headers
            .iter()
            .filter_map(|(name, val)| {
                rate_limit::RateLimitHeader::parse(name.as_str())
                    .and_then(|hdr| val.to_str().ok()?.parse::<u32>().ok().map(|v| (hdr, v)))
            })
            .collect();
        Self { values }
    }
}

#[derive(Debug, Clone)]
pub struct RestResponse<T> {
    pub data: T,
    pub headers: ResponseHeaders,
}

/// Type alias for results returned by Binance Spot API operations
pub type RestResult<T> = Result<RestResponse<T>, Errors>;
