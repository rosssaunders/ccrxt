mod enums;
mod errors;
mod rate_limit;
mod tests;

// Private API modules
pub mod private;
// Public API modules
pub mod public;

// Re-export key components
pub use enums::*;
pub use errors::{ApiError, GateIoError, Result};
pub use private::RestClient as PrivateRestClient;
pub use public::RestClient as PublicRestClient;
pub use rate_limit::{RateLimitHeader, RateLimitStatus, RateLimiter};

pub use crate::gateio::errors::ErrorResponse;

/// Represents the relevant response headers returned by the Gate.io API for rate limiting.
pub type ResponseHeaders = RateLimitHeader;

/// A general response wrapper for Gate.io API responses
#[derive(Debug, Clone, serde::Deserialize)]
pub struct RestResponse<T> {
    pub code: String,
    pub data: T,
}

impl<T> RestResponse<T> {
    /// Check if the response indicates success
    pub fn is_success(&self) -> bool {
        self.code == "0"
    }
}
