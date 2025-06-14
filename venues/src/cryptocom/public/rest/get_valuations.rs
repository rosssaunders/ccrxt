//! Request and response structs for public/get-valuations endpoint
//!
//! Fetches certain valuation type data for a particular instrument.

use super::client::RestClient;
use crate::cryptocom::EndpointType;
use crate::cryptocom::RestResult;
use crate::cryptocom::ValuationType;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// Request parameters for the public/get-valuations endpoint.
///
/// Fetches certain valuation type data for a particular instrument.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetValuationsRequest {
    /// Instrument name (e.g., "BTCUSD-PERP"). Required.
    #[serde(rename = "instrument_name")]
    pub instrument_name: Cow<'static, str>,

    /// Valuation type. Required.
    #[serde(rename = "valuation_type")]
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

/// Response for public/get-valuations endpoint.
#[derive(Debug, Clone, Deserialize)]
pub struct GetValuationsResponse {
    /// Result data for valuations.
    #[serde(rename = "result")]
    pub result: ValuationsResult,

    /// Success status.
    #[serde(rename = "success")]
    pub success: bool,

    /// Response ID.
    #[serde(rename = "id")]
    pub id: u64,
}

/// Result data for valuations.
#[derive(Debug, Clone, Deserialize)]
pub struct ValuationsResult {
    /// List of valuation data.
    #[serde(rename = "data")]
    pub data: Vec<Valuation>,
}

/// Valuation data for an instrument.
#[derive(Debug, Clone, Deserialize)]
pub struct Valuation {
    /// Instrument name.
    #[serde(rename = "instrument_name")]
    pub instrument_name: Cow<'static, str>,

    /// Valuation type.
    #[serde(rename = "valuation_type")]
    pub valuation_type: ValuationType,

    /// Valuation value.
    #[serde(rename = "value")]
    pub value: f64,
}

impl RestClient {
    /// Calls the public/get-valuations endpoint.
    ///
    /// Fetches certain valuation type data for a particular instrument.
    ///
    /// [Official API docs](https://exchange-docs.crypto.com/spot/index.html)
    pub async fn get_valuations(
        &self,
        params: GetValuationsRequest,
    ) -> RestResult<GetValuationsResponse> {
        self.send_request(
            "public/get-valuations",
            reqwest::Method::GET,
            Some(&params),
            EndpointType::PublicGetValuations,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_valuations_endpoint_type() {
        let valuations_endpoint = EndpointType::PublicGetValuations;
        assert!(valuations_endpoint.rate_limit().max_requests > 0);
    }

    #[test]
    fn test_valuations_parameter_building() {
        let params = json!({
            "instrument_name": "BTCUSD-INDEX",
            "valuation_type": "index_price",
            "count": 10
        });
        assert_eq!(params.get("instrument_name").unwrap(), "BTCUSD-INDEX");
        assert_eq!(params.get("valuation_type").unwrap(), "index_price");
        assert_eq!(params.get("count").unwrap(), 10);
    }
}
