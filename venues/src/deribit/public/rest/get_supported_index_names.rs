//! Implements the /public/get_supported_index_names endpoint for Deribit.
//!
//! Retrieves the list of supported index names.

use super::RestClient;
use crate::deribit::{EndpointType, JsonRpcResult, RestResult};

const SUPPORTED_INDEX_NAMES_ENDPOINT: &str = "public/get_supported_index_names";

/// Response for public/get_supported_index_names endpoint following Deribit JSON-RPC 2.0 format.
pub type GetSupportedIndexNamesResponse = JsonRpcResult<Vec<String>>;

impl RestClient {
    /// Calls the /public/get_supported_index_names endpoint.
    ///
    /// Retrieves the list of supported index names.
    ///
    /// [Official API docs](https://docs.deribit.com/#public-get_supported_index_names)
    pub async fn get_supported_index_names(&self) -> RestResult<GetSupportedIndexNamesResponse> {
        self.send_post_request(
            SUPPORTED_INDEX_NAMES_ENDPOINT,
            None::<&()>,
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
    fn test_deserialize_response() {
        let data = r#"{
            "id": 26,
            "jsonrpc": "2.0",
            "result": [
                "btc_usd",
                "eth_usd"
            ]
        }"#;
        let resp: GetSupportedIndexNamesResponse = serde_json::from_str(data).unwrap();
        assert_eq!(resp.id, 26);
        assert_eq!(resp.jsonrpc, "2.0");
        assert_eq!(resp.result.len(), 2);
        assert_eq!(resp.result[0], "btc_usd");
        assert_eq!(resp.result[1], "eth_usd");
    }
}
