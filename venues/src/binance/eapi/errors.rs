use thiserror::Error;
use serde::Deserialize;
use std::fmt;

/// Represents all possible errors that can occur when interacting with the Binance Options API (EAPI)
#[derive(Debug)]
pub enum Errors {
    /// Invalid API key or signature
    InvalidApiKey(),
    
    /// Http error occurred while making a request
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
#[derive(Debug, Clone, Deserialize)]
pub struct ErrorResponse {
    pub code: i32,
    pub msg: String,
}

/// API errors specific to Binance Options API
#[derive(Error, Debug, Clone, Deserialize)]
pub enum ApiError {
    #[error("{msg}")]
    UnknownApiError { msg: String },

    #[error("{msg}")]
    TooManyRequests { msg: String },

    #[error("{msg}")]
    TooManyOrders { msg: String },

    #[error("{msg}")]
    Unauthorized { msg: String },

    #[error("{msg}")]
    InvalidTimestamp { msg: String },

    #[error("{msg}")]
    InvalidSignature { msg: String },

    /// Returned when the API responds with HTTP 429 (Too Many Requests).
    /// This error includes the original error message and relevant Binance rate limit headers.
    #[error("429 Too Many Requests: {msg} (used_weight_1m={used_weight_1m:?}, order_count_1m={order_count_1m:?}, retry_after={retry_after:?})")]
    RateLimitExceeded {
        msg: String,
        used_weight_1m: Option<u32>,
        order_count_1m: Option<u32>,
        retry_after: Option<u64>,
    },

    /// Returned when the API responds with HTTP 418 (IP Auto-Banned).
    #[error("418 IP Auto-Banned: {msg}")]
    IpAutoBanned { msg: String },

    /// Unmapped API error - for error codes not explicitly handled
    #[error("API error (code: {code}): {msg}")]
    UnmappedApiError { code: i32, msg: String },
}

// Conversion from ErrorResponse to ApiError
impl From<ErrorResponse> for ApiError {
    fn from(err: ErrorResponse) -> Self {
        match err.code {
            -1000 => ApiError::UnknownApiError { msg: err.msg },
            -1003 => ApiError::TooManyRequests { msg: err.msg },
            -1015 => ApiError::TooManyOrders { msg: err.msg },
            -1002 => ApiError::Unauthorized { msg: err.msg },
            -1021 => ApiError::InvalidTimestamp { msg: err.msg },
            -1022 => ApiError::InvalidSignature { msg: err.msg },
            _ => ApiError::UnmappedApiError {
                code: err.code,
                msg: err.msg,
            },
        }
    }
}