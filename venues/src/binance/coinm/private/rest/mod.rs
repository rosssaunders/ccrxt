// Private REST endpoints module for Binance Coin-M

pub mod account;
pub mod account_trades;
pub mod batch_order;
pub mod client;

pub use client::RestClient;