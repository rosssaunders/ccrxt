//! Implements the /public/get_funding_rate_history endpoint for Deribit.
//!
//! Retrieves historical funding rates for a given instrument.
use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::deribit::{EndpointType, JsonRpcResult, RestResult};

const FUNDING_RATE_HISTORY_ENDPOINT: &str = "public/get_funding_rate_history";

/// Request parameters for the get_funding_rate_history endpoint.
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetFundingRateHistoryRequest {
    /// Instrument name (e.g., "BTC-PERPETUAL").
    #[serde(rename = "instrument_name")]
    pub instrument_name: String,

    /// Number of data points to retrieve (default 10, max 2000).
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<u32>,

    /// End timestamp in milliseconds since epoch (optional).
    #[serde(rename = "end_timestamp", skip_serializing_if = "Option::is_none")]
    pub end_timestamp: Option<u64>,
}

/// Represents a single funding rate history data point.
#[derive(Debug, Clone, Deserialize)]
pub struct FundingRateHistoryData {
    /// Funding rate value.
    #[serde(rename = "funding_rate")]
    pub funding_rate: f64,

    /// Timestamp in milliseconds since epoch.
    #[serde(rename = "timestamp")]
    pub timestamp: u64,
}

/// The result object for get_funding_rate_history.
#[derive(Debug, Clone, Deserialize)]
pub struct GetFundingRateHistoryResult {
    /// Array of funding rate history data points.
    #[serde(rename = "data")]
    pub data: Vec<FundingRateHistoryData>,
}

/// Response for the get_funding_rate_history endpoint.
pub type GetFundingRateHistoryResponse = JsonRpcResult<GetFundingRateHistoryResult>;

impl RestClient {
    /// Calls the /public/get_funding_rate_history endpoint.
    ///
    /// Retrieves historical funding rates for a given instrument.
    ///
    /// [Official API docs](https://docs.deribit.com/#public-get_funding_rate_history)
    pub async fn get_funding_rate_history(
        &self,
        params: GetFundingRateHistoryRequest,
    ) -> RestResult<GetFundingRateHistoryResponse> {
        self.send_request(
            FUNDING_RATE_HISTORY_ENDPOINT,
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
        let req = GetFundingRateHistoryRequest {
            instrument_name: "BTC-PERPETUAL".to_string(),
            count: Some(5),
            end_timestamp: Some(1680310800000),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("BTC-PERPETUAL"));
        assert!(json.contains("count"));
        assert!(json.contains("end_timestamp"));
    }

    #[test]
    fn test_deserialize_response() {
        let data = r#"{
            "id": 8,
            "jsonrpc": "2.0",
            "result": {
                "data": [
                    { "funding_rate": 0.0001, "timestamp": 1680307200000 },
                    { "funding_rate": 0.0002, "timestamp": 1680310800000 }
                ]
            }
        }"#;
        let resp: GetFundingRateHistoryResponse = serde_json::from_str(data).unwrap();
        assert_eq!(resp.id, 8);
        assert_eq!(resp.jsonrpc, "2.0");
        assert_eq!(resp.result.data.len(), 2);
        assert!((resp.result.data[0].funding_rate - 0.0001).abs() < 1e-8);
        assert_eq!(resp.result.data[1].timestamp, 1680310800000);
    }
}
