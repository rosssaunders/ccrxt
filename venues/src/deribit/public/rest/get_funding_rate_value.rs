//! Implements the /public/get_funding_rate_value endpoint for Deribit.
//!
//! Retrieves the current funding rate value for a given instrument.
use serde::Serialize;

use super::RestClient;
use crate::deribit::{EndpointType, JsonRpcResult, RestResult};

const FUNDING_RATE_VALUE_ENDPOINT: &str = "public/get_funding_rate_value";

/// Request parameters for the get_funding_rate_value endpoint.
#[derive(Debug, Clone, Serialize)]
pub struct GetFundingRateValueRequest {
    /// Instrument name (e.g., "BTC-PERPETUAL").
    #[serde(rename = "instrument_name")]
    pub instrument_name: String,

    /// Start timestamp for the funding rate data (required by API)
    #[serde(rename = "start_timestamp")]
    pub start_timestamp: u64,

    /// End timestamp for the funding rate data (required by API)
    #[serde(rename = "end_timestamp")]
    pub end_timestamp: u64,
}

/// Response for public/get_funding_rate_value endpoint following Deribit JSON-RPC 2.0 format.
pub type GetFundingRateValueResponse = JsonRpcResult<f64>;

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
        self.send_post_request(
            FUNDING_RATE_VALUE_ENDPOINT,
            Some(&params),
            EndpointType::NonMatchingEngine,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use serde_json;

    use super::*;

    #[test]
    fn test_serialize_request() {
        let req = GetFundingRateValueRequest {
            instrument_name: "BTC-PERPETUAL".to_string(),
            start_timestamp: 1680310800000,
            end_timestamp: 1680310800000 + 3600000, // 1 hour later
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("BTC-PERPETUAL"));
        assert!(json.contains("1680310800000"));
    }

    #[test]
    fn test_deserialize_response() {
        let data = r#"{
            "id": 9,
            "jsonrpc": "2.0",
            "result": 6.431212309587254e-7
        }"#;
        let resp: GetFundingRateValueResponse = serde_json::from_str(data).unwrap();
        assert_eq!(resp.id, 9);
        assert_eq!(resp.jsonrpc, "2.0");
        assert!((resp.result - 6.431212309587254e-7).abs() < 1e-15);
    }
}
