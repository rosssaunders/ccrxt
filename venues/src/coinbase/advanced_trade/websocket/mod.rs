mod authenticated;
mod base;
pub mod error;
mod public;
mod types;

pub use authenticated::CoinbaseAdvancedTradeWebSocketAuthenticated;
pub use error::CoinbaseAdvancedTradeError;
pub use public::CoinbaseAdvancedTradeWebSocket;
pub use types::*;
