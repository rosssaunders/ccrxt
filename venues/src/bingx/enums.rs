use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// Account types supported by BingX
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountType {
    /// Spot/Fund account
    Spot,
    /// Standard futures account
    StdFutures,
    /// Coin-M perpetual futures account
    CoinMPerp,
    /// USDT-M perpetual futures account
    UsdtMPerp,
    /// Copy trading account
    CopyTrading,
    /// Grid trading account
    Grid,
    /// Wealth management account
    Earn,
    /// C2C trading account
    C2C,
}

impl AccountType {
    pub fn as_str(&self) -> &'static str {
        match self {
            AccountType::Spot => "spot",
            AccountType::StdFutures => "stdFutures",
            AccountType::CoinMPerp => "coinMPerp",
            AccountType::UsdtMPerp => "USDTMPerp",
            AccountType::CopyTrading => "copyTrading",
            AccountType::Grid => "grid",
            AccountType::Earn => "earn",
            AccountType::C2C => "c2c",
        }
    }
}

/// Deposit and withdrawal status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum DepositStatus {
    /// In progress
    InProgress = 0,
    /// Completed
    Completed = 1,
    /// Chain uploaded
    ChainUploaded = 6,
}

/// Withdrawal status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum WithdrawStatus {
    /// Under review
    UnderReview = 4,
    /// Failed
    Failed = 5,
    /// Completed
    Completed = 6,
}

/// Transfer types for asset transfers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransferType {
    /// Funding Account -> Standard Contract
    #[serde(rename = "FUND_SFUTURES")]
    FundToStandardFutures,
    /// Standard Contract -> Funding Account
    #[serde(rename = "SFUTURES_FUND")]
    StandardFuturesToFund,
    /// Funding Account -> Perpetual Futures
    #[serde(rename = "FUND_PFUTURES")]
    FundToPerpetualFutures,
    /// Perpetual Futures -> Funding Account
    #[serde(rename = "PFUTURES_FUND")]
    PerpetualFuturesToFund,
    /// Standard Contract -> Perpetual Futures
    #[serde(rename = "SFUTURES_PFUTURES")]
    StandardToPerpetualFutures,
    /// Perpetual Futures -> Standard Contract
    #[serde(rename = "PFUTURES_SFUTURES")]
    PerpetualToStandardFutures,
    /// Funding Account -> Grid Account
    #[serde(rename = "FUND_STRADING")]
    FundToGrid,
    /// Grid Account -> Funding Account
    #[serde(rename = "STRADING_FUND")]
    GridToFund,
    /// Funding Account -> Copy Trade Account
    #[serde(rename = "FUND_CTRADING")]
    FundToCopyTrade,
    /// Standard Contract -> Copy Trade Account
    #[serde(rename = "SFUTURES_CTRADING")]
    StandardFuturesToCopyTrade,
    /// Perpetual Futures -> Copy Trade Account
    #[serde(rename = "PFUTURES_CTRADING")]
    PerpetualFuturesToCopyTrade,
    /// Copy Trade Account -> Funding Account
    #[serde(rename = "CTRADING_FUND")]
    CopyTradeToFund,
    /// Copy Trade Account -> Standard Contract
    #[serde(rename = "CTRADING_SFUTURES")]
    CopyTradeToStandardFutures,
    /// Copy Trade Account -> Perpetual Futures
    #[serde(rename = "CTRADING_PFUTURES")]
    CopyTradeToPerpetualFutures,
    /// Funding Account -> Spot Account
    #[serde(rename = "FUND_SPOT")]
    FundToSpot,
    /// Spot Account -> Funding Account
    #[serde(rename = "SPOT_FUND")]
    SpotToFund,
}

/// Wallet types for withdrawals and deposits
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WalletType {
    /// Fund account
    #[serde(rename = "1")]
    Fund = 1,
    /// Standard account
    #[serde(rename = "2")]
    Standard = 2,
    /// Perpetual account
    #[serde(rename = "3")]
    Perpetual = 3,
    /// Spot account
    #[serde(rename = "15")]
    Spot = 15,
}

/// User account types for internal transfers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserAccountType {
    /// UID
    #[serde(rename = "1")]
    Uid = 1,
    /// Phone number
    #[serde(rename = "2")]
    Phone = 2,
    /// Email
    #[serde(rename = "3")]
    Email = 3,
}

