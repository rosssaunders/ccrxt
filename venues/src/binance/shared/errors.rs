use std::{fmt, time::Duration};

use serde::Deserialize;
use thiserror::Error;

/// Represents all possible errors that can occur when interacting with Binance APIs
#[derive(Debug)]
#[non_exhaustive]
pub enum Errors {
    /// Invalid API key or signature
    InvalidApiKey,

    /// HTTP layer error (network, status handling etc.)
    Http { message: String },

    /// An error returned by the Binance API
    Api(ApiError),

    /// Rate limit exceeded with optional retry-after duration
    RateLimitExceeded { retry_after: Option<Duration> },

    /// Serialization error when preparing a request
    Serialize { message: String },

    /// Deserialization error when parsing a response
    Deserialize { message: String },

    /// Generic catch‑all (should be phased out over time)
    Generic { message: String },
}

impl fmt::Display for Errors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Errors::InvalidApiKey => write!(f, "Invalid API key or signature"),
            Errors::Http { message } => write!(f, "HTTP error: {message}"),
            Errors::Api(err) => write!(f, "API error: {err}"),
            Errors::RateLimitExceeded { retry_after } => match retry_after {
                Some(duration) => write!(f, "Rate limit exceeded, retry after {duration:?}"),
                None => write!(f, "Rate limit exceeded"),
            },
            Errors::Serialize { message } => write!(f, "Serialization error: {message}"),
            Errors::Deserialize { message } => write!(f, "Deserialization error: {message}"),
            Errors::Generic { message } => write!(f, "Error: {message}"),
        }
    }
}

impl std::error::Error for Errors {}

impl From<serde_urlencoded::ser::Error> for Errors {
    fn from(err: serde_urlencoded::ser::Error) -> Self {
        Errors::Serialize {
            message: err.to_string(),
        }
    }
}

/// Represents an error response from the Binance API.
#[derive(Debug, Clone, Deserialize)]
pub struct ErrorResponse {
    pub code: i32,
    pub msg: String,
}

/// API errors for all Binance venues
/// Error code ranges:
/// -1000 to -1999: General Server or Network issues
/// -2000 to -2999: Authentication and Authorization errors  
/// -3000 to -3999: Rate limiting and internal errors
/// -4000 to -4999: Futures/Derivatives specific errors
/// -5000 to -5999: Filters and advanced features
#[derive(Error, Debug, Clone)]
#[non_exhaustive]
pub enum ApiError {
    // -1000 to -1999: General Server or Network issues
    #[error("{msg}")]
    UnknownApiError { code: i32, msg: String },

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
    TifNotRequired { msg: String },

    #[error("{msg}")]
    InvalidTif { msg: String },

    #[error("{msg}")]
    InvalidOrderType { msg: String },

    #[error("{msg}")]
    InvalidSide { msg: String },

    #[error("{msg}")]
    EmptyNewCLOrderId { msg: String },

    #[error("{msg}")]
    EmptyOriginalCLOrderId { msg: String },

    #[error("{msg}")]
    BadInterval { msg: String },

    #[error("{msg}")]
    BadSymbol { msg: String },

    #[error("{msg}")]
    InvalidListenKey { msg: String },

    #[error("{msg}")]
    MoreThanXxHours { msg: String },

    #[error("{msg}")]
    OptionalParamsBadCombination { msg: String },

    #[error("{msg}")]
    InvalidParameter { msg: String },

    #[error("{msg}")]
    ServerBusy { msg: String },

    // -2000 to -2999: Authentication, Authorization and Order errors
    #[error("{msg}")]
    InvalidSenderIp { msg: String },

    #[error("{msg}")]
    NewOrderRejected { msg: String },

    #[error("{msg}")]
    CancelRejected { msg: String },

    #[error("{msg}")]
    NoSuchOrder { msg: String },

    #[error("{msg}")]
    BadApiKeyFormat { msg: String },

    #[error("{msg}")]
    RejectedMBXKey { msg: String },

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
    ReduceOnlyRejectWithLongPosition { msg: String },

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

    // -3000 to -3999: Rate limiting and internal errors
    #[error("{msg}")]
    InnerFailure { msg: String },

