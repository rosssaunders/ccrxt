mod base;
mod public;
mod authenticated;
mod types;
pub mod error;

pub use public::CoinbaseAdvancedTradeWebSocket;
pub use authenticated::CoinbaseAdvancedTradeWebSocketAuthenticated;
pub use types::*;
pub use error::CoinbaseAdvancedTradeError;
