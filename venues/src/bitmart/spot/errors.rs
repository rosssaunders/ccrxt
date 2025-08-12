use std::fmt;

use serde::Deserialize;
use thiserror::Error;

/// Represents all possible errors that can occur when interacting with the BitMart API
#[derive(Debug)]
pub enum Errors {
    /// Invalid API key or signature
    InvalidApiKey(),

    /// Network error occurred while making a request
    /// This variant is used to represent errors that are not specific to the BitMart API,
    /// such as network issues or HTTP errors.
    /// It can be used to wrap any error that occurs during the request process.
    /// This variant is not used for errors returned by the BitMart API itself.
    NetworkError(String),

    /// An error returned by the BitMart API
    ApiError(ApiError),

    /// A general error with a descriptive message
    Error(String),
}

impl fmt::Display for Errors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Errors::InvalidApiKey() => write!(f, "Invalid API key or signature"),
            Errors::NetworkError(err) => write!(f, "Network error: {err}"),
            Errors::ApiError(err) => write!(f, "API error: {err}"),
            Errors::Error(msg) => write!(f, "Error: {msg}"),
        }
    }
}

impl std::error::Error for Errors {}


impl From<serde_json::Error> for Errors {
    fn from(err: serde_json::Error) -> Self {
        Errors::Error(format!("JSON serialization error: {err}"))
    }
}

impl From<crate::bitmart::rate_limit::RateLimitError> for Errors {
    fn from(err: crate::bitmart::rate_limit::RateLimitError) -> Self {
        Errors::Error(format!("Rate limit error: {err}"))
    }
}

/// Represents an error response from the BitMart API.
///
/// This is public as it is used by API responses.
#[derive(Debug, Clone, Deserialize)]
pub struct ErrorResponse {
    pub code: i32,
    pub message: String,
    #[serde(default)]
    pub trace: String,
}

/// BitMart API error codes as documented in their REST API specification
///
/// For complete error code documentation, see the BitMart API documentation.
#[derive(Error, Debug, Clone, Deserialize)]
pub enum ApiError {
    // Success
    #[error("Success")]
    Success,

    // Authentication Errors (30000 series)
    #[error("Not found")]
    NotFound,

    #[error("Header X-BM-KEY is empty")]
    EmptyApiKey,

    #[error("Header X-BM-KEY not found")]
    ApiKeyNotFound,

    #[error("Header X-BM-KEY has frozen")]
    ApiKeyFrozen,

    #[error("Header X-BM-SIGN is empty")]
    EmptySignature,

    #[error("Header X-BM-SIGN is wrong")]
    InvalidSignature,

    #[error("Header X-BM-TIMESTAMP is empty")]
    EmptyTimestamp,

    #[error("Header X-BM-TIMESTAMP range error")]
    TimestampOutOfRange,

    #[error("Header X-BM-TIMESTAMP invalid format")]
    InvalidTimestampFormat,

    #[error("IP is forbidden")]
    IpForbidden,

    #[error("Header X-BM-KEY over expire time")]
    ApiKeyExpired,

    #[error("Header X-BM-KEY is forbidden to request it")]
    ApiKeyRequestForbidden,

    #[error("Request too many requests")]
    TooManyRequests,

    #[error("Service unavailable")]
    ServiceUnavailable,

    #[error("Service maintenance, the function is temporarily unavailable")]
    ServiceMaintenance,

    #[error(
        "Your account request is temporarily rejected due to violation of current limiting rules"
    )]
    AccountRequestRejected,

    #[error("Request Body requires JSON format")]
    InvalidJsonFormat,

    #[error("You do not have the permissions to perform this operation")]
    InsufficientPermissions,

    #[error("This endpoint has been deprecated")]
    EndpointDeprecated,

    // Trading Errors (50000 series)
    #[error("Invalid parameter")]
    InvalidParameter,

    #[error("Insufficient balance")]
    InsufficientBalance,

    #[error("Order not found")]
    OrderNotFound,

    #[error("Invalid order type")]
    InvalidOrderType,

    #[error("Invalid order side")]
    InvalidOrderSide,

    #[error("Invalid price")]
    InvalidPrice,

    #[error("Invalid size")]
    InvalidSize,

    #[error("Symbol not found")]
    SymbolNotFound,

    /// Unmapped API error - for error codes not explicitly handled
    #[error("API error (code: {code}): {message}")]
    UnmappedApiError { code: i32, message: String },
}

impl From<ErrorResponse> for ApiError {
    fn from(err: ErrorResponse) -> Self {
        match err.code {
            1000 => ApiError::Success,
            30000 => ApiError::NotFound,
            30001 => ApiError::EmptyApiKey,
            30002 => ApiError::ApiKeyNotFound,
            30003 => ApiError::ApiKeyFrozen,
            30004 => ApiError::EmptySignature,
            30005 => ApiError::InvalidSignature,
            30006 => ApiError::EmptyTimestamp,
            30007 => ApiError::TimestampOutOfRange,
            30008 => ApiError::InvalidTimestampFormat,
            30010 => ApiError::IpForbidden,
            30011 => ApiError::ApiKeyExpired,
            30012 => ApiError::ApiKeyRequestForbidden,
            30013 => ApiError::TooManyRequests,
            30014 => ApiError::ServiceUnavailable,
            30016 => ApiError::ServiceMaintenance,
            30017 => ApiError::AccountRequestRejected,
            30018 => ApiError::InvalidJsonFormat,
            30019 => ApiError::InsufficientPermissions,
            30031 => ApiError::EndpointDeprecated,
            50000 => ApiError::InvalidParameter,
            50101 => ApiError::InsufficientBalance,
            50104 => ApiError::OrderNotFound,
            50004 => ApiError::InvalidOrderType,
            50005 => ApiError::InvalidOrderSide,
            50007 => ApiError::InvalidPrice,
            50006 => ApiError::InvalidSize,
            50002 => ApiError::SymbolNotFound,
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
    fn test_success_error_code() {
        let error_response = ErrorResponse {
            code: 1000,
            message: "OK".to_string(),
            trace: "test-trace".to_string(),
        };

        let api_error: ApiError = error_response.into();
        matches!(api_error, ApiError::Success);
    }

    #[test]
    fn test_empty_api_key_error() {
        let error_response = ErrorResponse {
            code: 30001,
            message: "Header X-BM-KEY is empty".to_string(),
            trace: "test-trace".to_string(),
        };

        let api_error: ApiError = error_response.into();
        matches!(api_error, ApiError::EmptyApiKey);
    }

    #[test]
    fn test_unmapped_error_code() {
        let error_response = ErrorResponse {
            code: 99999,
            message: "Unknown error".to_string(),
            trace: "test-trace".to_string(),
        };

        let api_error: ApiError = error_response.into();
        match api_error {
            ApiError::UnmappedApiError { code, message } => {
                assert_eq!(code, 99999);
                assert_eq!(message, "Unknown error");
            }
            _ => assert_eq!(true, false, "Expected UnmappedApiError"),
        }
    }

    #[test]
    fn test_errors_enum_display() {
        let general_error = Errors::Error("Test error".to_string());
        let error_string = format!("{}", general_error);
        assert_eq!(error_string, "Error: Test error");

        let invalid_key_error = Errors::InvalidApiKey();
        let error_string = format!("{}", invalid_key_error);
        assert_eq!(error_string, "Invalid API key or signature");
    }
}
