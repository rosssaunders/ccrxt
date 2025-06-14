use std::fmt;

use serde::{Deserialize, Serialize};



/// Order side (side)
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

/// Position side for Futures (positionSide)
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

/// Time in force (timeInForce)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum TimeInForce {
    /// Good Till Cancel
    GTC,
    /// Immediate or Cancel
    IOC,
    /// Fill or Kill
    FOK,
    /// Good Till Crossing (Post Only)
    GTX,
}

impl fmt::Display for TimeInForce {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TimeInForce::GTC => write!(f, "GTC"),
            TimeInForce::IOC => write!(f, "IOC"),
            TimeInForce::FOK => write!(f, "FOK"),
            TimeInForce::GTX => write!(f, "GTX"),
        }
    }
}

/// Stop-Limit Time in force (stopLimitTimeInForce)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum StopLimitTimeInForce {
    /// Good Till Cancel
    GTC,
    /// Immediate or Cancel
    IOC,
    /// Fill or Kill
    FOK,
}

impl fmt::Display for StopLimitTimeInForce {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StopLimitTimeInForce::GTC => write!(f, "GTC"),
            StopLimitTimeInForce::IOC => write!(f, "IOC"),
            StopLimitTimeInForce::FOK => write!(f, "FOK"),
        }
    }
}

/// Side Effect Type (sideEffectType)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SideEffectType {
    NoSideEffect,
    MarginBuy,
    AutoRepay,
}

impl fmt::Display for SideEffectType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SideEffectType::NoSideEffect => write!(f, "NO_SIDE_EFFECT"),
            SideEffectType::MarginBuy => write!(f, "MARGIN_BUY"),
            SideEffectType::AutoRepay => write!(f, "AUTO_REPAY"),
        }
    }
}

/// Price Match (priceMatch)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PriceMatch {
    /// No price match
    None,
    /// Counterparty best price
    Opponent,
    /// Counterparty 5th best price
    Opponent5,
    /// Counterparty 10th best price
    Opponent10,
    /// Counterparty 20th best price
    Opponent20,
    /// The best price on the same side of the order book
    Queue,
    /// The 5th best price on the same side of the order book
    Queue5,
    /// The 10th best price on the same side of the order book
    Queue10,
    /// The 20th best price on the same side of the order book
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

/// Self-Trade Prevention mode (selfTradePreventionMode)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SelfTradePreventionMode {
    /// No Self-Trade Prevention
    None,
    /// Expire taker order when STP trigger
    ExpireTaker,
    /// Expire taker and maker order when STP trigger
    ExpireBoth,
    /// Expire maker order when STP trigger
    ExpireMaker,
}

impl fmt::Display for SelfTradePreventionMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SelfTradePreventionMode::None => write!(f, "NONE"),
            SelfTradePreventionMode::ExpireTaker => write!(f, "EXPIRE_TAKER"),
            SelfTradePreventionMode::ExpireBoth => write!(f, "EXPIRE_BOTH"),
            SelfTradePreventionMode::ExpireMaker => write!(f, "EXPIRE_MAKER"),
        }
    }
}

/// Response Type (newOrderRespType)
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

/// Order types (type)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderType {
    Limit,
    Market,
}

impl fmt::Display for OrderType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrderType::Limit => write!(f, "LIMIT"),
            OrderType::Market => write!(f, "MARKET"),
        }
    }
}

/// Conditional Order types (strategyType)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum StrategyType {
    Stop,
    StopMarket,
    TakeProfit,
    TakeProfitMarket,
    TrailingStopMarket,
}

impl fmt::Display for StrategyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StrategyType::Stop => write!(f, "STOP"),
            StrategyType::StopMarket => write!(f, "STOP_MARKET"),
            StrategyType::TakeProfit => write!(f, "TAKE_PROFIT"),
            StrategyType::TakeProfitMarket => write!(f, "TAKE_PROFIT_MARKET"),
            StrategyType::TrailingStopMarket => write!(f, "TRAILING_STOP_MARKET"),
        }
    }
}

/// Working Type for Futures Conditional Orders (workingType)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WorkingType {
    MarkPrice,
}

impl fmt::Display for WorkingType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WorkingType::MarkPrice => write!(f, "MARK_PRICE"),
        }
    }
}

/// Order status (status)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderStatus {
    New,
    Canceled,
    Rejected,
    PartiallyFilled,
    Filled,
    Expired,
}

impl fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrderStatus::New => write!(f, "NEW"),
            OrderStatus::Canceled => write!(f, "CANCELED"),
            OrderStatus::Rejected => write!(f, "REJECTED"),
            OrderStatus::PartiallyFilled => write!(f, "PARTIALLY_FILLED"),
            OrderStatus::Filled => write!(f, "FILLED"),
            OrderStatus::Expired => write!(f, "EXPIRED"),
        }
    }
}

/// Conditional Order status (strategyStatus)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum StrategyStatus {
    New,
    Canceled,
    /// Conditional order is triggered
    Triggered,
    /// Triggered order is filled
    Finished,
    Expired,
}

impl fmt::Display for StrategyStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StrategyStatus::New => write!(f, "NEW"),
            StrategyStatus::Canceled => write!(f, "CANCELED"),
            StrategyStatus::Triggered => write!(f, "TRIGGERED"),
            StrategyStatus::Finished => write!(f, "FINISHED"),
            StrategyStatus::Expired => write!(f, "EXPIRED"),
        }
    }
}

/// Futures Contract type (contractType)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ContractType {
    Perpetual,
    CurrentMonth,
    NextMonth,
    CurrentQuarter,
    NextQuarter,
    PerpetualDelivering,
}

impl fmt::Display for ContractType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ContractType::Perpetual => write!(f, "PERPETUAL"),
            ContractType::CurrentMonth => write!(f, "CURRENT_MONTH"),
            ContractType::NextMonth => write!(f, "NEXT_MONTH"),
            ContractType::CurrentQuarter => write!(f, "CURRENT_QUARTER"),
            ContractType::NextQuarter => write!(f, "NEXT_QUARTER"),
            ContractType::PerpetualDelivering => write!(f, "PERPETUAL_DELIVERING"),
        }
    }
}

/// Contract status (contractStatus, status)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ContractStatus {
    PendingTrading,
    Trading,
    PreDelivering,
    Delivering,
    Delivered,
    PreSettle,
    Settling,
    Close,
}

impl fmt::Display for ContractStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ContractStatus::PendingTrading => write!(f, "PENDING_TRADING"),
            ContractStatus::Trading => write!(f, "TRADING"),
            ContractStatus::PreDelivering => write!(f, "PRE_DELIVERING"),
            ContractStatus::Delivering => write!(f, "DELIVERING"),
            ContractStatus::Delivered => write!(f, "DELIVERED"),
            ContractStatus::PreSettle => write!(f, "PRE_SETTLE"),
            ContractStatus::Settling => write!(f, "SETTLING"),
            ContractStatus::Close => write!(f, "CLOSE"),
        }
    }
}

/// Rate limiters (rateLimitType)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RateLimitType {
    RequestWeight,
    Orders,
}

impl fmt::Display for RateLimitType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RateLimitType::RequestWeight => write!(f, "REQUEST_WEIGHT"),
            RateLimitType::Orders => write!(f, "ORDERS"),
        }
    }
}

/// Rate limit intervals (interval)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RateLimitInterval {
    Minute,
}

impl fmt::Display for RateLimitInterval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RateLimitInterval::Minute => write!(f, "MINUTE"),
        }
    }
}
