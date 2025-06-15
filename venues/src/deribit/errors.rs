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

    /// Rate limiting error
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

/// Represents an error response from the Deribit API.
/// 
/// Error responses follow the JSON-RPC 2.0 format:
/// ```json
/// {
///   "jsonrpc": "2.0",
///   "id": 1,
///   "error": {
///     "code": -32602,
///     "message": "Invalid params"
///   }
/// }
/// ```
#[derive(Debug, Clone, Deserialize)]
pub struct ApiError {
    /// Error code as defined by JSON-RPC 2.0 specification
    /// or Deribit-specific error codes
    pub code: i32,
    /// Human-readable error message
    pub message: String,
    /// Optional data field with additional error information
    pub data: Option<serde_json::Value>,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Deribit API Error {}: {}", self.code, self.message)
    }
}

impl std::error::Error for ApiError {}

/// Represents a complete error response from the Deribit API
#[derive(Debug, Clone, Deserialize)]
pub struct ErrorResponse {
    /// JSON-RPC version (always "2.0")
    pub jsonrpc: String,
    /// Request ID that was sent with the original request
    pub id: Option<serde_json::Value>,
    /// Error details
    pub error: ApiError,
}

/// Type alias for results returned by Deribit API operations
pub type RestResult<T> = Result<T, Errors>;