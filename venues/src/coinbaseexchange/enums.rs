//! Enums for Coinbase Exchange API
//!
//! All enums implement Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize.
//! Variants use API naming conventions, but enum names and variants follow idiomatic Rust conventions.

use serde::{Deserialize, Serialize};

/// Order side for trading
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OrderSide {
    Buy,
    Sell,
}

/// Order type for trading
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OrderType {
    Limit,
    Market,
    Stop,
}

/// Time in force for orders
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum TimeInForce {
    /// Good till canceled (default)
    #[serde(rename = "GTC")]
    GoodTillCanceled,
    /// Good till time
    #[serde(rename = "GTT")]
    GoodTillTime,
    /// Immediate or cancel
    #[serde(rename = "IOC")]
    ImmediateOrCancel,
    /// Fill or kill
    #[serde(rename = "FOK")]
    FillOrKill,
}

/// Order status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OrderStatus {
    Open,
    Pending,
    Rejected,
    Done,
    Active,
    Received,
    All,
}

/// Self-trade prevention flag
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SelfTradePrevention {
    /// Decrement and cancel (default)
    #[serde(rename = "dc")]
    DecrementAndCancel,
    /// Cancel oldest
    #[serde(rename = "co")]
    CancelOldest,
    /// Cancel newest
    #[serde(rename = "cn")]
    CancelNewest,
    /// Cancel both
    #[serde(rename = "cb")]
    CancelBoth,
}

/// Stop order direction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StopDirection {
    Loss,
    Entry,
}

/// Liquidity type for fills
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Liquidity {
    /// Maker
    M,
    /// Taker
    T,
    /// Other
    O,
}

impl Default for TimeInForce {
    fn default() -> Self {
        Self::GoodTillCanceled
    }
}

impl Default for SelfTradePrevention {
    fn default() -> Self {
        Self::DecrementAndCancel
    }
}
