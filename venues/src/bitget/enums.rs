use std::fmt;

use serde::{Deserialize, Deserializer, Serialize};

/// Custom deserializer for OrderSide that handles both lowercase and capitalized variants
fn deserialize_order_side<'de, D>(deserializer: D) -> Result<OrderSide, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.to_lowercase().as_str() {
        "buy" => Ok(OrderSide::Buy),
        "sell" => Ok(OrderSide::Sell),
        _ => Err(serde::de::Error::unknown_variant(
            &s,
            &["buy", "sell", "Buy", "Sell"],
        )),
    }
}

/// Order side for trading
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderSide {
    Buy,
    Sell,
}

impl Serialize for OrderSide {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s = match self {
            OrderSide::Buy => "buy",
            OrderSide::Sell => "sell",
        };
        serializer.serialize_str(s)
    }
}

impl<'de> Deserialize<'de> for OrderSide {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserialize_order_side(deserializer)
    }
}

impl fmt::Display for OrderSide {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrderSide::Buy => write!(f, "buy"),
            OrderSide::Sell => write!(f, "sell"),
        }
    }
}

/// Order type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OrderType {
    Limit,
    Market,
}

impl fmt::Display for OrderType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrderType::Limit => write!(f, "limit"),
            OrderType::Market => write!(f, "market"),
        }
    }
}

/// Time in force for orders
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum TimeInForce {
    /// Good Till Cancel
    GTC,
    /// Immediate or Cancel
    IOC,
    /// Fill or Kill
    FOK,
}

impl fmt::Display for TimeInForce {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TimeInForce::GTC => write!(f, "GTC"),
            TimeInForce::IOC => write!(f, "IOC"),
            TimeInForce::FOK => write!(f, "FOK"),
        }
    }
}

/// Order status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OrderStatus {
    /// The order is waiting to be filled
    Init,
    /// The order is waiting to be filled  
    New,
    /// The order has been partially filled
    #[serde(rename = "partial_fill")]
    PartiallyFilled,
    /// The order has been completely filled
    #[serde(rename = "full_fill")]
    Filled,
    /// The order has been canceled
    Cancelled,
}

impl fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrderStatus::Init => write!(f, "init"),
            OrderStatus::New => write!(f, "new"),
            OrderStatus::PartiallyFilled => write!(f, "partial_fill"),
            OrderStatus::Filled => write!(f, "full_fill"),
            OrderStatus::Cancelled => write!(f, "cancelled"),
        }
    }
}

/// Asset type for balance queries
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssetType {
    /// Position coin - assets that have holdings
    HoldOnly,
    /// All coins including zero balances
    All,
}

impl fmt::Display for AssetType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AssetType::HoldOnly => write!(f, "hold_only"),
            AssetType::All => write!(f, "all"),
        }
    }
}

/// Rate limit interval types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RateLimitInterval {
    Second,
    Minute,
}

impl fmt::Display for RateLimitInterval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RateLimitInterval::Second => write!(f, "SECOND"),
            RateLimitInterval::Minute => write!(f, "MINUTE"),
        }
    }
}

/// Rate limit type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RateLimitType {
    /// Request frequency limit (IP-based)
    RequestFrequency,
    /// Order placement limit (UID-based)
    Orders,
}

impl fmt::Display for RateLimitType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RateLimitType::RequestFrequency => write!(f, "REQUEST_FREQUENCY"),
            RateLimitType::Orders => write!(f, "ORDERS"),
        }
    }
}

/// Symbol status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SymbolStatus {
    Online,
    Offline,
    Halt,
    Gray,
}

impl fmt::Display for SymbolStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SymbolStatus::Online => write!(f, "online"),
            SymbolStatus::Offline => write!(f, "offline"),
            SymbolStatus::Halt => write!(f, "halt"),
            SymbolStatus::Gray => write!(f, "gray"),
        }
    }
}

