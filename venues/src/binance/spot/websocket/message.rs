use serde::{Deserialize, Serialize};
use websockets::VenueMessage;

use super::ws_public_agg_trades::AggTradeData;
use super::ws_public_trade::TradeData;
use super::ws_public_klines::KlineData;
use super::ws_public_orderbook::{DepthUpdateData, PartialDepthData};
use super::ws_public_ticker::Ticker24hrData;
use super::ws_public_mini_ticker::MiniTicker24hrData;
use super::ws_public_book_ticker::BookTickerData;
use super::ws_public_avg_price::AvgPriceData;
use super::ws_public_rolling_window_ticker::RollingWindowTickerData;

/// Binance WebSocket message wrapper
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BinanceMessage {
    /// Subscription/unsubscription request
    Request(BinanceRequest),
    /// Subscription response
    Response(BinanceResponse),
    /// Error message
    Error(ErrorMessage),
    /// Trade stream data
    Trade(TradeData),
    /// Aggregate trade stream data
    AggTrade(AggTradeData),
    /// Kline stream data
    Kline(KlineData),
    /// Depth update stream data
    DepthUpdate(DepthUpdateData),
    /// 24hr ticker stream data
    Ticker24hr(Ticker24hrData),
    /// Mini ticker stream data
    MiniTicker24hr(MiniTicker24hrData),
    /// Book ticker stream data
    BookTicker(BookTickerData),
    /// Partial book depth (snapshot)
    PartialDepth(PartialDepthData),
    /// Average price stream data
    AvgPrice(AvgPriceData),
    /// Rolling window ticker data
    RollingWindowTicker(RollingWindowTickerData),
}

impl VenueMessage for BinanceMessage {}

/// Request message for subscribe/unsubscribe
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinanceRequest {
    /// Method: SUBSCRIBE or UNSUBSCRIBE
    pub method: String,

    /// Stream names to subscribe/unsubscribe
    pub params: Vec<String>,

    /// Request ID for matching responses
    pub id: u64,
}

/// Response to subscription requests
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinanceResponse {
    /// Result (null on success)
    pub result: Option<serde_json::Value>,

    /// Request ID from the original request
    pub id: u64,
}

/// Error message from Binance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorMessage {
    /// Error code
    pub code: i32,

    /// Error message
    pub msg: String,
}