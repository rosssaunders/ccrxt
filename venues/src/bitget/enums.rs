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
