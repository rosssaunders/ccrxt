pub mod enums;
mod errors;

pub mod rate_limit;
pub mod private {
    mod rest;
    pub use self::rest::RestClient;
    pub use self::rest::{GetBalancesRequest, GetBalancesResponse, Balance};
}

pub use enums::*;
pub use errors::{Errors, ErrorResponse};
pub use private::RestClient as PrivateRestClient;
pub use private::{GetBalancesRequest, GetBalancesResponse, Balance};
pub use rate_limit::{EndpointType, RateLimit, RateLimitError, RateLimiter};

/// Type alias for results returned by BingX API operations
pub type RestResult<T> = Result<T, Errors>;