//! Request and response structs for public/get_time endpoint
//!
//! Retrieves the current time (in milliseconds). This API endpoint can be used to
//! check the clock skew between your software and Deribit's systems.

use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::deribit::{EndpointType, JsonRpcResult, RestResult};

const TIME_ENDPOINT: &str = "public/get_time";

/// Request parameters for the public/get_time endpoint.
///
/// This method takes no parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTimeRequest {}

/// Response for public/get_time endpoint following Deribit JSON-RPC 2.0 format.
pub type GetTimeResponse = JsonRpcResult<i64>;

impl RestClient {
    /// Calls the public/get_time endpoint.
    ///
    /// Retrieves the current time (in milliseconds). This API endpoint can be used to
    /// check the clock skew between your software and Deribit's systems.
    ///
    /// # Arguments
    /// * `params` - The request parameters (empty for this endpoint)
    ///
    /// # Returns
    /// A result containing the response with the current timestamp or an error
    ///
    /// [Official API docs](https://docs.deribit.com/#public-get_time)
    pub async fn get_time(&self, params: GetTimeRequest) -> RestResult<GetTimeResponse> {
        self.send_request(
            TIME_ENDPOINT,
            Some(&params),
            EndpointType::NonMatchingEngine,
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
    fn test_get_time_request_serialization() {
        let request = GetTimeRequest {};

        let json_value =
            serde_json::to_value(&request).expect("Failed to convert request to value");
        // Should serialize to an empty object
        assert!(json_value.is_object());
        assert!(json_value.as_object().expect("Expected object").is_empty());
    }

    #[test]
    fn test_get_time_response_structure() {
        let response_json = json!({
            "id": 123,
            "jsonrpc": "2.0",
            "result": 1609459200000i64
        });

        let response: GetTimeResponse =
            serde_json::from_value(response_json).expect("Failed to deserialize response");
        assert_eq!(response.id, 123);
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.result, 1609459200000i64); // January 1, 2021 00:00:00 UTC
    }

    #[test]
    fn test_get_time_response_current_time() {
        // Test with a more recent timestamp
        let current_time = chrono::Utc::now().timestamp_millis();
        let response_json = json!({
            "id": 456,
            "jsonrpc": "2.0",
            "result": current_time
        });

        let response: GetTimeResponse =
            serde_json::from_value(response_json).expect("Failed to deserialize response");
        assert_eq!(response.id, 456);
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.result, current_time);
    }

    #[test]
    fn test_response_deserialization_edge_cases() {
        // Test with timestamp 0 (Unix epoch)
        let response_json = json!({
            "id": 1,
            "jsonrpc": "2.0",
            "result": 0
        });

        let response: GetTimeResponse =
            serde_json::from_value(response_json).expect("Failed to deserialize response");
        assert_eq!(response.result, 0);

        // Test with very large timestamp (year 2050+)
        let response_json = json!({
            "id": 2,
            "jsonrpc": "2.0",
            "result": 2524608000000i64
        });

        let response: GetTimeResponse =
            serde_json::from_value(response_json).expect("Failed to deserialize response");
        assert_eq!(response.result, 2524608000000i64);
    }

    #[tokio::test]
    async fn test_endpoint_type_usage() {
        let client = reqwest::Client::new();
        let rate_limiter = RateLimiter::new(AccountTier::Tier4);

        let rest_client = RestClient::new("https://test.deribit.com", client, rate_limiter);

        // Test that we can create a request - this doesn't actually call the API
        let _request = GetTimeRequest {};

        // Test that rate limiting works for this endpoint type
        let result = rest_client
            .rate_limiter
            .check_limits(EndpointType::NonMatchingEngine)
            .await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_timestamp_validation() {
        // Test that we can handle typical timestamp ranges
        let test_cases = vec![
            0i64,                                  // Unix epoch
            1000000000000i64,                      // September 9, 2001
            1609459200000i64,                      // January 1, 2021
            chrono::Utc::now().timestamp_millis(), // Current time
            4102444800000i64,                      // January 1, 2100
        ];

        for timestamp in test_cases {
            let response_json = json!({
                "id": 1,
                "jsonrpc": "2.0",
                "result": timestamp
            });

            let response: GetTimeResponse =
                serde_json::from_value(response_json).expect("Failed to deserialize response");
            assert_eq!(response.result, timestamp);
        }
    }

    #[test]
    fn test_json_rpc_compliance() {
        // Test that the response follows JSON-RPC 2.0 specification
        let response_json = json!({
            "id": 789,
            "jsonrpc": "2.0",
            "result": 1609459200000i64
        });

        let response: GetTimeResponse =
            serde_json::from_value(response_json).expect("Failed to deserialize response");

        // Verify JSON-RPC 2.0 compliance
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.id, 789);

        // Verify result is present and correct type
        assert!(response.result > 0);
    }
}
