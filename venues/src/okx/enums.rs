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

/// Bar size (timeframe) for candlestick data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Bar {
    /// 1 minute
    #[serde(rename = "1m")]
    M1,
    /// 3 minutes
    #[serde(rename = "3m")]
    M3,
    /// 5 minutes
    #[serde(rename = "5m")]
    M5,
    /// 15 minutes
    #[serde(rename = "15m")]
    M15,
    /// 30 minutes
    #[serde(rename = "30m")]
    M30,
    /// 1 hour
    #[serde(rename = "1H")]
    H1,
    /// 2 hours
    #[serde(rename = "2H")]
    H2,
    /// 4 hours
    #[serde(rename = "4H")]
    H4,
    /// 6 hours (Hong Kong time)
    #[serde(rename = "6H")]
    H6,
    /// 12 hours (Hong Kong time)
    #[serde(rename = "12H")]
    H12,
    /// 1 day (Hong Kong time)
    #[serde(rename = "1D")]
    D1,
    /// 1 week (Hong Kong time)
    #[serde(rename = "1W")]
    W1,
    /// 1 month (Hong Kong time)
    #[serde(rename = "1M")]
    Month1,
    /// 3 months (Hong Kong time)
    #[serde(rename = "3M")]
    Month3,
    /// 6 hours (UTC time)
    #[serde(rename = "6Hutc")]
    H6Utc,
    /// 12 hours (UTC time)
    #[serde(rename = "12Hutc")]
    H12Utc,
    /// 1 day (UTC time)
    #[serde(rename = "1Dutc")]
    D1Utc,
    /// 1 week (UTC time)
    #[serde(rename = "1Wutc")]
    W1Utc,
    /// 1 month (UTC time)
    #[serde(rename = "1Mutc")]
    Month1Utc,
    /// 3 months (UTC time)
    #[serde(rename = "3Mutc")]
    Month3Utc,
}

/// ADL related events
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AdlType {
    /// ADL begins due to high insurance fund decline rate
    RateAdlStart,
    /// ADL begins due to insurance fund balance falling
    BalAdlStart,
    /// ADL begins due to the volume of liquidation orders falls to a certain level (only applicable to premarket symbols)
    PosAdlStart,
    /// ADL ends
    AdlEnd,
}

