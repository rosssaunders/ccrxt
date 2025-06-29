use std::fmt;

use serde::Deserialize;
use thiserror::Error;

/// Result type for KuCoin API operations
pub type Result<T> = std::result::Result<T, KucoinError>;

/// Represents all possible errors that can occur when interacting with the KuCoin API
#[derive(Debug, Error)]
pub enum KucoinError {
    /// Invalid API key or signature
    #[error("Invalid API key or signature")]
    InvalidApiKey,

    /// HTTP error occurred while making a request
    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),

    /// An error returned by the KuCoin API
    #[error("API error: {0}")]
    ApiError(#[from] ApiError),

    /// A general error with a descriptive message
    #[error("Error: {0}")]
    Error(String),

    /// JSON serialization/deserialization error
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    /// URL parsing error
    #[error("URL error: {0}")]
    UrlError(#[from] url::ParseError),
}

/// Represents an error response from the KuCoin API.
#[derive(Debug, Clone, Deserialize)]
pub struct ErrorResponse {
    pub code: String,
    pub msg: String,
}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "KuCoin API Error {}: {}", self.code, self.msg)
    }
}

/// Specific API error with code and message
#[derive(Debug, Error)]
pub enum ApiError {
    /// Authentication errors (40x codes)
    #[error("Authentication failed: {message} (code: {code})")]
    AuthenticationError { code: String, message: String },

    /// Bad request errors (400xxx codes)
    #[error("Bad request: {message} (code: {code})")]
    BadRequest { code: String, message: String },

    /// Rate limit exceeded (429xxx codes)
    #[error("Rate limit exceeded: {message} (code: {code})")]
    RateLimitExceeded { code: String, message: String },

    /// Server errors (500xxx codes)
    #[error("Server error: {message} (code: {code})")]
    ServerError { code: String, message: String },

    /// Other API errors
    #[error("API error: {message} (code: {code})")]
    Other { code: String, message: String },

    /// HTTP-related errors
    #[error("HTTP error: {0}")]
    Http(String),

    /// JSON parsing errors
    #[error("JSON parsing error: {0}")]
    JsonParsing(String),
}

impl From<ErrorResponse> for ApiError {
    fn from(error: ErrorResponse) -> Self {
        match error.code.as_str() {
            // Authentication errors
            "400001" => ApiError::AuthenticationError {
                code: error.code,
                message: error.msg,
            },
            "400002" => ApiError::AuthenticationError {
                code: error.code,
                message: error.msg,
            },
            "400003" => ApiError::AuthenticationError {
                code: error.code,
                message: error.msg,
            },
            "400004" => ApiError::AuthenticationError {
                code: error.code,
                message: error.msg,
            },
            "400005" => ApiError::AuthenticationError {
                code: error.code,
                message: error.msg,
            },
            "400006" => ApiError::AuthenticationError {
                code: error.code,
                message: error.msg,
            },

            // Bad request errors
            code if code.starts_with("400") => ApiError::BadRequest {
                code: error.code,
                message: error.msg,
            },

            // Rate limiting
            code if code.starts_with("429") => ApiError::RateLimitExceeded {
                code: error.code,
                message: error.msg,
            },

            // Server errors
            code if code.starts_with("500") => ApiError::ServerError {
                code: error.code,
                message: error.msg,
            },

            // Other errors
            _ => ApiError::Other {
                code: error.code,
                message: error.msg,
            },
        }
    }
}
