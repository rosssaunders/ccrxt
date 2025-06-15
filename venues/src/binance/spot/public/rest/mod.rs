// Public REST endpoints module for Binance Spot

pub mod client;
pub mod exchange_info;
pub mod market_data;

#[cfg(test)]
mod tests;

pub use client::RestClient;