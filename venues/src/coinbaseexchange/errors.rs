use std::fmt;

use serde::Deserialize;

/// Represents all possible errors that can occur when interacting with the Coinbase API
#[derive(Debug)]
pub enum Errors {
    /// Invalid API key or signature
    InvalidApiKey(),

    /// HTTP error occurred while making a request
    /// This variant is used to represent errors that are not specific to the Coinbase API,
    /// such as network issues or HTTP errors.
    HttpError(reqwest::Error),

    /// An error returned by the Coinbase API
    ApiError(ApiError),

    /// Rate limit error
    RateLimitError(crate::coinbaseexchange::rate_limit::RateLimitError),

    /// A general error with a descriptive message
    Error(String),
}

impl fmt::Display for Errors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Errors::InvalidApiKey() => write!(f, "Invalid API key or signature"),
            Errors::HttpError(err) => write!(f, "HTTP error: {err}"),
            Errors::ApiError(err) => write!(f, "API error: {err}"),
            Errors::RateLimitError(err) => write!(f, "Rate limit error: {err}"),
            Errors::Error(msg) => write!(f, "Error: {msg}"),
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
        Errors::Error(format!("JSON serialization error: {err}"))
    }
}

impl From<crate::coinbaseexchange::rate_limit::RateLimitError> for Errors {
    fn from(err: crate::coinbaseexchange::rate_limit::RateLimitError) -> Self {
        Errors::RateLimitError(err)
    }
}

/// Coinbase API error response
#[derive(Debug, Clone, Deserialize)]
pub struct ErrorResponse {
    /// Error message
    pub message: String,
}

/// API errors that can be returned by the Coinbase Exchange API
#[derive(Debug, Clone)]
pub enum ApiError {
    /// Bad Request - Invalid request format
    BadRequest { msg: String },

    /// Unauthorized - Invalid API Key
    Unauthorized { msg: String },

    /// Forbidden - You do not have access to the requested resource
    Forbidden { msg: String },

    /// Not Found
    NotFound { msg: String },

    /// Too Many Requests - Rate limit exceeded
    TooManyRequests { msg: String },

    /// Internal Server Error
    InternalServerError { msg: String },

    /// Invalid Price
    InvalidPrice { msg: String },

    /// Insufficient Funds
    InsufficientFunds { msg: String },

    /// Invalid Order Size
    InvalidOrderSize { msg: String },

    /// Invalid Product
    InvalidProduct { msg: String },

    /// Order Not Found
    OrderNotFound { msg: String },

    /// Order Already Cancelled
    OrderAlreadyCancelled { msg: String },

    /// Order Already Filled
    OrderAlreadyFilled { msg: String },

    /// Post Only Order Would Trade
    PostOnlyOrderWouldTrade { msg: String },

    /// Timestamp Invalid
    TimestampInvalid { msg: String },

    /// Signature Invalid
    SignatureInvalid { msg: String },

    /// Passphrase Invalid
    PassphraseInvalid { msg: String },

    /// Profile Not Found
    ProfileNotFound { msg: String },

    /// Account Not Found
    AccountNotFound { msg: String },

    /// Unknown API error with code and message
    UnknownApiError { code: Option<i32>, msg: String },
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::BadRequest { msg } => write!(f, "Bad Request: {msg}"),
            ApiError::Unauthorized { msg } => write!(f, "Unauthorized: {msg}"),
            ApiError::Forbidden { msg } => write!(f, "Forbidden: {msg}"),
            ApiError::NotFound { msg } => write!(f, "Not Found: {msg}"),
            ApiError::TooManyRequests { msg } => write!(f, "Too Many Requests: {msg}"),
            ApiError::InternalServerError { msg } => write!(f, "Internal Server Error: {msg}"),
            ApiError::InvalidPrice { msg } => write!(f, "Invalid Price: {msg}"),
            ApiError::InsufficientFunds { msg } => write!(f, "Insufficient Funds: {msg}"),
            ApiError::InvalidOrderSize { msg } => write!(f, "Invalid Order Size: {msg}"),
            ApiError::InvalidProduct { msg } => write!(f, "Invalid Product: {msg}"),
            ApiError::OrderNotFound { msg } => write!(f, "Order Not Found: {msg}"),
            ApiError::OrderAlreadyCancelled { msg } => {
                write!(f, "Order Already Cancelled: {msg}")
            }
            ApiError::OrderAlreadyFilled { msg } => write!(f, "Order Already Filled: {msg}"),
            ApiError::PostOnlyOrderWouldTrade { msg } => {
                write!(f, "Post Only Order Would Trade: {msg}")
            }
            ApiError::TimestampInvalid { msg } => write!(f, "Timestamp Invalid: {msg}"),
            ApiError::SignatureInvalid { msg } => write!(f, "Signature Invalid: {msg}"),
            ApiError::PassphraseInvalid { msg } => write!(f, "Passphrase Invalid: {msg}"),
            ApiError::ProfileNotFound { msg } => write!(f, "Profile Not Found: {msg}"),
            ApiError::AccountNotFound { msg } => write!(f, "Account Not Found: {msg}"),
            ApiError::UnknownApiError { code, msg } => {
                if let Some(code) = code {
                    write!(f, "Unknown API Error {code}: {msg}")
                } else {
                    write!(f, "Unknown API Error: {msg}")
                }
            }
        }
    }
}

impl std::error::Error for ApiError {}

impl From<ErrorResponse> for ApiError {
    fn from(err: ErrorResponse) -> Self {
        // Map common error messages to specific error types
        match err.message.as_str() {
            msg if msg.contains("Invalid price") => ApiError::InvalidPrice { msg: err.message },
            msg if msg.contains("Insufficient funds") => {
                ApiError::InsufficientFunds { msg: err.message }
            }
            msg if msg.contains("Invalid order size") => {
                ApiError::InvalidOrderSize { msg: err.message }
            }
            msg if msg.contains("Invalid product") => ApiError::InvalidProduct { msg: err.message },
            msg if msg.contains("Order not found") => ApiError::OrderNotFound { msg: err.message },
            msg if msg.contains("already cancelled") => {
                ApiError::OrderAlreadyCancelled { msg: err.message }
            }
            msg if msg.contains("already filled") => {
                ApiError::OrderAlreadyFilled { msg: err.message }
            }
            msg if msg.contains("Post only") => {
                ApiError::PostOnlyOrderWouldTrade { msg: err.message }
            }
            msg if msg.contains("Timestamp") => ApiError::TimestampInvalid { msg: err.message },
            msg if msg.contains("Signature") => ApiError::SignatureInvalid { msg: err.message },
            msg if msg.contains("Passphrase") => ApiError::PassphraseInvalid { msg: err.message },
            msg if msg.contains("Profile") => ApiError::ProfileNotFound { msg: err.message },
            msg if msg.contains("Account") => ApiError::AccountNotFound { msg: err.message },
            _ => ApiError::UnknownApiError {
                code: None,
                msg: err.message,
            },
        }
    }
}
