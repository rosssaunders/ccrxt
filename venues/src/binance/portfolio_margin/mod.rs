mod enums;
mod errors;
mod integration_tests;
mod rate_limit;
mod request;

// Re-export modules for client structure
mod public {
    pub mod rest;
    pub use self::rest::RestClient as PublicRestClient;
}

mod private {
    pub mod rest;
    // Re-export RestClient so it can be re-exported by the parent
    pub use self::rest::RestClient as PrivateRestClient;
}

// Re-export public modules
pub use enums::*;
pub use errors::{ApiError, Errors};
pub use rate_limit::{PortfolioMarginRateLimiter, RateLimitHeader, RateLimiter};

// Export clients
pub use private::PrivateRestClient;
pub use public::PublicRestClient;

/// Portfolio Margin specific response headers
/// Uses the same structure as COIN-M since rate limiting works identically
pub use crate::binance::coinm::{ResponseHeaders, RestResponse, RestResult};

pub use crate::binance::portfolio_margin::errors::ErrorResponse;
pub(crate) use crate::binance::portfolio_margin::request::execute_request;

pub mod rest {
    pub mod common;
}
