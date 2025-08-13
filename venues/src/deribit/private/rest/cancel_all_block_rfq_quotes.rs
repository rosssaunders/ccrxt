use serde::Serialize;

use super::RestClient;
use crate::deribit::{EndpointType, JsonRpcResult, RestResult};

/// REST API endpoint constant
const CANCEL_ALL_BLOCK_RFQ_QUOTES_ENDPOINT: &str = "private/cancel_all_block_rfq_quotes";

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
pub type CancelAllBlockRfqQuotesResponse = JsonRpcResult<i64>;

impl RestClient {
    /// Cancel all user quotes in all Block RFQs
    ///
    /// This method cancels all user quotes in all Block RFQs. Optionally cancels all
    /// quotes in a specific RFQ if the `block_rfq_id` is provided.
    ///
    /// This endpoint requires block_rfq:read_write scope.
    ///
    /// [docs](https://docs.deribit.com/v2/#private-cancel_all_block_rfq_quotes)
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
        request: CancelAllBlockRfqQuotesRequest,
    ) -> RestResult<CancelAllBlockRfqQuotesResponse> {
        self.send_signed_request(
            CANCEL_ALL_BLOCK_RFQ_QUOTES_ENDPOINT,
            &request,
            EndpointType::MatchingEngine,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{Value, json};
    use std::sync::Arc;

    use super::*;
    use crate::deribit::AccountTier;
    use crate::deribit::private::rest::credentials::Credentials;
    use rest::secrets::SecretString;

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
        let credentials = Credentials {
            api_key: SecretString::from("test_key".to_string()),
            api_secret: SecretString::from("test_secret".to_string()),
        };
        let http_client = Arc::new(rest::native::NativeHttpClient::default());
        let rate_limiter = crate::deribit::RateLimiter::new(AccountTier::Tier4);

        let rest_client = RestClient::new(
            credentials,
            "https://test.deribit.com",
            rate_limiter,
            http_client,
        );

        // Test that we can get a function reference to the method
        let _ = RestClient::cancel_all_block_rfq_quotes;

        // Verify the client exists
        let _ = &rest_client;

        println!("cancel_all_block_rfq_quotes method is accessible and properly typed");
    }
}
