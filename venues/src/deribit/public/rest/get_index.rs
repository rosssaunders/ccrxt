//! Implements the /public/get_index endpoint for Deribit.
//!
//! Retrieves the current index price for a given index name.
use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::deribit::{EndpointType, JsonRpcResult, RestResult};

const INDEX_ENDPOINT: &str = "public/get_index";

/// Request parameters for the get_index endpoint.
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetIndexRequest {
    /// Index name (e.g., "btc_usd").
    #[serde(rename = "index_name")]
    pub index_name: String,

    /// Currency to retrieve the index for (e.g., "BTC", "ETH").
    #[serde(rename = "currency")]
    pub currency: String,
}

/// The result object for get_index.
#[derive(Debug, Clone, Deserialize)]
pub struct GetIndexResult {
    /// The estimated delivery price.
    #[serde(rename = "edp")]
    pub estimated_delivery_price: f64,

    /// The index price (mapped by currency).
    #[serde(flatten)]
    pub currency_price: std::collections::HashMap<String, f64>,
}

/// Response for the get_index endpoint.
pub type GetIndexResponse = JsonRpcResult<GetIndexResult>;

impl RestClient {
    /// Calls the /public/get_index endpoint.
    ///
    /// Retrieves the current index price for a given index name.
    ///
    /// [Official API docs](https://docs.deribit.com/#public-get_index)
    pub async fn get_index(&self, params: GetIndexRequest) -> RestResult<GetIndexResponse> {
        self.send_request(
            INDEX_ENDPOINT,
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
        let req = GetIndexRequest {
            index_name: "btc_usd".to_string(),
            currency: "BTC".to_string(),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("btc_usd"));
    }

    #[test]
    fn test_deserialize_response() {
        let data = r#"{
            "id": 11,
            "jsonrpc": "2.0",
            "result": {
                "edp": 65100.0,
                "BTC": 65000.0
            }
        }"#;
        let resp: GetIndexResponse = serde_json::from_str(data).unwrap();
        assert_eq!(resp.id, 11);
        assert_eq!(resp.jsonrpc, "2.0");
        assert!((resp.result.estimated_delivery_price - 65100.0).abs() < 1e-8);
        assert!(resp.result.currency_price.contains_key("BTC"));
        assert!((resp.result.currency_price["BTC"] - 65000.0).abs() < 1e-8);
    }
}
