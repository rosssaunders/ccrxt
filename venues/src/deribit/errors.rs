use serde::Deserialize;
use std::fmt;

/// Represents all possible errors that can occur when interacting with the Deribit API
#[derive(Debug)]
pub enum Errors {
    /// Invalid API key or signature
    InvalidApiKey(),

    /// Http error occurred while making a request
    /// This variant is used to represent errors that are not specific to the Deribit API,
    /// such as network issues or HTTP errors.
    /// It can be used to wrap any error that occurs during the request process.
    /// This variant is not used for errors returned by the Deribit API itself.
    HttpError(reqwest::Error),

    /// An error returned by the Deribit API
    ApiError(ApiError),

    /// A general error with a descriptive message
    Error(String),

    /// Rate limit error
    RateLimitError(String),
}

impl fmt::Display for Errors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Errors::InvalidApiKey() => write!(f, "Invalid API key or signature"),
            Errors::HttpError(err) => write!(f, "HTTP error: {}", err),
            Errors::ApiError(err) => write!(f, "API error: {}", err),
            Errors::Error(msg) => write!(f, "Error: {}", msg),
            Errors::RateLimitError(msg) => write!(f, "Rate limit error: {}", msg),
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

impl From<crate::deribit::rate_limit::RateLimitError> for Errors {
    fn from(err: crate::deribit::rate_limit::RateLimitError) -> Self {
        Errors::RateLimitError(err.to_string())
    }
}

/// Represents an error response from the Deribit API using JSON-RPC 2.0 format
#[derive(Debug, Deserialize)]
pub struct ApiError {
    /// JSON-RPC version (always "2.0")
    pub jsonrpc: String,
    /// The error object containing details about the error
    pub error: ErrorDetails,
    /// The request ID (if provided in the request)
    pub id: Option<serde_json::Value>,
}

/// Error details in JSON-RPC 2.0 format
#[derive(Debug, Deserialize)]
pub struct ErrorDetails {
    /// Error code (integer)
    pub code: i32,
    /// Error message (string)
    pub message: String,
    /// Additional error data (optional)
    pub data: Option<serde_json::Value>,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Code {}: {}", self.error.code, self.error.message)
    }
}

impl std::error::Error for ApiError {}

/// Type alias for REST API results
pub type RestResult<T> = Result<T, Errors>;