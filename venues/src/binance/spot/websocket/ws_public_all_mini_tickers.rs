use websockets::WebSocketError;

use super::client::BinanceSpotWebSocketClient;
pub use super::ws_public_mini_ticker::MiniTicker24hrData;

impl BinanceSpotWebSocketClient {
    /// Subscribe to all market mini tickers stream
    /// 
    /// 24hr rolling window mini-ticker statistics for all symbols that changed in an array.
    /// These are NOT the statistics of the UTC day, but a 24hr rolling window
    /// for the previous 24hrs.
    /// 
    /// # Stream Name
    /// `!miniTicker@arr`
    /// 
    /// # Update Speed
    /// 1000ms
    /// 
    /// # API Documentation
    /// https://developers.binance.com/docs/binance-spot-api-docs/web-socket-streams#all-market-mini-tickers-stream
    pub async fn subscribe_all_mini_tickers(&mut self) -> Result<(), WebSocketError> {
        self.subscribe(&["!miniTicker@arr".to_string()]).await
    }
    
    /// Unsubscribe from all market mini tickers stream
    pub async fn unsubscribe_all_mini_tickers(&mut self) -> Result<(), WebSocketError> {
        self.unsubscribe(&["!miniTicker@arr".to_string()]).await
    }
}