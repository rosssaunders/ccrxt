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

/// Insurance fund type enum for OKX
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum InsuranceFundType {
    /// Regular update
    RegularUpdate,
    /// Liquidation balance deposit
    LiquidationBalanceDeposit,
    /// Bankruptcy loss
    BankruptcyLoss,
    /// Platform revenue
    PlatformRevenue,
    /// ADL historical data
    Adl,
}

/// ADL type enum for insurance fund
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AdlType {
    /// ADL begins due to high insurance fund decline rate
    RateAdlStart,
    /// ADL begins due to insurance fund balance falling
    BalAdlStart,
    /// ADL begins due to volume of liquidation orders falls to certain level
    PosAdlStart,
    /// ADL ends
    AdlEnd,
}
