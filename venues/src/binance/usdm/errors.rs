use serde::Deserialize;
use thiserror::Error;

// Allow conversion from shared errors to USDM errors for endpoint propagation
impl From<crate::binance::shared::errors::Errors> for Errors {
    fn from(err: crate::binance::shared::errors::Errors) -> Self {
        match err {
            crate::binance::shared::errors::Errors::InvalidApiKey() => Errors::InvalidApiKey(),
            crate::binance::shared::errors::Errors::ApiError(e) => {
                Errors::ApiError(ApiError::UnknownApiError { msg: e.to_string() })
            }
            crate::binance::shared::errors::Errors::HttpError(e) => Errors::HttpError(e),
            crate::binance::shared::errors::Errors::RateLimitExceeded { retry_after } => {
                Errors::RateLimitExceeded {
                    retry_after: retry_after.map(|d| d.as_secs()),
                }
            }
            crate::binance::shared::errors::Errors::SerializationError(msg) => {
                Errors::SerializationError(msg)
            }
            crate::binance::shared::errors::Errors::Error(msg) => Errors::Error(msg),
        }
    }
}

/// Represents all possible errors that can occur when interacting with the Binance API
#[derive(Error, Debug)]
pub enum Errors {
    /// Invalid API key or signature
    #[error("Invalid API key or signature")]
    InvalidApiKey(),

    /// Http error occurred while making a request
    /// This variant is used to represent errors that are not specific to the Binance API,
    /// such as network issues or HTTP errors.
    /// It can be used to wrap any error that occurs during the request process.
    /// This variant is not used for errors returned by the Binance API itself.
    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),

    /// An error returned by the Binance API
    #[error("API error: {0}")]
    ApiError(#[from] ApiError),

    /// A general error with a descriptive message
    #[error("Error: {0}")]
    Error(String),

    /// Error occurred during serialization or deserialization
    #[error("Serialization error: {0}")]
    SerializationError(String),

    /// Rate limit exceeded error with retry information
    #[error("Rate limit exceeded, retry after: {retry_after:?}")]
    RateLimitExceeded { retry_after: Option<u64> },
}

/// Result type alias for Binance USDM operations
#[allow(dead_code)]
pub type Result<T> = std::result::Result<T, Errors>;

/// Represents an error response from the Binance API.
///
/// This is public as it is used by Batch responses.
#[derive(Debug, Clone, Deserialize)]
pub struct ErrorResponse {
    pub code: i32,
    pub msg: String,
}

