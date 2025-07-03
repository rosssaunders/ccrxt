mod enums;
mod errors;
mod rate_limit;

// Private API modules
pub mod private;
// Public API modules
pub mod public;

// Re-export key components
pub use enums::*;
pub use errors::{ApiError, KucoinError, Result};
pub use private::RestClient as PrivateRestClient;
pub use public::RestClient as PublicRestClient;
pub use rate_limit::{RateLimitHeader, RateLimitStatus, RateLimiter, ResourcePool, VipLevel};

pub use crate::kucoin::errors::ErrorResponse;

// Futures are accessible through public::futures and private::futures

/// Represents the relevant response headers returned by the KuCoin API for rate limiting.
pub type ResponseHeaders = RateLimitHeader;

/// A general response wrapper for KuCoin API responses
#[derive(Debug, Clone, serde::Deserialize)]
pub struct RestResponse<T> {
    pub code: String,
    pub data: T,
}

impl<T> RestResponse<T> {
    /// Check if the response indicates success
    pub fn is_success(&self) -> bool {
        self.code == "200000"
    }
}
