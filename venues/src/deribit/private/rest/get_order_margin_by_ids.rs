//! Retrieves initial margins of given orders via /private/get_order_margin_by_ids
//!
//! This module defines the request/response types and logic for the Deribit private endpoint.

use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::deribit::{EndpointType, RestResult};

/// Request for /private/get_order_margin_by_ids
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetOrderMarginByIdsRequest {
    pub ids: Vec<String>,
}

/// Response for /private/get_order_margin_by_ids
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetOrderMarginByIdsResponse {
    pub jsonrpc: String,
    pub id: u64,
    pub result: Vec<OrderMarginInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderMarginInfo {
    pub initial_margin: f64,
    pub initial_margin_currency: String,
    pub order_id: String,
}

impl RestClient {
    pub async fn get_order_margin_by_ids(
        &self,
        request: GetOrderMarginByIdsRequest,
    ) -> RestResult<GetOrderMarginByIdsResponse> {
        self.send_signed_request(
            "private/get_order_margin_by_ids",
            &request,
            EndpointType::MatchingEngine,
        )
        .await
    }
}

// Unit tests for serialization/deserialization
#[cfg(test)]
mod tests {
    use serde_json;

    use super::*;

    #[test]
    fn test_serialize_request() {
        let req = GetOrderMarginByIdsRequest {
            ids: vec!["12345".to_string(), "67890".to_string()],
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("ids"));
    }

    #[test]
    fn test_deserialize_response() {
        let data = r#"
        {
            "jsonrpc": "2.0",
            "id": 1,
            "result": [
                {
                    "initial_margin": 0.123,
                    "initial_margin_currency": "USD",
                    "order_id": "12345"
                }
            ]
        }
        "#;
        let resp: GetOrderMarginByIdsResponse = serde_json::from_str(data).unwrap();
        assert_eq!(resp.result[0].order_id, "12345");
    }
}
