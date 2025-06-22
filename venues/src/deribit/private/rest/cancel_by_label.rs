use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::deribit::{Currency, EndpointType, RestResult};

/// Request parameters for cancel by label endpoint
#[derive(Debug, Clone, Serialize)]
pub struct CancelByLabelRequest {
    /// User defined label for the order (maximum 64 characters) (required)
    pub label: String,
    /// The currency symbol (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
}

/// Response for cancel by label endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelByLabelResponse {
    /// The id that was sent in the request
    pub id: i64,
    /// The JSON-RPC version (2.0)
    pub jsonrpc: String,
    /// Total number of successfully cancelled orders
    pub result: i64,
}

impl RestClient {
    /// Cancel orders by label
    ///
    /// Cancels orders by label. All user's orders (trigger orders too), with a given
    /// label are cancelled in all currencies or in one given currency (in this case
    /// currency queue is used). This endpoint requires trade:read_write scope.
    ///
    /// See: <https://docs.deribit.com/v2/#private-cancel_by_label>
    ///
    /// Rate limit: Matching engine endpoint (tier-based rate limiting)
    /// Scope: trade:read_write
    ///
    /// # Arguments
    /// * `label` - User defined label for the order (maximum 64 characters)
    /// * `currency` - Optional currency symbol to limit cancellation to specific currency
    ///
    /// # Returns
    /// Result with total number of successfully cancelled orders
    pub async fn cancel_by_label(
        &self,
        request: CancelByLabelRequest,
    ) -> RestResult<CancelByLabelResponse> {
        self.send_signed_request(
            "private/cancel_by_label",
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
        let request = CancelByLabelRequest {
            label: "my_label_123".to_string(),
            currency: None,
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        // Should contain required label field
        assert_eq!(json_value.get("label").unwrap(), "my_label_123");
        // Should not contain optional currency field when None
        assert!(json_value.get("currency").is_none());
    }

    #[test]
    fn test_request_parameters_serialization_with_currency() {
        let request = CancelByLabelRequest {
            label: "test_label".to_string(),
            currency: Some(Currency::BTC),
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("label").unwrap(), "test_label");
        assert_eq!(json_value.get("currency").unwrap(), "BTC");
    }

    #[test]
    fn test_request_parameters_serialization_all_currencies() {
        // Test different currency values
        let currencies = vec![
            Currency::BTC,
            Currency::ETH,
            Currency::USDC,
            Currency::USDT,
            Currency::EURR,
        ];

        for currency in currencies {
            let request = CancelByLabelRequest {
                label: "label_test".to_string(),
                currency: Some(currency),
            };

            let json_str = serde_json::to_string(&request).unwrap();
            let json_value: Value = serde_json::from_str(&json_str).unwrap();

            assert_eq!(json_value.get("label").unwrap(), "label_test");
            assert!(json_value.get("currency").is_some());
        }
    }

    #[test]
    fn test_request_parameters_serialization_long_label() {
        // Test with maximum allowed label length (64 characters)
        let long_label = "a".repeat(64);
        let request = CancelByLabelRequest {
            label: long_label.clone(),
            currency: Some(Currency::ETH),
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("label").unwrap(), &long_label);
        assert_eq!(json_value.get("currency").unwrap(), "ETH");
    }

    #[test]
    fn test_response_structures_deserialization() {
        let response_json = json!({
            "id": 1,
            "jsonrpc": "2.0",
            "result": 5
        });

        let response: CancelByLabelResponse = serde_json::from_value(response_json).unwrap();

        assert_eq!(response.id, 1);
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.result, 5);
    }

    #[test]
    fn test_response_structures_deserialization_zero_cancelled() {
        let response_json = json!({
            "id": 42,
            "jsonrpc": "2.0",
            "result": 0
        });

        let response: CancelByLabelResponse = serde_json::from_value(response_json).unwrap();

        assert_eq!(response.id, 42);
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.result, 0);
    }

    #[test]
    fn test_response_structures_deserialization_large_result() {
        let response_json = json!({
            "id": 123,
            "jsonrpc": "2.0",
            "result": 999
        });

        let response: CancelByLabelResponse = serde_json::from_value(response_json).unwrap();

        assert_eq!(response.id, 123);
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.result, 999);
    }

    #[tokio::test]
    async fn test_cancel_by_label_method_exists() {
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
        let _ = RestClient::cancel_by_label;

        // Verify the client exists
        let _ = &rest_client;

        println!("cancel_by_label method is accessible and properly typed");
    }
}
