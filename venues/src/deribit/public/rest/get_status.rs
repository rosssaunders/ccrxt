//! Request and response structs for public/status endpoint
//!
//! Method used to get information about locked currencies

use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::deribit::{EndpointType, RestResult};

/// Request parameters for the public/status endpoint.
///
/// This method takes no parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetStatusRequest {}

/// Response for public/status endpoint following Deribit JSON-RPC 2.0 format.
#[derive(Debug, Clone, Deserialize)]
pub struct GetStatusResponse {
    /// The id that was sent in the request
    pub id: i64,

    /// The JSON-RPC version (2.0)
    pub jsonrpc: String,

    /// Result object containing status information
    pub result: GetStatusResult,
}

/// Result object for the public/status endpoint.
#[derive(Debug, Clone, Deserialize)]
pub struct GetStatusResult {
    /// Platform lock status:
    /// - "true" when platform is locked in all currencies
    /// - "partial" when some currencies are locked
    /// - "false" when there are no currencies locked
    pub locked: String,

    /// List of currency indices locked platform-wise
    pub locked_indices: Vec<String>,
}

impl RestClient {
    /// Calls the public/status endpoint.
    ///
    /// Method used to get information about locked currencies.
    ///
    /// # Arguments
    /// * `params` - The request parameters (empty for this endpoint)
    ///
    /// # Returns
    /// A result containing the response with status information or an error
    ///
    /// [Official API docs](https://docs.deribit.com/#public-status)
    pub async fn get_status(&self, params: GetStatusRequest) -> RestResult<GetStatusResponse> {
        self.send_request(
            "public/status",
            reqwest::Method::GET,
            Some(&params),
            EndpointType::PublicGetStatus,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::deribit::{AccountTier, RateLimiter};

    #[test]
    fn test_get_status_request_serialization() {
        let request = GetStatusRequest {};

        let json_value = serde_json::to_value(&request).unwrap();
        // Should serialize to an empty object
        assert!(json_value.is_object());
        assert!(json_value.as_object().unwrap().is_empty());
    }

    #[test]
    fn test_get_status_response_structure() {
        let response_json = json!({
            "id": 123,
            "jsonrpc": "2.0",
            "result": {
                "locked": "false",
                "locked_indices": []
            }
        });

        let response: GetStatusResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.id, 123);
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.result.locked, "false");
        assert!(response.result.locked_indices.is_empty());
    }

    #[test]
    fn test_get_status_response_with_locked_currencies() {
        let response_json = json!({
            "id": 456,
            "jsonrpc": "2.0",
            "result": {
                "locked": "partial",
                "locked_indices": ["BTC", "ETH"]
            }
        });

        let response: GetStatusResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.id, 456);
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.result.locked, "partial");
        assert_eq!(response.result.locked_indices.len(), 2);
        assert_eq!(response.result.locked_indices[0], "BTC");
        assert_eq!(response.result.locked_indices[1], "ETH");
    }

    #[test]
    fn test_get_status_response_all_locked() {
        let response_json = json!({
            "id": 789,
            "jsonrpc": "2.0",
            "result": {
                "locked": "true",
                "locked_indices": ["BTC", "ETH", "SOL", "USDC"]
            }
        });

        let response: GetStatusResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.id, 789);
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.result.locked, "true");
        assert_eq!(response.result.locked_indices.len(), 4);
        assert!(response.result.locked_indices.contains(&"BTC".to_string()));
        assert!(response.result.locked_indices.contains(&"ETH".to_string()));
        assert!(response.result.locked_indices.contains(&"SOL".to_string()));
        assert!(response.result.locked_indices.contains(&"USDC".to_string()));
    }

    #[test]
    fn test_locked_status_values() {
        // Test all possible values for the locked field
        let locked_values = vec!["true", "false", "partial"];

        for locked_value in locked_values {
            let response_json = json!({
                "id": 1,
                "jsonrpc": "2.0",
                "result": {
                    "locked": locked_value,
                    "locked_indices": []
                }
            });

            let response: GetStatusResponse = serde_json::from_value(response_json).unwrap();
            assert_eq!(response.result.locked, locked_value);
        }
    }

    #[tokio::test]
    async fn test_endpoint_type_usage() {
        let client = reqwest::Client::new();
        let rate_limiter = RateLimiter::new(AccountTier::Tier4);

        let rest_client = RestClient::new("https://test.deribit.com", client, rate_limiter);

        // Test that we can create a request - this doesn't actually call the API
        let _request = GetStatusRequest {};

        // Test that rate limiting works for this endpoint type
        let result = rest_client
            .rate_limiter
            .check_limits(EndpointType::PublicGetStatus)
            .await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_response_deserialization_edge_cases() {
        // Test with empty locked_indices
        let response_json = json!({
            "id": 1,
            "jsonrpc": "2.0",
            "result": {
                "locked": "false",
                "locked_indices": []
            }
        });

        let response: GetStatusResponse = serde_json::from_value(response_json).unwrap();
        assert!(response.result.locked_indices.is_empty());

        // Test with single locked index
        let response_json = json!({
            "id": 2,
            "jsonrpc": "2.0",
            "result": {
                "locked": "partial",
                "locked_indices": ["BTC"]
            }
        });

        let response: GetStatusResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.result.locked_indices.len(), 1);
        assert_eq!(response.result.locked_indices[0], "BTC");
    }
}