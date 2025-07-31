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

mod enums;
mod errors;
mod rate_limit;
mod request;
mod signing;
use std::time::Duration;

// Re-export modules for new structure
pub mod public {
    pub mod rest;
    pub use self::rest::{RestClient as PublicRestClient, exchange_info::*};
}

use crate::binance::shared::RateLimits;
use crate::binance::shared::VenueConfig;

mod private {
    pub mod rest;
    // Re-export RestClient so it can be re-exported by the parent
    pub use self::rest::UsdmClient as PrivateRestClient;
}

// Only expose RestClient at the usdm level, not via private::rest
pub use enums::*;
pub use errors::{ApiError, Errors};
pub use private::*;
pub use public::*;
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

impl ResponseHeaders {
    /// Create ResponseHeaders from shared ResponseHeaders
    pub fn from_shared(shared: crate::binance::shared::client::ResponseHeaders) -> Self {
        let mut values = std::collections::HashMap::new();
        
        // Iterate through all headers from the shared response
        for (header_name, header_value) in shared.headers.iter() {
            // Try to parse the header name as a RateLimitHeader
            if let Some(rate_limit_header) = RateLimitHeader::parse(header_name) {
                // Try to parse the header value as a u32
                if let Ok(value) = header_value.parse::<u32>() {
                    values.insert(rate_limit_header, value);
                }
            }
        }
        
        Self { values }
    }
}

/// Type alias for results returned by Binance API operations
pub type RestResult<T> = Result<rest::common::RestResponse<T>, Errors>;

pub mod rest {
    pub mod common;
}

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

#[cfg(test)]
mod tests;
