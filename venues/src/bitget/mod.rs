mod enums;
mod errors;
mod rate_limit;

// Private API modules
mod private;

// Re-export key components
pub use enums::*;
pub use errors::{ApiError, Errors};
pub use rate_limit::{RateLimiter, RateLimitHeader};

// Export clients and endpoint types
pub use private::PrivateRestClient;
pub use private::{GetAccountAssetsRequest, GetAccountAssetsResponse, AssetInfo};

pub use crate::bitget::errors::ErrorResponse;

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