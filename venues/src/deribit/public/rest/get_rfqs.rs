//! Implements the /public/get_rfqs endpoint for Deribit.
//!
//! Retrieves the list of current RFQs (Request For Quotes).

use super::RestClient;
use crate::deribit::EndpointType;
use crate::deribit::RestResult;
use crate::deribit::enums::ComboState;

use reqwest::Method;
use serde::{Deserialize, Serialize};

/// Request parameters for the get_rfqs endpoint.
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetRfqsRequest {
    /// State of the RFQ to filter (optional).
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<ComboState>,

    /// Number of results to return (default: 10, max: 1000).
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<u32>,
}

/// Represents a single RFQ entry.
#[derive(Debug, Clone, Deserialize)]
pub struct RfqEntry {
    /// RFQ ID.
    #[serde(rename = "rfq_id")]
    pub rfq_id: String,

    /// State of the RFQ.
    #[serde(rename = "state")]
    pub state: ComboState,

    /// Timestamp in milliseconds since epoch.
    #[serde(rename = "timestamp")]
    pub timestamp: u64,
}

/// The result object for get_rfqs.
#[derive(Debug, Clone, Deserialize)]
pub struct GetRfqsResult {
    /// List of RFQ entries.
    #[serde(rename = "rfqs")]
    pub rfqs: Vec<RfqEntry>,
}

/// Response for the get_rfqs endpoint.
#[derive(Debug, Clone, Deserialize)]
pub struct GetRfqsResponse {
    /// The id that was sent in the request.
    #[serde(rename = "id")]
    pub id: u64,

    /// The JSON-RPC version (2.0).
    #[serde(rename = "jsonrpc")]
    pub jsonrpc: String,

    /// The result object containing the RFQs.
    #[serde(rename = "result")]
    pub result: GetRfqsResult,
}

impl RestClient {
    /// Calls the /public/get_rfqs endpoint.
    ///
    /// Retrieves the list of current RFQs (Request For Quotes).
    ///
    /// [Official API docs](https://docs.deribit.com/#public-get_rfqs)
    pub async fn get_rfqs(&self, params: GetRfqsRequest) -> RestResult<GetRfqsResponse> {
        self.send_request(
            "get_rfqs",
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
    use crate::deribit::enums::ComboState;
    use serde_json;

    #[test]
    fn test_serialize_request() {
        let req = GetRfqsRequest {
            state: Some(ComboState::Active),
            count: Some(2),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("active"));
        assert!(json.contains("count"));
    }

    #[test]
    fn test_deserialize_response() {
        let data = r#"{
            "id": 25,
            "jsonrpc": "2.0",
            "result": {
                "rfqs": [
                    {
                        "rfq_id": "rfq-123",
                        "state": "active",
                        "timestamp": 1680310800000
                    }
                ]
            }
        }"#;
        let resp: GetRfqsResponse = serde_json::from_str(data).unwrap();
        assert_eq!(resp.id, 25);
        assert_eq!(resp.jsonrpc, "2.0");
        assert_eq!(resp.result.rfqs.len(), 1);
        let rfq = &resp.result.rfqs[0];
        assert_eq!(rfq.rfq_id, "rfq-123");
        assert_eq!(rfq.state, ComboState::Active);
        assert_eq!(rfq.timestamp, 1680310800000);
    }
}
