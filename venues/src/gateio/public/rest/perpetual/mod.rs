pub mod candlestick;
pub mod contract;
pub mod funding_rate;
pub mod get_futures_candlesticks;
pub mod get_futures_contract;
pub mod get_futures_contracts;
pub mod get_futures_index_price_candlesticks;
pub mod get_futures_mark_price_candlesticks;
pub mod index_constituents;
pub mod insurance;
pub mod order_book;
pub mod premium_index;
pub mod risk_limit_tiers;
pub mod stats;
pub mod tickers;
pub mod trades;

pub use crate::gateio::public_client::PublicRestClient as RestClient;
