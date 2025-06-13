//! Request and response structs for public/get-expired-settlement-price endpoint
//!
//! Fetches settlement price of expired instruments.

use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use crate::crypto_com::enums::InstrumentType;

/// Request for public/get-expired-settlement-price
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetExpiredSettlementPriceRequest {
    /// Instrument type, e.g. FUTURE
    pub instrument_type: InstrumentType,

    /// Page number. Default is 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
}

/// Response for public/get-expired-settlement-price
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetExpiredSettlementPriceResponse {
    /// Response id
    pub id: i64,

    /// Method name
    pub method: Cow<'static, str>,

    /// Response code
    pub code: i32,

    /// Result data
    pub result: ExpiredSettlementPriceResult,
}

/// Result data for expired settlement prices
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpiredSettlementPriceResult {
    /// List of expired settlement prices
    pub data: Vec<ExpiredSettlementPrice>,
}

/// Expired settlement price data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpiredSettlementPrice {
    /// Instrument name
    pub i: Cow<'static, str>,

    /// Expiry timestamp (millisecond)
    pub x: i64,

    /// Value
    pub v: Cow<'static, str>,

    /// Timestamp
    pub t: i64,
}
