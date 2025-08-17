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
    pub index_name: String,
}

/// The result object for get_index_price.
#[derive(Debug, Clone, Deserialize)]
pub struct GetIndexPriceResult {
    /// Value of requested index.
    pub index_price: f64,

    /// Estimated delivery price for the market. For more details, see Documentation > General > Expiration Price.
    pub estimated_delivery_price: f64,
}

/// Response for public/get_index_price endpoint following Deribit JSON-RPC 2.0 format.
pub type GetIndexPriceResponse = JsonRpcResult<GetIndexPriceResult>;

impl RestClient {
    /// Calls the /public/get_index_price endpoint.
    ///
    /// Retrieves the current index price for a given index name (alias for get_index).
    ///
    /// [docs](https://docs.deribit.com/#public-get_index_price)
    pub async fn get_index_price(
        &self,
        params: GetIndexPriceRequest,
    ) -> RestResult<GetIndexPriceResponse> {
        self.send_post_request(
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
                "estimated_delivery_price": 65000.0,
                "timestamp": 65000.0
            }
        }"#;
        let resp: GetIndexPriceResponse = serde_json::from_str(data).unwrap();
        assert_eq!(resp.id, 12);
        assert_eq!(resp.jsonrpc, "2.0");
        assert!((resp.result.index_price - 65000.0).abs() < 1e-8);
        assert!((resp.result.estimated_delivery_price - 65000.0).abs() < 1e-8);
    }
}
