pub mod candlesticks;
pub mod currencies;
pub mod currency_pair;
pub mod currency_pairs;
pub mod get_currency;
pub mod insurance;
pub mod order_book;
pub mod server_time;
pub mod tickers;
pub mod trades;
pub mod trading_fee;

pub use crate::gateio::public_client::PublicRestClient as RestClient;
