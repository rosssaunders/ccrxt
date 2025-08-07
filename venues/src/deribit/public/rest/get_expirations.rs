//! Implements the /public/get_expirations endpoint for Deribit.
//!
//! Retrieves available expiration timestamps for a given currency and instrument kind.

use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::deribit::{EndpointType, JsonRpcResult, RestResult};

const EXPIRATIONS_ENDPOINT: &str = "public/get_expirations";

/// Currency for get_expirations endpoint.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ExpirationsCurrency {
    #[serde(rename = "BTC")]
    BTC,
    #[serde(rename = "ETH")]
    ETH,
    #[serde(rename = "USDC")]
    USDC,
    #[serde(rename = "USDT")]
    USDT,
    #[serde(rename = "any")]
    Any,
    #[serde(rename = "grouped")]
    Grouped,
}

/// Instrument kind for get_expirations endpoint.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ExpirationsInstrumentKind {
    /// Futures
    #[serde(rename = "future")]
    Future,
    /// Options
    #[serde(rename = "option")]
    Option,
    /// Any kind
    #[serde(rename = "any")]
    Any,
}

/// Request parameters for the get_expirations endpoint.
#[derive(Debug, Clone, Serialize)]
pub struct GetExpirationsRequest {
    /// Currency for which to retrieve expirations.
    #[serde(rename = "currency")]
    pub currency: ExpirationsCurrency,

    /// Instrument kind: "future" or "option". Optional.
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<ExpirationsInstrumentKind>,
}

/// The result object for get_expirations.
#[derive(Debug, Clone, Deserialize)]
pub struct GetExpirationsResult {
    /// Future expirations as strings (e.g., "21SEP24", "PERPETUAL").
    #[serde(rename = "future")]
    pub future: Option<Vec<String>>,

    /// Option expirations as strings (e.g., "21SEP24", "22SEP24").
    #[serde(rename = "option")]
    pub option: Option<Vec<String>>,
}

/// Response for public/get_expirations endpoint following Deribit JSON-RPC 2.0 format.
pub type GetExpirationsResponse = JsonRpcResult<GetExpirationsResult>;

impl RestClient {
    /// Calls the /public/get_expirations endpoint.
    ///
    /// Retrieves available expiration timestamps for a given currency and instrument kind.
    ///
    /// [Official API docs](https://docs.deribit.com/#public-get_expirations)
    pub async fn get_expirations(
        &self,
        params: GetExpirationsRequest,
    ) -> RestResult<GetExpirationsResponse> {
    self.send_post_request(
            EXPIRATIONS_ENDPOINT,
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
        let req = GetExpirationsRequest {
            currency: ExpirationsCurrency::BTC,
            kind: Some(ExpirationsInstrumentKind::Option),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("BTC"));
        assert!(json.contains("option"));
    }

    #[test]
    fn test_deserialize_response() {
        let data = r#"{
            "id": 42,
            "jsonrpc": "2.0",
            "result": {
                "future": [
                    "21SEP24",
                    "22SEP24",
                    "PERPETUAL"
                ],
                "option": [
                    "21SEP24",
                    "22SEP24",
                    "23SEP24"
                ]
            }
        }"#;
        let resp: GetExpirationsResponse = serde_json::from_str(data).unwrap();
        assert_eq!(resp.id, 42);
        assert_eq!(resp.jsonrpc, "2.0");
        assert_eq!(resp.result.future.as_ref().unwrap().len(), 3);
        assert_eq!(resp.result.future.as_ref().unwrap()[0], "21SEP24");
        assert_eq!(resp.result.option.as_ref().unwrap().len(), 3);
        assert_eq!(resp.result.option.as_ref().unwrap()[2], "23SEP24");
    }
}
