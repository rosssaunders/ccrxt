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
    /// The order has been accepted by the system but not yet filled.
    New,
    /// The order has been partially filled.
    PartiallyFilled,
    /// The order has been completely filled.
    Filled,
    /// The order has been canceled by the user.
    Canceled,
    /// The order has expired.
    Expired,
}

impl fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrderStatus::New => write!(f, "NEW"),
            OrderStatus::PartiallyFilled => write!(f, "PARTIALLY_FILLED"),
            OrderStatus::Filled => write!(f, "FILLED"),
            OrderStatus::Canceled => write!(f, "CANCELED"),
            OrderStatus::Expired => write!(f, "EXPIRED"),
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

/// Represents the kline/candlestick chart intervals.
///
/// m -> minutes; h -> hours; d -> days; w -> weeks; M -> months
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum KlineInterval {
    #[serde(rename = "1m")] I1m,
    #[serde(rename = "3m")] I3m,
    #[serde(rename = "5m")] I5m,
    #[serde(rename = "15m")] I15m,
    #[serde(rename = "30m")] I30m,
    #[serde(rename = "1h")] I1h,
    #[serde(rename = "2h")] I2h,
    #[serde(rename = "4h")] I4h,
    #[serde(rename = "6h")] I6h,
    #[serde(rename = "8h")] I8h,
    #[serde(rename = "12h")] I12h,
    #[serde(rename = "1d")] I1d,
    #[serde(rename = "3d")] I3d,
    #[serde(rename = "1w")] I1w,
    #[serde(rename = "1M")] I1M,
}