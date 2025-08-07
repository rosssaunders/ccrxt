//! Implements the /public/get_mark_price_history endpoint for Deribit.
//!
//! Retrieves historical mark prices for a given instrument.

use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::deribit::{EndpointType, JsonRpcResult, RestResult};

const MARK_PRICE_HISTORY_ENDPOINT: &str = "public/get_mark_price_history";

/// Request parameters for the get_mark_price_history endpoint.
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetMarkPriceHistoryRequest {
    /// Instrument name for which to retrieve mark price history.
    #[serde(rename = "instrument_name")]
    pub instrument_name: String,

    /// Start timestamp in milliseconds since epoch (inclusive).
    #[serde(rename = "start_timestamp")]
    pub start_timestamp: u64,

    /// End timestamp in milliseconds since epoch (inclusive).
    #[serde(rename = "end_timestamp")]
    pub end_timestamp: u64,

    /// Number of data points to return (default: 10, max: 1000).
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<u32>,
}

/// Represents a single mark price entry as a tuple [timestamp, mark_price].
#[derive(Debug, Clone, Deserialize)]
#[serde(transparent)]
pub struct MarkPriceEntry(pub (u64, f64));

impl MarkPriceEntry {
    /// Get the timestamp in milliseconds since epoch.
    pub fn timestamp(&self) -> u64 {
        self.0.0
    }

    /// Get the mark price.
    pub fn mark_price(&self) -> f64 {
        self.0.1
    }
}

/// Response for public/get_mark_price_history endpoint following Deribit JSON-RPC 2.0 format.
pub type GetMarkPriceHistoryResponse = JsonRpcResult<Vec<MarkPriceEntry>>;

impl RestClient {
    /// Calls the /public/get_mark_price_history endpoint.
    ///
    /// Retrieves historical mark prices for a given instrument.
    ///
    /// [Official API docs](https://docs.deribit.com/#public-get_mark_price_history)
    pub async fn get_mark_price_history(
        &self,
        params: GetMarkPriceHistoryRequest,
    ) -> RestResult<GetMarkPriceHistoryResponse> {
    self.send_post_request(
            MARK_PRICE_HISTORY_ENDPOINT,
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
        let req = GetMarkPriceHistoryRequest {
            instrument_name: "BTC-PERPETUAL".to_string(),
            start_timestamp: 1680310000000,
            end_timestamp: 1680310800000,
            count: Some(2),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("BTC-PERPETUAL"));
        assert!(json.contains("1680310000000"));
        assert!(json.contains("1680310800000"));
        assert!(json.contains("count"));
    }

    #[test]
    fn test_deserialize_response() {
        let data = r#"{
            "id": 22,
            "jsonrpc": "2.0",
            "result": [
                [1608142381229, 0.5165791606037885],
                [1608142380231, 0.5165737855432504],
                [1608142379227, 0.5165768236356326]
            ]
        }"#;
        let resp: GetMarkPriceHistoryResponse = serde_json::from_str(data).unwrap();
        assert_eq!(resp.id, 22);
        assert_eq!(resp.jsonrpc, "2.0");
        assert_eq!(resp.result.len(), 3);

        let entry = &resp.result[0];
        assert_eq!(entry.timestamp(), 1608142381229);
        assert!((entry.mark_price() - 0.5165791606037885).abs() < 1e-15);

        let entry = &resp.result[1];
        assert_eq!(entry.timestamp(), 1608142380231);
        assert!((entry.mark_price() - 0.5165737855432504).abs() < 1e-15);
    }
}
