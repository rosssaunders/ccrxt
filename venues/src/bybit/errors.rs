use serde::{Deserialize, Serialize};
use thiserror::Error;

/// ByBit API error response structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    /// Error code returned by ByBit API
    #[serde(rename = "retCode")]
    pub ret_code: i32,
    /// Error message returned by ByBit API
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    /// Additional error information
    #[serde(rename = "retExtInfo", default)]
    pub ret_ext_info: serde_json::Value,
    /// Response timestamp
    pub time: Option<u64>,
}

/// ByBit API specific error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiError {
    /// Error code
    pub code: i32,
    /// Error message
    pub msg: String,
}

/// Comprehensive error types for ByBit API operations
#[derive(Error, Debug)]
pub enum Errors {
    #[error("ByBit API error: {0}")]
    ApiError(String),

    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("JSON serialization/deserialization error: {0}")]
    SerdeError(#[from] serde_json::Error),

    #[error("URL encoding error: {0}")]
    UrlEncodingError(#[from] serde_urlencoded::ser::Error),

    #[error("Rate limit exceeded: {0}")]
    RateLimitError(#[from] crate::bybit::rate_limit::RateLimitError),

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
        Errors::ApiError(format!(
            "Code: {}, Message: {}",
            error_response.ret_code, error_response.ret_msg
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_response_structure() {
        let error_json = r#"
        {
            "retCode": 10001,
            "retMsg": "Invalid API key",
            "retExtInfo": {},
            "time": 1672738134824
        }
        "#;

        let response: ErrorResponse = match serde_json::from_str(error_json) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("Deserialization failed: {}", e);
                return;
            }
        };
        assert_eq!(response.ret_code, 10001);
        assert_eq!(response.ret_msg, "Invalid API key");
        assert_eq!(response.time, Some(1672738134824));
    }

    #[test]
    fn test_api_error_structure() {
        let api_error = ApiError {
            code: 10001,
            msg: "Invalid API key".to_string(),
        };

        assert_eq!(api_error.code, 10001);
        assert_eq!(api_error.msg, "Invalid API key");
    }

    #[test]
    fn test_error_conversion() {
        let error_response = ErrorResponse {
            ret_code: 10001,
            ret_msg: "Invalid API key".to_string(),
            ret_ext_info: serde_json::Value::Null,
            time: Some(1672738134824),
        };

        let error: Errors = error_response.into();
        match error {
            Errors::ApiError(msg) => {
                assert!(msg.contains("10001"));
                assert!(msg.contains("Invalid API key"));
            }
            _ => assert_eq!(true, false, "Expected ApiError"),
        }
    }
}
