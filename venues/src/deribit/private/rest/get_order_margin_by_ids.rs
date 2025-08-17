use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::deribit::{EndpointType, JsonRpcResult, RestResult};

/// REST API endpoint constant
const GET_ORDER_MARGIN_BY_IDS_ENDPOINT: &str = "private/get_order_margin_by_ids";

/// Retrieves initial margins of given orders by their unique identifiers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetOrderMarginByIdsRequest {
    /// Order identifiers for which to retrieve initial margin information
    pub ids: Vec<String>,
}

/// Response for /private/get_order_margin_by_ids
pub type GetOrderMarginByIdsResponse = JsonRpcResult<Vec<OrderMarginInfo>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderMarginInfo {
    /// Initial margin value required for the order
    pub initial_margin: f64,

    /// Currency of the initial margin value
    pub initial_margin_currency: String,

    /// Unique identifier of the order
    pub order_id: String,
}

impl RestClient {
    /// Retrieves initial margins of given orders.
    ///
    /// [docs](https://docs.deribit.com/v2/#private-get_order_margin_by_ids)
    pub async fn get_order_margin_by_ids(
        &self,
        request: GetOrderMarginByIdsRequest,
    ) -> RestResult<GetOrderMarginByIdsResponse> {
        self.send_signed_request(
            GET_ORDER_MARGIN_BY_IDS_ENDPOINT,
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

    /// REST API endpoint constant
    use super::*;

    #[test]
    fn test_serialize_request() {
        let req = GetOrderMarginByIdsRequest {
            ids: vec!["12345".to_string(), "67890".to_string()],
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("ids"));
        // Should serialize ids as an array of strings
        assert!(json.contains("12345") && json.contains("67890"));
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
                    "initial_margin_currency": "ETH",
                    "order_id": "12345"
                }
            ]
        }
        "#;
        let resp: GetOrderMarginByIdsResponse = serde_json::from_str(data).unwrap();
        assert_eq!(resp.id, 1);
        assert_eq!(resp.jsonrpc, "2.0");
        assert_eq!(resp.result.len(), 1);
        assert_eq!(resp.result[0].order_id, "12345");
        assert_eq!(resp.result[0].initial_margin_currency, "ETH");
        assert!((resp.result[0].initial_margin - 0.123).abs() < f64::EPSILON);
    }
}
