mod enums;
mod errors;
mod rate_limit;

// Private API modules
mod private;
// Public API modules
pub mod public;
// Futures API modules
pub mod futures;

// Re-export key components
pub use enums::*;
pub use errors::{ApiError, Errors, BitgetError};

// Create error module for backward compatibility  
pub mod error {
    pub use super::errors::BitgetError;
}
// Export clients and endpoint types
pub use private::PrivateRestClient;
// Alias for backward compatibility  
pub use private::PrivateRestClient as BitgetRestClient;
pub use private::{AssetInfo, GetAccountAssetsRequest, GetAccountAssetsResponse};
pub use rate_limit::{RateLimitHeader, RateLimiter};

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