/// Price type for merge depth
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PricePrecision {
    #[serde(rename = "scale0")]
    Scale0,
    #[serde(rename = "scale1")]
    Scale1,
    #[serde(rename = "scale2")]
    Scale2,
    #[serde(rename = "scale3")]
    Scale3,
}

impl fmt::Display for PricePrecision {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PricePrecision::Scale0 => write!(f, "scale0"),
            PricePrecision::Scale1 => write!(f, "scale1"),
            PricePrecision::Scale2 => write!(f, "scale2"),
            PricePrecision::Scale3 => write!(f, "scale3"),
        }
    }
}

/// Order book depth types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DepthType {
    #[serde(rename = "step0")]
    Step0,
    #[serde(rename = "step1")]
    Step1,
    #[serde(rename = "step2")]
    Step2,
    #[serde(rename = "step3")]
    Step3,
    #[serde(rename = "step4")]
    Step4,
    #[serde(rename = "step5")]
    Step5,
}

impl fmt::Display for DepthType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DepthType::Step0 => write!(f, "step0"),
            DepthType::Step1 => write!(f, "step1"),
            DepthType::Step2 => write!(f, "step2"),
            DepthType::Step3 => write!(f, "step3"),
            DepthType::Step4 => write!(f, "step4"),
            DepthType::Step5 => write!(f, "step5"),
        }
    }
}

/// Candlestick timeframes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CandlestickGranularity {
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
    #[serde(rename = "1h")]
    OneHour,
    #[serde(rename = "4h")]
    FourHours,
    #[serde(rename = "6h")]
    SixHours,
    #[serde(rename = "12h")]
    TwelveHours,
    #[serde(rename = "1day")]
    OneDay,
    #[serde(rename = "3day")]
    ThreeDays,
    #[serde(rename = "1week")]
    OneWeek,
    #[serde(rename = "1M")]
    OneMonth,
    #[serde(rename = "6Hutc")]
    SixHoursUtc,
    #[serde(rename = "12Hutc")]
    TwelveHoursUtc,
    #[serde(rename = "1Dutc")]
    OneDayUtc,
    #[serde(rename = "3Dutc")]
    ThreeDaysUtc,
    #[serde(rename = "1Wutc")]
    OneWeekUtc,
    #[serde(rename = "1Mutc")]
    OneMonthUtc,
}

impl fmt::Display for CandlestickGranularity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CandlestickGranularity::OneMinute => write!(f, "1min"),
            CandlestickGranularity::ThreeMinutes => write!(f, "3min"),
            CandlestickGranularity::FiveMinutes => write!(f, "5min"),
            CandlestickGranularity::FifteenMinutes => write!(f, "15min"),
            CandlestickGranularity::ThirtyMinutes => write!(f, "30min"),
            CandlestickGranularity::OneHour => write!(f, "1h"),
            CandlestickGranularity::FourHours => write!(f, "4h"),
            CandlestickGranularity::SixHours => write!(f, "6h"),
            CandlestickGranularity::TwelveHours => write!(f, "12h"),
            CandlestickGranularity::OneDay => write!(f, "1day"),
            CandlestickGranularity::ThreeDays => write!(f, "3day"),
            CandlestickGranularity::OneWeek => write!(f, "1week"),
            CandlestickGranularity::OneMonth => write!(f, "1M"),
            CandlestickGranularity::SixHoursUtc => write!(f, "6Hutc"),
            CandlestickGranularity::TwelveHoursUtc => write!(f, "12Hutc"),
            CandlestickGranularity::OneDayUtc => write!(f, "1Dutc"),
            CandlestickGranularity::ThreeDaysUtc => write!(f, "3Dutc"),
            CandlestickGranularity::OneWeekUtc => write!(f, "1Wutc"),
            CandlestickGranularity::OneMonthUtc => write!(f, "1Mutc"),
        }
    }
}

/// Trade scope for transaction history
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TradeScope {
    Taker,
    Maker,
}

impl fmt::Display for TradeScope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TradeScope::Taker => write!(f, "taker"),
            TradeScope::Maker => write!(f, "maker"),
        }
    }
}

