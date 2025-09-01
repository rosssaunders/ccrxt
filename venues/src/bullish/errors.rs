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

    #[error("Transport error: {0}")]
    Transport(TransportDiagnostics),
}

/// Detailed transport diagnostics preserved from the underlying HTTP client abstraction.
/// This allows upstream callers (e.g. integration tests or higher-level retry logic) to
/// introspect the root cause without re‑exposing the raw HTTP client implementation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransportDiagnostics {
    /// High-level kind/classification of the transport error
    pub kind: TransportErrorKind,
    /// Human readable message (never empty)
    pub message: String,
    /// Optional HTTP status (only for Http kind)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<u16>,
    /// Optional response body (only for Http kind when available)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
}

impl fmt::Display for TransportDiagnostics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.kind, self.status, self.body.as_ref()) {
            (TransportErrorKind::Http, Some(status), Some(body)) => {
                write!(f, "HTTP status {status}: {} (body: {})", self.message, body)
            }
            (TransportErrorKind::Http, Some(status), None) => {
                write!(f, "HTTP status {status}: {}", self.message)
            }
            _ => write!(f, "{}: {}", self.kind, self.message),
        }
    }
}

/// Classification kinds for transport errors
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TransportErrorKind {
    Network,
    Timeout,
    InvalidUrl,
    Decode,
    Http,
    Unknown,
}

impl fmt::Display for TransportErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            TransportErrorKind::Network => "Network",
            TransportErrorKind::Timeout => "Timeout",
            TransportErrorKind::InvalidUrl => "InvalidUrl",
            TransportErrorKind::Decode => "Decode",
            TransportErrorKind::Http => "Http",
            TransportErrorKind::Unknown => "Unknown",
        };
        write!(f, "{s}")
    }
}

// Map rest::HttpError into a structured diagnostics variant
impl From<rest::HttpError> for Errors {
    fn from(err: rest::HttpError) -> Self {
        use rest::HttpError as H;
        let diag = match err {
            H::Network(e) => TransportDiagnostics {
                kind: TransportErrorKind::Network,
                message: e,
                status: None,
                body: None,
            },
            H::Timeout => TransportDiagnostics {
                kind: TransportErrorKind::Timeout,
                message: "request timed out".to_string(),
                status: None,
                body: None,
            },
            H::InvalidUrl(u) => TransportDiagnostics {
                kind: TransportErrorKind::InvalidUrl,
                message: u,
                status: None,
                body: None,
            },
            H::Decode(e) => TransportDiagnostics {
                kind: TransportErrorKind::Decode,
                message: e,
                status: None,
                body: None,
            },
            H::Http { status, body } => TransportDiagnostics {
                kind: TransportErrorKind::Http,
                message: format!("HTTP error status {status}"),
                status: Some(status),
                body: Some(body),
            },
            H::Unknown(e) => TransportDiagnostics {
                kind: TransportErrorKind::Unknown,
                message: e,
                status: None,
                body: None,
            },
            #[allow(unreachable_patterns)]
            _ => TransportDiagnostics {
                kind: TransportErrorKind::Unknown,
                message: "Unclassified HTTP error".to_string(),
                status: None,
                body: None,
            },
        };
        Errors::Transport(diag)
    }
}

impl Errors {
    /// Returns transport diagnostics if this error originated from the HTTP layer.
    pub fn transport_diagnostics(&self) -> Option<&TransportDiagnostics> {
        match self {
            Errors::Transport(d) => Some(d),
            _ => None,
        }
    }
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

    #[test]
    fn test_transport_diagnostics_variant() {
        // Simulate mapping from underlying rest::HttpError
        let http_err = rest::HttpError::Http {
            status: 503,
            body: "Service Unavailable".to_string(),
        };
        let err: Errors = http_err.into();
        match err.transport_diagnostics() {
            Some(diag) => {
                assert!(matches!(diag.kind, TransportErrorKind::Http));
                assert_eq!(diag.status, Some(503));
                assert_eq!(diag.body.as_deref(), Some("Service Unavailable"));
            }
            None => assert!(false, "expected transport diagnostics"),
        }
    }
}
