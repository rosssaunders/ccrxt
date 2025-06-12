//! Request and response structs for public/get-tickers endpoint
//!
//! Fetches the public tickers for all or a particular instrument.

use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// Request for public/get-tickers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTickersRequest {
    /// Instrument name, e.g. BTCUSD-PERP
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instrument_name: Option<Cow<'static, str>>,
}

/// Response for public/get-tickers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTickersResponse {
    /// Response id
    pub id: i64,

    /// Method name
    pub method: Cow<'static, str>,

    /// Response code
    pub code: i32,

    /// Result data
    pub result: TickersResult,
}

/// Result data for tickers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TickersResult {
    /// List of tickers
    pub data: Vec<Ticker>,
}

/// Ticker data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ticker {
    /// Price of the 24h highest trade
    pub h: Option<Cow<'static, str>>,

    /// Price of the 24h lowest trade, null if there weren't any trades
    pub l: Option<Cow<'static, str>>,

    /// The price of the latest trade, null if there weren't any trades
    pub a: Option<Cow<'static, str>>,

    /// Instrument name
    pub i: Cow<'static, str>,

    /// The total 24h traded volume
    pub v: Option<Cow<'static, str>>,

    /// The total 24h traded volume value (in USD)
    pub vv: Option<Cow<'static, str>>,

    /// The open interest
    pub oi: Option<Cow<'static, str>>,

    /// 24-hour price change, null if there weren't any trades
    pub c: Option<Cow<'static, str>>,

    /// The current best bid price, null if there aren't any bids
    pub b: Option<Cow<'static, str>>,

    /// The current best ask price, null if there aren't any asks
    pub k: Option<Cow<'static, str>>,

    /// The published timestamp in ms
    pub t: i64,
}