    #[error("{msg}")]
    NeedEnableMargin { msg: String },

    #[error("{msg}")]
    AssetDeficitInMargin { msg: String },

    #[error("{msg}")]
    MarginAccountAlreadyExists { msg: String },

    #[error("{msg}")]
    MarginBalanceOverflow { msg: String },

    #[error("{msg}")]
    RequestFrequencyExceeded { msg: String },

    // -4000 to -4999: Futures/Derivatives specific errors
    // These are only applicable to futures venues (USDM, COINM, Portfolio)
    #[error("{msg}")]
    InvalidPriceFilter { msg: String },

    #[error("{msg}")]
    InvalidQuantityFilter { msg: String },

    #[error("{msg}")]
    InvalidMinNotionalFilter { msg: String },

    #[error("{msg}")]
    InvalidMarketLotSizeFilter { msg: String },

    #[error("{msg}")]
    InvalidMaxNumOrdersFilter { msg: String },

    #[error("{msg}")]
    InvalidMaxNumAlgoOrdersFilter { msg: String },

    #[error("{msg}")]
    InvalidPercentPriceFilter { msg: String },

    #[error("{msg}")]
    InvalidNotionalFilter { msg: String },

    #[error("{msg}")]
    InvalidMaxNumIcebergOrdersFilter { msg: String },

    #[error("{msg}")]
    InvalidPositionFilter { msg: String },

    #[error("{msg}")]
    InvalidLeverage { msg: String },

    #[error("{msg}")]
    InvalidTimeInForce { msg: String },

    #[error("{msg}")]
    InvalidOrderTypeForLeverage { msg: String },

    #[error("{msg}")]
    InvalidPriceForPosition { msg: String },

    #[error("{msg}")]
    InvalidContractType { msg: String },

    #[error("{msg}")]
    NoNeedToChangeMarginType { msg: String },

    #[error("{msg}")]
    NoNeedToChangePositionSide { msg: String },

    #[error("{msg}")]
    PositionSideNotMatch { msg: String },

    #[error("{msg}")]
    ReduceOnlyConflict { msg: String },

    #[error("{msg}")]
    InvalidOptionsRequestType { msg: String },

    #[error("{msg}")]
    InvalidOptionsPremium { msg: String },

    #[error("{msg}")]
    InvalidBatchPlaceOrderSize { msg: String },

    #[error("{msg}")]
    PlaceOrderFailed { msg: String },

    #[error("{msg}")]
    DuplicateOrderId { msg: String },

    #[error("{msg}")]
    InvalidMaxOpenPosition { msg: String },

    #[error("{msg}")]
    OrderNotModifyable { msg: String },

    #[error("{msg}")]
    InvalidQuantityAdjustPct { msg: String },

    #[error("{msg}")]
    IsolatedLeverageRejectWithPosition { msg: String },

    #[error("{msg}")]
    InvalidCrossRatioForCoinM { msg: String },

    #[error("{msg}")]
    AdjustLeverageKycFailed { msg: String },

    #[error("{msg}")]
    PositionUpdateFailed { msg: String },

    // -5000 to -5999: Additional filters and errors
    #[error("{msg}")]
    PmAccountTradeNotAllowed { msg: String },

    #[error("{msg}")]
    PmAccountTransferNotAllowed { msg: String },

    #[error("{msg}")]
    SymbolNotAllowedInPm { msg: String },

    #[error("{msg}")]
    ReduceOnlyMarginCheckFailed { msg: String },

    #[error("{msg}")]
    PmAccountCannotLiquidate { msg: String },

    #[error("{msg}")]
    UnifiedAccountModeRestriction { msg: String },

    #[error("{msg}")]
    MarginModeSwitchFailed { msg: String },

    #[error("{msg}")]
    HasPendingTransactions { msg: String },
}

