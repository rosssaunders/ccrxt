use serde::Deserialize;
use std::fmt;
use thiserror::Error;

/// Represents all possible errors that can occur when interacting with the Deribit API
#[derive(Debug)]
pub enum Errors {
    /// Http error occurred while making a request
    /// This variant is used to represent errors that are not specific to the Deribit API,
    /// such as network issues or HTTP errors.
    HttpError(reqwest::Error),

    /// An error returned by the Deribit API
    ApiError(ApiError),

    /// A general error with a descriptive message
    Error(String),
}

impl fmt::Display for Errors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
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

/// Deribit API error codes as documented in their JSON-RPC API specification
#[derive(Error, Debug, Clone)]
pub enum ApiError {
    #[error("Invalid request")]
    InvalidRequest,

    #[error("Method not found")]
    MethodNotFound,

    #[error("Invalid parameters")]
    InvalidParams,

    #[error("Internal error")]
    InternalError,

    #[error("Parse error")]
    ParseError,

    #[error("Authentication required")]
    AuthenticationRequired,

    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    /// Unmapped API error - for error codes not explicitly handled
    #[error("API error (code: {code}): {message}")]
    UnmappedApiError { code: i32, message: String },
}

impl From<ErrorResponse> for ApiError {
    fn from(err: ErrorResponse) -> Self {
        match err.code {
            -32600 => ApiError::InvalidRequest,
            -32601 => ApiError::MethodNotFound,
            -32602 => ApiError::InvalidParams,
            -32603 => ApiError::InternalError,
            -32700 => ApiError::ParseError,
            10000 => ApiError::AuthenticationRequired,
            10001 => ApiError::InvalidCredentials,
            10009 => ApiError::RateLimitExceeded,
            _ => ApiError::UnmappedApiError {
                code: err.code,
                message: err.message,
            },
        }
    }
}

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

    #[test]
    fn test_errors_enum_display() {
        let general_error = Errors::Error("Test error".to_string());
        let error_string = format!("{}", general_error);
        assert_eq!(error_string, "Error: Test error");
    }
}