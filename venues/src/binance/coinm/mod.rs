use std::time::Duration;

mod enums;
mod errors;
mod rate_limit;
mod request;

// Re-export modules for new structure
mod public {
    mod rest;
    pub use self::rest::exchange_info::*;
    pub use self::rest::RestClient as PublicRestClient;
}

mod private {
    mod rest;
    // Re-export RestClient so it can be re-exported by the parent
    pub use self::rest::account::*;
    pub use self::rest::account_trades::*;
    pub use self::rest::all_orders::*;
    pub use self::rest::batch_order::*;
    pub use self::rest::open_orders::*;
    pub use self::rest::order::*;
    pub use self::rest::position_risk::*;
    pub use self::rest::query_order::*;
    pub use self::rest::RestClient as PrivateRestClient;
}

// Only expose RestClient at the coinm level, not via private::rest
pub use private::*;
pub use public::*;

pub use enums::*;
pub use errors::{ApiError, Errors};
pub use rate_limit::{RateLimitHeader, RateLimiter};

pub use crate::binance::coinm::errors::ErrorResponse;
pub(crate) use crate::binance::coinm::request::execute_request;

/// Represents the relevant response headers returned by the Binance API for rate limiting and order tracking.
///
/// Each field corresponds to a specific header returned by the API, such as used weights or order counts for various intervals.
/// This structure is now strongly typed for high performance and correctness.
#[derive(Debug, Clone, Default)]
pub struct ResponseHeaders {
    /// Map of parsed rate limit/order count headers to their integer values.
    ///
    /// For example:
    /// - RateLimitHeader { kind: UsedWeight, interval_value: 1, interval_unit: Minute } => 123
    /// - RateLimitHeader { kind: OrderCount, interval_value: 1, interval_unit: Day } => 10
    ///
    /// This map is keyed by strongly-typed header descriptors for maximum performance and correctness.
    pub values: std::collections::HashMap<RateLimitHeader, u32>,
}

#[derive(Debug, Clone)]
pub struct RestResponse<T> {
    pub data: T,
    pub request_duration: Duration,
    pub headers: ResponseHeaders,
}

/// Type alias for results returned by Binance API operations
pub type RestResult<T> = Result<RestResponse<T>, Errors>;

pub mod rest {
    pub mod common;
}
