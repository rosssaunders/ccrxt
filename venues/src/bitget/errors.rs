use thiserror::Error;
use serde::Deserialize;
use std::fmt;

/// Represents all possible errors that can occur when interacting with the Bitget API
#[derive(Debug)]
pub enum Errors {
    /// Invalid API key or signature
    InvalidApiKey(),
    
    /// Http error occurred while making a request
    /// This variant is used to represent errors that are not specific to the Bitget API,
    /// such as network issues or HTTP errors.
    /// It can be used to wrap any error that occurs during the request process.
    /// This variant is not used for errors returned by the Bitget API itself.
    HttpError(reqwest::Error),

    /// An error returned by the Bitget API
    ApiError(ApiError),
    
    /// A general error with a descriptive message
    Error(String),
}

impl fmt::Display for Errors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Errors::InvalidApiKey() => write!(f, "Invalid API key or signature"),
            Errors::HttpError(err) => write!(f, "HTTP error: {}", err),
            Errors::ApiError(err) => write!(f, "API error: {}", err),
            Errors::Error(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for Errors {}

/// Represents an error response from the Bitget API.
/// 
/// This is public as it is used by Batch responses.
#[derive(Debug, Clone, Deserialize)]
pub struct ErrorResponse {
    pub code: String,
    pub msg: String,
}

/// Error code ranges for Bitget API:
/// Based on the documentation, Bitget uses string error codes like "00000" for success,
/// and various error codes like "30001", "70001", etc.
#[derive(Error, Debug, Clone, Deserialize)]
pub enum ApiError {
    #[error("{msg}")]
    UnknownApiError { msg: String },

    #[error("{msg}")]
    Disconnected { msg: String },

    #[error("{msg}")]
    Unauthorized { msg: String },

    #[error("{msg}")]
    TooManyRequests { msg: String },

    #[error("{msg}")]
    IpBanned { msg: String },

    #[error("{msg}")]
    InvalidTimestamp { msg: String },

    #[error("{msg}")]
    InvalidSignature { msg: String },

    #[error("{msg}")]
    MandatoryParamEmptyOrMalformed { msg: String },

    #[error("{msg}")]
    UnknownParam { msg: String },

    #[error("{msg}")]
    ParamEmpty { msg: String },

    #[error("{msg}")]
    BadAsset { msg: String },

    #[error("{msg}")]
    BadSymbol { msg: String },

    #[error("{msg}")]
    InvalidParameter { msg: String },

    #[error("{msg}")]
    NewOrderRejected { msg: String },

    #[error("{msg}")]
    CancelRejected { msg: String },

    #[error("{msg}")]
    NoSuchOrder { msg: String },

    #[error("{msg}")]
    BadApiKeyFmt { msg: String },

    #[error("{msg}")]
    BalanceNotSufficient { msg: String },

    #[error("{msg}")]
    UnableToFill { msg: String },

    #[error("{msg}")]
    MaxOpenOrderExceeded { msg: String },

    #[error("{msg}")]
    InvalidAccessKey { msg: String },

    #[error("{msg}")]
    InvalidAccessPassphrase { msg: String },

    #[error("{msg}")]
    RequestExpired { msg: String },

    #[error("{msg}")]
    ParamError { msg: String },

    #[error("{msg}")]
    ChannelDoesNotExist { msg: String },

    #[error("{msg}")]
    IllegalRequest { msg: String },

    #[error("{msg}")]
    InvalidOp { msg: String },

    #[error("{msg}")]
    UserNeedsToLogin { msg: String },

    #[error("{msg}")]
    LoginFailed { msg: String },

    #[error("{msg}")]
    RequestTooMany { msg: String },

    #[error("{msg}")]
    RequestOverLimit { msg: String },

    /// Unmapped API error - for error codes not explicitly handled
    #[error("API error (code: {code}): {msg}")]
    UnmappedApiError { code: String, msg: String },

    /// Returned when the API responds with HTTP 429 (Too Many Requests).
    /// This error includes the original error message.
    ///
    /// Fields:
    /// - `msg`: The error message from the API.
    /// - `retry_after`: The value of the `Retry-After` header, if present.
    #[error("429 Too Many Requests: {msg} (retry_after={retry_after:?})")]
    RateLimitExceeded {
        msg: String,
        retry_after: Option<u64>,
    },

    /// Returned when the API responds with HTTP 403 (Forbidden).
    /// This error indicates access is forbidden.
    ///
    /// Fields:
    /// - `msg`: The error message from the API.
    #[error("403 Forbidden: {msg}")]
    Forbidden { msg: String },

    /// Returned when the API responds with HTTP 408 (Timeout).
    /// This error indicates a timeout occurred while waiting for a response from the backend server.
    ///
    /// Fields:
    /// - `msg`: The error message from the API.
    #[error("408 Request Timeout: {msg}")]
    RequestTimeout { msg: String },

    /// Returned when the API responds with HTTP 5XX (Internal Server Error).
    /// This error indicates an internal error on Bitget's side.
    ///
    /// Fields:
    /// - `msg`: The error message from the API.
    #[error("5XX Internal Server Error: {msg}")]
    InternalServerError { msg: String },

    /// Returned when the API responds with HTTP 503 (Service Unavailable).
    /// This error indicates the service is unavailable or the execution status is unknown.
    ///
    /// Fields:
    /// - `msg`: The error message from the API.
    #[error("503 Service Unavailable: {msg}")]
    ServiceUnavailable { msg: String },
}

// Conversion from ErrorResponse to ApiError
impl From<ErrorResponse> for ApiError {
    fn from(err: ErrorResponse) -> Self {
        match err.code.as_str() {
            "30001" => ApiError::ChannelDoesNotExist { msg: err.msg },
            "30002" => ApiError::IllegalRequest { msg: err.msg },
            "30003" => ApiError::InvalidOp { msg: err.msg },
            "30004" => ApiError::UserNeedsToLogin { msg: err.msg },
            "30005" => ApiError::LoginFailed { msg: err.msg },
            "30006" => ApiError::RequestTooMany { msg: err.msg },
            "30007" => ApiError::RequestOverLimit { msg: err.msg },
            "30011" => ApiError::InvalidAccessKey { msg: err.msg },
            "30012" => ApiError::InvalidAccessPassphrase { msg: err.msg },
            "30013" => ApiError::InvalidTimestamp { msg: err.msg },
            "30014" => ApiError::RequestExpired { msg: err.msg },
            "30015" => ApiError::InvalidSignature { msg: err.msg },
            "30016" => ApiError::ParamError { msg: err.msg },
            "70001" => ApiError::BadSymbol { msg: err.msg },
            "70002" => ApiError::InvalidParameter { msg: err.msg },
            "70003" => ApiError::MandatoryParamEmptyOrMalformed { msg: err.msg },
            "70004" => ApiError::UnknownParam { msg: err.msg },
            "70005" => ApiError::ParamEmpty { msg: err.msg },
            "70006" => ApiError::BadAsset { msg: err.msg },
            "70007" => ApiError::BadApiKeyFmt { msg: err.msg },
            "70008" => ApiError::Unauthorized { msg: err.msg },
            "70009" => ApiError::TooManyRequests { msg: err.msg },
            "70010" => ApiError::NewOrderRejected { msg: err.msg },
            "70011" => ApiError::CancelRejected { msg: err.msg },
            "70012" => ApiError::NoSuchOrder { msg: err.msg },
            "70013" => ApiError::BalanceNotSufficient { msg: err.msg },
            "70014" => ApiError::UnableToFill { msg: err.msg },
            "70015" => ApiError::MaxOpenOrderExceeded { msg: err.msg },
            "400172" => ApiError::ParamError { msg: err.msg },
            _ => ApiError::UnmappedApiError {
                code: err.code,
                msg: err.msg,
            },
        }
    }
}