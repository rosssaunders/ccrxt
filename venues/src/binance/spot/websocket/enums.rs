use serde::{Deserialize, Serialize};

/// WebSocket stream types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StreamType {
    /// Aggregate trade stream
    AggTrade,
    /// Trade stream
    Trade,
    /// Kline/candlestick stream
    Kline,
    /// Individual symbol ticker stream
    Ticker,
    /// All market tickers stream
    AllMarketTickers,
    /// Individual symbol mini ticker stream
    MiniTicker,
    /// All market mini tickers stream
    AllMarketMiniTickers,
    /// Individual symbol book ticker stream
    BookTicker,
    /// All book tickers stream
    AllBookTickers,
    /// Partial book depth stream
    PartialBookDepth,
    /// Diff depth stream
    DiffDepth,
    /// Average price stream
    AvgPrice,
    /// Rolling window ticker stream
    RollingWindowTicker,
}

impl StreamType {
    /// Convert to stream name component
    pub fn to_stream_name(&self) -> &'static str {
        match self {
            Self::AggTrade => "aggTrade",
            Self::Trade => "trade",
            Self::Kline => "kline",
            Self::Ticker => "ticker",
            Self::AllMarketTickers => "!ticker@arr",
            Self::MiniTicker => "miniTicker",
            Self::AllMarketMiniTickers => "!miniTicker@arr",
            Self::BookTicker => "bookTicker",
            Self::AllBookTickers => "!bookTicker",
            Self::PartialBookDepth => "depth",
            Self::DiffDepth => "depthUpdate",
            Self::AvgPrice => "avgPrice",
            Self::RollingWindowTicker => "ticker",  // Special case, requires window suffix
        }
    }
}

/// Kline intervals
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum KlineInterval {
    /// 1 second
    #[serde(rename = "1s")]
    OneSecond,
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

impl KlineInterval {
    /// Convert to string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::OneSecond => "1s",
            Self::OneMinute => "1m",
            Self::ThreeMinutes => "3m",
            Self::FiveMinutes => "5m",
            Self::FifteenMinutes => "15m",
            Self::ThirtyMinutes => "30m",
            Self::OneHour => "1h",
            Self::TwoHours => "2h",
            Self::FourHours => "4h",
            Self::SixHours => "6h",
            Self::EightHours => "8h",
            Self::TwelveHours => "12h",
            Self::OneDay => "1d",
            Self::ThreeDays => "3d",
            Self::OneWeek => "1w",
            Self::OneMonth => "1M",
        }
    }
}

/// Depth levels for partial book depth streams
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DepthLevel {
    /// 5 levels
    Five,
    /// 10 levels
    Ten,
    /// 20 levels
    Twenty,
}

impl DepthLevel {
    /// Convert to string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Five => "5",
            Self::Ten => "10",
            Self::Twenty => "20",
        }
    }
}

/// Update speed for book depth streams
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UpdateSpeed {
    /// 1000ms
    Slow,
    /// 100ms
    Fast,
}

impl UpdateSpeed {
    /// Get update speed in milliseconds
    pub fn as_millis(&self) -> u64 {
        match self {
            Self::Slow => 1000,
            Self::Fast => 100,
        }
    }
    
    /// Convert to string for stream name
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Slow => "1000ms",
            Self::Fast => "100ms",
        }
    }
}