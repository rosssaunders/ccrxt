use serde::Deserialize;
use std::fmt;
use thiserror::Error;

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

impl From<serde_json::Error> for Errors {
    fn from(err: serde_json::Error) -> Self {
        Errors::Error(format!("JSON serialization error: {}", err))
    }
}

/// Represents an error response from the Deribit API
#[derive(Debug, Clone, Deserialize)]
pub struct ErrorResponse {
    pub error: ErrorDetail,
}

/// Deribit API error detail
#[derive(Debug, Clone, Deserialize)]
pub struct ErrorDetail {
    pub code: i32,
    pub message: String,
}

/// Deribit API error codes as documented in their API specification
#[derive(Error, Debug, Clone)]
pub enum ApiError {
    // Authentication errors
    #[error("API authentication failed")]
    AuthenticationFailed,

    #[error("Insufficient permissions")]
    InsufficientPermissions,

    #[error("Invalid credentials")]
    InvalidCredentials,

    // Request errors
    #[error("Invalid request")]
    InvalidRequest,

    #[error("Method not found")]
    MethodNotFound,

    #[error("Invalid parameters")]
    InvalidParameters,

    // Rate limiting
    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    // Generic error for unmapped cases
    #[error("API error (code: {code}): {message}")]
    UnmappedApiError { code: i32, message: String },
}

impl From<ErrorDetail> for ApiError {
    fn from(err: ErrorDetail) -> Self {
        match err.code {
            13004 => ApiError::AuthenticationFailed,
            13003 => ApiError::InvalidCredentials,
            13011 => ApiError::InsufficientPermissions,
            -32601 => ApiError::MethodNotFound,
            -32602 => ApiError::InvalidParameters,
            -32600 => ApiError::InvalidRequest,
            10024 => ApiError::RateLimitExceeded,
            _ => ApiError::UnmappedApiError {
                code: err.code,
                message: err.message,
            },
        }
    }
}

/// Type alias for results returned by Deribit API operations
pub type RestResult<T> = Result<T, Errors>;