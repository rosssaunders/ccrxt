mod enums;
mod errors;
mod integration_tests;
mod rate_limit;

// Re-export public modules
pub use enums::*;
pub use errors::{ApiError, Errors};
pub use rate_limit::{PortfolioMarginRateLimiter, RateLimitHeader, RateLimiter};

/// Portfolio Margin specific response headers
/// Uses the same structure as COIN-M since rate limiting works identically
pub use crate::binance::coinm::{ResponseHeaders, RestResponse, RestResult};
