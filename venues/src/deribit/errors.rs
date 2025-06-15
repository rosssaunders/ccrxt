//! Error types for Deribit API operations

use thiserror::Error;

/// Result type for Deribit API operations
pub type DeribitResult<T> = Result<T, DeribitError>;

/// Errors that can occur when using the Deribit API
#[derive(Error, Debug)]
pub enum DeribitError {
    /// HTTP request error
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),

    /// JSON-RPC error response from Deribit
    #[error("Deribit API error {code}: {message}")]
    ApiError {
        /// Error code from Deribit
        code: i32,
        /// Error message from Deribit
        message: String,
    },

    /// JSON parsing error
    #[error("Failed to parse JSON response: {0}")]
    JsonError(#[from] serde_json::Error),

    /// Rate limit error
    #[error("Rate limit error: {0}")]
    RateLimitError(#[from] crate::deribit::RateLimitError),

    /// Generic error
    #[error("Error: {0}")]
    Error(String),
}