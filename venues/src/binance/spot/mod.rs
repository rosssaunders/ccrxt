mod enums;
mod errors;
mod rate_limit;

// Re-export key components
pub use errors::{Errors, ApiError};
pub use rate_limit::{RateLimiter, RateLimitHeader};
pub use enums::*;

pub use crate::binance::spot::errors::ErrorResponse;

/// Represents the relevant response headers returned by the Binance Spot API for rate limiting and order tracking.
///
/// Each field corresponds to a specific header returned by the API, such as used weights or order counts for various intervals.
/// This structure is now strongly typed for high performance and correctness.
#[derive(Debug, Clone, Default)]
pub struct ResponseHeaders {
    pub values: std::collections::HashMap<rate_limit::RateLimitHeader, u32>,
}

#[derive(Debug, Clone)]
pub struct RestResponse<T> {
    pub data: T,
    pub headers: ResponseHeaders,
}

/// Type alias for results returned by Binance Spot API operations
pub type RestResult<T> = Result<RestResponse<T>, Errors>;