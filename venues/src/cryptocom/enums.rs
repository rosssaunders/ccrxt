//! Enums for Crypto.com Exchange API (public endpoints)
//
// All enums implement Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize.
// Variants use API naming conventions.

use serde::{Deserialize, Serialize};

/// Announcement category
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AnnouncementCategory {
    #[serde(rename = "system")] System,
    #[serde(rename = "list")] List,
    #[serde(rename = "delist")] Delist,
    #[serde(rename = "event")] Event,
    #[serde(rename = "product")] Product,
}

/// Product type for announcements
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ProductType {
    Spot,
    Margin,
    Derivative,
    TradingArena,
    VIPProgramme,
    MMProgramme,
    Supercharger,
    TradingBot,
    Documents,
    DefiStaking,
    Staking,
    LiquidStaking,
    Affiliate,
    Referral,
    CROLockup,
    AccountManagement,
    OtcConvert,
    Transfer,
    ZeroFeeToken,
}

/// Impacted status for impacted_params
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum ImpactedStatus {
    PARTIAL,
    BAU,
}

/// Instrument type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InstrumentType {
    PERPETUAL_SWAP,
    FUTURE,
    SPOT,
}

/// Timeframe for candlesticks
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Timeframe {
    M1,
    M5,
    M15,
    M30,
    H1,
    H2,
    H4,
    H12,
    D1,
    D7,
    D14,
    M1M,
}

/// Trade side
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum TradeSide {
    BUY,
    SELL,
}

/// Valuation type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValuationType {
    IndexPrice,
    MarkPrice,
    FundingHist,
    FundingRate,
    EstimatedFundingRate,
}

/// Order side for trading
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum OrderSide {
    BUY,
    SELL,
}

/// Order type for trading
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderType {
    Limit,
    Market,
    StopLoss,
    StopLimit,
    TakeProfit,
    TakeProfitLimit,
}

/// Time in force for orders
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TimeInForce {
    GoodTillCancel,
    FillOrKill,
    ImmediateOrCancel,
}

/// Execution instruction for orders
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ExecInst {
    PostOnly,
}

/// Contingency type for order lists
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum ContingencyType {
    LIST,
    OCO,
}

/// STP (Self-Trade Prevention) scope
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum StpScope {
    M, // Matches Master or Sub a/c
    S, // Matches Sub a/c only
}

/// STP (Self-Trade Prevention) instruction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum StpInst {
    M, // Cancel Maker
    T, // Cancel Taker
    B, // Cancel Both Maker and Taker
}
