use std::fmt;

use serde::Deserialize;
use thiserror::Error;

/// Result type for KuCoin API operations
pub type Result<T> = std::result::Result<T, KucoinError>;

/// Represents all possible errors that can occur when interacting with the KuCoin API
#[derive(Debug, Error)]
pub enum KucoinError {
    /// Invalid API key or signature
    #[error("Invalid API key or signature")]
    InvalidApiKey,

    /// Network error occurred while making a request
    #[error("Network error: {0}")]
    NetworkError(String),

    /// An error returned by the KuCoin API
    #[error("API error: {0}")]
    ApiError(#[from] ApiError),

    /// A general error with a descriptive message
    #[error("Error: {0}")]
    Error(String),

    /// JSON serialization/deserialization error
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    /// URL parsing error
    #[error("URL error: {0}")]
    UrlError(#[from] url::ParseError),
}

/// Represents an error response from the KuCoin API.
#[derive(Debug, Clone, Deserialize)]
pub struct ErrorResponse {
    pub code: String,
    pub msg: String,
}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "KuCoin API Error {}: {}", self.code, self.msg)
    }
}

/// Specific API error with code and message
#[derive(Debug, Error)]
pub enum ApiError {
    /// Authentication errors (40x codes)
    #[error("Authentication failed: {message} (code: {code})")]
    AuthenticationError { code: String, message: String },

    /// Bad request errors (400xxx codes)
    #[error("Bad request: {message} (code: {code})")]
    BadRequest { code: String, message: String },

    /// Rate limit exceeded (429xxx codes)
    #[error("Rate limit exceeded: {message} (code: {code})")]
    RateLimitExceeded { code: String, message: String },

    /// Server errors (500xxx codes)
    #[error("Server error: {message} (code: {code})")]
    ServerError { code: String, message: String },

    /// Other API errors
    #[error("API error: {message} (code: {code})")]
    Other { code: String, message: String },

    /// HTTP-related errors
    #[error("HTTP error: {0}")]
    Http(String),

    /// JSON parsing errors
    #[error("JSON parsing error: {0}")]
    JsonParsing(String),
}

