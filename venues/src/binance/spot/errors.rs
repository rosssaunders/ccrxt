use serde::Deserialize;
use std::fmt;
use thiserror::Error;

/// Represents all possible errors that can occur when interacting with the Binance API
#[derive(Debug)]
pub enum Errors {
    /// Invalid API key or signature
    InvalidApiKey(),

    /// Http error occurred while making a request
    /// This variant is used to represent errors that are not specific to the Binance API,
    /// such as network issues or HTTP errors.
    /// It can be used to wrap any error that occurs during the request process.
    /// This variant is not used for errors returned by the Binance API itself.
    HttpError(reqwest::Error),

    /// An error returned by the Binance API
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

/// Represents an error response from the Binance API.
///
/// This is public as it is used by Batch responses.
#[derive(Debug, Clone, Deserialize)]
pub struct ErrorResponse {
    pub code: i32,
    pub msg: String,
}

/// Error code ranges for Binance Spot API:
/// -1000 to -1999: General Server or Network issues
/// -2000 to -2999: Authentication and Authorization errors
/// -3000 to -3999: Rate limiting errors
/// -4000 to -4999: Validation and Processing errors
/// -5000 to -5999: System errors
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
    DuplicateIp { msg: String },

    #[error("{msg}")]
    NoSuchIp { msg: String },

    #[error("{msg}")]
    UnexpectedResponse { msg: String },

    #[error("{msg}")]
    Timeout { msg: String },

    #[error("{msg}")]
    ErrorMsgReceived { msg: String },

    #[error("{msg}")]
    NonWhiteList { msg: String },

    #[error("{msg}")]
    InvalidMessage { msg: String },

    #[error("{msg}")]
    UnknownOrderComposition { msg: String },

    #[error("{msg}")]
    TooManyOrders { msg: String },

    #[error("{msg}")]
    ServiceShuttingDown { msg: String },

    #[error("{msg}")]
    UnsupportedOperation { msg: String },

    #[error("{msg}")]
    InvalidTimestamp { msg: String },

    #[error("{msg}")]
    InvalidSignature { msg: String },

    #[error("{msg}")]
    StartTimeGreaterThanEndTime { msg: String },

    // 11xx - Request issues
    #[error("{msg}")]
    IllegalChars { msg: String },

    #[error("{msg}")]
    TooManyParameters { msg: String },

    #[error("{msg}")]
    MandatoryParamEmptyOrMalformed { msg: String },

    #[error("{msg}")]
    UnknownParam { msg: String },

    #[error("{msg}")]
    UnreadParameters { msg: String },

    #[error("{msg}")]
    ParamEmpty { msg: String },

    #[error("{msg}")]
    ParamNotRequired { msg: String },

    #[error("{msg}")]
    BadAsset { msg: String },

    #[error("{msg}")]
    BadAccount { msg: String },

    #[error("{msg}")]
    BadInstrumentType { msg: String },

    #[error("{msg}")]
    BadPrecision { msg: String },

    #[error("{msg}")]
    NoDepth { msg: String },

    #[error("{msg}")]
    WithdrawNotNegative { msg: String },

    #[error("{msg}")]
    TifNotRequired { msg: String },

    #[error("{msg}")]
    InvalidTif { msg: String },

    #[error("{msg}")]
    InvalidOrderType { msg: String },

    #[error("{msg}")]
    InvalidSide { msg: String },

    #[error("{msg}")]
    EmptyNewClOrdId { msg: String },

    #[error("{msg}")]
    EmptyOrgClOrdId { msg: String },

    #[error("{msg}")]
    BadInterval { msg: String },

    #[error("{msg}")]
    BadSymbol { msg: String },

    #[error("{msg}")]
    InvalidListenKey { msg: String },

    #[error("{msg}")]
    MoreThanXxHours { msg: String },

    #[error("{msg}")]
    OptionalParamsBadCombo { msg: String },

    #[error("{msg}")]
    InvalidParameter { msg: String },

    #[error("{msg}")]
    InvalidNewOrderRespType { msg: String },

