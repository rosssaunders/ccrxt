//! Implements the /public/get_index_price endpoint for Deribit.
//!
//! Retrieves the current index price for a given index name (alias for get_index).

use super::RestClient;
use crate::deribit::{EndpointType, RestResult};

use reqwest::Method;
use serde::{Deserialize, Serialize};

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

/// Response for the get_index_price endpoint.
#[derive(Debug, Clone, Deserialize)]
pub struct GetIndexPriceResponse {
    /// The id that was sent in the request.
    #[serde(rename = "id")]
    pub id: u64,

    /// The JSON-RPC version (2.0).
    #[serde(rename = "jsonrpc")]
    pub jsonrpc: String,

    /// The result object containing the index price.
    #[serde(rename = "result")]
    pub result: GetIndexPriceResult,
}

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
            "public/get_index_price",
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
