//! Implements the /public/get_volatility_index_data endpoint for Deribit.
//!
//! Retrieves volatility index data for a given index name.
//!
//! [Official API docs](https://docs.deribit.com/#public-get_volatility_index_data)

use std::borrow::Cow;


use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::deribit::{EndpointType, JsonRpcResult, RestResult};

const VOLATILITY_INDEX_DATA_ENDPOINT: &str = "public/get_volatility_index_data";

/// Request parameters for the get_volatility_index_data endpoint.
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetVolatilityIndexDataRequest {
    /// Index name (e.g., "btc_usd").
    #[serde(rename = "index_name")]
    pub index_name: Cow<'static, str>,
}

/// The result object for get_volatility_index_data.
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct GetVolatilityIndexDataResult {
    /// The current volatility index value.
    #[serde(rename = "volatility_index")]
    pub volatility_index: f64,

    /// Timestamp in milliseconds since epoch for the volatility index value.
    #[serde(rename = "timestamp")]
    pub timestamp: u64,
}

/// Response for public/get_volatility_index_data endpoint following Deribit JSON-RPC 2.0 format.
pub type GetVolatilityIndexDataResponse = JsonRpcResult<GetVolatilityIndexDataResult>;

impl RestClient {
    /// Calls the /public/get_volatility_index_data endpoint.
    ///
    /// Retrieves volatility index data for a given index name.
    ///
    /// [Official API docs](https://docs.deribit.com/#public-get_volatility_index_data)
    pub async fn get_volatility_index_data(
        &self,
        params: GetVolatilityIndexDataRequest,
    ) -> RestResult<GetVolatilityIndexDataResponse> {
        self.send_request(
            VOLATILITY_INDEX_DATA_ENDPOINT,

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
        let req = GetVolatilityIndexDataRequest {
            index_name: Cow::Borrowed("btc_usd"),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("btc_usd"));
    }

    #[test]
    fn test_deserialize_response() {
        let data = r#"{
    "id": 21,
    "jsonrpc": "2.0",
    "result": {
        "volatility_index": 5.4321,
        "timestamp": 1680310800000
    }
}"#;
        let resp: GetVolatilityIndexDataResponse = serde_json::from_str(data).unwrap();
        assert_eq!(resp.id, 21);
        assert_eq!(resp.jsonrpc, "2.0");
        assert!((resp.result.volatility_index - 5.4321).abs() < 1e-8);
        assert_eq!(resp.result.timestamp, 1680310800000);
    }
}
