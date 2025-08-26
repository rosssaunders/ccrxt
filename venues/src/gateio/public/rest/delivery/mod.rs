pub mod candlesticks_index_price;
pub mod candlesticks_mark_price;
pub mod candlesticks_standard;
pub mod contracts;
pub mod insurance;
pub mod order_book;
pub mod risk_limit_tiers;
pub mod tickers;
pub mod trades;

pub use crate::gateio::public_client::PublicRestClient as RestClient;
