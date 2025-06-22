use std::fmt;

use serde::Deserialize;
use thiserror::Error;

/// Represents all possible errors that can occur when interacting with the OKX API
#[derive(Debug)]
pub enum Errors {
    /// Invalid API key or signature
    InvalidApiKey(),

    /// Http error occurred while making a request
    /// This variant is used to represent errors that are not specific to the OKX API,
    /// such as network issues or HTTP errors.
    /// It can be used to wrap any error that occurs during the request process.
    /// This variant is not used for errors returned by the OKX API itself.
    HttpError(reqwest::Error),

    /// An error returned by the OKX API
    ApiError(ApiError),

    /// A general error with a descriptive message
    Error(String),
}

impl fmt::Display for Errors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Errors::InvalidApiKey() => write!(f, "Invalid API key or signature"),
            Errors::HttpError(err) => write!(f, "HTTP error: {err}"),
            Errors::ApiError(err) => write!(f, "API error: {err}"),
            Errors::Error(msg) => write!(f, "Error: {msg}"),
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
        Errors::Error(format!("JSON serialization error: {err}"))
    }
}

impl From<crate::okx::rate_limit::RateLimitError> for Errors {
    fn from(err: crate::okx::rate_limit::RateLimitError) -> Self {
        Errors::Error(format!("Rate limit error: {err}"))
    }
}

/// Represents an error response from the OKX API.
///
/// This is public as it is used by API responses.
#[derive(Debug, Clone, Deserialize)]
pub struct ErrorResponse {
    pub code: String,
    pub msg: String,
}

/// OKX API error codes as documented in their REST API specification
/// See: https://www.okx.com/docs-v5/en/#overview-error-code
#[derive(Error, Debug, Clone, Deserialize)]
pub enum ApiError {
    // Success
    #[error("Success")]
    Success,

    // Parameter Errors
    #[error("Invalid parameter")]
    InvalidParameter,

    #[error("Missing parameter")]
    MissingParameter,

    #[error("Invalid instrument ID")]
    InvalidInstrumentId,

    #[error("Invalid order type")]
    InvalidOrderType,

    #[error("Invalid order side")]
    InvalidOrderSide,

    #[error("Invalid order size")]
    InvalidOrderSize,

    #[error("Invalid price")]
    InvalidPrice,

    // Account Errors
    #[error("Insufficient balance")]
    InsufficientBalance,

    #[error("Account suspended")]
    AccountSuspended,

    #[error("Position not found")]
    PositionNotFound,

    #[error("Order not found")]
    OrderNotFound,

    // Rate Limiting
    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    // Authentication Errors
    #[error("Invalid API key")]
    InvalidApiKey,

    #[error("Invalid signature")]
    InvalidSignature,

    #[error("Invalid timestamp")]
    InvalidTimestamp,

    // System Errors
    #[error("System maintenance")]
    SystemMaintenance,

    #[error("Internal server error")]
    InternalServerError,

    /// Unmapped API error - for error codes not explicitly handled
    #[error("API error (code: {code}): {msg}")]
    UnmappedApiError { code: String, msg: String },
}

impl From<ErrorResponse> for ApiError {
    fn from(err: ErrorResponse) -> Self {
        match err.code.as_str() {
            "0" => ApiError::Success,
            "50000" => ApiError::InvalidParameter,
            "50001" => ApiError::MissingParameter,
            "50002" => ApiError::InvalidInstrumentId,
            "50004" => ApiError::InvalidOrderType,
            "50005" => ApiError::InvalidOrderSide,
            "50006" => ApiError::InvalidOrderSize,
            "50007" => ApiError::InvalidPrice,
            "50101" => ApiError::InsufficientBalance,
            "50102" => ApiError::AccountSuspended,
            "50103" => ApiError::PositionNotFound,
            "50104" => ApiError::OrderNotFound,
            "50011" => ApiError::RateLimitExceeded,
            "50100" => ApiError::InvalidApiKey,
            "50105" => ApiError::InvalidSignature,
            "50113" => ApiError::InvalidTimestamp,
            "50012" => ApiError::SystemMaintenance,
            "50013" => ApiError::InternalServerError,
            _ => ApiError::UnmappedApiError {
                code: err.code,
                msg: err.msg,
            },
        }
    }
}

#[cfg(test)]
#[allow(clippy::assertions_on_constants)]
mod tests {
    use super::*;

    #[test]
    fn test_success_error_code() {
        let error_response = ErrorResponse {
            code: "0".to_string(),
            msg: "Success".to_string(),
        };

        let api_error: ApiError = error_response.into();
        matches!(api_error, ApiError::Success);
    }

    #[test]
    fn test_invalid_parameter_error() {
        let error_response = ErrorResponse {
            code: "50000".to_string(),
            msg: "Invalid parameter".to_string(),
        };

        let api_error: ApiError = error_response.into();
        matches!(api_error, ApiError::InvalidParameter);
    }

    #[test]
    fn test_unmapped_error_code() {
        let error_response = ErrorResponse {
            code: "99999".to_string(),
            msg: "Unknown error".to_string(),
        };

        let api_error: ApiError = error_response.into();
        match api_error {
            ApiError::UnmappedApiError { code, msg } => {
                assert_eq!(code, "99999");
                assert_eq!(msg, "Unknown error");
            }
            _ => assert!(false, "Expected UnmappedApiError"),
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
