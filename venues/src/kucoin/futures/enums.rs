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

/// Futures position side
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PositionSide {
    Long,
    Short,
}

/// Futures margin mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MarginMode {
    CrossMargin,
    IsolatedMargin,
}

/// Futures order stop type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StopType {
    #[serde(rename = "up")]
    Up,
    #[serde(rename = "down")]
    Down,
}

/// Futures leverage
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Leverage {
    #[serde(rename = "1")]
    One,
    #[serde(rename = "2")]
    Two,
    #[serde(rename = "3")]
    Three,
    #[serde(rename = "5")]
    Five,
    #[serde(rename = "10")]
    Ten,
    #[serde(rename = "20")]
    Twenty,
    #[serde(rename = "50")]
    Fifty,
    #[serde(rename = "100")]
    OneHundred,
}

/// Futures contract type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContractType {
    #[serde(rename = "FFWCSX")]
    Perpetual,
    #[serde(rename = "FFICSX")]
    Futures,
}

/// Futures contract status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContractStatus {
    #[serde(rename = "Open")]
    Open,
    #[serde(rename = "BeingSettled")]
    BeingSettled,
    #[serde(rename = "Paused")]
    Paused,
    #[serde(rename = "CancelOnly")]
    CancelOnly,
    #[serde(rename = "PostOnly")]
    PostOnly,
}

/// Futures position status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PositionStatus {
    #[serde(rename = "opened")]
    Opened,
    #[serde(rename = "closed")]
    Closed,
}

/// Auto deposit margin status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AutoDepositStatus {
    #[serde(rename = "on")]
    On,
    #[serde(rename = "off")]
    Off,
}
