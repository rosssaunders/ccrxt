use serde::{Deserialize, Serialize};

/// Order side enum for OKX orders
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum OrderSide {
    /// Buy order
    Buy,
    /// Sell order  
    Sell,
}

/// Order type enum for OKX orders
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum OrderType {
    /// Market order
    Market,
    /// Limit order
    Limit,
    /// Post-only order
    #[serde(rename = "post_only")]
    PostOnly,
    /// Fill or kill order
    #[serde(rename = "fok")]
    Fok,
    /// Immediate or cancel order
    #[serde(rename = "ioc")]
    Ioc,
}

/// Instrument type enum for OKX
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum InstrumentType {
    /// Spot trading
    Spot,
    /// Margin trading
    Margin,
    /// Perpetual swap
    Swap,
    /// Futures contract
    Futures,
    /// Options contract
    Option,
}

/// Trading status for instruments
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum InstrumentState {
    /// Live trading
    Live,
    /// Suspended
    Suspend,
    /// Pre-open phase
    #[serde(rename = "preopen")]
    PreOpen,
    /// Testing phase
    Test,
}