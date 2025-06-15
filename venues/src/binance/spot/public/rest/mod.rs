// Public REST endpoints module for Binance Spot

pub mod client;
pub mod depth;
pub mod exchange_info;
pub mod klines;
pub mod ticker_24hr;
pub mod ticker_price;
pub mod trades;

#[cfg(test)]
mod tests;

pub use client::RestClient;