/// Account types for transfers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountType {
    /// Spot account
    Spot,
    /// P2P account
    P2p,
    /// Coin-M futures account
    CoinFutures,
    /// USDT-M futures account
    UsdtFutures,
    /// USDC-M futures account
    UsdcFutures,
    /// Cross margin account
    CrossedMargin,
    /// Isolated margin account
    IsolatedMargin,
}

impl fmt::Display for AccountType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AccountType::Spot => write!(f, "spot"),
            AccountType::P2p => write!(f, "p2p"),
            AccountType::CoinFutures => write!(f, "coin_futures"),
            AccountType::UsdtFutures => write!(f, "usdt_futures"),
            AccountType::UsdcFutures => write!(f, "usdc_futures"),
            AccountType::CrossedMargin => write!(f, "crossed_margin"),
            AccountType::IsolatedMargin => write!(f, "isolated_margin"),
        }
    }
}

/// Withdrawal type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WithdrawType {
    /// Withdrawal on chain
    OnChain,
    /// Internal transfer
    InternalTransfer,
}

impl fmt::Display for WithdrawType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WithdrawType::OnChain => write!(f, "on_chain"),
            WithdrawType::InternalTransfer => write!(f, "internal_transfer"),
        }
    }
}

/// Inner transfer type for internal withdrawals
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum InnerTransferType {
    /// Email address
    Email,
    /// Mobile phone number
    Mobile,
    /// UID
    Uid,
}

impl fmt::Display for InnerTransferType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InnerTransferType::Email => write!(f, "email"),
            InnerTransferType::Mobile => write!(f, "mobile"),
            InnerTransferType::Uid => write!(f, "uid"),
        }
    }
}

/// Identity type for withdrawals
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum IdentityType {
    /// Normal user
    User,
    /// Company
    Company,
}

impl fmt::Display for IdentityType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IdentityType::User => write!(f, "user"),
            IdentityType::Company => write!(f, "company"),
        }
    }
}

/// Product types for futures trading
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProductType {
    /// USDT-M Futures, Futures settled in USDT
    #[serde(rename = "USDT-FUTURES")]
    UsdtFutures,
    /// USDC-M Futures, Futures settled in USDC
    #[serde(rename = "USDC-FUTURES")]
    UsdcFutures,
    /// Coin-M Futures, Futures settled in cryptocurrencies
    #[serde(rename = "COIN-FUTURES")]
    CoinFutures,
    /// USDT-M Futures Demo (Try out USDT-M futures trading)
    #[serde(rename = "SUSDT-FUTURES")]
    SUsdtFutures,
    /// USDC-M Futures Demo (Try out USDC-M futures)
    #[serde(rename = "SUSDC-FUTURES")]
    SUsdcFutures,
    /// Coin-M Futures Demo (Try out Coin-M futures trading)
    #[serde(rename = "SCOIN-FUTURES")]
    SCoinFutures,
}

impl fmt::Display for ProductType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProductType::UsdtFutures => write!(f, "USDT-FUTURES"),
            ProductType::UsdcFutures => write!(f, "USDC-FUTURES"),
            ProductType::CoinFutures => write!(f, "COIN-FUTURES"),
            ProductType::SUsdtFutures => write!(f, "SUSDT-FUTURES"),
            ProductType::SUsdcFutures => write!(f, "SUSDC-FUTURES"),
            ProductType::SCoinFutures => write!(f, "SCOIN-FUTURES"),
        }
    }
}

/// Hold side for futures positions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HoldSide {
    /// Long position
    #[serde(rename = "long")]
    Long,
    /// Short position  
    #[serde(rename = "short")]
    Short,
}

impl fmt::Display for HoldSide {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HoldSide::Long => write!(f, "long"),
            HoldSide::Short => write!(f, "short"),
        }
    }
}

/// Margin mode for futures trading
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MarginMode {
    /// Isolated margin
    #[serde(rename = "isolated")]
    Isolated,
    /// Cross margin
    #[serde(rename = "crossed")]
    Crossed,
}

