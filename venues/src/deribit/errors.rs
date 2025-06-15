use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Deribit JSON-RPC error response structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcError {
    /// Error code
    pub code: i32,
    /// Error message
    pub message: String,
    /// Additional error data
    #[serde(default)]
    pub data: Option<serde_json::Value>,
}

/// Deribit JSON-RPC response structure for errors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    /// JSON-RPC version (should be "2.0")
    pub jsonrpc: String,
    /// Request ID
    pub id: Option<serde_json::Value>,
    /// Error information
    pub error: JsonRpcError,
}

/// Comprehensive error types for Deribit API operations
#[derive(Error, Debug)]
pub enum Errors {
    #[error("Deribit API error: {code} - {message}")]
    ApiError { code: i32, message: String },

    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("JSON serialization/deserialization error: {0}")]
    SerdeError(#[from] serde_json::Error),

    #[error("URL encoding error: {0}")]
    UrlEncodingError(#[from] serde_urlencoded::ser::Error),

    #[error("Rate limit exceeded: {0}")]
    RateLimitError(#[from] crate::deribit::rate_limit::RateLimitError),

    #[error("Authentication error: {0}")]
    AuthError(String),

    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    #[error("Network timeout")]
    Timeout,

    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl From<ErrorResponse> for Errors {
    fn from(error_response: ErrorResponse) -> Self {
        Errors::ApiError {
            code: error_response.error.code,
            message: error_response.error.message,
        }
    }
}

impl From<JsonRpcError> for Errors {
    fn from(error: JsonRpcError) -> Self {
        Errors::ApiError {
            code: error.code,
            message: error.message,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_rpc_error_structure() {
        let error_json = r#"
        {
            "jsonrpc": "2.0",
            "id": 1,
            "error": {
                "code": 10001,
                "message": "Invalid API key"
            }
        }
        "#;

        let error: ErrorResponse = serde_json::from_str(error_json).unwrap();
        assert_eq!(error.jsonrpc, "2.0");
        assert_eq!(error.error.code, 10001);
        assert_eq!(error.error.message, "Invalid API key");
    }

    #[test]
    fn test_error_conversion() {
        let error_response = ErrorResponse {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::Value::Number(1.into())),
            error: JsonRpcError {
                code: 10001,
                message: "Invalid API key".to_string(),
                data: None,
            },
        };

        let error: Errors = error_response.into();
        match error {
            Errors::ApiError { code, message } => {
                assert_eq!(code, 10001);
                assert_eq!(message, "Invalid API key");
            }
            _ => panic!("Expected ApiError"),
        }
    }

    #[test]
    fn test_json_rpc_error_with_data() {
        let error_json = r#"
        {
            "jsonrpc": "2.0",
            "id": "test",
            "error": {
                "code": 11030,
                "message": "Invalid signature",
                "data": {
                    "reason": "signature_verification_failed",
                    "details": "The provided signature does not match"
                }
            }
        }
        "#;

        let error: ErrorResponse = serde_json::from_str(error_json).unwrap();
        assert_eq!(error.jsonrpc, "2.0");
        assert_eq!(error.error.code, 11030);
        assert_eq!(error.error.message, "Invalid signature");
        assert!(error.error.data.is_some());
    }
}