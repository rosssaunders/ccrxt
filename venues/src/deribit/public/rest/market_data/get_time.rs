use crate::deribit::{EndpointType, JsonRpcResult, PublicRestClient, RestResult};

// Endpoint constant
const GET_TIME_ENDPOINT: &str = "public/get_time";

/// Request parameters for the public/get_time endpoint.
///
/// This endpoint does not require any parameters. The struct exists to satisfy the
/// project rule that all endpoint methods take a single parameter struct.
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct GetTimeRequest {}

/// Response payload for public/get_time containing the current server time in milliseconds.
pub type GetTimeResponse = JsonRpcResult<i64>;

impl PublicRestClient {
    /// public/get_time
    ///
    /// Returns the current Deribit server time in milliseconds since Unix epoch. Can be
    /// used to measure clock skew between client and server.
    ///
    /// [docs](https://docs.deribit.com/#public-get_time)
    ///
    /// Rate limit: non-matching engine (500 credits)
    ///
    /// # Arguments
    /// * `request` - Empty request struct (no parameters required)
    ///
    /// # Returns
    /// Server time wrapped in JSON-RPC response container.
    pub async fn get_time(&self, _request: GetTimeRequest) -> RestResult<GetTimeResponse> {
        self.send_post_request::<GetTimeResponse, _>(
            GET_TIME_ENDPOINT,
            Some(&()), // Empty params map
            EndpointType::NonMatchingEngine,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use serde_json::json;

    use super::*;
    use crate::deribit::{AccountTier, RateLimiter};

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
        let http_client = Arc::new(rest::native::NativeHttpClient::default());
        let rate_limiter = RateLimiter::new(AccountTier::Tier4);

        let rest_client =
            PublicRestClient::new("https://test.deribit.com", http_client, rate_limiter);

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
