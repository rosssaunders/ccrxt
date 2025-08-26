use serde::{Deserialize, Serialize};
use websockets::WebSocketError;

use super::client::BinanceSpotWebSocketClient;
use super::enums::StreamType;

/// Aggregate trade data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggTradeData {
    /// Event type (always "aggTrade")
    #[serde(rename = "e")]
    pub event_type: String,

    /// Event time
    #[serde(rename = "E")]
    pub event_time: u64,

    /// Symbol
    #[serde(rename = "s")]
    pub symbol: String,

    /// Aggregate trade ID
    #[serde(rename = "a")]
    pub agg_trade_id: u64,

    /// Price
    #[serde(rename = "p")]
    pub price: String,

    /// Quantity
    #[serde(rename = "q")]
    pub quantity: String,

    /// First trade ID
    #[serde(rename = "f")]
    pub first_trade_id: u64,

    /// Last trade ID
    #[serde(rename = "l")]
    pub last_trade_id: u64,

    /// Trade time
    #[serde(rename = "T")]
    pub trade_time: u64,

    /// Is buyer maker
    #[serde(rename = "m")]
    pub is_buyer_maker: bool,

    /// Ignore
    #[serde(rename = "M")]
    pub ignore: bool,
}

impl BinanceSpotWebSocketClient {
    /// Subscribe to aggregate trade stream
    /// 
    /// The Aggregate Trade Streams push trade information that is aggregated for a single taker order.
    /// 
    /// # Stream Name
    /// `<symbol>@aggTrade`
    /// 
    /// # Update Speed
    /// Real-time
    /// 
    /// # API Documentation
    /// https://developers.binance.com/docs/binance-spot-api-docs/web-socket-streams#aggregate-trade-streams
    pub async fn subscribe_agg_trades(&mut self, symbol: &str) -> Result<(), WebSocketError> {
        let stream = Self::build_stream_name(symbol, StreamType::AggTrade, None);
        self.subscribe(&[stream]).await
    }
    
    /// Unsubscribe from aggregate trade stream
    pub async fn unsubscribe_agg_trades(&mut self, symbol: &str) -> Result<(), WebSocketError> {
        let stream = Self::build_stream_name(symbol, StreamType::AggTrade, None);
        self.unsubscribe(&[stream]).await
    }
}