impl fmt::Display for MarginMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MarginMode::Isolated => write!(f, "isolated"),
            MarginMode::Crossed => write!(f, "crossed"),
        }
    }
}

/// Margin coin for futures trading
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MarginCoin {
    /// USDT margin
    #[serde(rename = "USDT")]
    Usdt,
    /// USDC margin
    #[serde(rename = "USDC")]
    Usdc,
}

impl fmt::Display for MarginCoin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MarginCoin::Usdt => write!(f, "USDT"),
            MarginCoin::Usdc => write!(f, "USDC"),
        }
    }
}

/// Trigger price type for futures orders
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TriggerType {
    /// Fill price
    #[serde(rename = "fill_price")]
    FillPrice,
    /// Mark price
    #[serde(rename = "mark_price")]
    MarkPrice,
}

impl fmt::Display for TriggerType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TriggerType::FillPrice => write!(f, "fill_price"),
            TriggerType::MarkPrice => write!(f, "mark_price"),
        }
    }
}

/// Plan type for futures trigger orders
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlanType {
    /// Normal trigger order
    #[serde(rename = "normal_plan")]
    NormalPlan,
    /// Take profit trigger order
    #[serde(rename = "profit_plan")]
    ProfitPlan,
    /// Stop loss trigger order
    #[serde(rename = "loss_plan")]
    LossPlan,
    /// Position take profit
    #[serde(rename = "pos_profit")]
    PosProfit,
    /// Position stop loss
    #[serde(rename = "pos_loss")]
    PosLoss,
    /// Moving stop loss
    #[serde(rename = "moving_plan")]
    MovingPlan,
    /// Trailing stop
    #[serde(rename = "track_plan")]
    TrackPlan,
}

impl fmt::Display for PlanType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PlanType::NormalPlan => write!(f, "normal_plan"),
            PlanType::ProfitPlan => write!(f, "profit_plan"),
            PlanType::LossPlan => write!(f, "loss_plan"),
            PlanType::PosProfit => write!(f, "pos_profit"),
            PlanType::PosLoss => write!(f, "pos_loss"),
            PlanType::MovingPlan => write!(f, "moving_plan"),
            PlanType::TrackPlan => write!(f, "track_plan"),
        }
    }
}

/// Plan status for futures trigger orders
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlanStatus {
    /// Not triggered
    #[serde(rename = "not_trigger")]
    NotTrigger,
    /// Triggered
    #[serde(rename = "triggered")]
    Triggered,
    /// Cancelled
    #[serde(rename = "cancel")]
    Cancel,
    /// Failed
    #[serde(rename = "fail")]
    Fail,
}

impl fmt::Display for PlanStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PlanStatus::NotTrigger => write!(f, "not_trigger"),
            PlanStatus::Triggered => write!(f, "triggered"),
            PlanStatus::Cancel => write!(f, "cancel"),
            PlanStatus::Fail => write!(f, "fail"),
        }
    }
}

/// Position mode for futures trading
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PositionMode {
    /// One-way mode
    #[serde(rename = "one_way_mode")]
    OneWayMode,
    /// Hedge mode
    #[serde(rename = "hedge_mode")]
    HedgeMode,
}

impl fmt::Display for PositionMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PositionMode::OneWayMode => write!(f, "one_way_mode"),
            PositionMode::HedgeMode => write!(f, "hedge_mode"),
        }
    }
}

/// Transfer role for main-sub account transfers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TransferRole {
    Initiator,
    Receiver,
}

/// Deposit account type for modifying deposit accounts
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DepositAccountType {
    #[serde(rename = "usdt_futures")]
    UsdtFutures,
    #[serde(rename = "usdc_futures")]
    UsdcFutures,
    #[serde(rename = "coin_futures")]
    CoinFutures,
    #[serde(rename = "spot")]
    Spot,
}

/// BGB deduct status for switching BGB deduction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BgbDeductStatus {
    On,
    Off,
}
