use thiserror::Error;
use serde::Deserialize;
use std::fmt;

/// Represents all possible errors that can occur when interacting with the Binance Options API
#[derive(Debug)]
pub enum Errors {
    /// Invalid API key or signature
    InvalidApiKey(),
    
    /// Http error occurred while making a request
    /// This variant is used to represent errors that are not specific to the Binance API,
    /// such as network issues or HTTP errors.
    /// It can be used to wrap any error that occurs during the request process.
    /// This variant is not used for errors returned by the Binance API itself.
    HttpError(reqwest::Error),

    /// An error returned by the Binance Options API
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

/// Represents an error response from the Binance Options API.
/// 
/// This is public as it is used by responses.
#[derive(Debug, Clone, Deserialize)]
pub struct ErrorResponse {
    pub code: i32,
    pub msg: String,
}

/// Error code ranges for Binance Options API:
/// -1000 to -1999: General Server or Network issues
/// -2000 to -2999: Authentication and Authorization errors
/// -3000 to -3999: Rate limiting errors
/// -4000 to -4999: Options specific errors
#[derive(Debug, Error)]
pub enum ApiError {
    /// Rate limit exceeded
    #[error("Rate limit exceeded")]
    RateLimitExceeded(ErrorResponse),
    
    /// Invalid symbol
    #[error("Invalid symbol")]
    InvalidSymbol(ErrorResponse),
    
    /// Invalid filter parameters
    #[error("Invalid filter parameters")]
    InvalidFilter(ErrorResponse),
    
    /// Other API errors
    #[error("API error")]
    Other(ErrorResponse),
}

impl From<ErrorResponse> for ApiError {
    fn from(error: ErrorResponse) -> Self {
        match error.code {
            -1003 => ApiError::RateLimitExceeded(error),
            -1121 => ApiError::InvalidSymbol(error),
            -1013 => ApiError::InvalidFilter(error),
            _ => ApiError::Other(error),
        }
    }
}