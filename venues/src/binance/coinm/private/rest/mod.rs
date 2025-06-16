// Private REST endpoints module for Binance Coin-M

pub mod account;
pub mod account_trades;
pub mod all_orders;
pub mod batch_order;
pub mod cancel_order;
pub mod client;
pub mod open_orders;
pub mod order;
pub mod position_risk;
pub mod query_order;

pub use client::RestClient;