/// Order types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderType {
    /// Market order
    #[serde(rename = "MARKET")]
    Market,
    /// Limit order
    #[serde(rename = "LIMIT")]
    Limit,
    /// Take profit/stop loss limit order
    #[serde(rename = "TAKE_STOP_LIMIT")]
    TakeStopLimit,
    /// Take profit/stop loss market order
    #[serde(rename = "TAKE_STOP_MARKET")]
    TakeStopMarket,
    /// Trigger limit order
    #[serde(rename = "TRIGGER_LIMIT")]
    TriggerLimit,
    /// Trigger market order
    #[serde(rename = "TRIGGER_MARKET")]
    TriggerMarket,
}

/// Order sides
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderSide {
    /// Buy order
    #[serde(rename = "BUY")]
    Buy,
    /// Sell order
    #[serde(rename = "SELL")]
    Sell,
}

/// Order status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderStatus {
    /// New order
    #[serde(rename = "NEW")]
    New,
    /// Pending order
    #[serde(rename = "PENDING")]
    Pending,
    /// Partially filled order
    #[serde(rename = "PARTIALLY_FILLED")]
    PartiallyFilled,
    /// Fully filled order
    #[serde(rename = "FILLED")]
    Filled,
    /// Canceled order
    #[serde(rename = "CANCELED")]
    Canceled,
    /// Failed order
    #[serde(rename = "FAILED")]
    Failed,
}

/// Time in force options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimeInForce {
    /// Good till canceled
    #[serde(rename = "GTC")]
    Gtc,
    /// Immediate or cancel
    #[serde(rename = "IOC")]
    Ioc,
    /// Fill or kill
    #[serde(rename = "FOK")]
    Fok,
    /// Post only
    #[serde(rename = "PostOnly")]
    PostOnly,
}

/// API permissions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApiPermission {
    /// Spot trading permission
    #[serde(rename = "1")]
    SpotTrading = 1,
    /// Read permission
    #[serde(rename = "2")]
    Read = 2,
    /// Perpetual futures trading permission
    #[serde(rename = "3")]
    PerpetualFuturesTrading = 3,
    /// Universal transfer permission
    #[serde(rename = "4")]
    UniversalTransfer = 4,
    /// Withdrawal permission
    #[serde(rename = "5")]
    Withdrawal = 5,
    /// Allow internal transfer of sub accounts
    #[serde(rename = "7")]
    SubAccountInternalTransfer = 7,
}

/// Cancel replace modes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CancelReplaceMode {
    /// Stop on failure - if cancel fails, don't place new order
    #[serde(rename = "STOP_ON_FAILURE")]
    StopOnFailure,
    /// Allow failure - place new order regardless of cancel result
    #[serde(rename = "ALLOW_FAILURE")]
    AllowFailure,
}

/// OCO order types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OcoOrderType {
    /// OCO limit order
    #[serde(rename = "ocoLimit")]
    OcoLimit,
    /// OCO stop limit order
    #[serde(rename = "ocoTps")]
    OcoTps,
}

/// Cancel all after status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CancelAllAfterStatus {
    /// Activated
    #[serde(rename = "ACTIVATED")]
    Activated,
    /// Closed
    #[serde(rename = "CLOSED")]
    Closed,
    /// Failed
    #[serde(rename = "FAILED")]
    Failed,
}

/// Cancel all after request type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CancelAllAfterType {
    /// Activate countdown
    #[serde(rename = "ACTIVATE")]
    Activate,
    /// Close countdown
    #[serde(rename = "CLOSE")]
    Close,
}

/// Transfer status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransferStatus {
    /// Pending review
    #[serde(rename = "4")]
    PendingReview = 4,
    /// Failed
    #[serde(rename = "5")]
    Failed = 5,
    /// Completed
    #[serde(rename = "6")]
    Completed = 6,
}

/// Address status for deposits
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum AddressStatus {
    /// Activated
    Activated = 0,
    /// Applied
    Applied = 1,
    /// Not applied
    NotApplied = 2,
}

/// Sub-account types for transfers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SubAccountType {
    /// Master account
    #[serde(rename = "master")]
    Master,
    /// Sub-account
    #[serde(rename = "sub")]
    SubAccount,
    /// Spot account
    #[serde(rename = "spot")]
    Spot,
}

