//! Implements the /public/get_index_price_names endpoint for Deribit.
//!
//! Retrieves the list of all supported index price names.

use super::RestClient;
use crate::deribit::{EndpointType, RestResult};

use reqwest::Method;
use serde::{Deserialize, Serialize};

/// Request parameters for the get_index_price_names endpoint.
/// This endpoint does not require any parameters.
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetIndexPriceNamesRequest;

/// The result object for get_index_price_names.
#[derive(Debug, Clone, Deserialize)]
pub struct GetIndexPriceNamesResult {
    /// List of all supported index price names (e.g., "btc_usd").
    #[serde(rename = "index_price_names")]
    pub index_price_names: Vec<String>,
}

/// Response for the get_index_price_names endpoint.
#[derive(Debug, Clone, Deserialize)]
pub struct GetIndexPriceNamesResponse {
    /// The id that was sent in the request.
    #[serde(rename = "id")]
    pub id: u64,

    /// The JSON-RPC version (2.0).
    #[serde(rename = "jsonrpc")]
    pub jsonrpc: String,

    /// The result object containing the list of index price names.
    #[serde(rename = "result")]
    pub result: GetIndexPriceNamesResult,
}

impl RestClient {
    /// Calls the /public/get_index_price_names endpoint.
    ///
    /// Retrieves the list of all supported index price names.
    ///
    /// [Official API docs](https://docs.deribit.com/#public-get_index_price_names)
    pub async fn get_index_price_names(&self, params: GetIndexPriceNamesRequest) -> RestResult<GetIndexPriceNamesResponse> {
        self.send_request(
            "get_index_price_names",
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
        let req = GetIndexPriceNamesRequest;
        let json = serde_json::to_string(&req).unwrap();
        assert_eq!(json, "{}");
    }

    #[test]
    fn test_deserialize_response() {
        let data = r#"{
            \"id\": 13,
            \"jsonrpc\": \"2.0\",
            \"result\": {
                \"index_price_names\": [\"btc_usd\", \"eth_usd\"]
            }
        }"#;
        let resp: GetIndexPriceNamesResponse = serde_json::from_str(data).unwrap();
        assert_eq!(resp.id, 13);
        assert_eq!(resp.jsonrpc, "2.0");
        assert_eq!(resp.result.index_price_names, vec!["btc_usd", "eth_usd"]);
    }
}
