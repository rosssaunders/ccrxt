//! Enums for Crypto.com Exchange API (public endpoints)
//
// All enums implement Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize.
// Variants use API naming conventions, but enum names and variants follow idiomatic Rust conventions (PascalCase for types and variants).

use serde::{Deserialize, Serialize};

/// Announcement category
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AnnouncementCategory {
    System,
    List,
    Delist,
    Event,
    Product,
}

/// Product type for announcements
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ProductType {
    Spot,
    Margin,
    Derivative,
    TradingArena,
    VipProgramme,
    MmProgramme,
    Supercharger,
    TradingBot,
    Documents,
    DefiStaking,
    Staking,
    LiquidStaking,
    Affiliate,
    Referral,
    CroLockup,
    AccountManagement,
    OtcConvert,
    Transfer,
    ZeroFeeToken,
}

/// Impacted status for impacted_params
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum ImpactedStatus {
    Yes,
    Partial,
    Bau,
}

/// Instrument type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InstrumentType {
    PerpetualSwap,
    Future,
    Spot,
}

/// Timeframe for candlesticks
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "UPPERCASE")]
pub enum Timeframe {
    #[default]
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
    M1m,
}

/// Trade side
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum TradeSide {
    Buy,
    Sell,
}

/// Valuation type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ValuationType {
    #[default]
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
    Buy,
    Sell,
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
///
/// Note: `POST_ONLY` and `SMART_POST_ONLY` cannot be used together.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ExecInst {
    PostOnly,
    SmartPostOnly,
}

/// Contingency type for order lists
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum ContingencyType {
    List,
    Oco,
}

/// Response code for API responses
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(from = "i32")]
#[repr(i32)]
pub enum ResponseCode {
    /// Success (No error)
    NoError = 0,
    /// Other error codes
    Error(i32),
}

impl From<i32> for ResponseCode {
    fn from(code: i32) -> Self {
        match code {
            0 => ResponseCode::NoError,
            other => ResponseCode::Error(other),
        }
    }
}

impl Serialize for ResponseCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            ResponseCode::NoError => serializer.serialize_i32(0),
            ResponseCode::Error(code) => serializer.serialize_i32(*code),
        }
    }
}

/// STP (Self-Trade Prevention) scope
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum StpScope {
    #[serde(rename = "M")]
    MasterOrSubAccount, // Matches Master or Sub a/c
    #[serde(rename = "S")]
    SubAccountOnly, // Matches Sub a/c only
}

/// STP (Self-Trade Prevention) instruction
/// <https://exchange-docs.crypto.com/exchange/v1/rest-ws/index.html#private-create-order>
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum StpInst {
    #[serde(rename = "M")]
    CancelMaker, // Cancel Maker
    #[serde(rename = "T")]
    CancelTaker, // Cancel Taker
    #[serde(rename = "B")]
    CancelBoth, // Cancel Both Maker and Taker
}

/// Reference price type for ref_price
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RefPriceType {
    MarkPrice,
    IndexPrice,
    LastPrice,
}

/// Order category
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum SpotMarginType {
    Spot,   // Non-margin order
    Margin, // Margin order
}

/// Withdrawal status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WithdrawalStatus {
    /// Pending
    #[serde(rename = "0")]
    Pending,
    /// Processing
    #[serde(rename = "1")]
    Processing,
    /// Rejected
    #[serde(rename = "2")]
    Rejected,
    /// Payment In-progress
    #[serde(rename = "3")]
    PaymentInProgress,
    /// Payment Failed
    #[serde(rename = "4")]
    PaymentFailed,
    /// Completed
    #[serde(rename = "5")]
    Completed,
    /// Cancelled
    #[serde(rename = "6")]
    Cancelled,
}
