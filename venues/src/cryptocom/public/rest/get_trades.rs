//! Request and response structs for public/get-trades endpoint
//!
//! Fetches the public trades for a particular instrument.

use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use crate::crypto_com::enums::TradeSide;

/// Request for public/get-trades
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTradesRequest {
    /// Instrument name, e.g. BTCUSD-PERP
    pub instrument_name: Cow<'static, str>,

    /// The maximum number of trades to be retrieved. Default: 25, Max: 150
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u32>,

    /// Start time in Unix time format (inclusive)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_ts: Option<i64>,

    /// End time in Unix time format (exclusive)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_ts: Option<i64>,
}

/// Response for public/get-trades
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTradesResponse {
    /// Response id
    pub id: i64,

    /// Method name
    pub method: Cow<'static, str>,

    /// Response code
    pub code: i32,

    /// Result data
    pub result: TradesResult,
}

/// Result data for trades
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradesResult {
    /// List of trades
    pub data: Vec<Trade>,
}

/// Trade data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trade {
    /// Trade ID
    pub d: Cow<'static, str>,

    /// Trade timestamp milliseconds
    pub t: i64,

    /// Trade timestamp nanoseconds
    pub tn: Cow<'static, str>,

    /// Quantity
    pub q: Cow<'static, str>,

    /// Price
    pub p: Cow<'static, str>,

    /// Side (BUY or SELL)
    pub s: TradeSide,

    /// Instrument name
    pub i: Cow<'static, str>,

    /// Trade match ID
    pub m: Cow<'static, str>,
}
