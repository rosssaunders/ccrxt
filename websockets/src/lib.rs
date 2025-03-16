use async_trait::async_trait;
use std::error::Error;
use futures::Stream;
use std::pin::Pin;

/// Common trait for venue-specific message types
pub trait VenueMessage: Send + Sync + std::fmt::Debug {}

/// Common trait for all WebSocket connections
#[async_trait]
pub trait WebSocketConnection<T>: Send + Sync 
where 
    T: VenueMessage
{
    /// Connect to the WebSocket stream
    async fn connect(&mut self) -> Result<(), Box<dyn Error + Send + Sync>>;
    
    /// Disconnect from the WebSocket stream
    async fn disconnect(&mut self) -> Result<(), Box<dyn Error + Send + Sync>>;
    
    /// Check if the connection is alive
    fn is_connected(&self) -> bool;
    
    /// Subscribe to specific channels
    async fn subscribe(&mut self, channels: Vec<String>) -> Result<(), Box<dyn Error + Send + Sync>>;
    
    /// Unsubscribe from specific channels
    async fn unsubscribe(&mut self, channels: Vec<String>) -> Result<(), Box<dyn Error + Send + Sync>>;
    
    /// Get a stream of messages from the WebSocket
    /// Returns a Stream that yields venue-specific messages
    fn message_stream(&mut self) -> Pin<Box<dyn Stream<Item = Result<T, Box<dyn Error + Send + Sync>>> + Send>>;
}
