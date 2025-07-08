//! Implements the /public/get_expirations endpoint for Deribit.
//!
//! Retrieves available expiration timestamps for a given currency and instrument kind.

use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::deribit::{EndpointType, JsonRpcResult, RestResult, enums::Currency};

const EXPIRATIONS_ENDPOINT: &str = "public/get_expirations";

/// Instrument kind for get_expirations endpoint.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum InstrumentKind {
    /// Futures
    #[serde(rename = "future")]
    Future,
    /// Options
    #[serde(rename = "option")]
    Option,
}

/// Request parameters for the get_expirations endpoint.
#[derive(Debug, Clone, Serialize)]
pub struct GetExpirationsRequest {
    /// Currency for which to retrieve expirations.
    #[serde(rename = "currency")]
    pub currency: Currency,

    /// Instrument kind: "future" or "option". Optional.
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<InstrumentKind>,
}

/// The result object for get_expirations.
#[derive(Debug, Clone, Deserialize)]
pub struct GetExpirationsResult {
    /// List of available expiration timestamps (milliseconds since epoch).
    #[serde(rename = "expirations")]
    pub expirations: Vec<u64>,
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
        self.send_request(
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
            currency: Currency::BTC,
            kind: Some(InstrumentKind::Option),
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
                "expirations": [1680307200000, 1682918400000, 1685529600000]
            }
        }"#;
        let resp: GetExpirationsResponse = serde_json::from_str(data).unwrap();
        assert_eq!(resp.id, 42);
        assert_eq!(resp.jsonrpc, "2.0");
        assert_eq!(resp.result.expirations.len(), 3);
        assert_eq!(resp.result.expirations[0], 1680307200000);
    }
}
