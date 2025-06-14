use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Common BingX API errors
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum Errors {
    #[error("Authentication error: {0}")]
    AuthenticationError(String),

    #[error("Rate limit exceeded: {0}")]
    RateLimitExceeded(String),

    #[error("Invalid API key")]
    InvalidApiKey,

    #[error("Invalid timestamp: {0}")]
    InvalidTimestamp(String),

    #[error("IP whitelist error: {0}")]
    IpWhitelistError(String),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("API error {code}: {msg}")]
    ApiError { code: i32, msg: String },

    #[error("General error: {0}")]
    Error(String),
}

/// Standard BingX API error response format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub code: i32,
    pub msg: String,
}

impl From<reqwest::Error> for Errors {
    fn from(err: reqwest::Error) -> Self {
        Errors::NetworkError(err.to_string())
    }
}

impl From<serde_json::Error> for Errors {
    fn from(err: serde_json::Error) -> Self {
        Errors::ParseError(err.to_string())
    }
}

impl From<ErrorResponse> for Errors {
    fn from(err: ErrorResponse) -> Self {
        match err.code {
            100413 => Errors::InvalidApiKey,
            100419 => Errors::IpWhitelistError(err.msg),
            100410 => Errors::RateLimitExceeded(err.msg),
            80014 => Errors::InvalidTimestamp(err.msg),
            100421 => Errors::InvalidTimestamp(err.msg),
            _ => Errors::ApiError { code: err.code, msg: err.msg },
        }
    }
}