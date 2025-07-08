//! Implements the /public/get_index_price endpoint for Deribit.
//!
//! Retrieves the current index price for a given index name (alias for get_index).
use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::deribit::{EndpointType, JsonRpcResult, RestResult};

const INDEX_PRICE_ENDPOINT: &str = "public/get_index_price";

/// Request parameters for the get_index_price endpoint.
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetIndexPriceRequest {
    /// Index name (e.g., "btc_usd").
    #[serde(rename = "index_name")]
    pub index_name: String,
}

/// The result object for get_index_price.
#[derive(Debug, Clone, Deserialize)]
pub struct GetIndexPriceResult {
    /// The current index price.
    #[serde(rename = "index_price")]
    pub index_price: f64,

    /// Timestamp in milliseconds since epoch for the index price.
    #[serde(rename = "timestamp")]
    pub timestamp: u64,
}

/// Response for public/get_index_price endpoint following Deribit JSON-RPC 2.0 format.
pub type GetIndexPriceResponse = JsonRpcResult<GetIndexPriceResult>;

impl RestClient {
    /// Calls the /public/get_index_price endpoint.
    ///
    /// Retrieves the current index price for a given index name (alias for get_index).
    ///
    /// [Official API docs](https://docs.deribit.com/#public-get_index_price)
    pub async fn get_index_price(
        &self,
        params: GetIndexPriceRequest,
    ) -> RestResult<GetIndexPriceResponse> {
        self.send_request(
            INDEX_PRICE_ENDPOINT,
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
        let req = GetIndexPriceRequest {
            index_name: "btc_usd".to_string(),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("btc_usd"));
    }

    #[test]
    fn test_deserialize_response() {
        let data = r#"{
            "id": 12,
            "jsonrpc": "2.0",
            "result": {
                "index_price": 65000.0,
                "timestamp": 1680310800000
            }
        }"#;
        let resp: GetIndexPriceResponse = serde_json::from_str(data).unwrap();
        assert_eq!(resp.id, 12);
        assert_eq!(resp.jsonrpc, "2.0");
        assert!((resp.result.index_price - 65000.0).abs() < 1e-8);
        assert_eq!(resp.result.timestamp, 1680310800000);
    }
}
