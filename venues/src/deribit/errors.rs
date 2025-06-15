use serde::Deserialize;
use std::fmt;

/// Represents all possible errors that can occur when interacting with the Deribit API
#[derive(Debug)]
pub enum Errors {
    /// Http error occurred while making a request
    HttpError(reqwest::Error),

    /// An error returned by the Deribit API
    ApiError(ApiError),

    /// A general error with a descriptive message
    Error(String),

    /// Rate limiting error
    RateLimit(crate::deribit::RateLimitError),
}

impl fmt::Display for Errors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Errors::HttpError(err) => write!(f, "HTTP error: {}", err),
            Errors::ApiError(err) => write!(f, "API error: {}", err),
            Errors::Error(msg) => write!(f, "Error: {}", msg),
            Errors::RateLimit(err) => write!(f, "Rate limit error: {}", err),
        }
    }
}

impl std::error::Error for Errors {}

impl From<reqwest::Error> for Errors {
    fn from(err: reqwest::Error) -> Self {
        Errors::HttpError(err)
    }
}

impl From<serde_json::Error> for Errors {
    fn from(err: serde_json::Error) -> Self {
        Errors::Error(format!("JSON serialization error: {}", err))
    }
}

impl From<crate::deribit::RateLimitError> for Errors {
    fn from(err: crate::deribit::RateLimitError) -> Self {
        Errors::RateLimit(err)
    }
}

/// Represents an error response from the Deribit API.
#[derive(Debug, Deserialize)]
pub struct ApiError {
    /// Error code from Deribit API
    pub code: i32,
    /// Error message from Deribit API
    pub message: String,
    /// Optional error data
    pub data: Option<serde_json::Value>,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Deribit API Error {}: {}", self.code, self.message)
    }
}

impl std::error::Error for ApiError {}

/// Standard Deribit JSON-RPC error response
#[derive(Debug, Deserialize)]
pub struct ErrorResponse {
    /// JSON-RPC version
    pub jsonrpc: String,
    /// Request ID
    pub id: Option<serde_json::Value>,
    /// Error details
    pub error: ApiError,
}