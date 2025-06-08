// Private REST endpoints module for Binance Coin-M

use serde::Serialize;

pub mod account;
pub mod account_trades;
pub mod batch_order;
pub mod client;

pub use client::RestClient;

/// Trait for private API requests that require authentication.
/// All private request structs must implement this trait.
pub(super) trait PrivateRequest: Serialize {
    /// Returns the timestamp for the request in milliseconds since epoch.
    /// This is used for request signing and validation.
    fn timestamp(&self) -> u64;
}
