use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

/// Represents all possible errors that can occur when interacting with the Deribit API
#[derive(Debug)]
pub enum Errors {
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

/// Deribit JSON-RPC error response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcError {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

/// Deribit API error codes as documented in their API specification
#[derive(Error, Debug, Clone)]
pub enum ApiError {
    #[error("Authentication failed")]
    AuthenticationFailed,

    #[error("Authorization failed")]
    AuthorizationFailed,

    #[error("Invalid parameters: {msg}")]
    InvalidParameters { msg: String },

    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    #[error("Internal server error")]
    InternalServerError,

    #[error("Service unavailable")]
    ServiceUnavailable,

    /// JSON-RPC specific error
    #[error("JSON-RPC error (code: {code}): {message}")]
    JsonRpcError { code: i32, message: String },
}

impl From<JsonRpcError> for ApiError {
    fn from(err: JsonRpcError) -> Self {
        match err.code {
            -32700 => ApiError::InvalidParameters { msg: "Parse error".to_string() },
            -32600 => ApiError::InvalidParameters { msg: "Invalid Request".to_string() },
            -32601 => ApiError::InvalidParameters { msg: "Method not found".to_string() },
            -32602 => ApiError::InvalidParameters { msg: "Invalid params".to_string() },
            -32603 => ApiError::InternalServerError,
            10000 => ApiError::AuthenticationFailed,
            10001 => ApiError::AuthorizationFailed,
            10002 => ApiError::InvalidParameters { msg: err.message.clone() },
            10029 => ApiError::RateLimitExceeded,
            11029 => ApiError::ServiceUnavailable,
            _ => ApiError::JsonRpcError {
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
    fn test_json_rpc_error_conversion() {
        let json_rpc_error = JsonRpcError {
            code: -32602,
            message: "Invalid params".to_string(),
            data: None,
        };

        let api_error: ApiError = json_rpc_error.into();
        match api_error {
            ApiError::InvalidParameters { msg } => {
                assert_eq!(msg, "Invalid params");
            }
            _ => panic!("Expected InvalidParameters"),
        }
    }

    #[test]
    fn test_errors_enum_display() {
        let general_error = Errors::Error("Test error".to_string());
        let error_string = format!("{}", general_error);
        assert_eq!(error_string, "Error: Test error");
    }

    #[test]
    fn test_rate_limit_error_conversion() {
        let json_rpc_error = JsonRpcError {
            code: 10029,
            message: "Rate limit exceeded".to_string(),
            data: None,
        };

        let api_error: ApiError = json_rpc_error.into();
        matches!(api_error, ApiError::RateLimitExceeded);
    }
}