impl ApiError {
    /// Create an ApiError from a numeric code and message
    pub fn from_code(code: i32, msg: String) -> Self {
        match code {
            // -1000 to -1999: General Server or Network issues
            -1000 => ApiError::UnknownApiError { code, msg },
            -1001 => ApiError::Disconnected { msg },
            -1002 => ApiError::Unauthorized { msg },
            -1003 => ApiError::TooManyRequests { msg },
            -1006 => ApiError::UnexpectedResponse { msg },
            -1007 => ApiError::Timeout { msg },
            -1008 => ApiError::ServerBusy { msg },
            -1010 => ApiError::ErrorMsgReceived { msg },
            -1011 => ApiError::NonWhiteList { msg },
            -1013 => ApiError::InvalidMessage { msg },
            -1014 => ApiError::UnknownOrderComposition { msg },
            -1015 => ApiError::TooManyOrders { msg },
            -1016 => ApiError::ServiceShuttingDown { msg },
            -1020 => ApiError::UnsupportedOperation { msg },
            -1021 => ApiError::InvalidTimestamp { msg },
            -1022 => ApiError::InvalidSignature { msg },
            -1100 => ApiError::IllegalChars { msg },
            -1101 => ApiError::TooManyParameters { msg },
            -1102 => ApiError::MandatoryParamEmptyOrMalformed { msg },
            -1103 => ApiError::UnknownParam { msg },
            -1104 => ApiError::UnreadParameters { msg },
            -1105 => ApiError::ParamEmpty { msg },
            -1106 => ApiError::ParamNotRequired { msg },
            -1108 => ApiError::BadAsset { msg },
            -1109 => ApiError::BadAccount { msg },
            -1110 => ApiError::BadInstrumentType { msg },
            -1111 => ApiError::BadPrecision { msg },
            -1112 => ApiError::NoDepth { msg },
            -1114 => ApiError::TifNotRequired { msg },
            -1115 => ApiError::InvalidTif { msg },
            -1116 => ApiError::InvalidOrderType { msg },
            -1117 => ApiError::InvalidSide { msg },
            -1118 => ApiError::EmptyNewCLOrderId { msg },
            -1119 => ApiError::EmptyOriginalCLOrderId { msg },
            -1120 => ApiError::BadInterval { msg },
            -1121 => ApiError::BadSymbol { msg },
            -1125 => ApiError::InvalidListenKey { msg },
            -1127 => ApiError::MoreThanXxHours { msg },
            -1128 => ApiError::OptionalParamsBadCombination { msg },
            -1130 => ApiError::InvalidParameter { msg },

            // -2000 to -2999: Authentication, Authorization and Order errors
            -2008 => ApiError::InvalidSenderIp { msg },
            -2010 => ApiError::NewOrderRejected { msg },
            -2011 => ApiError::CancelRejected { msg },
            -2013 => ApiError::NoSuchOrder { msg },
            -2014 => ApiError::BadApiKeyFormat { msg },
            -2015 => ApiError::RejectedMBXKey { msg },
            -2016 => ApiError::NoTradingWindow { msg },
            -2018 => ApiError::BalanceNotSufficient { msg },
            -2019 => ApiError::MarginNotSufficient { msg },
            -2020 => ApiError::UnableToFill { msg },
            -2021 => ApiError::OrderWouldImmediatelyTrigger { msg },
            -2022 => ApiError::ReduceOnlyRejectWithLongPosition { msg },
            -2023 => ApiError::UserInLiquidation { msg },
            -2024 => ApiError::PositionNotSufficient { msg },
            -2025 => ApiError::MaxOpenOrderExceeded { msg },
            -2026 => ApiError::ReduceOnlyOrderTypeNotSupported { msg },
            -2027 => ApiError::MaxLeverageRatio { msg },
            -2028 => ApiError::MinLeverageRatio { msg },

            // -3000 to -3999: Rate limiting and internal errors
            -3000 => ApiError::InnerFailure { msg },
            -3001 => ApiError::NeedEnableMargin { msg },
            -3003 => ApiError::AssetDeficitInMargin { msg },
            -3004 => ApiError::MarginAccountAlreadyExists { msg },
            -3005 => ApiError::MarginBalanceOverflow { msg },
            -3045 => ApiError::RequestFrequencyExceeded { msg },

            // -4000 to -4999: Futures/Derivatives specific errors
            -4001 => ApiError::InvalidPriceFilter { msg },
            -4002 => ApiError::InvalidQuantityFilter { msg },
            -4003 => ApiError::InvalidMinNotionalFilter { msg },
            -4004 => ApiError::InvalidMarketLotSizeFilter { msg },
            -4005 => ApiError::InvalidMaxNumOrdersFilter { msg },
            -4006 => ApiError::InvalidMaxNumAlgoOrdersFilter { msg },
            -4007 => ApiError::InvalidPercentPriceFilter { msg },
            -4009 => ApiError::InvalidNotionalFilter { msg },
            -4010 => ApiError::InvalidMaxNumIcebergOrdersFilter { msg },
            -4011 => ApiError::InvalidPositionFilter { msg },
            -4028 => ApiError::InvalidLeverage { msg },
            -4029 => ApiError::InvalidTimeInForce { msg },
            -4030 => ApiError::InvalidOrderTypeForLeverage { msg },
            -4031 => ApiError::InvalidPriceForPosition { msg },
            -4032 => ApiError::InvalidContractType { msg },
            -4046 => ApiError::NoNeedToChangeMarginType { msg },
            -4059 => ApiError::NoNeedToChangePositionSide { msg },
            -4061 => ApiError::PositionSideNotMatch { msg },
            -4062 => ApiError::ReduceOnlyConflict { msg },
            -4081 => ApiError::InvalidOptionsRequestType { msg },
            -4082 => ApiError::InvalidOptionsPremium { msg },
            -4137 => ApiError::InvalidBatchPlaceOrderSize { msg },
            -4138 => ApiError::PlaceOrderFailed { msg },
            -4139 => ApiError::DuplicateOrderId { msg },
            -4141 => ApiError::InvalidMaxOpenPosition { msg },
            -4142 => ApiError::OrderNotModifyable { msg },
            -4144 => ApiError::InvalidQuantityAdjustPct { msg },
            -4150 => ApiError::IsolatedLeverageRejectWithPosition { msg },
            -4162 => ApiError::InvalidCrossRatioForCoinM { msg },
            -4194 => ApiError::AdjustLeverageKycFailed { msg },
            -4202 => ApiError::PositionUpdateFailed { msg },

            // -5000 to -5999: Additional filters and errors
            -5001 => ApiError::PmAccountTradeNotAllowed { msg },
            -5002 => ApiError::PmAccountTransferNotAllowed { msg },
            -5003 => ApiError::SymbolNotAllowedInPm { msg },
            -5004 => ApiError::ReduceOnlyMarginCheckFailed { msg },
            -5009 => ApiError::PmAccountCannotLiquidate { msg },
            -5011 => ApiError::UnifiedAccountModeRestriction { msg },
            -5012 => ApiError::MarginModeSwitchFailed { msg },
            -5013 => ApiError::HasPendingTransactions { msg },

            // Default to unknown error
            _ => ApiError::UnknownApiError { code, msg },
        }
    }
}

/// Convert HTTP status codes to appropriate errors. Must be used so callers do not ignore rate limit / auth failures.
#[must_use = "You must check the returned Result to propagate HTTP / rate limit / auth errors"]
pub fn handle_http_status(status: u16, response_text: &str) -> Result<(), Errors> {
    match status {
        200 => Ok(()),
        429 => {
            // Extract retry-after header if available
            Err(Errors::RateLimitExceeded { retry_after: None })
        }
        403 => {
            if response_text.contains("banned") {
                Err(Errors::Api(ApiError::IpBanned {
                    msg: "IP banned".to_string(),
                }))
            } else {
                Err(Errors::Api(ApiError::Unauthorized {
                    msg: "Forbidden".to_string(),
                }))
            }
        }
        401 => Err(Errors::Api(ApiError::Unauthorized {
            msg: "Unauthorized".to_string(),
        })),
        418 => Err(Errors::Api(ApiError::IpBanned {
            msg: "IP banned (418)".to_string(),
        })),
        500..=599 => Err(Errors::Api(ApiError::InnerFailure {
            msg: format!("Server error: {status}"),
        })),
        _ => Ok(()),
    }
}
