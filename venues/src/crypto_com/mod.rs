mod errors;
mod examples;
mod integration_tests;

pub mod rate_limit;
pub mod private {
    mod rest;
    pub use self::rest::RestClient;
}

pub mod public {
    mod rest;
    pub use self::rest::RestClient;
}

pub use errors::{ApiError, ErrorResponse, Errors};
pub use private::RestClient as PrivateRestClient;
pub use public::RestClient as PublicRestClient;
pub use rate_limit::{EndpointType, RateLimit, RateLimitError, RateLimiter};

/// Type alias for results returned by Crypto.com API operations
pub type RestResult<T> = Result<T, Errors>;
