//! Implements the /public/get_supported_index_names endpoint for Deribit.
//!
//! Retrieves the list of supported index names.

use super::RestClient;
use crate::deribit::{EndpointType, RestResult};

use reqwest::Method;
use serde::{Deserialize, Serialize};

const SUPPORTED_INDEX_NAMES_ENDPOINT: &str = "get_supported_index_names";

/// Request parameters for the get_supported_index_names endpoint.
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetSupportedIndexNamesRequest {}

/// The result object for get_supported_index_names.
#[derive(Debug, Clone, Deserialize)]
pub struct GetSupportedIndexNamesResult {
    /// List of supported index names.
    #[serde(rename = "index_names")]
    pub index_names: Vec<String>,
}

/// Response for the get_supported_index_names endpoint.
#[derive(Debug, Clone, Deserialize)]
pub struct GetSupportedIndexNamesResponse {
    /// The id that was sent in the request.
    #[serde(rename = "id")]
    pub id: u64,

    /// The JSON-RPC version (2.0).
    #[serde(rename = "jsonrpc")]
    pub jsonrpc: String,

    /// The result object containing the supported index names.
    #[serde(rename = "result")]
    pub result: GetSupportedIndexNamesResult,
}

impl RestClient {
    /// Calls the /public/get_supported_index_names endpoint.
    ///
    /// Retrieves the list of supported index names.
    ///
    /// [Official API docs](https://docs.deribit.com/#public-get_supported_index_names)
    pub async fn get_supported_index_names(
        &self,
        params: GetSupportedIndexNamesRequest,
    ) -> RestResult<GetSupportedIndexNamesResponse> {
        self.send_request(
            SUPPORTED_INDEX_NAMES_ENDPOINT,
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
        let req = GetSupportedIndexNamesRequest {};
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("{}"));
    }

    #[test]
    fn test_deserialize_response() {
        let data = r#"{
            "id": 26,
            "jsonrpc": "2.0",
            "result": {
                "index_names": ["btc_usd", "eth_usd"]
            }
        }"#;
        let resp: GetSupportedIndexNamesResponse = serde_json::from_str(data).unwrap();
        assert_eq!(resp.id, 26);
        assert_eq!(resp.jsonrpc, "2.0");
        assert_eq!(resp.result.index_names.len(), 2);
        assert_eq!(resp.result.index_names[0], "btc_usd");
        assert_eq!(resp.result.index_names[1], "eth_usd");
    }
}
