use std::fmt;

use serde::{Deserialize, Serialize};

/// Options contract type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OptionsContractType {
    Call,
    Put,
}

impl fmt::Display for OptionsContractType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OptionsContractType::Call => write!(f, "CALL"),
            OptionsContractType::Put => write!(f, "PUT"),
        }
    }
}

/// Position side (positionSide) for Options
/// Note: Options only supports LONG and SHORT, not BOTH like futures
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OptionsPositionSide {
    Long,
    Short,
}

impl fmt::Display for OptionsPositionSide {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OptionsPositionSide::Long => write!(f, "LONG"),
            OptionsPositionSide::Short => write!(f, "SHORT"),
        }
    }
}

/// Order types (type) for Options
/// Note: Options currently only supports LIMIT orders
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OptionsOrderType {
    Limit,
}

impl fmt::Display for OptionsOrderType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OptionsOrderType::Limit => write!(f, "LIMIT"),
        }
    }
}

/// Order status (status) for Options
/// Note: Options uses different status names than futures
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OptionsOrderStatus {
    Accepted,
    Rejected,
    PartiallyFilled,
    Filled,
    Cancelled,
}

impl fmt::Display for OptionsOrderStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OptionsOrderStatus::Accepted => write!(f, "ACCEPTED"),
            OptionsOrderStatus::Rejected => write!(f, "REJECTED"),
            OptionsOrderStatus::PartiallyFilled => write!(f, "PARTIALLY_FILLED"),
            OptionsOrderStatus::Filled => write!(f, "FILLED"),
            OptionsOrderStatus::Cancelled => write!(f, "CANCELLED"),
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

/// Order side (side) for Options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OptionsOrderSide {
    Buy,
    Sell,
}

impl fmt::Display for OptionsOrderSide {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OptionsOrderSide::Buy => write!(f, "BUY"),
            OptionsOrderSide::Sell => write!(f, "SELL"),
        }
    }
}

/// Time in force (timeInForce) for Options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OptionsTimeInForce {
    Gtc,
    Ioc,
    Fok,
}

impl fmt::Display for OptionsTimeInForce {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OptionsTimeInForce::Gtc => write!(f, "GTC"),
            OptionsTimeInForce::Ioc => write!(f, "IOC"),
            OptionsTimeInForce::Fok => write!(f, "FOK"),
        }
    }
}

/// Response Type (newOrderRespType) for Options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OptionsOrderResponseType {
    Ack,
    Result,
}

impl fmt::Display for OptionsOrderResponseType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OptionsOrderResponseType::Ack => write!(f, "ACK"),
            OptionsOrderResponseType::Result => write!(f, "RESULT"),
        }
    }
}

/// Account risk level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OptionsRiskLevel {
    Normal,
    Medium,
    High,
}

impl fmt::Display for OptionsRiskLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OptionsRiskLevel::Normal => write!(f, "NORMAL"),
            OptionsRiskLevel::Medium => write!(f, "MEDIUM"),
            OptionsRiskLevel::High => write!(f, "HIGH"),
        }
    }
}

/// Bill/Funding flow type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OptionsBillType {
    Fee,
    Contract,
    Transfer,
}

impl fmt::Display for OptionsBillType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OptionsBillType::Fee => write!(f, "FEE"),
            OptionsBillType::Contract => write!(f, "CONTRACT"),
            OptionsBillType::Transfer => write!(f, "TRANSFER"),
        }
    }
}

/// Order liquidity type (TAKER or MAKER)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OptionsLiquidity {
    Taker,
    Maker,
}

impl fmt::Display for OptionsLiquidity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OptionsLiquidity::Taker => write!(f, "TAKER"),
            OptionsLiquidity::Maker => write!(f, "MAKER"),
        }
    }
}

/// Block trade order status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OptionsBlockTradeStatus {
    Received,
    Accepted,
    Cancelled,
    Expired,
}

impl fmt::Display for OptionsBlockTradeStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OptionsBlockTradeStatus::Received => write!(f, "RECEIVED"),
            OptionsBlockTradeStatus::Accepted => write!(f, "ACCEPTED"),
            OptionsBlockTradeStatus::Cancelled => write!(f, "CANCELLED"),
            OptionsBlockTradeStatus::Expired => write!(f, "EXPIRED"),
        }
    }
}
