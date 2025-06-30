use serde::Deserialize;
use std::fmt;
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

    #[error("Network timeout after {timeout_secs} seconds")]
    Timeout { timeout_secs: u64 },

    #[error("Invalid response format: {0}")]
    InvalidResponse(String),

    #[error("Insufficient balance: {currency} required: {required}, available: {available}")]
    InsufficientBalance {
        currency: String,
        required: String,
        available: String,
    },

    #[error("Order not found: {order_id}")]
    OrderNotFound { order_id: String },

    #[error("Currency pair not supported: {pair}")]
    UnsupportedCurrencyPair { pair: String },

    #[error("Account locked or suspended")]
    AccountLocked,

    #[error("Daily limit exceeded for {operation}")]
    DailyLimitExceeded { operation: String },

    #[error("Maintenance mode active: {message}")]
    MaintenanceMode { message: String },

    #[error("Unknown error: {0}")]
    Unknown(String),
}

/// API error response from Gate.io
#[derive(Debug, Clone, Deserialize, Error)]
pub struct ApiError {
    pub label: String,
    pub message: String,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.label, self.message)
    }
}

/// Error response wrapper from Gate.io API
#[derive(Debug, Clone, Deserialize)]
pub struct ErrorResponse {
    pub label: String,
    pub message: String,
}

impl From<ErrorResponse> for GateIoError {
    fn from(error: ErrorResponse) -> Self {
        // Map specific Gate.io error codes to our custom error types
        match error.label.as_str() {
            "INVALID_PARAM_VALUE" | "INVALID_PARAM" => {
                GateIoError::InvalidParameter(error.message)
            }
            "AUTHENTICATION_FAILED" | "INVALID_KEY" | "INVALID_SIGNATURE" => {
                GateIoError::Authentication(error.message)
            }
            "RATE_LIMIT_EXCEEDED" | "TOO_MANY_REQUESTS" => {
                GateIoError::RateLimitExceeded { message: error.message }
            }
            "INSUFFICIENT_BALANCE" => {
                // Try to parse balance details from message
                GateIoError::InsufficientBalance {
                    currency: "UNKNOWN".to_string(),
                    required: "UNKNOWN".to_string(),
                    available: "UNKNOWN".to_string(),
                }
            }
            "ORDER_NOT_FOUND" => {
                GateIoError::OrderNotFound { order_id: "UNKNOWN".to_string() }
            }
            "CURRENCY_PAIR_NOT_SUPPORTED" => {
                GateIoError::UnsupportedCurrencyPair { pair: "UNKNOWN".to_string() }
            }
            "ACCOUNT_LOCKED" | "ACCOUNT_SUSPENDED" => {
                GateIoError::AccountLocked
            }
            "DAILY_LIMIT_EXCEEDED" => {
                GateIoError::DailyLimitExceeded { operation: error.message }
            }
            "MAINTENANCE_MODE" | "SYSTEM_MAINTENANCE" => {
                GateIoError::MaintenanceMode { message: error.message }
            }
            _ => {
                GateIoError::Api(ApiError {
                    label: error.label,
                    message: error.message,
                })
            }
        }
    }
}

impl GateIoError {
    /// Check if the error is retryable
    pub fn is_retryable(&self) -> bool {
        match self {
            GateIoError::RateLimitExceeded { .. } => true,
            GateIoError::Timeout { .. } => true,
            GateIoError::Http(e) => e.is_timeout() || e.is_connect(),
            GateIoError::MaintenanceMode { .. } => true,
            _ => false,
        }
    }

    /// Get retry delay in seconds for retryable errors
    pub fn retry_delay_secs(&self) -> Option<u64> {
        match self {
            GateIoError::RateLimitExceeded { .. } => Some(60), // Wait 1 minute
            GateIoError::Timeout { .. } => Some(5), // Wait 5 seconds
            GateIoError::Http(_) => Some(10), // Wait 10 seconds
            GateIoError::MaintenanceMode { .. } => Some(300), // Wait 5 minutes
            _ => None,
        }
    }

    /// Check if error indicates authentication issues
    pub fn is_auth_error(&self) -> bool {
        matches!(self, GateIoError::Authentication(_))
    }

    /// Check if error indicates client-side issues (non-retryable)
    pub fn is_client_error(&self) -> bool {
        match self {
            GateIoError::InvalidParameter(_) => true,
            GateIoError::Authentication(_) => true,
            GateIoError::OrderNotFound { .. } => true,
            GateIoError::UnsupportedCurrencyPair { .. } => true,
            GateIoError::InsufficientBalance { .. } => true,
            _ => false,
        }
    }
}