use serde::{Deserialize, Serialize};
use std::fmt;

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
pub enum PositionSide {
    Both,
    Long,
    Short,
}

impl fmt::Display for PositionSide {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PositionSide::Both => write!(f, "BOTH"),
            PositionSide::Long => write!(f, "LONG"),
            PositionSide::Short => write!(f, "SHORT"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderType {
    Limit,
    Market,
    Stop,
    StopMarket,
    TakeProfit,
    TakeProfitMarket,
    TrailingStopMarket,
}

impl fmt::Display for OrderType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrderType::Limit => write!(f, "LIMIT"),
            OrderType::Market => write!(f, "MARKET"),
            OrderType::Stop => write!(f, "STOP"),
            OrderType::StopMarket => write!(f, "STOP_MARKET"),
            OrderType::TakeProfit => write!(f, "TAKE_PROFIT"),
            OrderType::TakeProfitMarket => write!(f, "TAKE_PROFIT_MARKET"),
            OrderType::TrailingStopMarket => write!(f, "TRAILING_STOP_MARKET"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum TimeInForce {
    GTC, // Good Till Cancel
    IOC, // Immediate or Cancel
    FOK, // Fill or Kill
    PostOnly,
}

impl fmt::Display for TimeInForce {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TimeInForce::GTC => write!(f, "GTC"),
            TimeInForce::IOC => write!(f, "IOC"),
            TimeInForce::FOK => write!(f, "FOK"),
            TimeInForce::PostOnly => write!(f, "POST_ONLY"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WorkingType {
    MarkPrice,
    ContractPrice,
}

impl fmt::Display for WorkingType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WorkingType::MarkPrice => write!(f, "MARK_PRICE"),
            WorkingType::ContractPrice => write!(f, "CONTRACT_PRICE"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderStatus {
    New,
    PartiallyFilled,
    Filled,
    Canceled,
    Rejected,
    Expired,
    NewInsurance,
    NewAdl,
}

impl fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrderStatus::New => write!(f, "NEW"),
            OrderStatus::PartiallyFilled => write!(f, "PARTIALLY_FILLED"),
            OrderStatus::Filled => write!(f, "FILLED"),
            OrderStatus::Canceled => write!(f, "CANCELED"),
            OrderStatus::Rejected => write!(f, "REJECTED"),
            OrderStatus::Expired => write!(f, "EXPIRED"),
            OrderStatus::NewInsurance => write!(f, "NEW_INSURANCE"),
            OrderStatus::NewAdl => write!(f, "NEW_ADL"),
        }
    }
}

/// Represents the response type for new order requests.
///
/// Can be set to ACK or RESULT. Default is ACK.
/// Used in the newOrderRespType field.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderResponseType {
    Ack,
    Result,
}

impl fmt::Display for OrderResponseType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrderResponseType::Ack => write!(f, "ACK"),
            OrderResponseType::Result => write!(f, "RESULT"),
        }
    }
}

/// Represents the self-trade prevention mode for orders.
///
/// Can be set to NONE, EXPIRE_TAKER, EXPIRE_MAKER, or EXPIRE_BOTH.
/// Used in the selfTradePreventionMode field.
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
pub enum IncomeType {
    Transfer,
    WelcomeBonus,
    RealizedPnl,
    FundingFee,
    Commission,
    InsuranceClear,
    ReferralKickback,
    CommissionRebate,
    ApiRebate,
    ContReward,
    UsdVsTokenSettlement,
    FeeReward,
    TokenReward,
    TransferIn,
    TransferOut,
}

impl fmt::Display for IncomeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IncomeType::Transfer => write!(f, "TRANSFER"),
            IncomeType::WelcomeBonus => write!(f, "WELCOME_BONUS"),
            IncomeType::RealizedPnl => write!(f, "REALIZED_PNL"),
            IncomeType::FundingFee => write!(f, "FUNDING_FEE"),
            IncomeType::Commission => write!(f, "COMMISSION"),
            IncomeType::InsuranceClear => write!(f, "INSURANCE_CLEAR"),
            IncomeType::ReferralKickback => write!(f, "REFERRAL_KICKBACK"),
            IncomeType::CommissionRebate => write!(f, "COMMISSION_REBATE"),
            IncomeType::ApiRebate => write!(f, "API_REBATE"),
            IncomeType::ContReward => write!(f, "CONT_REWARD"),
            IncomeType::UsdVsTokenSettlement => write!(f, "USD_VS_TOKEN_SETTLEMENT"),
            IncomeType::FeeReward => write!(f, "FEE_REWARD"),
            IncomeType::TokenReward => write!(f, "TOKEN_REWARD"),
            IncomeType::TransferIn => write!(f, "TRANSFER_IN"),
            IncomeType::TransferOut => write!(f, "TRANSFER_OUT"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MarginType {
    Cross,
    Isolated,
}

impl fmt::Display for MarginType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MarginType::Cross => write!(f, "cross"),
            MarginType::Isolated => write!(f, "isolated"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WebSocketEventType {
    DepthUpdate,
}

impl fmt::Display for WebSocketEventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WebSocketEventType::DepthUpdate => write!(f, "depthUpdate"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RateLimitType {
    RequestWeight,
    Orders,
    RawRequests,
    RequestSize,
    OrdersPerSecond,
    OrdersPerDay,
    OrdersPerCountdown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RateLimitInterval {
    Second,
    Minute,
    Day,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ExchangeFilterType {
    ExchangeMaxNumOrders,
    ExchangeMaxNumAlgoOrders,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ContractType {
    Perpetual,
    CurrentMonth,
    NextMonth,
    CurrentQuarter,
    NextQuarter,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UnderlyingType {
    Coin,
    Index,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SymbolFilterType {
    PriceFilter,
    LotSizeFilter,
    MinNotionalFilter,
    MaxNumOrdersFilter,
    MaxNumAlgoOrdersFilter,
    PercentPriceFilter,
    MaxPositionFilter,
    TrailingDataFilter,
}

/// Represents the price match mode for orders.
///
/// Can be set to OPPONENT, OPPONENT_5, OPPONENT_10, OPPONENT_20, QUEUE, QUEUE_5, QUEUE_10, QUEUE_20, or NONE.
/// See: https://developers.binance.com/docs/derivatives/coin-margined-futures/trade/rest-api/Place-Multiple-Orders
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PriceMatch {
    None,
    Opponent,
    Opponent5,
    Opponent10,
    Opponent20,
    Queue,
    Queue5,
    Queue10,
    Queue20,
}

impl fmt::Display for PriceMatch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PriceMatch::None => write!(f, "NONE"),
            PriceMatch::Opponent => write!(f, "OPPONENT"),
            PriceMatch::Opponent5 => write!(f, "OPPONENT_5"),
            PriceMatch::Opponent10 => write!(f, "OPPONENT_10"),
            PriceMatch::Opponent20 => write!(f, "OPPONENT_20"),
            PriceMatch::Queue => write!(f, "QUEUE"),
            PriceMatch::Queue5 => write!(f, "QUEUE_5"),
            PriceMatch::Queue10 => write!(f, "QUEUE_10"),
            PriceMatch::Queue20 => write!(f, "QUEUE_20"),
        }
    }
}
