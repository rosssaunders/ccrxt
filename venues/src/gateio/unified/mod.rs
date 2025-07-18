// Private API modules
pub mod private;
// Public API modules
pub mod public;

// Re-export key components from shared modules
pub use crate::gateio::shared::*;
pub use private::RestClient as PrivateRestClient;
pub use public::RestClient as PublicRestClient;

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
