use async_trait::async_trait;
use std::error::Error;
use futures::Stream;
use std::pin::Pin;

/// Type alias for boxed errors
pub type BoxError = Box<dyn Error + Send + Sync>;

/// Type alias for boxed error results
pub type BoxResult<T> = std::result::Result<T, BoxError>;

/// Common trait for venue-specific message types
pub trait VenueMessage: Send + Sync + std::fmt::Debug {}

/// Common trait for all WebSocket connections
#[async_trait]
pub trait WebSocketConnection<T>: Send + Sync 
where 
    T: VenueMessage
{
    /// Connect to the WebSocket stream
    async fn connect(&mut self) -> BoxResult<()>;
    
    /// Disconnect from the WebSocket stream
    async fn disconnect(&mut self) -> BoxResult<()>;
    
    /// Check if the connection is alive
    fn is_connected(&self) -> bool;
    
    /// Get a stream of messages from the WebSocket
    /// Returns a Stream that yields venue-specific messages
    fn message_stream(&mut self) -> Pin<Box<dyn Stream<Item = BoxResult<T>> + Send>>;
}
