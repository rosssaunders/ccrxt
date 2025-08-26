use serde::{Deserialize, Serialize};
use websockets::WebSocketError;

use super::client::BinanceSpotWebSocketClient;
use super::enums::{DepthLevel, UpdateSpeed};

/// Depth update data (diff depth stream)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepthUpdateData {
    /// Event type
    #[serde(rename = "e")]
    pub event_type: String,

    /// Event time
    #[serde(rename = "E")]
    pub event_time: u64,

    /// Symbol
    #[serde(rename = "s")]
    pub symbol: String,

    /// First update ID in event
    #[serde(rename = "U")]
    pub first_update_id: u64,

    /// Final update ID in event
    #[serde(rename = "u")]
    pub final_update_id: u64,

    /// Bids to update
    #[serde(rename = "b")]
    pub bids: Vec<[String; 2]>,

    /// Asks to update
    #[serde(rename = "a")]
    pub asks: Vec<[String; 2]>,
}

/// Partial depth data (depth5, depth10, depth20 streams)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartialDepthData {
    /// Last update ID
    #[serde(rename = "lastUpdateId")]
    pub last_update_id: u64,

    /// Bids (snapshot, not delta)
    pub bids: Vec<[String; 2]>,

    /// Asks (snapshot, not delta)
    pub asks: Vec<[String; 2]>,
}

impl BinanceSpotWebSocketClient {
    /// Subscribe to partial book depth stream
    /// 
    /// Top `levels` bids and asks, pushed every second or 100ms.
    /// Valid levels are 5, 10, or 20.
    /// 
    /// # Stream Name
    /// `<symbol>@depth<levels>@<speed>`
    /// 
    /// # Update Speed
    /// 100ms or 1000ms
    /// 
    /// # API Documentation
    /// https://developers.binance.com/docs/binance-spot-api-docs/web-socket-streams#partial-book-depth-streams
    pub async fn subscribe_book_depth(
        &mut self,
        symbol: &str,
        levels: DepthLevel,
        update_speed: UpdateSpeed,
    ) -> Result<(), WebSocketError> {
        let stream = format!(
            "{}@depth{}@{}",
            symbol.to_lowercase(),
            levels.as_str(),
            update_speed.as_str()
        );
        self.subscribe(&[stream]).await
    }
    
    /// Unsubscribe from partial book depth stream
    pub async fn unsubscribe_book_depth(
        &mut self,
        symbol: &str,
        levels: DepthLevel,
        update_speed: UpdateSpeed,
    ) -> Result<(), WebSocketError> {
        let stream = format!(
            "{}@depth{}@{}",
            symbol.to_lowercase(),
            levels.as_str(),
            update_speed.as_str()
        );
        self.unsubscribe(&[stream]).await
    }
    
    /// Subscribe to diff depth stream
    /// 
    /// Order book price and quantity depth updates used to locally manage an order book.
    /// 
    /// # Stream Name
    /// `<symbol>@depth@<speed>`
    /// 
    /// # Update Speed
    /// 100ms or 1000ms
    /// 
    /// # API Documentation
    /// https://developers.binance.com/docs/binance-spot-api-docs/web-socket-streams#diff-depth-stream
    pub async fn subscribe_diff_depth(
        &mut self,
        symbol: &str,
        update_speed: UpdateSpeed,
    ) -> Result<(), WebSocketError> {
        let stream = format!(
            "{}@depth@{}",
            symbol.to_lowercase(),
            update_speed.as_str()
        );
        self.subscribe(&[stream]).await
    }
    
    /// Unsubscribe from diff depth stream
    pub async fn unsubscribe_diff_depth(
        &mut self,
        symbol: &str,
        update_speed: UpdateSpeed,
    ) -> Result<(), WebSocketError> {
        let stream = format!(
            "{}@depth@{}",
            symbol.to_lowercase(),
            update_speed.as_str()
        );
        self.unsubscribe(&[stream]).await
    }
}