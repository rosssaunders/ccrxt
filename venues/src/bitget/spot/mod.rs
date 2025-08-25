mod enums;
mod errors;
mod rate_limit;
mod rate_limiter_trait;

// Re-export key components
pub use enums::*;
pub use errors::{ApiError, BitgetError, Errors};

// Create error module for backward compatibility
pub mod error {
    pub use super::errors::BitgetError;
}

pub mod public {
    pub mod rest;
    pub use self::rest::RestClient;
}

pub use public::RestClient as PublicRestClient;

pub mod private {
    pub mod rest;
    pub use self::rest::{Credentials, RestClient};
}

pub use errors::ErrorResponse;
pub use private::{Credentials, RestClient as PrivateRestClient};
// Alias for backward compatibility
pub use rate_limit::{RateLimitHeader, RateLimiter};
pub use rate_limiter_trait::{BitGetRateLimiter, BitGetUsageStats};

/// Represents the relevant response headers returned by the Bitget API for rate limiting.
///
/// Each field corresponds to a specific header returned by the API.
/// This structure follows the same pattern as other exchange implementations.
#[derive(Debug, Clone, Default)]
pub struct ResponseHeaders {
    pub values: std::collections::HashMap<rate_limit::RateLimitHeader, u32>,
}

#[derive(Debug, Clone)]
pub struct RestResponse<T> {
    pub data: T,
    pub headers: ResponseHeaders,
}

/// Type alias for results returned by Bitget API operations
pub type RestResult<T> = Result<RestResponse<T>, Errors>;
