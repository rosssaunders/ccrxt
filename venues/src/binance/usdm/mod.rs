//! Binance USD-M Futures (USDM) API client
//!
//! This module provides access to the Binance USD-M Futures API, including:
//!
//! - **Rate Limiting**: Automatic rate limiting for RAW_REQUEST, REQUEST_WEIGHT, and ORDER limits
//! - **Public Endpoints**: Market data, exchange info, etc. via `/fapi/v1/` endpoints  
//! - **Private Endpoints**: Account data, trading, etc. (basic structure provided)
//! - **Error Handling**: Comprehensive error types for API responses
//!
//! # Rate Limiting
//!
//! The USDM API enforces strict rate limits:
//! - Raw requests: 1,200 per minute
//! - Request weight: 2,400 per minute
//! - Orders: 100 per 10s, 1,200 per minute
//!
//! The rate limiter automatically tracks these limits and returns errors when they would be exceeded.
//!
//! # Example
//!
//! ```rust
//! use venues::binance::usdm::{PublicRestClient, RateLimiter};
//! use reqwest::Client;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let rate_limiter = RateLimiter::new();
//!     let client = PublicRestClient::new(
//!         "https://fapi.binance.com",
//!         Client::new(),
//!         rate_limiter,
//!     );
//!     
//!     // The client can be used to send requests with automatic rate limiting
//!     println!("Client created successfully");
//!     
//!     Ok(())
//! }
//! ```

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
    pub use self::rest::RestClient as PrivateRestClient;
}

// Only expose RestClient at the usdm level, not via private::rest
pub use private::*;
pub use public::*;

pub use enums::*;
pub use errors::{ApiError, Errors};
pub use rate_limit::{RateLimitHeader, RateLimiter};

pub use crate::binance::usdm::errors::ErrorResponse;
pub(crate) use crate::binance::usdm::request::execute_request;

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
