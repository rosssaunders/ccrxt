use serde::{Deserialize, Serialize};
use websockets::WebSocketError;

use super::client::BinanceSpotWebSocketClient;
use super::enums::StreamType;

/// Trade data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeData {
    /// Event type (always "trade")
    #[serde(rename = "e")]
    pub event_type: String,

    /// Event time
    #[serde(rename = "E")]
    pub event_time: u64,

    /// Symbol
    #[serde(rename = "s")]
    pub symbol: String,

    /// Trade ID
    #[serde(rename = "t")]
    pub trade_id: u64,

    /// Price
    #[serde(rename = "p")]
    pub price: String,

    /// Quantity
    #[serde(rename = "q")]
    pub quantity: String,

    /// Buyer order ID
    #[serde(rename = "b")]
    pub buyer_order_id: u64,

    /// Seller order ID
    #[serde(rename = "a")]
    pub seller_order_id: u64,

    /// Trade time
    #[serde(rename = "T")]
    pub trade_time: u64,

    /// Is buyer maker
    #[serde(rename = "m")]
    pub is_buyer_maker: bool,

    /// Ignore (used internally by Binance)
    #[serde(rename = "M", skip_serializing_if = "Option::is_none")]
    pub ignore: Option<bool>,
}

impl BinanceSpotWebSocketClient {
    /// Subscribe to trade stream
    /// 
    /// The Trade Streams push raw trade information; each trade has a unique buyer and seller.
    /// 
    /// # Stream Name
    /// `<symbol>@trade`
    /// 
    /// # Update Speed
    /// Real-time
    /// 
    /// # API Documentation
    /// https://developers.binance.com/docs/binance-spot-api-docs/web-socket-streams#trade-streams
    pub async fn subscribe_trades(&mut self, symbol: &str) -> Result<(), WebSocketError> {
        let stream = Self::build_stream_name(symbol, StreamType::Trade, None);
        self.subscribe(&[stream]).await
    }
    
    /// Unsubscribe from trade stream
    pub async fn unsubscribe_trades(&mut self, symbol: &str) -> Result<(), WebSocketError> {
        let stream = Self::build_stream_name(symbol, StreamType::Trade, None);
        self.unsubscribe(&[stream]).await
    }
}