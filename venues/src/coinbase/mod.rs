pub mod enums;
mod errors;
pub mod private;
pub mod public;
mod rate_limit;

pub use enums::*;
pub use errors::{ApiError, ErrorResponse, Errors};
pub use rate_limit::{EndpointType, RateLimit, RateLimitError, RateLimiter};

/// Type alias for results returned by Coinbase API operations
pub type RestResult<T> = Result<T, Errors>;
