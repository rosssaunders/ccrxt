use serde::{Deserialize, Serialize};
use std::fmt;

/// Order side for trading
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OrderSide {
    Buy,
    Sell,
}

impl fmt::Display for OrderSide {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrderSide::Buy => write!(f, "buy"),
            OrderSide::Sell => write!(f, "sell"),
        }
    }
}

/// Order type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OrderType {
    Limit,
    Market,
}

impl fmt::Display for OrderType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrderType::Limit => write!(f, "limit"),
            OrderType::Market => write!(f, "market"),
        }
    }
}

/// Time in force for orders
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

/// Order status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OrderStatus {
    /// The order is waiting to be filled
    Init,
    /// The order is waiting to be filled  
    New,
    /// The order has been partially filled
    #[serde(rename = "partial_fill")]
    PartiallyFilled,
    /// The order has been completely filled
    #[serde(rename = "full_fill")]
    Filled,
    /// The order has been canceled
    Cancelled,
}

impl fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrderStatus::Init => write!(f, "init"),
            OrderStatus::New => write!(f, "new"),
            OrderStatus::PartiallyFilled => write!(f, "partial_fill"),
            OrderStatus::Filled => write!(f, "full_fill"),
            OrderStatus::Cancelled => write!(f, "cancelled"),
        }
    }
}

/// Asset type for balance queries
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssetType {
    /// Position coin - assets that have holdings
    HoldOnly,
    /// All coins including zero balances
    All,
}

impl fmt::Display for AssetType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AssetType::HoldOnly => write!(f, "hold_only"),
            AssetType::All => write!(f, "all"),
        }
    }
}

/// Rate limit interval types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RateLimitInterval {
    Second,
    Minute,
}

impl fmt::Display for RateLimitInterval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RateLimitInterval::Second => write!(f, "SECOND"),
            RateLimitInterval::Minute => write!(f, "MINUTE"),
        }
    }
}

/// Rate limit type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RateLimitType {
    /// Request frequency limit (IP-based)
    RequestFrequency,
    /// Order placement limit (UID-based)
    Orders,
}

impl fmt::Display for RateLimitType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RateLimitType::RequestFrequency => write!(f, "REQUEST_FREQUENCY"),
            RateLimitType::Orders => write!(f, "ORDERS"),
        }
    }
}
