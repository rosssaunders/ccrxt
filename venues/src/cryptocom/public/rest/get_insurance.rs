//! Request and response structs for public/get-insurance endpoint
//!
//! Fetches balance of Insurance Fund for a particular currency.

use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// Request for public/get-insurance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetInsuranceRequest {
    /// Instrument name, e.g. USD
    pub instrument_name: Cow<'static, str>,

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

/// Response for public/get-insurance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetInsuranceResponse {
    /// Response id
    pub id: i64,

    /// Method name
    pub method: Cow<'static, str>,

    /// Response code
    pub code: i32,

    /// Result data
    pub result: InsuranceResult,
}

/// Result data for insurance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsuranceResult {
    /// List of insurance data
    pub data: Vec<Insurance>,

    /// Instrument name
    pub instrument_name: Cow<'static, str>,
}

/// Insurance data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Insurance {
    /// Value
    pub v: Cow<'static, str>,

    /// Timestamp (ms)
    pub t: i64,
}
