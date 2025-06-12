mod enums;
mod errors;
mod rate_limit;
mod integration_tests;

// Re-export public modules
pub use errors::{Errors, ApiError};
pub use rate_limit::{RateLimiter, RateLimitHeader, PortfolioMarginRateLimiter};
pub use enums::*;

/// Portfolio Margin specific response headers
/// Uses the same structure as COIN-M since rate limiting works identically
pub use crate::binance::coinm::{ResponseHeaders, RestResponse, RestResult};
