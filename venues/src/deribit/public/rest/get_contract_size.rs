//! Implements the /public/get_contract_size endpoint for Deribit.
//!
//! Retrieves contract size of provided instrument.

use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::deribit::{EndpointType, JsonRpcResult, RestResult};

const CONTRACT_SIZE_ENDPOINT: &str = "public/get_contract_size";

/// Request parameters for the get_contract_size endpoint.
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetContractSizeRequest {
    /// Instrument name.
    #[serde(rename = "instrument_name")]
    pub instrument_name: String,
}

/// The result object for get_contract_size.
#[derive(Debug, Clone, Deserialize)]
pub struct GetContractSizeResult {
    /// Contract size, for futures in USD, for options in base currency of the instrument (BTC, ETH, ...).
    #[serde(rename = "contract_size")]
    pub contract_size: f64,
}

/// Response for public/get_combo_ids endpoint following Deribit JSON-RPC 2.0 format.
pub type GetContractSizeResponse = JsonRpcResult<GetContractSizeResult>;

impl RestClient {
    /// Calls the /public/get_contract_size endpoint.
    ///
    /// Retrieves contract size of provided instrument.
    ///
    /// [Official API docs](https://docs.deribit.com/#public-get_contract_size)
    pub async fn get_contract_size(
        &self,
        params: GetContractSizeRequest,
    ) -> RestResult<GetContractSizeResponse> {
    self.send_post_request(
            CONTRACT_SIZE_ENDPOINT,
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
        let req = GetContractSizeRequest {
            instrument_name: "BTC-PERPETUAL".to_string(),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("BTC-PERPETUAL"));
    }

    #[test]
    fn test_deserialize_response() {
        let data = r#"{
            "id": 1,
            "jsonrpc": "2.0",
            "result": {
                "contract_size": 100
            }
        }"#;
        let resp: GetContractSizeResponse = serde_json::from_str(data).unwrap();
        assert_eq!(resp.id, 1);
        assert_eq!(resp.jsonrpc, "2.0");
        assert_eq!(resp.result.contract_size, 100.0);
    }
}
