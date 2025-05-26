use super::api_errors::BinanceCoinMAPIError;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct BinanceHeaders {
    pub used_weight_1m: Option<u32>,
    pub order_count_1m: Option<u32>,
    pub order_count_1d: Option<u32>,
    pub order_count_1s: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct BinanceCoinMResponse<T> {
    pub data: T,
    pub rate_limit_duration: Duration,
    pub request_duration: Duration,
    pub headers: BinanceHeaders,
}

/// Represents all possible errors that can occur when interacting with the Binance API
#[derive(Debug)]
pub enum BinanceCoinMError {
    /// An error returned by the Binance API
    ApiError(BinanceCoinMAPIError),
    /// An HTTP-level error (network, timeout, etc.)
    HttpError(reqwest::Error),
    /// A general error with a descriptive message
    Error(String),
}

impl fmt::Display for BinanceCoinMError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BinanceCoinMError::ApiError(err) => write!(f, "API error: {}", err),
            BinanceCoinMError::HttpError(err) => write!(f, "HTTP error: {}", err),
            BinanceCoinMError::Error(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for BinanceCoinMError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            BinanceCoinMError::HttpError(err) => Some(err),
            _ => None,
        }
    }
}

/// Type alias for results returned by Binance API operations
pub type BinanceCoinMResult<T> = Result<BinanceCoinMResponse<T>, BinanceCoinMError>;

/// Represents an error response from the Binance API.
#[derive(Debug, Clone, Deserialize)]
pub struct ErrorResponse {
    pub code: i32,
    pub msg: String,
}

/// Trait for private API requests that require authentication.
/// All private request structs must implement this trait.
pub trait PrivateRequest: Serialize {
    /// Returns the timestamp for the request in milliseconds since epoch.
    /// This is used for request signing and validation.
    fn timestamp(&self) -> u64;
}
