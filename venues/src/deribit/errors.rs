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

impl From<crate::deribit::rate_limit::RateLimitError> for Errors {
    fn from(err: crate::deribit::rate_limit::RateLimitError) -> Self {
        Errors::Error(format!("Rate limit error: {}", err))
    }
}

/// Represents an error response from the Deribit API
#[derive(Debug, Clone, Deserialize)]
pub struct ErrorResponse {
    pub code: i32,
    pub message: String,
}

/// Deribit API error codes as documented in their API specification
#[derive(Error, Debug, Clone, Deserialize)]
pub enum ApiError {
    #[error("Success")]
    Success,

    #[error("Insufficient funds")]
    InsufficientFunds,

    #[error("Invalid destination address")]
    InvalidDestination,

    #[error("Transfer limit exceeded")]
    TransferLimitExceeded,

    #[error("Invalid currency")]
    InvalidCurrency,

    #[error("Invalid amount")]
    InvalidAmount,

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    #[error("Internal server error")]
    InternalServerError,

    /// Unmapped API error - for error codes not explicitly handled
    #[error("API error (code: {code}): {message}")]
    UnmappedApiError { code: i32, message: String },
}

impl From<ErrorResponse> for ApiError {
    fn from(err: ErrorResponse) -> Self {
        match err.code {
            0 => ApiError::Success,
            9999 => ApiError::InsufficientFunds,
            11029 => ApiError::InvalidDestination,
            11030 => ApiError::TransferLimitExceeded,
            11031 => ApiError::InvalidCurrency,
            11032 => ApiError::InvalidAmount,
            13009 => ApiError::Unauthorized,
            10029 => ApiError::RateLimitExceeded,
            _ => ApiError::UnmappedApiError {
                code: err.code,
                message: err.message,
            },
        }
    }
}

/// Type alias for results returned by Deribit API operations
pub type RestResult<T> = Result<T, Errors>;