/// Error code ranges:
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

    // 40xx - Filters and other Issues
    #[error("{msg}")]
    InvalidOrderStatus { msg: String },

    #[error("{msg}")]
    PriceLessThanZero { msg: String },

    #[error("{msg}")]
    PriceGreaterThanMaxPrice { msg: String },

    #[error("{msg}")]
    QtyLessThanZero { msg: String },

    #[error("{msg}")]
    QtyLessThanMinQty { msg: String },

    #[error("{msg}")]
    QtyGreaterThanMaxQty { msg: String },

    #[error("{msg}")]
    StopPriceLessThanZero { msg: String },

    #[error("{msg}")]
    StopPriceGreaterThanMaxPrice { msg: String },

    #[error("{msg}")]
    TickSizeLessThanZero { msg: String },

    #[error("{msg}")]
    MaxPriceLessThanMinPrice { msg: String },

    #[error("{msg}")]
    MaxQtyLessThanMinQty { msg: String },

    #[error("{msg}")]
    StepSizeLessThanZero { msg: String },

    #[error("{msg}")]
    MaxNumOrdersLessThanZero { msg: String },

    #[error("{msg}")]
    PriceLessThanMinPrice { msg: String },

    #[error("{msg}")]
    PriceNotIncreasedByTickSize { msg: String },

    #[error("{msg}")]
    InvalidClOrdIdLen { msg: String },

    #[error("{msg}")]
    PriceHighterThanMultiplierUp { msg: String },

    #[error("{msg}")]
    MultiplierUpLessThanZero { msg: String },

    #[error("{msg}")]
    MultiplierDownLessThanZero { msg: String },

    #[error("{msg}")]
    CompositeScaleOverflow { msg: String },

    #[error("{msg}")]
    TargetStrategyInvalid { msg: String },

    #[error("{msg}")]
    InvalidDepthLimit { msg: String },

    #[error("{msg}")]
    WrongMarketStatus { msg: String },

    #[error("{msg}")]
    QtyNotIncreasedByStepSize { msg: String },

    #[error("{msg}")]
    PriceLowerThanMultiplierDown { msg: String },

    #[error("{msg}")]
    MultiplierDecimalLessThanZero { msg: String },

    #[error("{msg}")]
    CommissionInvalid { msg: String },

    #[error("{msg}")]
    InvalidAccountType { msg: String },

    #[error("{msg}")]
    InvalidLeverage { msg: String },

    #[error("{msg}")]
    InvalidTickSizePrecision { msg: String },

    #[error("{msg}")]
    InvalidStepSizePrecision { msg: String },

    #[error("{msg}")]
    InvalidWorkingType { msg: String },

    #[error("{msg}")]
    ExceedMaxCancelOrderSize { msg: String },

    #[error("{msg}")]
    InsuranceAccountNotFound { msg: String },

    #[error("{msg}")]
    InvalidBalanceType { msg: String },

    #[error("{msg}")]
    MaxStopOrderExceeded { msg: String },

    #[error("{msg}")]
    NoNeedToChangeMarginType { msg: String },

    #[error("{msg}")]
    ThereExistsOpenOrders { msg: String },

    #[error("{msg}")]
    ThereExistsQuantity { msg: String },

    #[error("{msg}")]
    AddIsolatedMarginReject { msg: String },

    #[error("{msg}")]
    CrossBalanceInsufficient { msg: String },

    #[error("{msg}")]
    IsolatedBalanceInsufficient { msg: String },

    #[error("{msg}")]
    NoNeedToChangeAutoAddMargin { msg: String },

    #[error("{msg}")]
    AutoAddCrossedMarginReject { msg: String },

    #[error("{msg}")]
    AddIsolatedMarginNoPositionReject { msg: String },

    #[error("{msg}")]
    AmountMustBePositive { msg: String },

    #[error("{msg}")]
    InvalidApiKeyType { msg: String },

    #[error("{msg}")]
    InvalidRsaPublicKey { msg: String },

    #[error("{msg}")]
    MaxPriceTooLarge { msg: String },

    #[error("{msg}")]
    NoNeedToChangePositionSide { msg: String },

    #[error("{msg}")]
    InvalidPositionSide { msg: String },

    #[error("{msg}")]
    PositionSideNotMatch { msg: String },

    #[error("{msg}")]
    ReduceOnlyConflict { msg: String },

    #[error("{msg}")]
    PositionSideChangeExistsOpenOrders { msg: String },

    #[error("{msg}")]
    PositionSideChangeExistsQuantity { msg: String },

    #[error("{msg}")]
    InvalidBatchPlaceOrderSize { msg: String },

    #[error("{msg}")]
    PlaceBatchOrdersFail { msg: String },

    #[error("{msg}")]
    UpcomingMethod { msg: String },

    #[error("{msg}")]
    InvalidPriceSpreadThreshold { msg: String },

    #[error("{msg}")]
    InvalidPair { msg: String },

    #[error("{msg}")]
    InvalidTimeInterval { msg: String },

    #[error("{msg}")]
    ReduceOnlyOrderPermission { msg: String },

    #[error("{msg}")]
    NoPlaceOrderPermission { msg: String },

    #[error("{msg}")]
    InvalidContractType { msg: String },

    #[error("{msg}")]
    InvalidClientTranIdLen { msg: String },

    #[error("{msg}")]
    DuplicatedClientTranId { msg: String },

    #[error("{msg}")]
    ReduceOnlyMarginCheckFailed { msg: String },

    #[error("{msg}")]
    MarketOrderReject { msg: String },

    #[error("{msg}")]
    InvalidActivationPrice { msg: String },

    #[error("{msg}")]
    QuantityExistsWithClosePosition { msg: String },

    #[error("{msg}")]
    ReduceOnlyMustBeTrue { msg: String },

    #[error("{msg}")]
    OrderTypeCannotBeMkt { msg: String },

    #[error("{msg}")]
    StrategyInvalidTriggerPrice { msg: String },

    #[error("{msg}")]
    IsolatedLeverageRejectWithPosition { msg: String },

    #[error("{msg}")]
    PriceHighterThanStopMultiplierUp { msg: String },

    #[error("{msg}")]
    PriceLowerThanStopMultiplierDown { msg: String },

    #[error("{msg}")]
    StopPriceHigherThanPriceMultiplierLimit { msg: String },

    #[error("{msg}")]
    StopPriceLowerThanPriceMultiplierLimit { msg: String },

    #[error("{msg}")]
    MinNotional { msg: String },

    #[error("{msg}")]
    CoolingOffPeriod { msg: String },

    #[error("{msg}")]
    AdjustLeverageKycFailed { msg: String },

    #[error("{msg}")]
    AdjustLeverageOneMonthFailed { msg: String },

    #[error("{msg}")]
    LimitOrderOnly { msg: String },

    #[error("{msg}")]
    SameOrder { msg: String },

    #[error("{msg}")]
    ExceedMaxModifyOrderLimit { msg: String },

    #[error("{msg}")]
    MoveOrderNotAllowedSymbolReason { msg: String },

    #[error("{msg}")]
    AdjustLeverageXDaysFailed { msg: String },

    #[error("{msg}")]
    AdjustLeverageKycLimit { msg: String },

    #[error("{msg}")]
    AdjustLeverageAccountSymbolFailed { msg: String },

    #[error("{msg}")]
    MeInvalidTimestamp { msg: String },

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
    #[error(
        "429 Too Many Requests: {msg} (used_weight_1m={used_weight_1m:?}, order_count_1m={order_count_1m:?}, retry_after={retry_after:?})"
    )]
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

