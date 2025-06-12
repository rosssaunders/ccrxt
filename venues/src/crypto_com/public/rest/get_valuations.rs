//! Request and response structs for public/get-valuations endpoint
//!
//! Fetches certain valuation type data for a particular instrument.

use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use crate::crypto_com::enums::ValuationType;

/// Request for public/get-valuations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetValuationsRequest {
    /// Instrument name, e.g. BTCUSD-INDEX
    pub instrument_name: Cow<'static, str>,

    /// Valuation type
    pub valuation_type: ValuationType,

    /// Number of records to retrieve. Default is 25.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u32>,

    /// Start timestamp (Unix timestamp)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_ts: Option<i64>,

    /// End timestamp (Unix timestamp)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_ts: Option<i64>,
}

/// Response for public/get-valuations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetValuationsResponse {
    /// Response id
    pub id: i64,

    /// Method name
    pub method: Cow<'static, str>,

    /// Response code
    pub code: i32,

    /// Result data
    pub result: ValuationsResult,
}

/// Result data for valuations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValuationsResult {
    /// List of valuations
    pub data: Vec<Valuation>,

    /// Instrument name
    pub instrument_name: Cow<'static, str>,
}

/// Valuation data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Valuation {
    /// Value
    pub v: Cow<'static, str>,

    /// Timestamp
    pub t: i64,
}
