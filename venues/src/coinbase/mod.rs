pub mod enums;
mod errors;
mod rate_limit;

pub mod private {
    mod rest;
    pub use self::rest::RestClient;
    pub use self::rest::{AccountBalance, GetAccountBalancesRequest, GetAccountBalancesResponse};
}

pub use enums::*;
pub use errors::{ApiError, ErrorResponse, Errors};
pub use private::RestClient as PrivateRestClient;
pub use private::{AccountBalance, GetAccountBalancesRequest, GetAccountBalancesResponse};
pub use rate_limit::{EndpointType, RateLimit, RateLimitError, RateLimiter};

/// Type alias for results returned by Coinbase API operations
pub type RestResult<T> = Result<T, Errors>;
