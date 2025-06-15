use serde::Deserialize;
use std::fmt;

/// Represents all possible errors that can occur when interacting with the Deribit API
#[derive(Debug)]
pub enum Errors {
    /// Invalid API key or signature
    InvalidApiKey(),
    
    /// Http error occurred while making a request
    HttpError(reqwest::Error),

    /// An error returned by the Deribit API
    ApiError(ApiError),
    
    /// A general error with a descriptive message
    Error(String),
}

impl fmt::Display for Errors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Errors::InvalidApiKey() => write!(f, "Invalid API key or signature"),
            Errors::HttpError(err) => write!(f, "HTTP error: {}", err),
            Errors::ApiError(err) => write!(f, "API error: {}", err),
            Errors::Error(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for Errors {}

impl From<reqwest::Error> for Errors {
    fn from(err: reqwest::Error) -> Self {
        Errors::HttpError(err)
    }
}

/// Represents an error response from the Deribit API using JSON-RPC 2.0 format
#[derive(Debug, Clone, Deserialize)]
pub struct ErrorResponse {
    pub id: Option<u64>,
    pub jsonrpc: String,
    pub error: ApiError,
}

/// API error details from Deribit JSON-RPC responses
#[derive(Debug, Clone, Deserialize)]
pub struct ApiError {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Code {}: {}", self.code, self.message)
    }
}

impl std::error::Error for ApiError {}

/// Type alias for results returned by Deribit API operations
pub type RestResult<T> = Result<T, Errors>;