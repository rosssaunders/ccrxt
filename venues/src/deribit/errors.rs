use std::fmt;

use serde::Deserialize;
use thiserror::Error;

/// Represents all possible errors that can occur when interacting with the Deribit API
#[derive(Debug)]
pub enum Errors {
    /// Invalid API key or signature
    InvalidApiKey(),

    /// Http error occurred while making a request
    /// This variant is used to represent errors that are not specific to the Deribit API,
    /// such as network issues or HTTP errors.
    HttpError(reqwest::Error),

    /// An error returned by the Deribit API
    ApiError(ApiError),

    /// A general error with a descriptive message
    Error(String),

    /// Error related to rate limiting
    RateLimitError(crate::deribit::rate_limit::RateLimitError),

    /// Error during JSON serialization or deserialization
    SerdeJsonError(serde_json::Error),
}

impl fmt::Display for Errors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Errors::InvalidApiKey() => write!(f, "Invalid API key or signature"),
            Errors::HttpError(err) => write!(f, "Http error: {}", err),
            Errors::ApiError(err) => write!(f, "Deribit API error: {}", err),
            Errors::Error(msg) => write!(f, "General error: {}", msg),
            Errors::RateLimitError(err) => write!(f, "Rate limit error: {}", err),
            Errors::SerdeJsonError(err) => write!(f, "JSON error: {}", err),
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
        Errors::SerdeJsonError(err)
    }
}

impl From<crate::deribit::rate_limit::RateLimitError> for Errors {
    fn from(err: crate::deribit::rate_limit::RateLimitError) -> Self {
        Errors::RateLimitError(err)
    }
}

/// Represents an error response from the Deribit API.
///
/// Deribit uses JSON-RPC 2.0 format for errors.
#[derive(Debug, Clone, Deserialize)]
pub struct ErrorResponse {
    /// Error code
    pub code: i32,
    /// Error message
    pub message: String,
    /// Optional additional data
    pub data: Option<serde_json::Value>,
}

/// Deribit API error codes as documented in their API specification
#[derive(Error, Debug, Clone)]
pub enum ApiError {
    #[error("Success")]
    Success, // Typically code 0, often not an error but included for completeness if API returns it

    #[error("Insufficient funds")]
    InsufficientFunds, // e.g., -32602 or a specific business logic code

    #[error("Invalid destination address")]
    InvalidDestination,

    #[error("Transfer limit exceeded")]
    TransferLimitExceeded,

    #[error("Invalid currency")]
    InvalidCurrency,

    #[error("Invalid amount")]
    InvalidAmount,

    #[error("Unauthorized")]
    Unauthorized, // e.g., -32000

    // JSON-RPC 2.0 Standard Errors
    #[error("Invalid request")]
    InvalidRequest, // -32600

    #[error("Method not found")]
    MethodNotFound, // -32601

    #[error("Invalid parameters")]
    InvalidParams, // -32602

    #[error("Internal error")]
    InternalError, // -32603

    #[error("Parse error")]
    ParseError, // -32700

    // Deribit Specific Errors (can overlap with general JSON-RPC if codes are reused)
    #[error("Authentication required")]
    AuthenticationRequired, // e.g., 10000

    #[error("Invalid credentials")]
    InvalidCredentials, // e.g., 10001

    #[error("Rate limit exceeded")]
    RateLimitExceeded, // e.g., 10002

    #[error("Internal server error")]
    InternalServerError, // e.g., 10003, distinct from JSON-RPC's InternalError if codes differ

    /// Unmapped API error - for error codes not explicitly handled
    #[error("API error (code: {code}): {message}")]
    UnmappedApiError { code: i32, message: String },
}

impl From<ErrorResponse> for ApiError {
    fn from(err: ErrorResponse) -> Self {
        match err.code {
            0 => ApiError::Success, // Assuming 0 is success, adjust if Deribit uses a different code
            // Specific business logic codes (examples, replace with actual Deribit codes)
            1000 => ApiError::InsufficientFunds,     // Example code
            1001 => ApiError::InvalidDestination,    // Example code
            1002 => ApiError::TransferLimitExceeded, // Example code
            1003 => ApiError::InvalidCurrency,       // Example code
            1004 => ApiError::InvalidAmount,         // Example code
            1005 => ApiError::Unauthorized,          // Example code

            // JSON-RPC Standard Error Codes
            -32700 => ApiError::ParseError,
            -32600 => ApiError::InvalidRequest,
            -32601 => ApiError::MethodNotFound,
            -32602 => ApiError::InvalidParams, // Note: This might conflict if InsufficientFunds also uses -32602
            -32603 => ApiError::InternalError,

            // Deribit Specific Error Codes (examples, replace with actual Deribit codes)
            10000 => ApiError::AuthenticationRequired,
            10001 => ApiError::InvalidCredentials,
            10002 => ApiError::RateLimitExceeded,
            10003 => ApiError::InternalServerError,
            // Add more specific Deribit error codes here
            _ => ApiError::UnmappedApiError {
                code: err.code,
                message: err.message,
            },
        }
    }
}

/// Type alias for results returned by Deribit API operations
pub type RestResult<T> = Result<T, Errors>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_request_error() {
        let error_response = ErrorResponse {
            code: -32600,
            message: "Invalid Request".to_string(),
            data: None,
        };

        let api_error: ApiError = error_response.into();
        matches!(api_error, ApiError::InvalidRequest);
    }

    #[test]
    fn test_unmapped_error_code() {
        let error_response = ErrorResponse {
            code: 99999,
            message: "Unknown error".to_string(),
            data: None,
        };

        let api_error: ApiError = error_response.into();
        match api_error {
            ApiError::UnmappedApiError { code, message } => {
                assert_eq!(code, 99999);
                assert_eq!(message, "Unknown error");
            }
            _ => panic!("Expected UnmappedApiError"),
        }
    }
}