// Conversion from BinanceErrorResponse to BinanceCoinMAPIError
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
            -4000 => ApiError::InvalidOrderStatus { msg: err.msg },
            -4001 => ApiError::PriceLessThanZero { msg: err.msg },
            -4002 => ApiError::PriceGreaterThanMaxPrice { msg: err.msg },
            -4003 => ApiError::QtyLessThanZero { msg: err.msg },
            -4004 => ApiError::QtyLessThanMinQty { msg: err.msg },
            -4005 => ApiError::QtyGreaterThanMaxQty { msg: err.msg },
            -4006 => ApiError::StopPriceLessThanZero { msg: err.msg },
            -4007 => ApiError::StopPriceGreaterThanMaxPrice { msg: err.msg },
            -4008 => ApiError::TickSizeLessThanZero { msg: err.msg },
            -4009 => ApiError::MaxPriceLessThanMinPrice { msg: err.msg },
            -4010 => ApiError::MaxQtyLessThanMinQty { msg: err.msg },
            -4011 => ApiError::StepSizeLessThanZero { msg: err.msg },
            -4012 => ApiError::MaxNumOrdersLessThanZero { msg: err.msg },
            -4013 => ApiError::PriceLessThanMinPrice { msg: err.msg },
            -4014 => ApiError::PriceNotIncreasedByTickSize { msg: err.msg },
            -4015 => ApiError::InvalidClOrdIdLen { msg: err.msg },
            -4016 => ApiError::PriceHighterThanMultiplierUp { msg: err.msg },
            -4017 => ApiError::MultiplierUpLessThanZero { msg: err.msg },
            -4018 => ApiError::MultiplierDownLessThanZero { msg: err.msg },
            -4019 => ApiError::CompositeScaleOverflow { msg: err.msg },
            -4020 => ApiError::TargetStrategyInvalid { msg: err.msg },
            -4021 => ApiError::InvalidDepthLimit { msg: err.msg },
            -4022 => ApiError::WrongMarketStatus { msg: err.msg },
            -4023 => ApiError::QtyNotIncreasedByStepSize { msg: err.msg },
            -4024 => ApiError::PriceLowerThanMultiplierDown { msg: err.msg },
            -4025 => ApiError::MultiplierDecimalLessThanZero { msg: err.msg },
            -4026 => ApiError::CommissionInvalid { msg: err.msg },
            -4027 => ApiError::InvalidAccountType { msg: err.msg },
            -4028 => ApiError::InvalidLeverage { msg: err.msg },
            -4029 => ApiError::InvalidTickSizePrecision { msg: err.msg },
            -4030 => ApiError::InvalidStepSizePrecision { msg: err.msg },
            -4031 => ApiError::InvalidWorkingType { msg: err.msg },
            -4032 => ApiError::ExceedMaxCancelOrderSize { msg: err.msg },
            -4033 => ApiError::InsuranceAccountNotFound { msg: err.msg },
            -4044 => ApiError::InvalidBalanceType { msg: err.msg },
            -4045 => ApiError::MaxStopOrderExceeded { msg: err.msg },
            -4046 => ApiError::NoNeedToChangeMarginType { msg: err.msg },
            -4047 => ApiError::ThereExistsOpenOrders { msg: err.msg },
            -4048 => ApiError::ThereExistsQuantity { msg: err.msg },
            -4049 => ApiError::AddIsolatedMarginReject { msg: err.msg },
            -4050 => ApiError::CrossBalanceInsufficient { msg: err.msg },
            -4051 => ApiError::IsolatedBalanceInsufficient { msg: err.msg },
            -4052 => ApiError::NoNeedToChangeAutoAddMargin { msg: err.msg },
            -4053 => ApiError::AutoAddCrossedMarginReject { msg: err.msg },
            -4054 => ApiError::AddIsolatedMarginNoPositionReject { msg: err.msg },
            -4055 => ApiError::AmountMustBePositive { msg: err.msg },
            -4056 => ApiError::InvalidApiKeyType { msg: err.msg },
            -4057 => ApiError::InvalidRsaPublicKey { msg: err.msg },
            -4058 => ApiError::MaxPriceTooLarge { msg: err.msg },
            -4059 => ApiError::NoNeedToChangePositionSide { msg: err.msg },
            -4060 => ApiError::InvalidPositionSide { msg: err.msg },
            -4061 => ApiError::PositionSideNotMatch { msg: err.msg },
            -4062 => ApiError::ReduceOnlyConflict { msg: err.msg },
            -4067 => ApiError::PositionSideChangeExistsOpenOrders { msg: err.msg },
            -4068 => ApiError::PositionSideChangeExistsQuantity { msg: err.msg },
            -4082 => ApiError::InvalidBatchPlaceOrderSize { msg: err.msg },
            -4083 => ApiError::PlaceBatchOrdersFail { msg: err.msg },
            -4084 => ApiError::UpcomingMethod { msg: err.msg },
            -4086 => ApiError::InvalidPriceSpreadThreshold { msg: err.msg },
            -4087 => ApiError::InvalidPair { msg: err.msg },
            -4088 => ApiError::InvalidTimeInterval { msg: err.msg },
            -4089 => ApiError::ReduceOnlyOrderPermission { msg: err.msg },
            -4090 => ApiError::NoPlaceOrderPermission { msg: err.msg },
            -4104 => ApiError::InvalidContractType { msg: err.msg },
            -4110 => ApiError::InvalidClientTranIdLen { msg: err.msg },
            -4111 => ApiError::DuplicatedClientTranId { msg: err.msg },
            -4112 => ApiError::ReduceOnlyMarginCheckFailed { msg: err.msg },
            -4113 => ApiError::MarketOrderReject { msg: err.msg },
            -4135 => ApiError::InvalidActivationPrice { msg: err.msg },
            -4137 => ApiError::QuantityExistsWithClosePosition { msg: err.msg },
            -4138 => ApiError::ReduceOnlyMustBeTrue { msg: err.msg },
            -4139 => ApiError::OrderTypeCannotBeMkt { msg: err.msg },
            -4142 => ApiError::StrategyInvalidTriggerPrice { msg: err.msg },
            -4150 => ApiError::IsolatedLeverageRejectWithPosition { msg: err.msg },
            -4151 => ApiError::PriceHighterThanStopMultiplierUp { msg: err.msg },
            -4152 => ApiError::PriceLowerThanStopMultiplierDown { msg: err.msg },
            -4154 => ApiError::StopPriceHigherThanPriceMultiplierLimit { msg: err.msg },
            -4155 => ApiError::StopPriceLowerThanPriceMultiplierLimit { msg: err.msg },
            -4178 => ApiError::MinNotional { msg: err.msg },
            -4188 => ApiError::MeInvalidTimestamp { msg: err.msg },
            -4192 => ApiError::CoolingOffPeriod { msg: err.msg },
            -4194 => ApiError::AdjustLeverageKycFailed { msg: err.msg },
            -4195 => ApiError::AdjustLeverageOneMonthFailed { msg: err.msg },
            -4196 => ApiError::LimitOrderOnly { msg: err.msg },
            -4197 => ApiError::SameOrder { msg: err.msg },
            -4198 => ApiError::ExceedMaxModifyOrderLimit { msg: err.msg },
            -4199 => ApiError::MoveOrderNotAllowedSymbolReason { msg: err.msg },
            -4200 => ApiError::AdjustLeverageXDaysFailed { msg: err.msg },
            -4201 => ApiError::AdjustLeverageKycLimit { msg: err.msg },
            -4202 => ApiError::AdjustLeverageAccountSymbolFailed { msg: err.msg },
            _ => ApiError::UnmappedApiError {
                code: err.code,
                msg: err.msg,
            },
        }
    }
}