impl From<ErrorResponse> for ApiError {
    fn from(error: ErrorResponse) -> Self {
        match error.code.as_str() {
            // Authentication errors
            "400001" => ApiError::AuthenticationError {
                code: error.code,
                message: error.msg,
            },
            "400002" => ApiError::AuthenticationError {
                code: error.code,
                message: error.msg,
            },
            "400003" => ApiError::AuthenticationError {
                code: error.code,
                message: error.msg,
            },
            "400004" => ApiError::AuthenticationError {
                code: error.code,
                message: error.msg,
            },
            "400005" => ApiError::AuthenticationError {
                code: error.code,
                message: error.msg,
            },
            "400006" => ApiError::AuthenticationError {
                code: error.code,
                message: error.msg,
            },

            // Bad request errors
            code if code.starts_with("400") => ApiError::BadRequest {
                code: error.code,
                message: error.msg,
            },

            // Rate limiting
            code if code.starts_with("429") => ApiError::RateLimitExceeded {
                code: error.code,
                message: error.msg,
            },

            // Server errors
            code if code.starts_with("500") => ApiError::ServerError {
                code: error.code,
                message: error.msg,
            },

            // Other errors
            _ => ApiError::Other {
                code: error.code,
                message: error.msg,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_response_display() {
        let error = ErrorResponse {
            code: "400001".to_string(),
            msg: "Invalid API key".to_string(),
        };
        assert_eq!(
            format!("{}", error),
            "KuCoin API Error 400001: Invalid API key"
        );
    }

    #[test]
    fn test_api_error_from_error_response_authentication() {
        let test_cases = vec![
            ("400001", "Invalid API key"),
            ("400002", "Invalid signature"),
            ("400003", "Invalid timestamp"),
            ("400004", "Invalid KC-API-KEY"),
            ("400005", "Invalid KC-API-PASSPHRASE"),
            ("400006", "Invalid KC-API-KEY-VERSION"),
        ];

        for (code, msg) in test_cases {
            let error_response = ErrorResponse {
                code: code.to_string(),
                msg: msg.to_string(),
            };

            let api_error = ApiError::from(error_response);

            match api_error {
                ApiError::AuthenticationError {
                    code: err_code,
                    message,
                } => {
                    assert_eq!(err_code, code);
                    assert_eq!(message, msg);
                }
                _ => panic!("Expected AuthenticationError for code {}", code),
            }
        }
    }

    #[test]
    fn test_api_error_from_error_response_bad_request() {
        let error_response = ErrorResponse {
            code: "400100".to_string(),
            msg: "Invalid parameter".to_string(),
        };

        let api_error = ApiError::from(error_response);

        match api_error {
            ApiError::BadRequest { code, message } => {
                assert_eq!(code, "400100");
                assert_eq!(message, "Invalid parameter");
            }
            _ => panic!("Expected BadRequest"),
        }
    }

    #[test]
    fn test_api_error_from_error_response_rate_limit() {
        let error_response = ErrorResponse {
            code: "429000".to_string(),
            msg: "Too many requests".to_string(),
        };

        let api_error = ApiError::from(error_response);

        match api_error {
            ApiError::RateLimitExceeded { code, message } => {
                assert_eq!(code, "429000");
                assert_eq!(message, "Too many requests");
            }
            _ => panic!("Expected RateLimitExceeded"),
        }
    }

    #[test]
    fn test_api_error_from_error_response_server_error() {
        let error_response = ErrorResponse {
            code: "500000".to_string(),
            msg: "Internal server error".to_string(),
        };

        let api_error = ApiError::from(error_response);

        match api_error {
            ApiError::ServerError { code, message } => {
                assert_eq!(code, "500000");
                assert_eq!(message, "Internal server error");
            }
            _ => panic!("Expected ServerError"),
        }
    }

    #[test]
    fn test_api_error_from_error_response_other() {
        let error_response = ErrorResponse {
            code: "200001".to_string(),
            msg: "Unknown error".to_string(),
        };

        let api_error = ApiError::from(error_response);

        match api_error {
            ApiError::Other { code, message } => {
                assert_eq!(code, "200001");
                assert_eq!(message, "Unknown error");
            }
            _ => panic!("Expected Other error"),
        }
    }

    #[test]
    fn test_kucoin_error_display() {
        let error = KucoinError::InvalidApiKey;
        assert_eq!(format!("{}", error), "Invalid API key or signature");

        let error = KucoinError::Error("Custom error".to_string());
        assert_eq!(format!("{}", error), "Error: Custom error");
    }

    #[test]
    fn test_api_error_display() {
        let error = ApiError::AuthenticationError {
            code: "400001".to_string(),
            message: "Invalid API key".to_string(),
        };
        assert_eq!(
            format!("{}", error),
            "Authentication failed: Invalid API key (code: 400001)"
        );

        let error = ApiError::BadRequest {
            code: "400100".to_string(),
            message: "Invalid parameter".to_string(),
        };
        assert_eq!(
            format!("{}", error),
            "Bad request: Invalid parameter (code: 400100)"
        );

        let error = ApiError::RateLimitExceeded {
            code: "429000".to_string(),
            message: "Too many requests".to_string(),
        };
        assert_eq!(
            format!("{}", error),
            "Rate limit exceeded: Too many requests (code: 429000)"
        );

        let error = ApiError::ServerError {
            code: "500000".to_string(),
            message: "Internal server error".to_string(),
        };
        assert_eq!(
            format!("{}", error),
            "Server error: Internal server error (code: 500000)"
        );

        let error = ApiError::Http("Connection timeout".to_string());
        assert_eq!(format!("{}", error), "HTTP error: Connection timeout");

        let error = ApiError::JsonParsing("Invalid JSON".to_string());
        assert_eq!(format!("{}", error), "JSON parsing error: Invalid JSON");
    }

    #[test]
    fn test_error_response_deserialization() {
        let json = r#"{"code":"400001","msg":"Invalid API key"}"#;
        let error: ErrorResponse = serde_json::from_str(json).unwrap();
        assert_eq!(error.code, "400001");
        assert_eq!(error.msg, "Invalid API key");
    }
}
