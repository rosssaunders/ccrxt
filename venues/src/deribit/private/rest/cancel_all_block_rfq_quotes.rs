use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::deribit::{EndpointType, RestResult};

/// Request parameters for cancel all block RFQ quotes endpoint
#[derive(Debug, Clone, Serialize)]
pub struct CancelAllBlockRfqQuotesRequest {
    /// ID of the Block RFQ (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_rfq_id: Option<i64>,
    /// When detailed is set to true output format is changed (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detailed: Option<bool>,
}

/// Response for cancel all block RFQ quotes endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelAllBlockRfqQuotesResponse {
    /// The id that was sent in the request
    pub id: i64,
    /// The JSON-RPC version (2.0)
    pub jsonrpc: String,
    /// Total number of successfully cancelled quotes
    pub result: i64,
}

impl RestClient {
    /// Cancel all user quotes in all Block RFQs
    ///
    /// This method cancels all user quotes in all Block RFQs. Optionally cancels all
    /// quotes in a specific RFQ if the `block_rfq_id` is provided.
    ///
    /// This endpoint requires block_rfq:read_write scope.
    ///
    /// See: <https://docs.deribit.com/v2/#private-cancel_all_block_rfq_quotes>
    ///
    /// Rate limit: Matching engine endpoint (tier-based rate limiting)
    /// Scope: block_rfq:read_write
    ///
    /// # Arguments
    /// * `block_rfq_id` - Optional ID of the Block RFQ
    /// * `detailed` - Optional flag for detailed output format
    ///
    /// # Returns
    /// Result with total number of successfully cancelled quotes
    pub async fn cancel_all_block_rfq_quotes(
        &self,
        block_rfq_id: Option<i64>,
        detailed: Option<bool>,
    ) -> RestResult<CancelAllBlockRfqQuotesResponse> {
        let request = CancelAllBlockRfqQuotesRequest {
            block_rfq_id,
            detailed,
        };
        self.send_signed_request(
            "private/cancel_all_block_rfq_quotes",
            &request,
            EndpointType::MatchingEngine,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use rest::secrets::ExposableSecret;
    use serde_json::{json, Value};

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
    fn test_request_parameters_serialization_minimal() {
        let request = CancelAllBlockRfqQuotesRequest {
            block_rfq_id: None,
            detailed: None,
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        // Should not contain optional fields when None
        assert!(json_value.get("block_rfq_id").is_none());
        assert!(json_value.get("detailed").is_none());
    }

    #[test]
    fn test_request_parameters_serialization_with_block_rfq_id() {
        let request = CancelAllBlockRfqQuotesRequest {
            block_rfq_id: Some(123),
            detailed: None,
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("block_rfq_id").unwrap(), 123);
        assert!(json_value.get("detailed").is_none());
    }

    #[test]
    fn test_request_parameters_serialization_with_detailed() {
        let request = CancelAllBlockRfqQuotesRequest {
            block_rfq_id: None,
            detailed: Some(true),
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert!(json_value.get("block_rfq_id").is_none());
        assert_eq!(json_value.get("detailed").unwrap(), true);
    }

    #[test]
    fn test_request_parameters_serialization_full() {
        let request = CancelAllBlockRfqQuotesRequest {
            block_rfq_id: Some(456),
            detailed: Some(false),
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("block_rfq_id").unwrap(), 456);
        assert_eq!(json_value.get("detailed").unwrap(), false);
    }

    #[test]
    fn test_response_structures_deserialization() {
        let response_json = json!({
            "id": 1,
            "jsonrpc": "2.0",
            "result": 3
        });

        let response: CancelAllBlockRfqQuotesResponse =
            serde_json::from_value(response_json).unwrap();

        assert_eq!(response.id, 1);
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.result, 3);
    }

    #[test]
    fn test_response_structures_deserialization_zero_cancelled() {
        let response_json = json!({
            "id": 42,
            "jsonrpc": "2.0",
            "result": 0
        });

        let response: CancelAllBlockRfqQuotesResponse =
            serde_json::from_value(response_json).unwrap();

        assert_eq!(response.id, 42);
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.result, 0);
    }

    #[tokio::test]
    async fn test_cancel_all_block_rfq_quotes_method_exists() {
        // Test that the method exists and compiles without needing to call it
        let api_key = Box::new(PlainTextSecret::new("test_key".to_string())) as Box<dyn ExposableSecret>;
        let api_secret = Box::new(PlainTextSecret::new("test_secret".to_string())) as Box<dyn ExposableSecret>;
        let client = reqwest::Client::new();
        let rate_limiter = crate::deribit::RateLimiter::new(AccountTier::Tier4);

        let rest_client = RestClient::new(
            api_key,
            api_secret,
            "https://test.deribit.com",
            rate_limiter,
            client,
        );

        // Test that we can get a function reference to the method
        let _ = RestClient::cancel_all_block_rfq_quotes;

        // Verify the client exists
        let _ = &rest_client;

        println!("cancel_all_block_rfq_quotes method is accessible and properly typed");
    }
}