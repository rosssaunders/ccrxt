use serde::{Deserialize, Serialize};
use websockets::WebSocketError;

use super::client::BinanceSpotWebSocketClient;

/// Book ticker data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookTickerData {
    /// Update ID
    #[serde(rename = "u")]
    pub update_id: u64,

    /// Symbol
    #[serde(rename = "s")]
    pub symbol: String,

    /// Best bid price
    #[serde(rename = "b")]
    pub bid_price: String,

    /// Best bid quantity
    #[serde(rename = "B")]
    pub bid_quantity: String,

    /// Best ask price
    #[serde(rename = "a")]
    pub ask_price: String,

    /// Best ask quantity
    #[serde(rename = "A")]
    pub ask_quantity: String,
}

impl BinanceSpotWebSocketClient {
    /// Subscribe to individual symbol book ticker stream
    /// 
    /// Pushes any update to the best bid or ask's price or quantity in real-time.
    /// 
    /// # Stream Name
    /// `<symbol>@bookTicker`
    /// 
    /// # Update Speed
    /// Real-time
    /// 
    /// # API Documentation
    /// https://developers.binance.com/docs/binance-spot-api-docs/web-socket-streams#individual-symbol-book-ticker-streams
    pub async fn subscribe_book_ticker(&mut self, symbol: &str) -> Result<(), WebSocketError> {
        let stream = format!("{}@bookTicker", symbol.to_lowercase());
        self.subscribe(&[stream]).await
    }
    
    /// Unsubscribe from individual symbol book ticker stream
    pub async fn unsubscribe_book_ticker(&mut self, symbol: &str) -> Result<(), WebSocketError> {
        let stream = format!("{}@bookTicker", symbol.to_lowercase());
        self.unsubscribe(&[stream]).await
    }
}