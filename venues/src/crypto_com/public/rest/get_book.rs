//! Request and response structs for public/get-book endpoint
//!
//! Fetches the public order book for a particular instrument and depth.

use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// Request for public/get-book
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetBookRequest {
    /// Instrument name, e.g. BTCUSD-PERP
    pub instrument_name: Cow<'static, str>,

    /// Number of bids and asks to return (up to 50)
    pub depth: u32,
}

/// Response for public/get-book
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetBookResponse {
    /// Response code
    pub code: i32,

    /// Method name
    pub method: Cow<'static, str>,

    /// Result data
    pub result: BookResult,
}

/// Result data for order book
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookResult {
    /// Number of bids and asks to return (up to 50)
    pub depth: u32,

    /// List of book data
    pub data: Vec<BookData>,

    /// Instrument name
    pub instrument_name: Cow<'static, str>,
}

/// Book data for a single snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookData {
    /// Asks array: [0] = Price, [1] = Quantity, [2] = Number of Orders
    pub asks: Vec<[Cow<'static, str>; 3]>,

    /// Bids array: [0] = Price, [1] = Quantity, [2] = Number of Orders
    pub bids: Vec<[Cow<'static, str>; 3]>,
}
