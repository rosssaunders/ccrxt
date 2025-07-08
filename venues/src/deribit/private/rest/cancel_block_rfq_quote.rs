use serde::{Deserialize, Serialize};

// Reuse the result structure from add_block_rfq_quote since the API returns the same quote object
use super::RestClient;
use super::add_block_rfq_quote::AddBlockRfqQuoteResult;
use crate::deribit::{EndpointType, JsonRpcResult, RestResult};

/// REST API endpoint constant
const CANCEL_BLOCK_RFQ_QUOTE_ENDPOINT: &str = "private/cancel_block_rfq_quote";

/// Request parameters for cancel block RFQ quote endpoint
#[derive(Debug, Clone, Serialize)]
pub struct CancelBlockRfqQuoteRequest {
    /// ID of the Block RFQ quote (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_rfq_quote_id: Option<i64>,
    /// User defined label for the Block RFQ quote (maximum 64 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// ID of the Block RFQ (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_rfq_id: Option<i64>,
}

/// Response for cancel block RFQ quote endpoint
pub type CancelBlockRfqQuoteResponse = JsonRpcResult<AddBlockRfqQuoteResult>;

impl RestClient {
    /// Cancel a Block RFQ quote
    ///
    /// This method cancels a Block RFQ quote using the specified `block_rfq_quote_id`.
    /// Alternatively, user can use a combination of `block_rfq_id` and `label` to
    /// cancel the quote.
    ///
    /// This endpoint requires block_rfq:read_write scope.
    ///
    /// See: <https://docs.deribit.com/v2/#private-cancel_block_rfq_quote>
    ///
    /// Rate limit: Matching engine endpoint (tier-based rate limiting)
    /// Scope: block_rfq:read_write
    ///
    /// # Arguments
    /// * `block_rfq_quote_id` - Optional ID of the Block RFQ quote
    /// * `label` - Optional user defined label for the Block RFQ quote (maximum 64 characters)
    /// * `block_rfq_id` - Optional ID of the Block RFQ
    ///
    /// # Returns
    /// Result containing the cancelled Block RFQ quote details
    pub async fn cancel_block_rfq_quote(
        &self,
        request: CancelBlockRfqQuoteRequest,
    ) -> RestResult<CancelBlockRfqQuoteResponse> {
        self.send_signed_request(
            CANCEL_BLOCK_RFQ_QUOTE_ENDPOINT,
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
        let request = CancelBlockRfqQuoteRequest {
            block_rfq_quote_id: None,
            label: None,
            block_rfq_id: None,
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        // Should not contain optional fields when None
        assert!(json_value.get("block_rfq_quote_id").is_none());
        assert!(json_value.get("label").is_none());
        assert!(json_value.get("block_rfq_id").is_none());
    }

    #[test]
    fn test_request_parameters_serialization_with_quote_id() {
        let request = CancelBlockRfqQuoteRequest {
            block_rfq_quote_id: Some(12345),
            label: None,
            block_rfq_id: None,
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("block_rfq_quote_id").unwrap(), 12345);
        assert!(json_value.get("label").is_none());
        assert!(json_value.get("block_rfq_id").is_none());
    }

    #[test]
    fn test_request_parameters_serialization_with_label_and_rfq_id() {
        let request = CancelBlockRfqQuoteRequest {
            block_rfq_quote_id: None,
            label: Some("my_quote_label".to_string()),
            block_rfq_id: Some(67890),
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert!(json_value.get("block_rfq_quote_id").is_none());
        assert_eq!(json_value.get("label").unwrap(), "my_quote_label");
        assert_eq!(json_value.get("block_rfq_id").unwrap(), 67890);
    }

    #[test]
    fn test_request_parameters_serialization_full() {
        let request = CancelBlockRfqQuoteRequest {
            block_rfq_quote_id: Some(11111),
            label: Some("test_label".to_string()),
            block_rfq_id: Some(22222),
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("block_rfq_quote_id").unwrap(), 11111);
        assert_eq!(json_value.get("label").unwrap(), "test_label");
        assert_eq!(json_value.get("block_rfq_id").unwrap(), 22222);
    }

    #[test]
    fn test_response_structures_deserialization() {
        let response_json = json!({
            "id": 1,
            "jsonrpc": "2.0",
            "result": {
                "amount": 1.5,
                "app_name": "test_app",
                "block_rfq_id": 123,
                "block_rfq_quote_id": 456,
                "creation_timestamp": 1640995200000i64,
                "direction": "buy",
                "execution_instruction": "any_part_of",
                "filled_amount": 0.0,
                "hedge": {
                    "amount": 100,
                    "direction": "sell",
                    "instrument_name": "BTC-PERPETUAL",
                    "price": 50000.0
                },
                "label": "my_cancelled_quote",
                "last_update_timestamp": 1640995300000i64,
                "legs": [
                    {
                        "direction": "buy",
                        "instrument_name": "ETH-PERPETUAL",
                        "price": 4000.0,
                        "ratio": 1
                    }
                ],
                "price": 4000.0,
                "quote_state": "cancelled",
                "quote_state_reason": "user_cancel",
                "replaced": false
            }
        });

        let response: CancelBlockRfqQuoteResponse = serde_json::from_value(response_json).unwrap();

        assert_eq!(response.id, 1);
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.result.amount, 1.5);
        assert_eq!(response.result.block_rfq_id, 123);
        assert_eq!(response.result.block_rfq_quote_id, 456);
        assert_eq!(response.result.quote_state, "cancelled");
        assert_eq!(
            response.result.quote_state_reason,
            Some("user_cancel".to_string())
        );
        assert!(!response.result.replaced);
    }

    #[tokio::test]
    async fn test_cancel_block_rfq_quote_method_exists() {
        // Test that the method exists and compiles without needing to call it
        let api_key =
            Box::new(PlainTextSecret::new("test_key".to_string())) as Box<dyn ExposableSecret>;
        let api_secret =
            Box::new(PlainTextSecret::new("test_secret".to_string())) as Box<dyn ExposableSecret>;
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
        let _ = RestClient::cancel_block_rfq_quote;

        // Verify the client exists
        let _ = &rest_client;

        println!("cancel_block_rfq_quote method is accessible and properly typed");
    }
}
