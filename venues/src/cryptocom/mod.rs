mod errors;
mod integration_tests;
mod examples;

pub mod rate_limit;
pub mod private;

pub use rate_limit::{RateLimiter, EndpointType, RateLimitError, RateLimit};
pub use errors::{Errors, ApiError, ErrorResponse};
pub use private::{RestClient};

/// Type alias for results returned by Crypto.com API operations
pub type RestResult<T> = Result<T, Errors>;
