use std::pin::Pin;

use async_trait::async_trait;
use futures::Stream;
use serde::{Deserialize, Serialize};

pub mod events;

#[cfg(feature = "native")]
pub mod native;

#[cfg(feature = "wasm")]
pub mod wasm;

pub mod builder;

// Re-export key types
pub use events::{ConnectionState, DisconnectReason, WebSocketError, WebSocketEvent};

// Note: BoxError and BoxResult were removed as they're not currently used.
// They can be re-added if needed in the future for error handling consistency.

/// Common trait for venue-specific message types
pub trait VenueMessage:
    Send + Sync + std::fmt::Debug + for<'de> Deserialize<'de> + Serialize
{
}

/// Common trait for all WebSocket connections (native version with Send + Sync)
#[cfg(not(target_arch = "wasm32"))]
#[async_trait]
pub trait WebSocketConnection<T>: Send + Sync
where
    T: VenueMessage,
{
    /// Connect to the WebSocket endpoint
    /// Returns error if connection fails
    /// Does NOT automatically retry
    async fn connect(&mut self) -> Result<(), WebSocketError>;

    /// Disconnect from the WebSocket endpoint
    /// Gracefully closes the connection
    async fn disconnect(&mut self) -> Result<(), WebSocketError>;

    /// Check if currently connected
    fn is_connected(&self) -> bool;

    /// Get detailed connection state
    fn connection_state(&self) -> ConnectionState;

    /// Get a stream of all WebSocket events
    /// Includes connection events and messages
    fn event_stream(&mut self) -> Pin<Box<dyn Stream<Item = WebSocketEvent<T>> + Send>>;

    /// Send a message over the WebSocket
    async fn send(&mut self, message: T) -> Result<(), WebSocketError>;
}

/// Common trait for all WebSocket connections (WASM version without Send + Sync)
#[cfg(target_arch = "wasm32")]
#[async_trait(?Send)]
pub trait WebSocketConnection<T>
where
    T: VenueMessage,
{
    /// Connect to the WebSocket endpoint
    /// Returns error if connection fails
    /// Does NOT automatically retry
    async fn connect(&mut self) -> Result<(), WebSocketError>;

    /// Disconnect from the WebSocket endpoint
    /// Gracefully closes the connection
    async fn disconnect(&mut self) -> Result<(), WebSocketError>;

    /// Check if currently connected
    fn is_connected(&self) -> bool;

    /// Get detailed connection state
    fn connection_state(&self) -> ConnectionState;

    /// Get a stream of all WebSocket events
    /// Includes connection events and messages
    fn event_stream(&mut self) -> Pin<Box<dyn Stream<Item = WebSocketEvent<T>>>>;

    /// Send a message over the WebSocket
    async fn send(&mut self, message: T) -> Result<(), WebSocketError>;
}
