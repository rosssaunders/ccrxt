//! Enums for BitMart API
//!
//! All enums implement Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize.
//! Variants use API naming conventions, but enum names and variants follow idiomatic Rust conventions (PascalCase for types and variants).

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
    #[serde(rename = "stop_limit")]
    StopLimit,
}

/// Account type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AccountType {
    Spot,
    Margin,
}

/// Currency network for deposits/withdrawals
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Network {
    #[serde(rename = "BTC")]
    Bitcoin,
    #[serde(rename = "ETH")]
    Ethereum,
    #[serde(rename = "ERC20")]
    Erc20,
    #[serde(rename = "TRC20")]
    Trc20,
    #[serde(rename = "BSC")]
    BinanceSmartChain,
    #[serde(rename = "POLYGON")]
    Polygon,
    #[serde(rename = "ARBITRUM")]
    Arbitrum,
    #[serde(rename = "OPTIMISM")]
    Optimism,
}