    // 20xx - Processing Issues
    #[error("{msg}")]
    NewOrderRejected { msg: String },

    #[error("{msg}")]
    CancelRejected { msg: String },

    #[error("{msg}")]
    NoSuchOrder { msg: String },

    #[error("{msg}")]
    BadApiKeyFmt { msg: String },

    #[error("{msg}")]
    RejectedMbxKey { msg: String },

    #[error("{msg}")]
    NoTradingWindow { msg: String },

    #[error("{msg}")]
    BalanceNotSufficient { msg: String },

    #[error("{msg}")]
    MarginNotSufficient { msg: String },

    #[error("{msg}")]
    UnableToFill { msg: String },

    #[error("{msg}")]
    OrderWouldImmediatelyTrigger { msg: String },

    #[error("{msg}")]
    ReduceOnlyReject { msg: String },

    #[error("{msg}")]
    UserInLiquidation { msg: String },

    #[error("{msg}")]
    PositionNotSufficient { msg: String },

    #[error("{msg}")]
    MaxOpenOrderExceeded { msg: String },

    #[error("{msg}")]
    ReduceOnlyOrderTypeNotSupported { msg: String },

    #[error("{msg}")]
    MaxLeverageRatio { msg: String },

    #[error("{msg}")]
    MinLeverageRatio { msg: String },

    /// Unmapped API error - for error codes not explicitly handled
    #[error("API error (code: {code}): {msg}")]
    UnmappedApiError { code: i32, msg: String },

    /// Returned when the API responds with HTTP 429 (Too Many Requests).
    /// This error includes the original error message and relevant Binance rate limit headers.
    ///
    /// Fields:
    /// - `msg`: The error message from the API.
    /// - `used_weight_1m`: The value of the `x-mbx-used-weight-1m` header, if present.
    /// - `order_count_1m`: The value of the `x-mbx-order-count-1m` header, if present.
    /// - `retry_after`: The value of the `Retry-After` header, if present.
    #[error("429 Too Many Requests: {msg} (used_weight_1m={used_weight_1m:?}, order_count_1m={order_count_1m:?}, retry_after={retry_after:?})")]
    RateLimitExceeded {
        msg: String,
        used_weight_1m: Option<u32>,
        order_count_1m: Option<u32>,
        retry_after: Option<u64>,
    },

    /// Returned when the API responds with HTTP 403 (WAF Limit Violation).
    /// This error indicates the Web Application Firewall has blocked the request.
    ///
    /// Fields:
    /// - `msg`: The error message from the API.
    #[error("403 Forbidden (WAF Limit Violation): {msg}")]
    WafLimitViolated { msg: String },

    /// Returned when the API responds with HTTP 408 (Timeout).
    /// This error indicates a timeout occurred while waiting for a response from the backend server.
    ///
    /// Fields:
    /// - `msg`: The error message from the API.
    #[error("408 Request Timeout: {msg}")]
    RequestTimeout { msg: String },

    /// Returned when the API responds with HTTP 418 (IP Auto-Banned).
    /// This error indicates the IP has been auto-banned for continuing to send requests after receiving 429 codes.
    ///
    /// Fields:
    /// - `msg`: The error message from the API.
    #[error("418 IP Auto-Banned: {msg}")]
    IpAutoBanned { msg: String },

