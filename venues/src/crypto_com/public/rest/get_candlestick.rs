//! Request and response structs for public/get-candlestick endpoint
//!
//! Retrieves candlesticks (k-line data history) over a given period for an instrument.

use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use crate::crypto_com::enums::Timeframe;

/// Request for public/get-candlestick
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCandlestickRequest {
    /// Instrument name, e.g. BTCUSD-PERP
    pub instrument_name: Cow<'static, str>,

    /// The period value (e.g. M5). Default is M1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeframe: Option<Timeframe>,

    /// Number of candlesticks to retrieve. Default is 25.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u32>,

    /// Start timestamp (Unix timestamp)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_ts: Option<i64>,

    /// End timestamp (Unix timestamp)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_ts: Option<i64>,
}

/// Response for public/get-candlestick
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCandlestickResponse {
    /// Response id
    pub id: i64,

    /// Method name
    pub method: Cow<'static, str>,

    /// Response code
    pub code: i32,

    /// Result data
    pub result: CandlestickResult,
}

/// Result data for candlesticks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CandlestickResult {
    /// The period (e.g. M5)
    pub interval: Cow<'static, str>,

    /// List of candlesticks
    pub data: Vec<Candlestick>,

    /// Instrument name
    pub instrument_name: Cow<'static, str>,
}

/// Candlestick data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Candlestick {
    /// Start time of candlestick (Unix timestamp)
    pub t: i64,

    /// Open price
    pub o: Cow<'static, str>,

    /// High price
    pub h: Cow<'static, str>,

    /// Low price
    pub l: Cow<'static, str>,

    /// Close price
    pub c: Cow<'static, str>,

    /// Volume
    pub v: Cow<'static, str>,
}
