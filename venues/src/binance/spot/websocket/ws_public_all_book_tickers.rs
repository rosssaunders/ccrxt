use websockets::WebSocketError;

use super::client::BinanceSpotWebSocketClient;
pub use super::ws_public_book_ticker::BookTickerData;

impl BinanceSpotWebSocketClient {
    /// Subscribe to all book tickers stream
    /// 
    /// Pushes any update to the best bid or ask's price or quantity in real-time for all symbols.
    /// 
    /// # Stream Name
    /// `!bookTicker`
    /// 
    /// # Update Speed
    /// Real-time
    /// 
    /// # API Documentation
    /// https://developers.binance.com/docs/binance-spot-api-docs/web-socket-streams#all-market-tickers-stream
    pub async fn subscribe_all_book_tickers(&mut self) -> Result<(), WebSocketError> {
        self.subscribe(&["!bookTicker".to_string()]).await
    }
    
    /// Unsubscribe from all book tickers stream
    pub async fn unsubscribe_all_book_tickers(&mut self) -> Result<(), WebSocketError> {
        self.unsubscribe(&["!bookTicker".to_string()]).await
    }
}