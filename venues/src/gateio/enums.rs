use std::fmt;

use serde::{Deserialize, Serialize};

/// Order side
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

/// Time in force
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TimeInForce {
    #[serde(rename = "gtc")]
    GoodTillCanceled,
    #[serde(rename = "ioc")]
    ImmediateOrCancel,
    #[serde(rename = "poc")]
    PendingOrCancelled,
    #[serde(rename = "fok")]
    FillOrKill,
}

impl fmt::Display for TimeInForce {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TimeInForce::GoodTillCanceled => write!(f, "gtc"),
            TimeInForce::ImmediateOrCancel => write!(f, "ioc"),
            TimeInForce::PendingOrCancelled => write!(f, "poc"),
            TimeInForce::FillOrKill => write!(f, "fok"),
        }
    }
}

/// Order status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OrderStatus {
    Open,
    Closed,
    Cancelled,
}

impl fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrderStatus::Open => write!(f, "open"),
            OrderStatus::Closed => write!(f, "closed"),
            OrderStatus::Cancelled => write!(f, "cancelled"),
        }
    }
}

/// Self-trade prevention strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StpMode {
    #[serde(rename = "cn")]
    CancelNewest,
    #[serde(rename = "co")]
    CancelOldest,
    #[serde(rename = "cb")]
    CancelBoth,
    #[serde(rename = "dc")]
    DecreaseCancel,
}

impl fmt::Display for StpMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StpMode::CancelNewest => write!(f, "cn"),
            StpMode::CancelOldest => write!(f, "co"),
            StpMode::CancelBoth => write!(f, "cb"),
            StpMode::DecreaseCancel => write!(f, "dc"),
        }
    }
}

/// Account type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AccountType {
    Spot,
    Margin,
    Futures,
    Delivery,
    Options,
    Cross,
    Isolated,
}

impl fmt::Display for AccountType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AccountType::Spot => write!(f, "spot"),
            AccountType::Margin => write!(f, "margin"),
            AccountType::Futures => write!(f, "futures"),
            AccountType::Delivery => write!(f, "delivery"),
            AccountType::Options => write!(f, "options"),
            AccountType::Cross => write!(f, "cross"),
            AccountType::Isolated => write!(f, "isolated"),
        }
    }
}

/// Candlestick interval
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CandlestickInterval {
    #[serde(rename = "10s")]
    Seconds10,
    #[serde(rename = "1m")]
    Minutes1,
    #[serde(rename = "5m")]
    Minutes5,
    #[serde(rename = "15m")]
    Minutes15,
    #[serde(rename = "30m")]
    Minutes30,
    #[serde(rename = "1h")]
    Hours1,
    #[serde(rename = "4h")]
    Hours4,
    #[serde(rename = "8h")]
    Hours8,
    #[serde(rename = "1d")]
    Days1,
    #[serde(rename = "7d")]
    Days7,
    #[serde(rename = "30d")]
    Days30,
}

impl fmt::Display for CandlestickInterval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CandlestickInterval::Seconds10 => write!(f, "10s"),
            CandlestickInterval::Minutes1 => write!(f, "1m"),
            CandlestickInterval::Minutes5 => write!(f, "5m"),
            CandlestickInterval::Minutes15 => write!(f, "15m"),
            CandlestickInterval::Minutes30 => write!(f, "30m"),
            CandlestickInterval::Hours1 => write!(f, "1h"),
            CandlestickInterval::Hours4 => write!(f, "4h"),
            CandlestickInterval::Hours8 => write!(f, "8h"),
            CandlestickInterval::Days1 => write!(f, "1d"),
            CandlestickInterval::Days7 => write!(f, "7d"),
            CandlestickInterval::Days30 => write!(f, "30d"),
        }
    }
}

impl Default for CandlestickInterval {
    fn default() -> Self {
        Self::Minutes1
    }
}
