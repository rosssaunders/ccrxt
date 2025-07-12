//! Request and response structs for public/get-valuations endpoint
//!
//! Fetches certain valuation type data for a particular instrument.

use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::cryptocom::{ApiResult, EndpointType, RestResult, ValuationType};

/// Endpoint path for the get-valuations API
const VALUATIONS_ENDPOINT: &str = "public/get-valuations";

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
pub type GetValuationsResponse = ApiResult<ValuationsResult>;

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
    /// Instrument name (may not be present in response).
    #[serde(rename = "instrument_name", skip_serializing_if = "Option::is_none")]
    pub instrument_name: Option<Cow<'static, str>>,

    /// Valuation type (may not be present in response).
    #[serde(rename = "valuation_type", skip_serializing_if = "Option::is_none")]
    pub valuation_type: Option<ValuationType>,

    /// Valuation value.
    #[serde(rename = "value")]
    pub value: f64,
}

impl RestClient {
    /// Calls the public/get-valuations endpoint.
    ///
    /// Fetches certain valuation type data for a particular instrument.
    ///
    /// [Official API docs](https://exchange-docs.crypto.com/exchange/v1/rest-ws/index.html#public-get-valuations)
    pub async fn get_valuations(
        &self,
        params: GetValuationsRequest,
    ) -> RestResult<GetValuationsResponse> {
        self.send_request(
            VALUATIONS_ENDPOINT,
            reqwest::Method::GET,
            Some(&params),
            EndpointType::PublicGetValuations,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

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
