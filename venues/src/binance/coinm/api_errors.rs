use thiserror::Error;
use super::types::ErrorResponse;

/// Error code ranges:
/// -1000 to -1999: General Server or Network issues
/// -2000 to -2999: Authentication and Authorization errors
/// -3000 to -3999: Rate limiting errors
/// -4000 to -4999: Validation and Processing errors
/// -5000 to -5999: System errors
#[derive(Error, Debug)]
pub enum BinanceCoinMAPIError {
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
}

// Conversion from BinanceErrorResponse to BinanceCoinMAPIError
impl From<ErrorResponse> for BinanceCoinMAPIError {
    fn from(err: ErrorResponse) -> Self {
        match err.code {
            -1000 => BinanceCoinMAPIError::UnknownApiError { msg: err.msg },
            -1001 => BinanceCoinMAPIError::Disconnected { msg: err.msg },
            -1002 => BinanceCoinMAPIError::Unauthorized { msg: err.msg },
            -1003 => BinanceCoinMAPIError::TooManyRequests { msg: err.msg },
            -1004 => BinanceCoinMAPIError::DuplicateIp { msg: err.msg },
            -1005 => BinanceCoinMAPIError::NoSuchIp { msg: err.msg },
            -1006 => BinanceCoinMAPIError::UnexpectedResponse { msg: err.msg },
            -1007 => BinanceCoinMAPIError::Timeout { msg: err.msg },
            -1010 => BinanceCoinMAPIError::ErrorMsgReceived { msg: err.msg },
            -1011 => BinanceCoinMAPIError::NonWhiteList { msg: err.msg },
            -1013 => BinanceCoinMAPIError::InvalidMessage { msg: err.msg },
            -1014 => BinanceCoinMAPIError::UnknownOrderComposition { msg: err.msg },
            -1015 => BinanceCoinMAPIError::TooManyOrders { msg: err.msg },
            -1016 => BinanceCoinMAPIError::ServiceShuttingDown { msg: err.msg },
            -1020 => BinanceCoinMAPIError::UnsupportedOperation { msg: err.msg },
            -1021 => BinanceCoinMAPIError::InvalidTimestamp { msg: err.msg },
            -1022 => BinanceCoinMAPIError::InvalidSignature { msg: err.msg },
            -1023 => BinanceCoinMAPIError::StartTimeGreaterThanEndTime { msg: err.msg },
            -1100 => BinanceCoinMAPIError::IllegalChars { msg: err.msg },
            -1101 => BinanceCoinMAPIError::TooManyParameters { msg: err.msg },
            -1102 => BinanceCoinMAPIError::MandatoryParamEmptyOrMalformed { msg: err.msg },
            -1103 => BinanceCoinMAPIError::UnknownParam { msg: err.msg },
            -1104 => BinanceCoinMAPIError::UnreadParameters { msg: err.msg },
            -1105 => BinanceCoinMAPIError::ParamEmpty { msg: err.msg },
            -1106 => BinanceCoinMAPIError::ParamNotRequired { msg: err.msg },
            -1108 => BinanceCoinMAPIError::BadAsset { msg: err.msg },
            -1109 => BinanceCoinMAPIError::BadAccount { msg: err.msg },
            -1110 => BinanceCoinMAPIError::BadInstrumentType { msg: err.msg },
            -1111 => BinanceCoinMAPIError::BadPrecision { msg: err.msg },
            -1112 => BinanceCoinMAPIError::NoDepth { msg: err.msg },
            -1113 => BinanceCoinMAPIError::WithdrawNotNegative { msg: err.msg },
            -1114 => BinanceCoinMAPIError::TifNotRequired { msg: err.msg },
            -1115 => BinanceCoinMAPIError::InvalidTif { msg: err.msg },
            -1116 => BinanceCoinMAPIError::InvalidOrderType { msg: err.msg },
            -1117 => BinanceCoinMAPIError::InvalidSide { msg: err.msg },
            -1118 => BinanceCoinMAPIError::EmptyNewClOrdId { msg: err.msg },
            -1119 => BinanceCoinMAPIError::EmptyOrgClOrdId { msg: err.msg },
            -1120 => BinanceCoinMAPIError::BadInterval { msg: err.msg },
            -1121 => BinanceCoinMAPIError::BadSymbol { msg: err.msg },
            -1125 => BinanceCoinMAPIError::InvalidListenKey { msg: err.msg },
            -1127 => BinanceCoinMAPIError::MoreThanXxHours { msg: err.msg },
            -1128 => BinanceCoinMAPIError::OptionalParamsBadCombo { msg: err.msg },
            -1130 => BinanceCoinMAPIError::InvalidParameter { msg: err.msg },
            -1136 => BinanceCoinMAPIError::InvalidNewOrderRespType { msg: err.msg },
            -2010 => BinanceCoinMAPIError::NewOrderRejected { msg: err.msg },
            -2011 => BinanceCoinMAPIError::CancelRejected { msg: err.msg },
            -2013 => BinanceCoinMAPIError::NoSuchOrder { msg: err.msg },
            -2014 => BinanceCoinMAPIError::BadApiKeyFmt { msg: err.msg },
            -2015 => BinanceCoinMAPIError::RejectedMbxKey { msg: err.msg },
            -2016 => BinanceCoinMAPIError::NoTradingWindow { msg: err.msg },
            -2018 => BinanceCoinMAPIError::BalanceNotSufficient { msg: err.msg },
            -2019 => BinanceCoinMAPIError::MarginNotSufficient { msg: err.msg },
            -2020 => BinanceCoinMAPIError::UnableToFill { msg: err.msg },
            -2021 => BinanceCoinMAPIError::OrderWouldImmediatelyTrigger { msg: err.msg },
            -2022 => BinanceCoinMAPIError::ReduceOnlyReject { msg: err.msg },
            -2023 => BinanceCoinMAPIError::UserInLiquidation { msg: err.msg },
            -2024 => BinanceCoinMAPIError::PositionNotSufficient { msg: err.msg },
            -2025 => BinanceCoinMAPIError::MaxOpenOrderExceeded { msg: err.msg },
            -2026 => BinanceCoinMAPIError::ReduceOnlyOrderTypeNotSupported { msg: err.msg },
            -2027 => BinanceCoinMAPIError::MaxLeverageRatio { msg: err.msg },
            -2028 => BinanceCoinMAPIError::MinLeverageRatio { msg: err.msg },
            -4000 => BinanceCoinMAPIError::InvalidOrderStatus { msg: err.msg },
            -4001 => BinanceCoinMAPIError::PriceLessThanZero { msg: err.msg },
            -4002 => BinanceCoinMAPIError::PriceGreaterThanMaxPrice { msg: err.msg },
            -4003 => BinanceCoinMAPIError::QtyLessThanZero { msg: err.msg },
            -4004 => BinanceCoinMAPIError::QtyLessThanMinQty { msg: err.msg },
            -4005 => BinanceCoinMAPIError::QtyGreaterThanMaxQty { msg: err.msg },
            -4006 => BinanceCoinMAPIError::StopPriceLessThanZero { msg: err.msg },
            -4007 => BinanceCoinMAPIError::StopPriceGreaterThanMaxPrice { msg: err.msg },
            -4008 => BinanceCoinMAPIError::TickSizeLessThanZero { msg: err.msg },
            -4009 => BinanceCoinMAPIError::MaxPriceLessThanMinPrice { msg: err.msg },
            -4010 => BinanceCoinMAPIError::MaxQtyLessThanMinQty { msg: err.msg },
            -4011 => BinanceCoinMAPIError::StepSizeLessThanZero { msg: err.msg },
            -4012 => BinanceCoinMAPIError::MaxNumOrdersLessThanZero { msg: err.msg },
            -4013 => BinanceCoinMAPIError::PriceLessThanMinPrice { msg: err.msg },
            -4014 => BinanceCoinMAPIError::PriceNotIncreasedByTickSize { msg: err.msg },
            -4015 => BinanceCoinMAPIError::InvalidClOrdIdLen { msg: err.msg },
            -4016 => BinanceCoinMAPIError::PriceHighterThanMultiplierUp { msg: err.msg },
            -4017 => BinanceCoinMAPIError::MultiplierUpLessThanZero { msg: err.msg },
            -4018 => BinanceCoinMAPIError::MultiplierDownLessThanZero { msg: err.msg },
            -4019 => BinanceCoinMAPIError::CompositeScaleOverflow { msg: err.msg },
            -4020 => BinanceCoinMAPIError::TargetStrategyInvalid { msg: err.msg },
            -4021 => BinanceCoinMAPIError::InvalidDepthLimit { msg: err.msg },
            -4022 => BinanceCoinMAPIError::WrongMarketStatus { msg: err.msg },
            -4023 => BinanceCoinMAPIError::QtyNotIncreasedByStepSize { msg: err.msg },
            -4024 => BinanceCoinMAPIError::PriceLowerThanMultiplierDown { msg: err.msg },
            -4025 => BinanceCoinMAPIError::MultiplierDecimalLessThanZero { msg: err.msg },
            -4026 => BinanceCoinMAPIError::CommissionInvalid { msg: err.msg },
            -4027 => BinanceCoinMAPIError::InvalidAccountType { msg: err.msg },
            -4028 => BinanceCoinMAPIError::InvalidLeverage { msg: err.msg },
            -4029 => BinanceCoinMAPIError::InvalidTickSizePrecision { msg: err.msg },
            -4030 => BinanceCoinMAPIError::InvalidStepSizePrecision { msg: err.msg },
            -4031 => BinanceCoinMAPIError::InvalidWorkingType { msg: err.msg },
            -4032 => BinanceCoinMAPIError::ExceedMaxCancelOrderSize { msg: err.msg },
            -4033 => BinanceCoinMAPIError::InsuranceAccountNotFound { msg: err.msg },
            -4044 => BinanceCoinMAPIError::InvalidBalanceType { msg: err.msg },
            -4045 => BinanceCoinMAPIError::MaxStopOrderExceeded { msg: err.msg },
            -4046 => BinanceCoinMAPIError::NoNeedToChangeMarginType { msg: err.msg },
            -4047 => BinanceCoinMAPIError::ThereExistsOpenOrders { msg: err.msg },
            -4048 => BinanceCoinMAPIError::ThereExistsQuantity { msg: err.msg },
            -4049 => BinanceCoinMAPIError::AddIsolatedMarginReject { msg: err.msg },
            -4050 => BinanceCoinMAPIError::CrossBalanceInsufficient { msg: err.msg },
            -4051 => BinanceCoinMAPIError::IsolatedBalanceInsufficient { msg: err.msg },
            -4052 => BinanceCoinMAPIError::NoNeedToChangeAutoAddMargin { msg: err.msg },
            -4053 => BinanceCoinMAPIError::AutoAddCrossedMarginReject { msg: err.msg },
            -4054 => BinanceCoinMAPIError::AddIsolatedMarginNoPositionReject { msg: err.msg },
            -4055 => BinanceCoinMAPIError::AmountMustBePositive { msg: err.msg },
            -4056 => BinanceCoinMAPIError::InvalidApiKeyType { msg: err.msg },
            -4057 => BinanceCoinMAPIError::InvalidRsaPublicKey { msg: err.msg },
            -4058 => BinanceCoinMAPIError::MaxPriceTooLarge { msg: err.msg },
            -4059 => BinanceCoinMAPIError::NoNeedToChangePositionSide { msg: err.msg },
            -4060 => BinanceCoinMAPIError::InvalidPositionSide { msg: err.msg },
            -4061 => BinanceCoinMAPIError::PositionSideNotMatch { msg: err.msg },
            -4062 => BinanceCoinMAPIError::ReduceOnlyConflict { msg: err.msg },
            -4067 => BinanceCoinMAPIError::PositionSideChangeExistsOpenOrders { msg: err.msg },
            -4068 => BinanceCoinMAPIError::PositionSideChangeExistsQuantity { msg: err.msg },
            -4082 => BinanceCoinMAPIError::InvalidBatchPlaceOrderSize { msg: err.msg },
            -4083 => BinanceCoinMAPIError::PlaceBatchOrdersFail { msg: err.msg },
            -4084 => BinanceCoinMAPIError::UpcomingMethod { msg: err.msg },
            -4086 => BinanceCoinMAPIError::InvalidPriceSpreadThreshold { msg: err.msg },
            -4087 => BinanceCoinMAPIError::InvalidPair { msg: err.msg },
            -4088 => BinanceCoinMAPIError::InvalidTimeInterval { msg: err.msg },
            -4089 => BinanceCoinMAPIError::ReduceOnlyOrderPermission { msg: err.msg },
            -4090 => BinanceCoinMAPIError::NoPlaceOrderPermission { msg: err.msg },
            -4104 => BinanceCoinMAPIError::InvalidContractType { msg: err.msg },
            -4110 => BinanceCoinMAPIError::InvalidClientTranIdLen { msg: err.msg },
            -4111 => BinanceCoinMAPIError::DuplicatedClientTranId { msg: err.msg },
            -4112 => BinanceCoinMAPIError::ReduceOnlyMarginCheckFailed { msg: err.msg },
            -4113 => BinanceCoinMAPIError::MarketOrderReject { msg: err.msg },
            -4135 => BinanceCoinMAPIError::InvalidActivationPrice { msg: err.msg },
            -4137 => BinanceCoinMAPIError::QuantityExistsWithClosePosition { msg: err.msg },
            -4138 => BinanceCoinMAPIError::ReduceOnlyMustBeTrue { msg: err.msg },
            -4139 => BinanceCoinMAPIError::OrderTypeCannotBeMkt { msg: err.msg },
            -4142 => BinanceCoinMAPIError::StrategyInvalidTriggerPrice { msg: err.msg },
            -4150 => BinanceCoinMAPIError::IsolatedLeverageRejectWithPosition { msg: err.msg },
            -4151 => BinanceCoinMAPIError::PriceHighterThanStopMultiplierUp { msg: err.msg },
            -4152 => BinanceCoinMAPIError::PriceLowerThanStopMultiplierDown { msg: err.msg },
            -4154 => BinanceCoinMAPIError::StopPriceHigherThanPriceMultiplierLimit { msg: err.msg },
            -4155 => BinanceCoinMAPIError::StopPriceLowerThanPriceMultiplierLimit { msg: err.msg },
            -4178 => BinanceCoinMAPIError::MinNotional { msg: err.msg },
            -4188 => BinanceCoinMAPIError::MeInvalidTimestamp { msg: err.msg },
            -4192 => BinanceCoinMAPIError::CoolingOffPeriod { msg: err.msg },
            -4194 => BinanceCoinMAPIError::AdjustLeverageKycFailed { msg: err.msg },
            -4195 => BinanceCoinMAPIError::AdjustLeverageOneMonthFailed { msg: err.msg },
            -4196 => BinanceCoinMAPIError::LimitOrderOnly { msg: err.msg },
            -4197 => BinanceCoinMAPIError::SameOrder { msg: err.msg },
            -4198 => BinanceCoinMAPIError::ExceedMaxModifyOrderLimit { msg: err.msg },
            -4199 => BinanceCoinMAPIError::MoveOrderNotAllowedSymbolReason { msg: err.msg },
            -4200 => BinanceCoinMAPIError::AdjustLeverageXDaysFailed { msg: err.msg },
            -4201 => BinanceCoinMAPIError::AdjustLeverageKycLimit { msg: err.msg },
            -4202 => BinanceCoinMAPIError::AdjustLeverageAccountSymbolFailed { msg: err.msg },
            _ => BinanceCoinMAPIError::UnmappedApiError { code: err.code, msg: err.msg },
        }
    }
}
