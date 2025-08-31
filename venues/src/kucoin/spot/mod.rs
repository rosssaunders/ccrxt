pub mod enums;
pub mod errors;
pub mod private;
pub mod private_client;
pub mod public;
pub mod public_client;
pub mod rate_limit;

// Re-export commonly used types
pub use enums::*;
pub use errors::{ApiError, ErrorResponse, KucoinError, Result};
// Import shared types from rate_limit module
pub use rate_limit::RateLimitHeader;

// Import shared credentials from shared module
pub use super::shared::credentials::Credentials;

// Type aliases for compatibility
pub type ResponseHeaders = RateLimitHeader;

/// Standard REST API response structure for KuCoin Spot
#[derive(Debug, Clone, serde::Deserialize)]
pub struct RestResponse<T> {
    /// Response code from KuCoin API
    pub code: String,
    /// Response data
    pub data: T,
}

impl<T> RestResponse<T> {
    /// Check if the response indicates success
    pub fn is_success(&self) -> bool {
        self.code == "200000"
    }
}
