//! Implements the /public/get_funding_rate_value endpoint for Deribit.
//!
//! Retrieves the current funding rate value for a given instrument.

use super::RestClient;
use crate::deribit::{EndpointType, RestResult};

use reqwest::Method;
use serde::{Deserialize, Serialize};

/// Request parameters for the get_funding_rate_value endpoint.
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetFundingRateValueRequest {
    /// Instrument name (e.g., "BTC-PERPETUAL").
    #[serde(rename = "instrument_name")]
    pub instrument_name: String,
}

/// The result object for get_funding_rate_value.
#[derive(Debug, Clone, Deserialize)]
pub struct GetFundingRateValueResult {
    /// The current funding rate value.
    #[serde(rename = "funding_rate")]
    pub funding_rate: f64,

    /// Timestamp in milliseconds since epoch for the funding rate.
    #[serde(rename = "timestamp")]
    pub timestamp: u64,
}

/// Response for the get_funding_rate_value endpoint.
#[derive(Debug, Clone, Deserialize)]
pub struct GetFundingRateValueResponse {
    /// The id that was sent in the request.
    #[serde(rename = "id")]
    pub id: u64,

    /// The JSON-RPC version (2.0).
    #[serde(rename = "jsonrpc")]
    pub jsonrpc: String,

    /// The result object containing the funding rate value.
    #[serde(rename = "result")]
    pub result: GetFundingRateValueResult,
}

impl RestClient {
    /// Calls the /public/get_funding_rate_value endpoint.
    ///
    /// Retrieves the current funding rate value for a given instrument.
    ///
    /// [Official API docs](https://docs.deribit.com/#public-get_funding_rate_value)
    pub async fn get_funding_rate_value(
        &self,
        params: GetFundingRateValueRequest,
    ) -> RestResult<GetFundingRateValueResponse> {
        self.send_request(
            "get_funding_rate_value",
            Method::POST,
            Some(&params),
            EndpointType::NonMatchingEngine,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_serialize_request() {
        let req = GetFundingRateValueRequest {
            instrument_name: "BTC-PERPETUAL".to_string(),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("BTC-PERPETUAL"));
    }

    #[test]
    fn test_deserialize_response() {
        let data = r#"{
            "id": 9,
            "jsonrpc": "2.0",
            "result": {
                "funding_rate": 0.0001,
                "timestamp": 1680310800000
            }
        }"#;
        let resp: GetFundingRateValueResponse = serde_json::from_str(data).unwrap();
        assert_eq!(resp.id, 9);
        assert_eq!(resp.jsonrpc, "2.0");
        assert!((resp.result.funding_rate - 0.0001).abs() < 1e-8);
        assert_eq!(resp.result.timestamp, 1680310800000);
    }
}
