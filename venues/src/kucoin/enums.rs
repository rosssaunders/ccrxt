use serde::{Deserialize, Serialize};

/// Order side for KuCoin trading
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OrderSide {
    Buy,
    Sell,
}

/// Order type for KuCoin trading
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OrderType {
    Limit,
    Market,
}

/// Time in force for orders
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimeInForce {
    #[serde(rename = "GTC")]
    GoodTillCanceled,
    #[serde(rename = "GTT")]
    GoodTillTime,
    #[serde(rename = "IOC")]
    ImmediateOrCancel,
    #[serde(rename = "FOK")]
    FillOrKill,
}

/// Order status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OrderStatus {
    Active,
    Done,
}

/// Trade side (for historical trades)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TradeSide {
    Buy,
    Sell,
}

/// Kline/Candlestick intervals
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum KlineInterval {
    #[serde(rename = "1min")]
    OneMinute,
    #[serde(rename = "3min")]
    ThreeMinutes,
    #[serde(rename = "5min")]
    FiveMinutes,
    #[serde(rename = "15min")]
    FifteenMinutes,
    #[serde(rename = "30min")]
    ThirtyMinutes,
    #[serde(rename = "1hour")]
    OneHour,
    #[serde(rename = "2hour")]
    TwoHours,
    #[serde(rename = "4hour")]
    FourHours,
    #[serde(rename = "6hour")]
    SixHours,
    #[serde(rename = "8hour")]
    EightHours,
    #[serde(rename = "12hour")]
    TwelveHours,
    #[serde(rename = "1day")]
    OneDay,
    #[serde(rename = "1week")]
    OneWeek,
}

/// Market types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Market {
    #[serde(rename = "BTC")]
    Bitcoin,
    #[serde(rename = "ETH")]
    Ethereum,
    #[serde(rename = "USDT")]
    Tether,
    #[serde(rename = "USDC")]
    UsdCoin,
    #[serde(rename = "KCS")]
    KuCoinShares,
}

/// Account types for KuCoin
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AccountType {
    Main,
    Trade,
    Margin,
    Pool,
}

/// Transfer direction for sub-account transfers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransferDirection {
    #[serde(rename = "IN")]
    In,
    #[serde(rename = "OUT")]
    Out,
}

/// Deposit status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DepositStatus {
    #[serde(rename = "PROCESSING")]
    Processing,
    #[serde(rename = "SUCCESS")]
    Success,
    #[serde(rename = "FAILURE")]
    Failure,
}

/// Withdrawal status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WithdrawalStatus {
    #[serde(rename = "PROCESSING")]
    Processing,
    #[serde(rename = "WALLET_PROCESSING")]
    WalletProcessing,
    #[serde(rename = "SUCCESS")]
    Success,
    #[serde(rename = "FAILURE")]
    Failure,
}

/// Fee deduction type for withdrawals
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FeeDeductType {
    #[serde(rename = "INTERNAL")]
    Internal,
    #[serde(rename = "EXTERNAL")]
    External,
}