    /// Returned when the API responds with HTTP 5XX (Internal Server Error).
    /// This error indicates an internal error on Binance's side.
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
        match err.code {
            -1000 => ApiError::UnknownApiError { msg: err.msg },
            -1001 => ApiError::Disconnected { msg: err.msg },
            -1002 => ApiError::Unauthorized { msg: err.msg },
            -1003 => ApiError::TooManyRequests { msg: err.msg },
            -1004 => ApiError::DuplicateIp { msg: err.msg },
            -1005 => ApiError::NoSuchIp { msg: err.msg },
            -1006 => ApiError::UnexpectedResponse { msg: err.msg },
            -1007 => ApiError::Timeout { msg: err.msg },
            -1010 => ApiError::ErrorMsgReceived { msg: err.msg },
            -1011 => ApiError::NonWhiteList { msg: err.msg },
            -1013 => ApiError::InvalidMessage { msg: err.msg },
            -1014 => ApiError::UnknownOrderComposition { msg: err.msg },
            -1015 => ApiError::TooManyOrders { msg: err.msg },
            -1016 => ApiError::ServiceShuttingDown { msg: err.msg },
            -1020 => ApiError::UnsupportedOperation { msg: err.msg },
            -1021 => ApiError::InvalidTimestamp { msg: err.msg },
            -1022 => ApiError::InvalidSignature { msg: err.msg },
            -1023 => ApiError::StartTimeGreaterThanEndTime { msg: err.msg },
            -1100 => ApiError::IllegalChars { msg: err.msg },
            -1101 => ApiError::TooManyParameters { msg: err.msg },
            -1102 => ApiError::MandatoryParamEmptyOrMalformed { msg: err.msg },
            -1103 => ApiError::UnknownParam { msg: err.msg },
            -1104 => ApiError::UnreadParameters { msg: err.msg },
            -1105 => ApiError::ParamEmpty { msg: err.msg },
            -1106 => ApiError::ParamNotRequired { msg: err.msg },
            -1108 => ApiError::BadAsset { msg: err.msg },
            -1109 => ApiError::BadAccount { msg: err.msg },
            -1110 => ApiError::BadInstrumentType { msg: err.msg },
            -1111 => ApiError::BadPrecision { msg: err.msg },
            -1112 => ApiError::NoDepth { msg: err.msg },
            -1113 => ApiError::WithdrawNotNegative { msg: err.msg },
            -1114 => ApiError::TifNotRequired { msg: err.msg },
            -1115 => ApiError::InvalidTif { msg: err.msg },
            -1116 => ApiError::InvalidOrderType { msg: err.msg },
            -1117 => ApiError::InvalidSide { msg: err.msg },
            -1118 => ApiError::EmptyNewClOrdId { msg: err.msg },
            -1119 => ApiError::EmptyOrgClOrdId { msg: err.msg },
            -1120 => ApiError::BadInterval { msg: err.msg },
            -1121 => ApiError::BadSymbol { msg: err.msg },
            -1125 => ApiError::InvalidListenKey { msg: err.msg },
            -1127 => ApiError::MoreThanXxHours { msg: err.msg },
            -1128 => ApiError::OptionalParamsBadCombo { msg: err.msg },
            -1130 => ApiError::InvalidParameter { msg: err.msg },
            -1136 => ApiError::InvalidNewOrderRespType { msg: err.msg },
            -2010 => ApiError::NewOrderRejected { msg: err.msg },
            -2011 => ApiError::CancelRejected { msg: err.msg },
            -2013 => ApiError::NoSuchOrder { msg: err.msg },
            -2014 => ApiError::BadApiKeyFmt { msg: err.msg },
            -2015 => ApiError::RejectedMbxKey { msg: err.msg },
            -2016 => ApiError::NoTradingWindow { msg: err.msg },
            -2018 => ApiError::BalanceNotSufficient { msg: err.msg },
            -2019 => ApiError::MarginNotSufficient { msg: err.msg },
            -2020 => ApiError::UnableToFill { msg: err.msg },
            -2021 => ApiError::OrderWouldImmediatelyTrigger { msg: err.msg },
            -2022 => ApiError::ReduceOnlyReject { msg: err.msg },
            -2023 => ApiError::UserInLiquidation { msg: err.msg },
            -2024 => ApiError::PositionNotSufficient { msg: err.msg },
            -2025 => ApiError::MaxOpenOrderExceeded { msg: err.msg },
            -2026 => ApiError::ReduceOnlyOrderTypeNotSupported { msg: err.msg },
            -2027 => ApiError::MaxLeverageRatio { msg: err.msg },
            -2028 => ApiError::MinLeverageRatio { msg: err.msg },
            _ => ApiError::UnmappedApiError {
                code: err.code,
                msg: err.msg,
            },
        }
    }
}
