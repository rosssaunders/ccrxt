//! Error types for Bullish Exchange API

use std::fmt;

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Comprehensive error type for Bullish API operations
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum Errors {
    #[error("API Error: {0}")]
    ApiError(#[from] ApiError),

    #[error("HTTP Error: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("JSON Parsing Error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Rate Limit Error: {0}")]
    RateLimitError(String),

    #[error("Authentication Error: {0}")]
    AuthenticationError(String),

    #[error("Invalid API Key")]
    InvalidApiKey(),

    #[error("Generic Error: {0}")]
    Error(String),
}

/// API error from Bullish exchange
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiError {
    /// Error code
    pub code: String,
    /// Error message
    pub message: String,
    /// Additional error details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "API Error {}: {}", self.code, self.message)
    }
}

impl std::error::Error for ApiError {}

/// Error response structure from Bullish API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    /// Error details
    pub error: ApiError,
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_api_error_structure() {
        let error_json = json!({
            "code": "INVALID_SYMBOL",
            "message": "Invalid symbol",
            "details": "Symbol 'INVALID' is not supported"
        });

        let api_error: ApiError = serde_json::from_value(error_json).unwrap();
        assert_eq!(api_error.code, "INVALID_SYMBOL");
        assert_eq!(api_error.message, "Invalid symbol");
        assert_eq!(
            api_error.details,
            Some("Symbol 'INVALID' is not supported".to_string())
        );
    }

    #[test]
    fn test_error_response_structure() {
        let response_json = json!({
            "error": {
                "code": "INSUFFICIENT_BALANCE",
                "message": "Insufficient balance for trade"
            }
        });

        let error_response: ErrorResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(error_response.error.code, "INSUFFICIENT_BALANCE");
        assert_eq!(
            error_response.error.message,
            "Insufficient balance for trade"
        );
        assert_eq!(error_response.error.details, None);
    }

    #[test]
    fn test_error_display() {
        let api_error = ApiError {
            code: "TEST_ERROR".to_string(),
            message: "Test error message".to_string(),
            details: None,
        };

        let error_msg = format!("{}", api_error);
        assert_eq!(error_msg, "API Error TEST_ERROR: Test error message");
    }

    #[test]
    fn test_errors_enum_variants() {
        let auth_error = Errors::AuthenticationError("Invalid token".to_string());
        assert!(auth_error.to_string().contains("Authentication Error"));

        let rate_limit_error = Errors::RateLimitError("Rate limit exceeded".to_string());
        assert!(rate_limit_error.to_string().contains("Rate Limit Error"));

        let invalid_key_error = Errors::InvalidApiKey();
        assert!(invalid_key_error.to_string().contains("Invalid API Key"));
    }
}
