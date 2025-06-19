use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::deribit::{EndpointType, RestResult};

/// Request parameters for get Block RFQ makers
#[derive(Debug, Clone, Serialize)]
pub struct GetBlockRfqMakersRequest {
    // This endpoint takes no parameters
}

/// Response for get Block RFQ makers endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetBlockRfqMakersResponse {
    /// The id that was sent in the request
    pub id: i64,
    /// The JSON-RPC version (2.0)
    pub jsonrpc: String,
    /// A list of available makers
    pub result: Vec<String>,
}

impl RestClient {
    /// Get a list of all available Block RFQ makers
    ///
    /// This method returns a list of all available Block RFQ makers.
    /// This endpoint requires block_rfq:read scope.
    ///
    /// See: <https://docs.deribit.com/v2/#private-get_block_rfq_makers>
    ///
    /// Rate limit: Depends on endpoint type (matching engine)
    /// Scope: block_rfq:read
    ///
    /// # Returns
    /// Result containing a list of available Block RFQ makers
    pub async fn get_block_rfq_makers(&self) -> RestResult<GetBlockRfqMakersResponse> {
        let request = GetBlockRfqMakersRequest {};
        self.send_signed_request(
            "private/get_block_rfq_makers",
            &request,
            EndpointType::MatchingEngine,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use rest::secrets::ExposableSecret;
    use serde_json::{Value, json};

    use super::*;
    use crate::deribit::AccountTier;

    // Test secret implementation
    #[derive(Clone)]
    struct PlainTextSecret {
        secret: String,
    }

    impl PlainTextSecret {
        fn new(secret: impl Into<String>) -> Self {
            Self {
                secret: secret.into(),
            }
        }
    }

    impl ExposableSecret for PlainTextSecret {
        fn expose_secret(&self) -> String {
            self.secret.clone()
        }
    }

    #[test]
    fn test_request_serialization() {
        let request = GetBlockRfqMakersRequest {};
        
        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();
        
        // Should serialize to an empty object since no parameters
        assert_eq!(json_value, json!({}));
    }

    #[test]
    fn test_response_structure() {
        let response_json = json!({
            "id": 123,
            "jsonrpc": "2.0",
            "result": ["maker1", "maker2", "maker3"]
        });

        let response: GetBlockRfqMakersResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.id, 123);
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.result.len(), 3);
        assert_eq!(response.result[0], "maker1");
        assert_eq!(response.result[1], "maker2");
        assert_eq!(response.result[2], "maker3");
    }

    #[test]
    fn test_response_empty_makers() {
        let response_json = json!({
            "id": 456,
            "jsonrpc": "2.0",
            "result": []
        });

        let response: GetBlockRfqMakersResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.id, 456);
        assert_eq!(response.jsonrpc, "2.0");
        assert!(response.result.is_empty());
    }

    #[test]
    fn test_endpoint_method_signature() {
        // Test that we can create a mock client
        let rate_limiter = crate::deribit::RateLimiter::new(AccountTier::Tier1);
        let client = RestClient::new(
            Box::new(PlainTextSecret::new("test_key")),
            Box::new(PlainTextSecret::new("test_secret")),
            "https://test.deribit.com",
            rate_limiter,
            reqwest::Client::new(),
        );

        // Test method signature - this ensures the method compiles correctly
        // We can't actually call it without a real connection, but we can verify the signature
        let _future = client.get_block_rfq_makers();
    }

    #[test]
    fn test_json_rpc_compliance() {
        let response_json = json!({
            "id": 999,
            "jsonrpc": "2.0",
            "result": ["test_maker"]
        });

        let response: GetBlockRfqMakersResponse = serde_json::from_value(response_json).unwrap();
        
        // Verify JSON-RPC 2.0 compliance
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.id, 999);
        assert!(!response.result.is_empty());
    }

    #[test]
    fn test_response_deserialization_edge_cases() {
        // Test with various maker name formats
        let response_json = json!({
            "id": 1,
            "jsonrpc": "2.0",
            "result": [
                "simple_maker",
                "maker-with-dashes",
                "maker.with.dots",
                "UPPERCASE_MAKER",
                "123numeric_maker"
            ]
        });

        let response: GetBlockRfqMakersResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.result.len(), 5);
        assert_eq!(response.result[0], "simple_maker");
        assert_eq!(response.result[1], "maker-with-dashes");
        assert_eq!(response.result[2], "maker.with.dots");
        assert_eq!(response.result[3], "UPPERCASE_MAKER");
        assert_eq!(response.result[4], "123numeric_maker");
    }
}