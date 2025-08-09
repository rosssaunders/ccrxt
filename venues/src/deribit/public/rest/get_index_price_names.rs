//! Implements the /public/get_index_price_names endpoint for Deribit.
//!
//! Retrieves the list of all supported index price names.
use serde::Serialize;

use super::RestClient;
use crate::deribit::{EndpointType, JsonRpcResult, RestResult};

const INDEX_PRICE_NAMES_ENDPOINT: &str = "public/get_index_price_names";

/// Request parameters for the get_index_price_names endpoint.
/// This endpoint does not require any parameters.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(default)]
pub struct GetIndexPriceNamesRequest {}

/// The result is a direct list of strings (index price names)
pub type GetIndexPriceNamesResult = Vec<String>;

/// Response for the get_index_price_names endpoint.
pub type GetIndexPriceNamesResponse = JsonRpcResult<GetIndexPriceNamesResult>;

impl RestClient {
    /// Calls the /public/get_index_price_names endpoint.
    ///
    /// Retrieves the list of all supported index price names.
    ///
    /// [Official API docs](https://docs.deribit.com/#public-get_index_price_names)
    pub async fn get_index_price_names(
        &self,
        params: GetIndexPriceNamesRequest,
    ) -> RestResult<GetIndexPriceNamesResponse> {
        self.send_post_request(
            INDEX_PRICE_NAMES_ENDPOINT,
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
        let req = GetIndexPriceNamesRequest {};
        let json = serde_json::to_string(&req).unwrap();
        assert_eq!(json, "{}");
    }

    #[test]
    fn test_deserialize_response() {
        let data = r#"{
            "id": 13,
            "jsonrpc": "2.0",
            "result": ["btc_usd", "eth_usd"]
        }"#;
        let resp: GetIndexPriceNamesResponse = serde_json::from_str(data).unwrap();
        assert_eq!(resp.id, 13);
        assert_eq!(resp.jsonrpc, "2.0");
        assert_eq!(resp.result, vec!["btc_usd", "eth_usd"]);
    }
}