/// Sub-account transfer types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SubAccountTransferType {
    /// To sub-account (simplified enum variant)
    #[serde(rename = "1")]
    ToSub,
    /// Master account capital to sub-account capital
    #[serde(rename = "MAIN_CAPITAL_TO_SUB_CAPITAL")]
    MainCapitalToSubCapital,
    /// Master account capital to sub-account contract
    #[serde(rename = "MAIN_CAPITAL_TO_SUB_CONTRACT")]
    MainCapitalToSubContract,
    /// Master account capital to sub-account perpetual swap
    #[serde(rename = "MAIN_CAPITAL_TO_SUB_SWAP")]
    MainCapitalToSubSwap,
    /// Master account contract to sub-account capital
    #[serde(rename = "MAIN_CONTRACT_TO_SUB_CAPITAL")]
    MainContractToSubCapital,
    /// Master account contract to sub-account contract
    #[serde(rename = "MAIN_CONTRACT_TO_SUB_CONTRACT")]
    MainContractToSubContract,
    /// Master account contract to sub-account perpetual swap
    #[serde(rename = "MAIN_CONTRACT_TO_SUB_SWAP")]
    MainContractToSubSwap,
    /// Master account perpetual swap to sub-account capital
    #[serde(rename = "MAIN_SWAP_TO_SUB_CAPITAL")]
    MainSwapToSubCapital,
    /// Master account perpetual swap to sub-account contract
    #[serde(rename = "MAIN_SWAP_TO_SUB_CONTRACT")]
    MainSwapToSubContract,
    /// Master account perpetual swap to sub-account perpetual swap
    #[serde(rename = "MAIN_SWAP_TO_SUB_SWAP")]
    MainSwapToSubSwap,
    /// Sub-account capital to master account capital
    #[serde(rename = "SUB_CAPITAL_TO_MAIN_CAPITAL")]
    SubCapitalToMainCapital,
    /// Sub-account capital to master account contract
    #[serde(rename = "SUB_CAPITAL_TO_MAIN_CONTRACT")]
    SubCapitalToMainContract,
    /// Sub-account capital to master account perpetual swap
    #[serde(rename = "SUB_CAPITAL_TO_MAIN_SWAP")]
    SubCapitalToMainSwap,
    /// Sub-account contract to master account capital
    #[serde(rename = "SUB_CONTRACT_TO_MAIN_CAPITAL")]
    SubContractToMainCapital,
    /// Sub-account contract to master account contract
    #[serde(rename = "SUB_CONTRACT_TO_MAIN_CONTRACT")]
    SubContractToMainContract,
    /// Sub-account contract to master account perpetual swap
    #[serde(rename = "SUB_CONTRACT_TO_MAIN_SWAP")]
    SubContractToMainSwap,
    /// Sub-account perpetual swap to master account capital
    #[serde(rename = "SUB_SWAP_TO_MAIN_CAPITAL")]
    SubSwapToMainCapital,
    /// Sub-account perpetual swap to master account contract
    #[serde(rename = "SUB_SWAP_TO_MAIN_CONTRACT")]
    SubSwapToMainContract,
    /// Sub-account perpetual swap to master account perpetual swap
    #[serde(rename = "SUB_SWAP_TO_MAIN_SWAP")]
    SubSwapToMainSwap,
}

/// New transfer account types for the v1 transfer API
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NewTransferAccount {
    /// Funding Account
    #[serde(rename = "fund")]
    Fund,
    /// Spot Account
    #[serde(rename = "spot")]
    Spot,
    /// Standard Contract
    #[serde(rename = "stdFutures")]
    StdFutures,
    /// COIN-M Perpetual Future
    #[serde(rename = "coinMPerp")]
    CoinMPerp,
    /// Perpetual Future
    #[serde(rename = "USDTMPerp")]
    UsdtMPerp,
}

/// Record types for internal transfers
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecordType {
    /// Transfer out record
    #[serde(rename = "out")]
    Out,
    /// Transfer in record
    #[serde(rename = "in")]
    In,
}

/// Cancel restrictions for orders
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CancelRestriction {
    /// New order
    #[serde(rename = "NEW")]
    New,
    /// Pending order
    #[serde(rename = "PENDING")]
    Pending,
    /// Partially filled order
    #[serde(rename = "PARTIALLY_FILLED")]
    PartiallyFilled,
}

/// OCO order status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OcoOrderStatus {
    /// Response when the order list has been placed or there is an update to the order list status
    #[serde(rename = "RESPONSE")]
    Response,
    /// Order list has been placed and execution started
    #[serde(rename = "EXEC_STARTED")]
    ExecStarted,
    /// Either all orders have been filled or one has been filled and the other has been canceled
    #[serde(rename = "ALL_DONE")]
    AllDone,
    /// Order is currently executing
    #[serde(rename = "EXECUTING")]
    Executing,
    /// Order has been completed
    #[serde(rename = "DONE")]
    Done,
}

/// Sub-account status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SubAccountStatus {
    /// Normal status
    #[serde(rename = "NORMAL")]
    Normal,
    /// Frozen status
    #[serde(rename = "FROZEN")]
    Frozen,
    /// Suspended status
    #[serde(rename = "SUSPENDED")]
    Suspended,
}

