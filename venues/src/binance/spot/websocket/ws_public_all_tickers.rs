use websockets::WebSocketError;

use super::client::BinanceSpotWebSocketClient;
pub use super::ws_public_ticker::Ticker24hrData;

impl BinanceSpotWebSocketClient {
    /// Subscribe to all market tickers stream
    /// 
    /// 24hr rolling window ticker statistics for all symbols that changed in an array.
    /// These are NOT the statistics of the UTC day, but a 24hr rolling window
    /// for the previous 24hrs.
    /// 
    /// # Stream Name
    /// `!ticker@arr`
    /// 
    /// # Update Speed
    /// 1000ms
    /// 
    /// # API Documentation
    /// https://developers.binance.com/docs/binance-spot-api-docs/web-socket-streams#all-market-tickers-stream
    pub async fn subscribe_all_tickers(&mut self) -> Result<(), WebSocketError> {
        self.subscribe(&["!ticker@arr".to_string()]).await
    }
    
    /// Unsubscribe from all market tickers stream
    pub async fn unsubscribe_all_tickers(&mut self) -> Result<(), WebSocketError> {
        self.unsubscribe(&["!ticker@arr".to_string()]).await
    }
}