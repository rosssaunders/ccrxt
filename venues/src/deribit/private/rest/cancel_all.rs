use super::client::RestClient;
use crate::deribit::{EndpointType, RestResult};
use serde::{Deserialize, Serialize};

/// Request parameters for cancel all endpoint
#[derive(Debug, Clone, Serialize)]
pub struct CancelAllRequest {
    /// When detailed is set to true output format is changed (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detailed: Option<bool>,
    /// Whether or not to reject incoming quotes for 1 second after cancelling (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub freeze_quotes: Option<bool>,
}

/// Response for cancel all endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelAllResponse {
    /// The id that was sent in the request
    pub id: i64,
    /// The JSON-RPC version (2.0)
    pub jsonrpc: String,
    /// Total number of successfully cancelled orders
    pub result: i64,
}

impl RestClient {
    /// Cancel all orders and trigger orders within all currencies
    ///
    /// This method cancels all users orders and trigger orders within all currencies
    /// and instrument kinds. This endpoint requires trade:read_write scope.
    ///
    /// See: <https://docs.deribit.com/v2/#private-cancel_all>
    ///
    /// Rate limit: Matching engine endpoint (tier-based rate limiting)
    /// Scope: trade:read_write
    ///
    /// # Arguments
    /// * `detailed` - Optional flag for detailed output format
    /// * `freeze_quotes` - Optional flag to reject incoming quotes for 1 second after cancelling
    ///
    /// # Returns
    /// Result with total number of successfully cancelled orders
    pub async fn cancel_all(
        &self,
        detailed: Option<bool>,
        freeze_quotes: Option<bool>,
    ) -> RestResult<CancelAllResponse> {
        let request = CancelAllRequest {
            detailed,
            freeze_quotes,
        };
        self.send_signed_request("private/cancel_all", &request, EndpointType::MatchingEngine)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::deribit::AccountTier;
    use rest::secrets::ExposableSecret;
    use serde_json::{json, Value};

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
        let request = CancelAllRequest {
            detailed: None,
            freeze_quotes: None,
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        // Should not contain optional fields when None
        assert!(json_value.get("detailed").is_none());
        assert!(json_value.get("freeze_quotes").is_none());
    }

    #[test]
    fn test_request_parameters_serialization_detailed() {
        let request = CancelAllRequest {
            detailed: Some(true),
            freeze_quotes: None,
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("detailed").unwrap(), true);
        assert!(json_value.get("freeze_quotes").is_none());
    }

    #[test]
    fn test_request_parameters_serialization_freeze_quotes() {
        let request = CancelAllRequest {
            detailed: None,
            freeze_quotes: Some(true),
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert!(json_value.get("detailed").is_none());
        assert_eq!(json_value.get("freeze_quotes").unwrap(), true);
    }

    #[test]
    fn test_request_parameters_serialization_full() {
        let request = CancelAllRequest {
            detailed: Some(false),
            freeze_quotes: Some(true),
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("detailed").unwrap(), false);
        assert_eq!(json_value.get("freeze_quotes").unwrap(), true);
    }

    #[test]
    fn test_response_structures_deserialization() {
        let response_json = json!({
            "id": 1,
            "jsonrpc": "2.0",
            "result": 5
        });

        let response: CancelAllResponse = serde_json::from_value(response_json).unwrap();
        
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

        let response: CancelAllResponse = serde_json::from_value(response_json).unwrap();
        
        assert_eq!(response.id, 42);
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.result, 0);
    }

    #[tokio::test]
    async fn test_cancel_all_method_exists() {
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
        let _ = RestClient::cancel_all;
        
        // Verify the client exists
        let _ = &rest_client;
        
        println!("cancel_all method is accessible and properly typed");
    }
}