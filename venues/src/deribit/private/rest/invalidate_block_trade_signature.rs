use serde::Serialize;

use super::RestClient;
use crate::deribit::{EndpointType, JsonRpcResult, RestResult};

/// REST API endpoint constant
const INVALIDATE_BLOCK_TRADE_SIGNATURE_ENDPOINT: &str = "private/invalidate_block_trade_signature";

/// Request parameters for invalidate block trade signature
#[derive(Debug, Clone, Serialize)]
pub struct InvalidateBlockTradeSignatureRequest {
    /// Signature of block trade that will be invalidated
    pub signature: String,
}

/// Response for invalidate block trade signature endpoint
pub type InvalidateBlockTradeSignatureResponse = JsonRpcResult<String>;

impl RestClient {
    /// Invalidate block trade signature
    ///
    /// User at any time (before the private/execute_block_trade is called) can
    /// invalidate its own signature effectively cancelling block trade.
    ///
    /// [docs](https://docs.deribit.com/v2/#private-invalidate_block_trade_signature)
    ///
    /// Rate limit: 500 credits per request (non-matching engine)
    /// Scope: block_trade:read_write
    ///
    /// # Arguments
    /// * `signature` - Signature of block trade that will be invalidated
    ///
    /// # Returns
    /// Result with "ok" string in case of success
    pub async fn invalidate_block_trade_signature(
        &self,
        request: InvalidateBlockTradeSignatureRequest,
    ) -> RestResult<InvalidateBlockTradeSignatureResponse> {
        self.send_signed_request(
            INVALIDATE_BLOCK_TRADE_SIGNATURE_ENDPOINT,
            &request,
            EndpointType::MatchingEngine,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use rest::secrets::ExposableSecret;
    /// REST API endpoint constant
    use serde_json::{Value, json};

    use super::*;
    use crate::deribit::AccountTier;

    // Test secret implementation
    #[derive(Clone)]
    struct PlainTextSecret {
        secret: String,
    }

    impl PlainTextSecret {
        fn new(secret: String) -> Self {
            Self { secret }
        }
    }

    impl ExposableSecret for PlainTextSecret {
        fn expose_secret(&self) -> String {
            self.secret.clone()
        }
    }

    #[test]
    fn test_request_parameters_serialization() {
        let request = InvalidateBlockTradeSignatureRequest {
            signature: "test_signature_123".to_string(),
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("signature").unwrap(), "test_signature_123");
    }

    #[test]
    fn test_request_parameters_serialization_long_signature() {
        let request = InvalidateBlockTradeSignatureRequest {
            signature: "very_long_signature_string_that_might_be_used_in_production_scenarios_456"
                .to_string(),
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(
            json_value.get("signature").unwrap(),
            "very_long_signature_string_that_might_be_used_in_production_scenarios_456"
        );
    }

    #[test]
    fn test_response_structures_deserialization() {
        let response_json = json!({
            "id": 1,
            "jsonrpc": "2.0",
            "result": "ok"
        });

        let response: InvalidateBlockTradeSignatureResponse =
            serde_json::from_value(response_json).unwrap();

        assert_eq!(response.id, 1);
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.result, "ok");
    }

    #[tokio::test]
    async fn test_invalidate_block_trade_signature_method_exists() {
        // Test that the method exists and compiles without needing to call it
        let api_key =
            Box::new(PlainTextSecret::new("test_key".to_string())) as Box<dyn ExposableSecret>;
        let api_secret =
            Box::new(PlainTextSecret::new("test_secret".to_string())) as Box<dyn ExposableSecret>;
        let http_client = Arc::new(rest::native::NativeHttpClient::default());
        let rate_limiter = crate::deribit::RateLimiter::new(AccountTier::Tier4);

        let rest_client = RestClient::new(
            api_key,
            api_secret,
            "https://test.deribit.com",
            rate_limiter,
            http_client,
        );

        // Test that we can get a function reference to the method
        let _ = RestClient::invalidate_block_trade_signature;

        // Verify the client exists
        let _ = &rest_client;

        println!("invalidate_block_trade_signature method is accessible and properly typed");
    }
}
