use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderSide {
    Buy,
    Sell,
}

impl fmt::Display for OrderSide {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrderSide::Buy => write!(f, "BUY"),
            OrderSide::Sell => write!(f, "SELL"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderType {
    Limit,
    Market,
    StopLoss,
    StopLossLimit,
    TakeProfit,
    TakeProfitLimit,
    LimitMaker,
}

impl fmt::Display for OrderType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrderType::Limit => write!(f, "LIMIT"),
            OrderType::Market => write!(f, "MARKET"),
            OrderType::StopLoss => write!(f, "STOP_LOSS"),
            OrderType::StopLossLimit => write!(f, "STOP_LOSS_LIMIT"),
            OrderType::TakeProfit => write!(f, "TAKE_PROFIT"),
            OrderType::TakeProfitLimit => write!(f, "TAKE_PROFIT_LIMIT"),
            OrderType::LimitMaker => write!(f, "LIMIT_MAKER"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum TimeInForce {
    /// Good Till Cancel
    GTC,
    /// Immediate or Cancel
    IOC,
    /// Fill or Kill
    FOK,
}

impl fmt::Display for TimeInForce {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TimeInForce::GTC => write!(f, "GTC"),
            TimeInForce::IOC => write!(f, "IOC"),
            TimeInForce::FOK => write!(f, "FOK"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderStatus {
    /// The order has been accepted by the system but not yet filled.
    New,
    /// The order has been partially filled.
    PartiallyFilled,
    /// The order has been completely filled.
    Filled,
    /// The order has been canceled by the user.
    Canceled,
    /// The order has been canceled due to STP.
    PendingCancel,
    /// The order has been rejected.
    Rejected,
    /// The order has expired.
    Expired,
    /// The order has expired in the matching engine.
    ExpiredInMatch,
}

impl fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrderStatus::New => write!(f, "NEW"),
            OrderStatus::PartiallyFilled => write!(f, "PARTIALLY_FILLED"),
            OrderStatus::Filled => write!(f, "FILLED"),
            OrderStatus::Canceled => write!(f, "CANCELED"),
            OrderStatus::PendingCancel => write!(f, "PENDING_CANCEL"),
            OrderStatus::Rejected => write!(f, "REJECTED"),
            OrderStatus::Expired => write!(f, "EXPIRED"),
            OrderStatus::ExpiredInMatch => write!(f, "EXPIRED_IN_MATCH"),
        }
    }
}

/// Represents the response type for new order requests.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderResponseType {
    Ack,
    Result,
    Full,
}

impl fmt::Display for OrderResponseType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrderResponseType::Ack => write!(f, "ACK"),
            OrderResponseType::Result => write!(f, "RESULT"),
            OrderResponseType::Full => write!(f, "FULL"),
        }
    }
}

/// Represents the self-trade prevention mode for orders.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SelfTradePreventionMode {
    None,
    ExpireTaker,
    ExpireMaker,
    ExpireBoth,
}

impl fmt::Display for SelfTradePreventionMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SelfTradePreventionMode::None => write!(f, "NONE"),
            SelfTradePreventionMode::ExpireTaker => write!(f, "EXPIRE_TAKER"),
            SelfTradePreventionMode::ExpireMaker => write!(f, "EXPIRE_MAKER"),
            SelfTradePreventionMode::ExpireBoth => write!(f, "EXPIRE_BOTH"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SymbolStatus {
    PreTrading,
    Trading,
    PostTrading,
    EndOfDay,
    Halt,
    AuctionMatch,
    Break,
}

/// Represents the kline/candlestick chart intervals.
///
/// m -> minutes; h -> hours; d -> days; w -> weeks; M -> months
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum KlineInterval {
    #[serde(rename = "1s")]
    I1s,
    #[serde(rename = "1m")]
    I1m,
    #[serde(rename = "3m")]
    I3m,
    #[serde(rename = "5m")]
    I5m,
    #[serde(rename = "15m")]
    I15m,
    #[serde(rename = "30m")]
    I30m,
    #[serde(rename = "1h")]
    I1h,
    #[serde(rename = "2h")]
    I2h,
    #[serde(rename = "4h")]
    I4h,
    #[serde(rename = "6h")]
    I6h,
    #[serde(rename = "8h")]
    I8h,
    #[serde(rename = "12h")]
    I12h,
    #[serde(rename = "1d")]
    I1d,
    #[serde(rename = "3d")]
    I3d,
    #[serde(rename = "1w")]
    I1w,
    #[serde(rename = "1M")]
    I1M,
}

/// Represents the cancel restrictions for order cancellation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CancelRestrictions {
    OnlyNew,
    OnlyPartiallyFilled,
}

impl fmt::Display for CancelRestrictions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CancelRestrictions::OnlyNew => write!(f, "ONLY_NEW"),
            CancelRestrictions::OnlyPartiallyFilled => write!(f, "ONLY_PARTIALLY_FILLED"),
        }
    }
}

/// Represents the cancel replace mode for cancel replace orders.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CancelReplaceMode {
    StopOnFailure,
    AllowFailure,
}

impl fmt::Display for CancelReplaceMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CancelReplaceMode::StopOnFailure => write!(f, "STOP_ON_FAILURE"),
            CancelReplaceMode::AllowFailure => write!(f, "ALLOW_FAILURE"),
        }
    }
}

/// Represents the order rate limit exceeded mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderRateLimitExceededMode {
    DoNothing,
    CancelOnly,
}

impl fmt::Display for OrderRateLimitExceededMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrderRateLimitExceededMode::DoNothing => write!(f, "DO_NOTHING"),
            OrderRateLimitExceededMode::CancelOnly => write!(f, "CANCEL_ONLY"),
        }
    }
}

/// Represents the order list status for OCO, OTO, OTOCO.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderListStatus {
    Response,
    ExecStarted,
    AllDone,
    Reject,
}

impl fmt::Display for OrderListStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrderListStatus::Response => write!(f, "RESPONSE"),
            OrderListStatus::ExecStarted => write!(f, "EXEC_STARTED"),
            OrderListStatus::AllDone => write!(f, "ALL_DONE"),
            OrderListStatus::Reject => write!(f, "REJECT"),
        }
    }
}

/// Represents the order list order status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderListOrderStatus {
    Executing,
    AllDone,
    Reject,
}

impl fmt::Display for OrderListOrderStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrderListOrderStatus::Executing => write!(f, "EXECUTING"),
            OrderListOrderStatus::AllDone => write!(f, "ALL_DONE"),
            OrderListOrderStatus::Reject => write!(f, "REJECT"),
        }
    }
}

/// Represents the contingency type for order lists.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ContingencyType {
    Oco,
    Oto,
    Otoco,
}

impl fmt::Display for ContingencyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ContingencyType::Oco => write!(f, "OCO"),
            ContingencyType::Oto => write!(f, "OTO"),
            ContingencyType::Otoco => write!(f, "OTOCO"),
        }
    }
}

/// Represents the allocation type for SOR.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AllocationType {
    Sor,
}

impl fmt::Display for AllocationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AllocationType::Sor => write!(f, "SOR"),
        }
    }
}
