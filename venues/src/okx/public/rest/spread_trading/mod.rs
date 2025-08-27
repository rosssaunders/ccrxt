// Public spread trading endpoints

pub mod get_books;
pub mod get_candles;
pub mod get_history_candles;
pub mod get_public_trades;
pub mod get_spreads;
pub mod get_ticker;

pub use crate::okx::public_client::RestClient;
