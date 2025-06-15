//! Deribit public WebSocket endpoints

pub mod unsubscribe;

// Re-export public endpoint types
pub use unsubscribe::{UnsubscribeRequest, UnsubscribeResponse, UnsubscribeError};