pub mod enums;
mod errors;
mod integration_tests;
mod rate_limit;

pub mod private {
    pub mod rest;
    pub use self::rest::RestClient;
}

pub mod public {
    mod rest;
    pub use self::rest::RestClient;
}

pub use enums::*;
pub use errors::{ApiError, ErrorResponse, Errors};
pub use private::RestClient as PrivateRestClient;
pub use public::RestClient as PublicRestClient;
pub use rate_limit::{EndpointType, RateLimit, RateLimitError, RateLimiter};
// Re-export trading account types for convenience
pub use private::rest::{TradingAccount, TradingAccountsResponse};

/// Type alias for results returned by Bullish API operations
pub type RestResult<T> = Result<T, Errors>;