use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Common response structure for BitMart API responses
#[derive(Debug)]
pub struct BitMartResponse<T> {
    pub data: T,
    pub request_duration: Duration,
}

/// Basic structure for BitMart error responses
#[derive(Debug, Deserialize, Serialize)]
pub struct BitMartErrorResponse {
    pub code: i32,
    pub msg: String,
}

/// Enum for BitMart API errors
#[derive(Debug, thiserror::Error)]
pub enum BitMartError {
    #[error("API key format invalid (code: {0})")]
    BadApiKeyFmt(i32),
    
    #[error("Invalid API key, IP, or permissions (code: {0})")]
    InvalidApiKey(i32),
    
    #[error("Request signature invalid (code: {0})")]
    InvalidSignature(i32),
    
    #[error("Too many requests (code: {0}, {1})")]
    TooManyRequests(i32, String),
    
    #[error("Server error or connection issue (code: {0})")]
    ServerError(i32),
    
    #[error("Unknown error (code: {0})")]
    Unknown(i32),
    
    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),
}

/// Type alias for BitMart API results
pub type BitMartResult<T> = Result<T, BitMartError>;