use std::fmt;

use serde::Deserialize;
use thiserror::Error;

/// Result type alias for Gate.io API operations
pub type Result<T> = std::result::Result<T, GateIoError>;

/// Main error type for Gate.io API operations
#[derive(Error, Debug)]
pub enum GateIoError {
    #[error("API error: {0}")]
    Api(#[from] ApiError),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    #[error("Authentication error: {0}")]
    Authentication(String),

    #[error("Rate limit exceeded: {message}")]
    RateLimitExceeded { message: String },

    #[error("WebSocket error: {0}")]
    WebSocket(String),

    #[error("Connection error: {0}")]
    Connection(String),

    #[error("Timeout error: {0}")]
    Timeout(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

/// API-specific error information
#[derive(Debug, Clone, Deserialize)]
pub struct ApiError {
    /// Error label/code
    pub label: String,
    /// Human-readable error message
    pub message: String,
    /// Optional error details
    #[serde(default)]
    pub detail: Option<String>,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.label, self.message)?;
        if let Some(detail) = &self.detail {
            write!(f, " ({})", detail)?;
        }
        Ok(())
    }
}

impl std::error::Error for ApiError {}

/// Error response structure from Gate.io API
#[derive(Debug, Clone, Deserialize)]
pub struct ErrorResponse {
    /// Error label/code
    pub label: String,
    /// Human-readable error message  
    pub message: String,
    /// Optional error details
    #[serde(default)]
    pub detail: Option<String>,
}

impl From<ErrorResponse> for ApiError {
    fn from(err: ErrorResponse) -> Self {
        Self {
            label: err.label,
            message: err.message,
            detail: err.detail,
        }
    }
}

impl From<ErrorResponse> for GateIoError {
    fn from(err: ErrorResponse) -> Self {
        Self::Api(err.into())
    }
}

// Standard error mappings for common HTTP status codes
impl GateIoError {
    /// Create an error from HTTP status and message
    pub fn from_status(status: u16, message: String) -> Self {
        match status {
            400 => Self::InvalidParameter(message),
            401 | 403 => Self::Authentication(message),
            429 => Self::RateLimitExceeded { message },
            500..=599 => Self::Internal(message),
            _ => Self::Api(ApiError {
                label: format!("HTTP_{}", status),
                message,
                detail: None,
            }),
        }
    }

    /// Check if the error is retryable
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            Self::Http(_)
                | Self::Connection(_)
                | Self::Timeout(_)
                | Self::RateLimitExceeded { .. }
                | Self::Internal(_)
        )
    }

    /// Check if the error is due to rate limiting
    pub fn is_rate_limited(&self) -> bool {
        matches!(self, Self::RateLimitExceeded { .. })
    }

    /// Check if the error is due to authentication
    pub fn is_auth_error(&self) -> bool {
        matches!(self, Self::Authentication(_))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let error = GateIoError::from_status(400, "Bad Request".to_string());
        assert!(matches!(error, GateIoError::InvalidParameter(_)));

        let error = GateIoError::from_status(401, "Unauthorized".to_string());
        assert!(matches!(error, GateIoError::Authentication(_)));

        let error = GateIoError::from_status(429, "Rate Limited".to_string());
        assert!(matches!(error, GateIoError::RateLimitExceeded { .. }));

        let error = GateIoError::from_status(500, "Internal Error".to_string());
        assert!(matches!(error, GateIoError::Internal(_)));
    }

    #[test]
    fn test_error_properties() {
        let rate_limit_error = GateIoError::RateLimitExceeded {
            message: "Too many requests".to_string(),
        };
        assert!(rate_limit_error.is_rate_limited());
        assert!(rate_limit_error.is_retryable());

        let auth_error = GateIoError::Authentication("Invalid key".to_string());
        assert!(auth_error.is_auth_error());
        assert!(!auth_error.is_retryable());

        let param_error = GateIoError::InvalidParameter("Invalid pair".to_string());
        assert!(!param_error.is_retryable());
        assert!(!param_error.is_auth_error());
    }

    #[test]
    fn test_error_response_conversion() {
        let error_response = ErrorResponse {
            label: "INVALID_PARAM".to_string(),
            message: "Invalid parameter provided".to_string(),
            detail: Some("Currency pair not found".to_string()),
        };

        let api_error: ApiError = error_response.clone().into();
        assert_eq!(api_error.label, "INVALID_PARAM");
        assert_eq!(api_error.message, "Invalid parameter provided");
        assert_eq!(api_error.detail, Some("Currency pair not found".to_string()));

        let gateio_error: GateIoError = error_response.into();
        assert!(matches!(gateio_error, GateIoError::Api(_)));
    }

    #[test]
    fn test_api_error_display() {
        let api_error = ApiError {
            label: "TEST_ERROR".to_string(),
            message: "Test message".to_string(),
            detail: Some("Additional details".to_string()),
        };

        let display_string = format!("{}", api_error);
        assert!(display_string.contains("TEST_ERROR"));
        assert!(display_string.contains("Test message"));
        assert!(display_string.contains("Additional details"));

        let api_error_no_detail = ApiError {
            label: "TEST_ERROR".to_string(),
            message: "Test message".to_string(),
            detail: None,
        };

        let display_string = format!("{}", api_error_no_detail);
        assert!(display_string.contains("TEST_ERROR"));
        assert!(display_string.contains("Test message"));
        assert!(!display_string.contains("Additional details"));
    }
}