/// Risk levels for risk records
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskLevel {
    /// Low risk
    #[serde(rename = "LOW")]
    Low,
    /// Medium risk
    #[serde(rename = "MEDIUM")]
    Medium,
    /// High risk
    #[serde(rename = "HIGH")]
    High,
    /// Critical risk
    #[serde(rename = "CRITICAL")]
    Critical,
}

/// Risk status for risk records
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskStatus {
    /// Active risk
    #[serde(rename = "ACTIVE")]
    Active,
    /// Resolved risk
    #[serde(rename = "RESOLVED")]
    Resolved,
    /// Mitigated risk
    #[serde(rename = "MITIGATED")]
    Mitigated,
}

/// API key status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApiKeyStatus {
    /// Active API key
    #[serde(rename = "ACTIVE")]
    Active,
    /// Inactive API key
    #[serde(rename = "INACTIVE")]
    Inactive,
    /// Suspended API key
    #[serde(rename = "SUSPENDED")]
    Suspended,
}

/// Sub-account transfer types for transfer history (renamed to avoid conflict)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SubAccountTransferDirection {
    /// To sub-account
    #[serde(rename = "1")]
    ToSub = 1,
    /// To master account
    #[serde(rename = "2")]
    ToMaster = 2,
}

/// Time intervals for kline/candlestick data
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Interval {
    /// 1 minute
    #[serde(rename = "1m")]
    OneMinute,
    /// 3 minutes
    #[serde(rename = "3m")]
    ThreeMinutes,
    /// 5 minutes
    #[serde(rename = "5m")]
    FiveMinutes,
    /// 15 minutes
    #[serde(rename = "15m")]
    FifteenMinutes,
    /// 30 minutes
    #[serde(rename = "30m")]
    ThirtyMinutes,
    /// 1 hour
    #[serde(rename = "1h")]
    OneHour,
    /// 2 hours
    #[serde(rename = "2h")]
    TwoHours,
    /// 4 hours
    #[serde(rename = "4h")]
    FourHours,
    /// 6 hours
    #[serde(rename = "6h")]
    SixHours,
    /// 8 hours
    #[serde(rename = "8h")]
    EightHours,
    /// 12 hours
    #[serde(rename = "12h")]
    TwelveHours,
    /// 1 day
    #[serde(rename = "1d")]
    OneDay,
    /// 3 days
    #[serde(rename = "3d")]
    ThreeDays,
    /// 1 week
    #[serde(rename = "1w")]
    OneWeek,
    /// 1 month
    #[serde(rename = "1M")]
    OneMonth,
}

impl Interval {
    /// Get the string representation of the interval
    pub fn as_str(&self) -> &'static str {
        match self {
            Interval::OneMinute => "1m",
            Interval::ThreeMinutes => "3m",
            Interval::FiveMinutes => "5m",
            Interval::FifteenMinutes => "15m",
            Interval::ThirtyMinutes => "30m",
            Interval::OneHour => "1h",
            Interval::TwoHours => "2h",
            Interval::FourHours => "4h",
            Interval::SixHours => "6h",
            Interval::EightHours => "8h",
            Interval::TwelveHours => "12h",
            Interval::OneDay => "1d",
            Interval::ThreeDays => "3d",
            Interval::OneWeek => "1w",
            Interval::OneMonth => "1M",
        }
    }
}

/// Symbol trading status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SymbolStatus {
    /// Offline
    #[serde(rename = "0")]
    Offline = 0,
    /// Online
    #[serde(rename = "1")]
    Online = 1,
    /// Pre-open
    #[serde(rename = "5")]
    PreOpen = 5,
    /// Trading suspended
    #[serde(rename = "25")]
    TradingSuspended = 25,
}

/// Order book aggregation types for depth precision
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DepthType {
    /// Default precision
    #[serde(rename = "step0")]
    Step0,
    /// 10x precision
    #[serde(rename = "step1")]
    Step1,
    /// 100x precision
    #[serde(rename = "step2")]
    Step2,
    /// 1000x precision
    #[serde(rename = "step3")]
    Step3,
    /// 10000x precision
    #[serde(rename = "step4")]
    Step4,
    /// 100000x precision
    #[serde(rename = "step5")]
    Step5,
}

impl DepthType {
    /// Get the string representation of the depth type
    pub fn as_str(&self) -> &'static str {
        match self {
            DepthType::Step0 => "step0",
            DepthType::Step1 => "step1",
            DepthType::Step2 => "step2",
            DepthType::Step3 => "step3",
            DepthType::Step4 => "step4",
            DepthType::Step5 => "step5",
        }
    }
}
