use serde::Deserialize;
use std::fmt;

/// Represents all possible errors that can occur when interacting with the Deribit API
#[derive(Debug)]
pub enum Errors {
    /// Invalid API key or authentication failure
    InvalidApiKey(),

    /// HTTP error occurred while making a request
    HttpError(reqwest::Error),

    /// An error returned by the Deribit API
    ApiError(ApiError),

    /// A general error with a descriptive message
    Error(String),

    /// Rate limiting error
    RateLimitError(crate::deribit::RateLimitError),
}

impl fmt::Display for Errors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Errors::InvalidApiKey() => write!(f, "Invalid API key or authentication failed"),
            Errors::HttpError(err) => write!(f, "HTTP error: {}", err),
            Errors::ApiError(err) => write!(f, "API error: {}", err),
            Errors::Error(msg) => write!(f, "Error: {}", msg),
            Errors::RateLimitError(err) => write!(f, "Rate limit error: {}", err),
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
        Errors::RateLimitError(err)
    }
}

/// Represents an error response from the Deribit API
#[derive(Debug, Clone, Deserialize)]
pub struct ApiError {
    /// Error code returned by the API
    pub code: i32,
    /// Human-readable error message
    pub message: String,
    /// Additional error data (optional)
    pub data: Option<serde_json::Value>,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Deribit API Error {}: {}", self.code, self.message)
    }
}

impl std::error::Error for ApiError {}

/// Standard Deribit API error response structure
#[derive(Debug, Clone, Deserialize)]
pub struct ErrorResponse {
    /// JSON-RPC version
    pub jsonrpc: String,
    /// Request ID
    pub id: Option<serde_json::Value>,
    /// Error details
    pub error: ApiError,
}

/// Type alias for results returned by Deribit API operations
pub type RestResult<T> = Result<T, Errors>;