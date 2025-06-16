pub mod enums;
mod errors;

pub mod rate_limit;
pub mod private {
    mod rest;
    pub use self::rest::RestClient;
    pub use self::rest::{GetAccountBalanceRequest, GetAccountBalanceResponse, WalletBalance};
}

pub use enums::*;
pub use errors::{ApiError, ErrorResponse, Errors};
pub use private::RestClient as PrivateRestClient;
pub use private::{GetAccountBalanceRequest, GetAccountBalanceResponse, WalletBalance};
pub use rate_limit::{EndpointType, RateLimit, RateLimitError, RateLimiter};

/// Type alias for results returned by BitMart API operations
pub type RestResult<T> = Result<T, Errors>;
