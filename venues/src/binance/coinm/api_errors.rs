use thiserror::Error;
use serde::{Deserialize, Serialize};
use super::http_errors::BinanceCoinMHttpError;

/// Error code ranges:
/// -1000 to -1999: General Server or Network issues
/// -2000 to -2999: Authentication and Authorization errors
/// -3000 to -3999: Rate limiting errors
/// -4000 to -4999: Validation and Processing errors
/// -5000 to -5999: System errors
#[derive(Error, Debug)]
pub enum BinanceCoinMError {
    #[error("HTTP error: {0}")]
    HttpError(#[from] BinanceCoinMHttpError),

    #[error("API error: {code} - {message}")]
    ApiError {
        code: i32,
        message: String,
    },

    /// Error -1000: Unknown error
    /// Occurs when an unexpected error occurs while processing the request
    /// Handle by retrying the request or checking system status
    #[error("Unknown error occurred while processing the request (code: {0})")]
    UnknownApiError(i32),
    
    /// Error -1001: Disconnected
    /// Occurs when the server is unable to process the request
    /// Handle by retrying the request
    #[error("Internal error; unable to process your request. Please try again (code: {0})")]
    Disconnected(i32),
    
    /// Error -1002: Unauthorized
    /// Occurs when the request is not authorized
    /// Handle by checking API key and permissions
    #[error("You are not authorized to execute this request (code: {0})")]
    Unauthorized(i32),
    
    /// Error -1003: Too Many Requests
    /// Occurs when rate limit is exceeded
    /// Handle by implementing rate limiting or reducing request frequency
    #[error("Rate limit exceeded: {1} (code: {0})")]
    TooManyRequests(i32, String),
    
    /// Error -1004: IP Banned
    /// Occurs when IP is banned
    /// Handle by using websocket for live updates
    #[error("IP banned until {1}. Please use the websocket for live updates to avoid bans (code: {0})")]
    IpBanned(i32, String),
    
    #[error("This IP is already on the white list (code: {0})")]
    DuplicateIp(i32),
    
    #[error("No such IP has been white listed (code: {0})")]
    NoSuchIp(i32),
    
    #[error("An unexpected response was received from the message bus. Execution status unknown (code: {0})")]
    UnexpectedResponse(i32),
    
    #[error("Timeout waiting for response from backend server. Send status unknown; execution status unknown (code: {0})")]
    Timeout(i32),
    
    #[error("ERROR_MSG_RECEIVED (code: {0})")]
    ErrorMsgReceived(i32),
    
    #[error("This IP cannot access this route (code: {0})")]
    NonWhiteList(i32),
    
    #[error("INVALID_MESSAGE (code: {0})")]
    InvalidMessage(i32),
    
    #[error("Unsupported order combination (code: {0})")]
    UnknownOrderComposition(i32),
    
    #[error("Too many new orders: {1} (code: {0})")]
    TooManyOrders(i32, String),
    
    #[error("This service is no longer available (code: {0})")]
    ServiceShuttingDown(i32),
    
    #[error("This operation is not supported (code: {0})")]
    UnsupportedOperation(i32),
    
    #[error("Timestamp for this request is outside of the recvWindow: {1} (code: {0})")]
    InvalidTimestamp(i32, String),
    
    #[error("Signature for this request is not valid (code: {0})")]
    InvalidSignature(i32),
    
    #[error("Start time is greater than end time (code: {0})")]
    StartTimeGreaterThanEndTime(i32),

    // 11xx - Request issues
    #[error("Illegal characters found in parameter '{1}'; legal range is '{2}' (code: {0})")]
    IllegalChars(i32, String, String),
    
    #[error("Too many parameters; expected '{1}' and received '{2}' (code: {0})")]
    TooManyParameters(i32, String, String),
    
    #[error("A mandatory parameter was not sent, was empty/null, or malformed: {1} (code: {0})")]
    MandatoryParamEmptyOrMalformed(i32, String),
    
    #[error("An unknown parameter was sent (code: {0})")]
    UnknownParam(i32),
    
    #[error("Not all sent parameters were read; read '{1}' parameter(s) but was sent '{2}' (code: {0})")]
    UnreadParameters(i32, String, String),
    
    #[error("Parameter '{1}' was empty (code: {0})")]
    ParamEmpty(i32, String),
    
    #[error("Parameter '{1}' sent when not required (code: {0})")]
    ParamNotRequired(i32, String),
    
    #[error("Invalid asset (code: {0})")]
    BadAsset(i32),
    
    #[error("Invalid account (code: {0})")]
    BadAccount(i32),
    
    #[error("Invalid symbolType (code: {0})")]
    BadInstrumentType(i32),
    
    #[error("Precision is over the maximum defined for this asset (code: {0})")]
    BadPrecision(i32),
    
    #[error("No orders on book for symbol (code: {0})")]
    NoDepth(i32),
    
    #[error("Withdrawal amount must be negative (code: {0})")]
    WithdrawNotNegative(i32),
    
    #[error("TimeInForce parameter sent when not required (code: {0})")]
    TifNotRequired(i32),
    
    #[error("Invalid timeInForce (code: {0})")]
    InvalidTif(i32),
    
    #[error("Invalid orderType (code: {0})")]
    InvalidOrderType(i32),
    
    #[error("Invalid side (code: {0})")]
    InvalidSide(i32),
    
    #[error("New client order ID was empty (code: {0})")]
    EmptyNewClOrdId(i32),
    
    #[error("Original client order ID was empty (code: {0})")]
    EmptyOrgClOrdId(i32),
    
    #[error("Invalid interval (code: {0})")]
    BadInterval(i32),
    
    #[error("Invalid symbol (code: {0})")]
    BadSymbol(i32),
    
    #[error("This listenKey does not exist. Please use POST /fapi/v1/listenKey to recreate listenKey (code: {0})")]
    InvalidListenKey(i32),
    
    #[error("Lookup interval is too big: {1} (code: {0})")]
    MoreThanXxHours(i32, String),
    
    #[error("Combination of optional parameters invalid (code: {0})")]
    OptionalParamsBadCombo(i32),
    
    #[error("Invalid data sent for parameter '{1}': {2} (code: {0})")]
    InvalidParameter(i32, String, String),
    
    #[error("Invalid newOrderRespType (code: {0})")]
    InvalidNewOrderRespType(i32),

    // 20xx - Processing Issues
    #[error("NEW_ORDER_REJECTED (code: {0})")]
    NewOrderRejected(i32),
    
    #[error("CANCEL_REJECTED (code: {0})")]
    CancelRejected(i32),
    
    #[error("Order does not exist (code: {0})")]
    NoSuchOrder(i32),
    
    #[error("API-key format invalid (code: {0})")]
    BadApiKeyFmt(i32),
    
    #[error("Invalid API-key, IP, or permissions for action (code: {0})")]
    RejectedMbxKey(i32),
    
    #[error("No trading window could be found for the symbol. Try ticker/24hrs instead (code: {0})")]
    NoTradingWindow(i32),
    
    #[error("Balance is insufficient (code: {0})")]
    BalanceNotSufficient(i32),
    
    #[error("Margin is insufficient (code: {0})")]
    MarginNotSufficient(i32),
    
    #[error("Unable to fill (code: {0})")]
    UnableToFill(i32),
    
    #[error("Order would immediately trigger (code: {0})")]
    OrderWouldImmediatelyTrigger(i32),
    
    #[error("ReduceOnly Order is rejected (code: {0})")]
    ReduceOnlyReject(i32),
    
    #[error("User in liquidation mode now (code: {0})")]
    UserInLiquidation(i32),
    
    #[error("Position is not sufficient (code: {0})")]
    PositionNotSufficient(i32),
    
    #[error("Reach max open order limit (code: {0})")]
    MaxOpenOrderExceeded(i32),
    
    #[error("This OrderType is not supported when reduceOnly (code: {0})")]
    ReduceOnlyOrderTypeNotSupported(i32),
    
    #[error("Exceeded the maximum allowable position at current leverage (code: {0})")]
    MaxLeverageRatio(i32),
    
    #[error("Leverage is smaller than permitted: insufficient margin balance (code: {0})")]
    MinLeverageRatio(i32),

    // 40xx - Filters and other Issues
    #[error("Invalid order status (code: {0})")]
    InvalidOrderStatus(i32),
    
    #[error("Price less than 0 (code: {0})")]
    PriceLessThanZero(i32),
    
    #[error("Price greater than max price (code: {0})")]
    PriceGreaterThanMaxPrice(i32),
    
    #[error("Quantity less than zero (code: {0})")]
    QtyLessThanZero(i32),
    
    #[error("Quantity less than min quantity (code: {0})")]
    QtyLessThanMinQty(i32),
    
    #[error("Quantity greater than max quantity (code: {0})")]
    QtyGreaterThanMaxQty(i32),
    
    #[error("Stop price less than zero (code: {0})")]
    StopPriceLessThanZero(i32),
    
    #[error("Stop price greater than max price (code: {0})")]
    StopPriceGreaterThanMaxPrice(i32),
    
    #[error("Tick size less than zero (code: {0})")]
    TickSizeLessThanZero(i32),
    
    #[error("Max price less than min price (code: {0})")]
    MaxPriceLessThanMinPrice(i32),
    
    #[error("Max qty less than min qty (code: {0})")]
    MaxQtyLessThanMinQty(i32),
    
    #[error("Step size less than zero (code: {0})")]
    StepSizeLessThanZero(i32),
    
    #[error("Max mum orders less than zero (code: {0})")]
    MaxNumOrdersLessThanZero(i32),
    
    #[error("Price less than min price (code: {0})")]
    PriceLessThanMinPrice(i32),
    
    #[error("Price not increased by tick size (code: {0})")]
    PriceNotIncreasedByTickSize(i32),
    
    #[error("Client order id length should not be more than 36 chars (code: {0})")]
    InvalidClOrdIdLen(i32),
    
    #[error("Price is higher than mark price multiplier cap (code: {0})")]
    PriceHighterThanMultiplierUp(i32),
    
    #[error("Multiplier up less than zero (code: {0})")]
    MultiplierUpLessThanZero(i32),
    
    #[error("Multiplier down less than zero (code: {0})")]
    MultiplierDownLessThanZero(i32),
    
    #[error("Composite scale too large (code: {0})")]
    CompositeScaleOverflow(i32),
    
    #[error("Target strategy invalid for orderType '{1}', reduceOnly '{2}' (code: {0})")]
    TargetStrategyInvalid(i32, String, bool),
    
    #[error("Invalid depth limit: {1} (code: {0})")]
    InvalidDepthLimit(i32, String),
    
    #[error("Market status sent is not valid (code: {0})")]
    WrongMarketStatus(i32),
    
    #[error("Qty not increased by step size (code: {0})")]
    QtyNotIncreasedByStepSize(i32),
    
    #[error("Price is lower than mark price multiplier floor (code: {0})")]
    PriceLowerThanMultiplierDown(i32),
    
    #[error("Multiplier decimal less than zero (code: {0})")]
    MultiplierDecimalLessThanZero(i32),
    
    #[error("Commission invalid: {1} (code: {0})")]
    CommissionInvalid(i32, String),
    
    #[error("Invalid account type (code: {0})")]
    InvalidAccountType(i32),
    
    #[error("Invalid leverage: {1} (code: {0})")]
    InvalidLeverage(i32, String),
    
    #[error("Tick size precision is invalid (code: {0})")]
    InvalidTickSizePrecision(i32),
    
    #[error("Step size precision is invalid (code: {0})")]
    InvalidStepSizePrecision(i32),
    
    #[error("Invalid parameter working type: {1} (code: {0})")]
    InvalidWorkingType(i32, String),
    
    #[error("Exceed maximum cancel order size (code: {0})")]
    ExceedMaxCancelOrderSize(i32),
    
    #[error("Insurance account not found (code: {0})")]
    InsuranceAccountNotFound(i32),
    
    #[error("Balance Type is invalid (code: {0})")]
    InvalidBalanceType(i32),
    
    #[error("Reach max stop order limit (code: {0})")]
    MaxStopOrderExceeded(i32),
    
    #[error("No need to change margin type (code: {0})")]
    NoNeedToChangeMarginType(i32),
    
    #[error("Margin type cannot be changed if there exists open orders (code: {0})")]
    ThereExistsOpenOrders(i32),
    
    #[error("Margin type cannot be changed if there exists position (code: {0})")]
    ThereExistsQuantity(i32),
    
    #[error("Add margin only support for isolated position (code: {0})")]
    AddIsolatedMarginReject(i32),
    
    #[error("Cross balance insufficient (code: {0})")]
    CrossBalanceInsufficient(i32),
    
    #[error("Isolated balance insufficient (code: {0})")]
    IsolatedBalanceInsufficient(i32),
    
    #[error("No need to change auto add margin (code: {0})")]
    NoNeedToChangeAutoAddMargin(i32),
    
    #[error("Auto add margin only support for isolated position (code: {0})")]
    AutoAddCrossedMarginReject(i32),
    
    #[error("Cannot add position margin: position is 0 (code: {0})")]
    AddIsolatedMarginNoPositionReject(i32),
    
    #[error("Amount must be positive (code: {0})")]
    AmountMustBePositive(i32),
    
    #[error("Invalid api key type (code: {0})")]
    InvalidApiKeyType(i32),
    
    #[error("Invalid api public key (code: {0})")]
    InvalidRsaPublicKey(i32),
    
    #[error("maxPrice and priceDecimal too large, please check (code: {0})")]
    MaxPriceTooLarge(i32),
    
    #[error("No need to change position side (code: {0})")]
    NoNeedToChangePositionSide(i32),
    
    #[error("Invalid position side (code: {0})")]
    InvalidPositionSide(i32),
    
    #[error("Order's position side does not match user's setting (code: {0})")]
    PositionSideNotMatch(i32),
    
    #[error("Invalid or improper reduceOnly value (code: {0})")]
    ReduceOnlyConflict(i32),
    
    #[error("Position side cannot be changed if there exists open orders (code: {0})")]
    PositionSideChangeExistsOpenOrders(i32),
    
    #[error("Position side cannot be changed if there exists position (code: {0})")]
    PositionSideChangeExistsQuantity(i32),
    
    #[error("Invalid number of batch place orders: {1} (code: {0})")]
    InvalidBatchPlaceOrderSize(i32, String),
    
    #[error("Fail to place batch orders (code: {0})")]
    PlaceBatchOrdersFail(i32),
    
    #[error("Method is not allowed currently. Upcoming soon (code: {0})")]
    UpcomingMethod(i32),
    
    #[error("Invalid price spread threshold (code: {0})")]
    InvalidPriceSpreadThreshold(i32),
    
    #[error("Invalid pair (code: {0})")]
    InvalidPair(i32),
    
    #[error("Invalid time interval: {1} (code: {0})")]
    InvalidTimeInterval(i32, String),
    
    #[error("User can only place reduce only order (code: {0})")]
    ReduceOnlyOrderPermission(i32),
    
    #[error("User can not place order currently (code: {0})")]
    NoPlaceOrderPermission(i32),
    
    #[error("Invalid contract type (code: {0})")]
    InvalidContractType(i32),
    
    #[error("clientTranId length should be less than 64 chars (code: {0})")]
    InvalidClientTranIdLen(i32),
    
    #[error("clientTranId is duplicated. Client tran id should be unique within 7 days (code: {0})")]
    DuplicatedClientTranId(i32),
    
    #[error("ReduceOnly Order Failed. Please check your existing position and open orders (code: {0})")]
    ReduceOnlyMarginCheckFailed(i32),
    
    #[error("The counterparty's best price does not meet the PERCENT_PRICE filter limit (code: {0})")]
    MarketOrderReject(i32),
    
    #[error("Invalid activation price (code: {0})")]
    InvalidActivationPrice(i32),
    
    #[error("Quantity must be zero with closePosition equals true (code: {0})")]
    QuantityExistsWithClosePosition(i32),
    
    #[error("Reduce only must be true with closePosition equals true (code: {0})")]
    ReduceOnlyMustBeTrue(i32),
    
    #[error("Order type can not be market if it's unable to cancel (code: {0})")]
    OrderTypeCannotBeMkt(i32),
    
    #[error("REJECT: take profit or stop order will be triggered immediately (code: {0})")]
    StrategyInvalidTriggerPrice(i32),
    
    #[error("Leverage reduction is not supported in Isolated Margin Mode with open positions (code: {0})")]
    IsolatedLeverageRejectWithPosition(i32),
    
    #[error("Price is higher than stop price multiplier cap. Limit price can't be higher than {1} (code: {0})")]
    PriceHighterThanStopMultiplierUp(i32, String),
    
    #[error("Price is lower than stop price multiplier floor. Limit price can't be lower than {1} (code: {0})")]
    PriceLowerThanStopMultiplierDown(i32, String),
    
    #[error("Stop price is higher than price multiplier cap. Stop price can't be higher than {1} (code: {0})")]
    StopPriceHigherThanPriceMultiplierLimit(i32, String),
    
    #[error("Stop price is lower than price multiplier floor. Stop price can't be lower than {1} (code: {0})")]
    StopPriceLowerThanPriceMultiplierLimit(i32, String),
    
    #[error("Order's notional must be no smaller than {1} (unless you choose reduce only) (code: {0})")]
    MinNotional(i32, String),
    
    #[error("Trade forbidden due to Cooling-off Period (code: {0})")]
    CoolingOffPeriod(i32),
    
    #[error("Intermediate Personal Verification is required for adjusting leverage over 20x (code: {0})")]
    AdjustLeverageKycFailed(i32),
    
    #[error("More than 20x leverage is available one month after account registration (code: {0})")]
    AdjustLeverageOneMonthFailed(i32),
    
    #[error("Only limit order is supported (code: {0})")]
    LimitOrderOnly(i32),
    
    #[error("No need to modify the order (code: {0})")]
    SameOrder(i32),
    
    #[error("Exceed maximum modify order limit (code: {0})")]
    ExceedMaxModifyOrderLimit(i32),
    
    #[error("Symbol is not in trading status. Order amendment is not permitted (code: {0})")]
    MoveOrderNotAllowedSymbolReason(i32),
    
    #[error("More than 20x leverage is available {1} days after Futures account registration (code: {0})")]
    AdjustLeverageXDaysFailed(i32, String),
    
    #[error("Users in your location/country can only access a maximum leverage of {1} (code: {0})")]
    AdjustLeverageKycLimit(i32, String),
    
    #[error("Current symbol leverage cannot exceed 20 when using position limit adjustment service (code: {0})")]
    AdjustLeverageAccountSymbolFailed(i32),
    
    #[error("Timestamp for this request is outside of the ME recvWindow (code: {0})")]
    MeInvalidTimestamp(i32),

    /// Error -5000: Invalid Margin Parameter
    /// Occurs when an invalid margin parameter is provided
    #[error("Invalid Margin Parameter (code: {0})")]
    InvalidMarginParameter(i32)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BinanceErrorResponse {
    pub code: i32,
    pub msg: String,
}

impl From<BinanceErrorResponse> for BinanceCoinMError {
    fn from(err: BinanceErrorResponse) -> Self {
        match err.code {
            // 10xx - General Server or Network issues
            -1000 => BinanceCoinMError::UnknownApiError(err.code),
            -1001 => BinanceCoinMError::Disconnected(err.code),
            -1002 => BinanceCoinMError::Unauthorized(err.code),
            -1003 => BinanceCoinMError::TooManyRequests(err.code, err.msg),
            -1004 => BinanceCoinMError::DuplicateIp(err.code),
            -1005 => BinanceCoinMError::NoSuchIp(err.code),
            -1006 => BinanceCoinMError::UnexpectedResponse(err.code),
            -1007 => BinanceCoinMError::Timeout(err.code),
            -1010 => BinanceCoinMError::ErrorMsgReceived(err.code),
            -1011 => BinanceCoinMError::NonWhiteList(err.code),
            -1013 => BinanceCoinMError::InvalidMessage(err.code),
            -1014 => BinanceCoinMError::UnknownOrderComposition(err.code),
            -1015 => BinanceCoinMError::TooManyOrders(err.code, err.msg),
            -1016 => BinanceCoinMError::ServiceShuttingDown(err.code),
            -1020 => BinanceCoinMError::UnsupportedOperation(err.code),
            -1021 => BinanceCoinMError::InvalidTimestamp(err.code, err.msg),
            -1022 => BinanceCoinMError::InvalidSignature(err.code),
            -1023 => BinanceCoinMError::StartTimeGreaterThanEndTime(err.code),

            // 11xx - Request issues
            -1100 => BinanceCoinMError::IllegalChars(err.code, err.msg, "unknown".to_string()),
            -1101 => BinanceCoinMError::TooManyParameters(err.code, "unknown".to_string(), "unknown".to_string()),
            -1102 => BinanceCoinMError::MandatoryParamEmptyOrMalformed(err.code, err.msg),
            -1103 => BinanceCoinMError::UnknownParam(err.code),
            -1104 => BinanceCoinMError::UnreadParameters(err.code, "unknown".to_string(), "unknown".to_string()),
            -1105 => BinanceCoinMError::ParamEmpty(err.code, err.msg),
            -1106 => BinanceCoinMError::ParamNotRequired(err.code, err.msg),
            -1108 => BinanceCoinMError::BadAsset(err.code),
            -1109 => BinanceCoinMError::BadAccount(err.code),
            -1110 => BinanceCoinMError::BadInstrumentType(err.code),
            -1111 => BinanceCoinMError::BadPrecision(err.code),
            -1112 => BinanceCoinMError::NoDepth(err.code),
            -1113 => BinanceCoinMError::WithdrawNotNegative(err.code),
            -1114 => BinanceCoinMError::TifNotRequired(err.code),
            -1115 => BinanceCoinMError::InvalidTif(err.code),
            -1116 => BinanceCoinMError::InvalidOrderType(err.code),
            -1117 => BinanceCoinMError::InvalidSide(err.code),
            -1118 => BinanceCoinMError::EmptyNewClOrdId(err.code),
            -1119 => BinanceCoinMError::EmptyOrgClOrdId(err.code),
            -1120 => BinanceCoinMError::BadInterval(err.code),
            -1121 => BinanceCoinMError::BadSymbol(err.code),
            -1125 => BinanceCoinMError::InvalidListenKey(err.code),
            -1127 => BinanceCoinMError::MoreThanXxHours(err.code, err.msg),
            -1128 => BinanceCoinMError::OptionalParamsBadCombo(err.code),
            -1130 => BinanceCoinMError::InvalidParameter(err.code, "unknown".to_string(), err.msg),
            -1136 => BinanceCoinMError::InvalidNewOrderRespType(err.code),

            // 20xx - Processing Issues
            -2010 => BinanceCoinMError::NewOrderRejected(err.code),
            -2011 => BinanceCoinMError::CancelRejected(err.code),
            -2013 => BinanceCoinMError::NoSuchOrder(err.code),
            -2014 => BinanceCoinMError::BadApiKeyFmt(err.code),
            -2015 => BinanceCoinMError::RejectedMbxKey(err.code),
            -2016 => BinanceCoinMError::NoTradingWindow(err.code),
            -2018 => BinanceCoinMError::BalanceNotSufficient(err.code),
            -2019 => BinanceCoinMError::MarginNotSufficient(err.code),
            -2020 => BinanceCoinMError::UnableToFill(err.code),
            -2021 => BinanceCoinMError::OrderWouldImmediatelyTrigger(err.code),
            -2022 => BinanceCoinMError::ReduceOnlyReject(err.code),
            -2023 => BinanceCoinMError::UserInLiquidation(err.code),
            -2024 => BinanceCoinMError::PositionNotSufficient(err.code),
            -2025 => BinanceCoinMError::MaxOpenOrderExceeded(err.code),
            -2026 => BinanceCoinMError::ReduceOnlyOrderTypeNotSupported(err.code),
            -2027 => BinanceCoinMError::MaxLeverageRatio(err.code),
            -2028 => BinanceCoinMError::MinLeverageRatio(err.code),

            // 40xx - Filters and other Issues
            -4000 => BinanceCoinMError::InvalidOrderStatus(err.code),
            -4001 => BinanceCoinMError::PriceLessThanZero(err.code),
            -4002 => BinanceCoinMError::PriceGreaterThanMaxPrice(err.code),
            -4003 => BinanceCoinMError::QtyLessThanZero(err.code),
            -4004 => BinanceCoinMError::QtyLessThanMinQty(err.code),
            -4005 => BinanceCoinMError::QtyGreaterThanMaxQty(err.code),
            -4006 => BinanceCoinMError::StopPriceLessThanZero(err.code),
            -4007 => BinanceCoinMError::StopPriceGreaterThanMaxPrice(err.code),
            -4008 => BinanceCoinMError::TickSizeLessThanZero(err.code),
            -4009 => BinanceCoinMError::MaxPriceLessThanMinPrice(err.code),
            -4010 => BinanceCoinMError::MaxQtyLessThanMinQty(err.code),
            -4011 => BinanceCoinMError::StepSizeLessThanZero(err.code),
            -4012 => BinanceCoinMError::MaxNumOrdersLessThanZero(err.code),
            -4013 => BinanceCoinMError::PriceLessThanMinPrice(err.code),
            -4014 => BinanceCoinMError::PriceNotIncreasedByTickSize(err.code),
            -4015 => BinanceCoinMError::InvalidClOrdIdLen(err.code),
            -4016 => BinanceCoinMError::PriceHighterThanMultiplierUp(err.code),
            -4017 => BinanceCoinMError::MultiplierUpLessThanZero(err.code),
            -4018 => BinanceCoinMError::MultiplierDownLessThanZero(err.code),
            -4019 => BinanceCoinMError::CompositeScaleOverflow(err.code),
            -4020 => BinanceCoinMError::TargetStrategyInvalid(err.code, "unknown".to_string(), false),
            -4021 => BinanceCoinMError::InvalidDepthLimit(err.code, err.msg),
            -4022 => BinanceCoinMError::WrongMarketStatus(err.code),
            -4023 => BinanceCoinMError::QtyNotIncreasedByStepSize(err.code),
            -4024 => BinanceCoinMError::PriceLowerThanMultiplierDown(err.code),
            -4025 => BinanceCoinMError::MultiplierDecimalLessThanZero(err.code),
            -4026 => BinanceCoinMError::CommissionInvalid(err.code, err.msg),
            -4027 => BinanceCoinMError::InvalidAccountType(err.code),
            -4028 => BinanceCoinMError::InvalidLeverage(err.code, err.msg),
            -4029 => BinanceCoinMError::InvalidTickSizePrecision(err.code),
            -4030 => BinanceCoinMError::InvalidStepSizePrecision(err.code),
            -4031 => BinanceCoinMError::InvalidWorkingType(err.code, err.msg),
            -4032 => BinanceCoinMError::ExceedMaxCancelOrderSize(err.code),
            -4033 => BinanceCoinMError::InsuranceAccountNotFound(err.code),
            -4044 => BinanceCoinMError::InvalidBalanceType(err.code),
            -4045 => BinanceCoinMError::MaxStopOrderExceeded(err.code),
            -4046 => BinanceCoinMError::NoNeedToChangeMarginType(err.code),
            -4047 => BinanceCoinMError::ThereExistsOpenOrders(err.code),
            -4048 => BinanceCoinMError::ThereExistsQuantity(err.code),
            -4049 => BinanceCoinMError::AddIsolatedMarginReject(err.code),
            -4050 => BinanceCoinMError::CrossBalanceInsufficient(err.code),
            -4051 => BinanceCoinMError::IsolatedBalanceInsufficient(err.code),
            -4052 => BinanceCoinMError::NoNeedToChangeAutoAddMargin(err.code),
            -4053 => BinanceCoinMError::AutoAddCrossedMarginReject(err.code),
            -4054 => BinanceCoinMError::AddIsolatedMarginNoPositionReject(err.code),
            -4055 => BinanceCoinMError::AmountMustBePositive(err.code),
            -4056 => BinanceCoinMError::InvalidApiKeyType(err.code),
            -4057 => BinanceCoinMError::InvalidRsaPublicKey(err.code),
            -4058 => BinanceCoinMError::MaxPriceTooLarge(err.code),
            -4059 => BinanceCoinMError::NoNeedToChangePositionSide(err.code),
            -4060 => BinanceCoinMError::InvalidPositionSide(err.code),
            -4061 => BinanceCoinMError::PositionSideNotMatch(err.code),
            -4062 => BinanceCoinMError::ReduceOnlyConflict(err.code),
            -4067 => BinanceCoinMError::PositionSideChangeExistsOpenOrders(err.code),
            -4068 => BinanceCoinMError::PositionSideChangeExistsQuantity(err.code),
            -4082 => BinanceCoinMError::InvalidBatchPlaceOrderSize(err.code, err.msg),
            -4083 => BinanceCoinMError::PlaceBatchOrdersFail(err.code),
            -4084 => BinanceCoinMError::UpcomingMethod(err.code),
            -4086 => BinanceCoinMError::InvalidPriceSpreadThreshold(err.code),
            -4087 => BinanceCoinMError::InvalidPair(err.code),
            -4088 => BinanceCoinMError::InvalidTimeInterval(err.code, err.msg),
            -4089 => BinanceCoinMError::ReduceOnlyOrderPermission(err.code),
            -4090 => BinanceCoinMError::NoPlaceOrderPermission(err.code),
            -4104 => BinanceCoinMError::InvalidContractType(err.code),
            -4110 => BinanceCoinMError::InvalidClientTranIdLen(err.code),
            -4111 => BinanceCoinMError::DuplicatedClientTranId(err.code),
            -4112 => BinanceCoinMError::ReduceOnlyMarginCheckFailed(err.code),
            -4113 => BinanceCoinMError::MarketOrderReject(err.code),
            -4135 => BinanceCoinMError::InvalidActivationPrice(err.code),
            -4137 => BinanceCoinMError::QuantityExistsWithClosePosition(err.code),
            -4138 => BinanceCoinMError::ReduceOnlyMustBeTrue(err.code),
            -4139 => BinanceCoinMError::OrderTypeCannotBeMkt(err.code),
            -4142 => BinanceCoinMError::StrategyInvalidTriggerPrice(err.code),
            -4150 => BinanceCoinMError::IsolatedLeverageRejectWithPosition(err.code),
            -4151 => BinanceCoinMError::PriceHighterThanStopMultiplierUp(err.code, err.msg),
            -4152 => BinanceCoinMError::PriceLowerThanStopMultiplierDown(err.code, err.msg),
            -4154 => BinanceCoinMError::StopPriceHigherThanPriceMultiplierLimit(err.code, err.msg),
            -4155 => BinanceCoinMError::StopPriceLowerThanPriceMultiplierLimit(err.code, err.msg),
            -4178 => BinanceCoinMError::MinNotional(err.code, err.msg),
            -4192 => BinanceCoinMError::CoolingOffPeriod(err.code),
            -4194 => BinanceCoinMError::AdjustLeverageKycFailed(err.code),
            -4195 => BinanceCoinMError::AdjustLeverageOneMonthFailed(err.code),
            -4196 => BinanceCoinMError::LimitOrderOnly(err.code),
            -4197 => BinanceCoinMError::SameOrder(err.code),
            -4198 => BinanceCoinMError::ExceedMaxModifyOrderLimit(err.code),
            -4199 => BinanceCoinMError::MoveOrderNotAllowedSymbolReason(err.code),
            -4200 => BinanceCoinMError::AdjustLeverageXDaysFailed(err.code, err.msg),
            -4201 => BinanceCoinMError::AdjustLeverageKycLimit(err.code, err.msg),
            -4202 => BinanceCoinMError::AdjustLeverageAccountSymbolFailed(err.code),
            -4188 => BinanceCoinMError::MeInvalidTimestamp(err.code),
            
            // 5xxx - System errors
            -5000 => BinanceCoinMError::InvalidMarginParameter(err.code),

            // Unknown error code
            _ => BinanceCoinMError::UnknownApiError(err.code),
        }
    }
}

pub type BinanceCoinMResult<T> = Result<T, BinanceCoinMError>; 