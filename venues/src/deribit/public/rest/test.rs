//! Request and response structs for public/test endpoint
//!
//! Tests the connection to the API server, and returns its version. You can use
//! this to make sure the API is reachable, and matches the expected version.

use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::deribit::{EndpointType, RestResult};

/// Request parameters for the public/test endpoint.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TestRequest {
    /// The value "exception" will trigger an error response. This may be useful for testing wrapper libraries.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_result: Option<String>,
}



/// Response for public/test endpoint following Deribit JSON-RPC 2.0 format.
#[derive(Debug, Clone, Deserialize)]
pub struct TestResponse {
    /// The id that was sent in the request
    pub id: i64,

    /// The JSON-RPC version (2.0)
    pub jsonrpc: String,

    /// Result object containing API version information
    pub result: TestResult,
}

/// Result object for the public/test endpoint.
#[derive(Debug, Clone, Deserialize)]
pub struct TestResult {
    /// The API version
    pub version: String,
}

impl RestClient {
    /// Calls the public/test endpoint.
    ///
    /// Tests the connection to the API server, and returns its version. You can use
    /// this to make sure the API is reachable, and matches the expected version.
    ///
    /// # Arguments
    /// * `params` - The request parameters
    ///
    /// # Returns
    /// A result containing the response with API version information or an error
    ///
    /// [Official API docs](https://docs.deribit.com/#public-test)
    pub async fn test(&self, params: TestRequest) -> RestResult<TestResponse> {
        self.send_request(
            "public/test",
            reqwest::Method::GET,
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
    fn test_test_request_serialization() {
        let request = TestRequest {
            expected_result: None,
        };

        let json_value = serde_json::to_value(&request).unwrap();
        // Should serialize to an empty object when no expected_result is set
        assert!(json_value.is_object());
        assert!(json_value.as_object().unwrap().is_empty());
    }

    #[test]
    fn test_test_request_with_exception() {
        let request = TestRequest {
            expected_result: Some("exception".to_string()),
        };

        let json_value = serde_json::to_value(&request).unwrap();
        assert_eq!(json_value["expected_result"], "exception");
    }

    #[test]
    fn test_test_request_default() {
        let request = TestRequest::default();
        assert!(request.expected_result.is_none());

        let json_value = serde_json::to_value(&request).unwrap();
        assert!(json_value.as_object().unwrap().is_empty());
    }

    #[test]
    fn test_test_response_structure() {
        let response_json = json!({
            "id": 123,
            "jsonrpc": "2.0",
            "result": {
                "version": "2.1.1"
            }
        });

        let response: TestResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.id, 123);
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.result.version, "2.1.1");
    }

    #[test]
    fn test_test_response_different_versions() {
        let test_versions = vec!["1.0.0", "2.0.0", "2.1.1", "3.0.0-beta", "2.1.1-rc1"];

        for version in test_versions {
            let response_json = json!({
                "id": 456,
                "jsonrpc": "2.0",
                "result": {
                    "version": version
                }
            });

            let response: TestResponse = serde_json::from_value(response_json).unwrap();
            assert_eq!(response.id, 456);
            assert_eq!(response.jsonrpc, "2.0");
            assert_eq!(response.result.version, version);
        }
    }

    #[test]
    fn test_response_deserialization_edge_cases() {
        // Test with empty version string
        let response_json = json!({
            "id": 1,
            "jsonrpc": "2.0",
            "result": {
                "version": ""
            }
        });

        let response: TestResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.result.version, "");

        // Test with very long version string
        let long_version = "v2.1.1-very-long-version-string-with-lots-of-details-and-metadata";
        let response_json = json!({
            "id": 2,
            "jsonrpc": "2.0",
            "result": {
                "version": long_version
            }
        });

        let response: TestResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.result.version, long_version);
    }

    #[tokio::test]
    async fn test_endpoint_type_usage() {
        let client = reqwest::Client::new();
        let rate_limiter = RateLimiter::new(AccountTier::Tier4);

        let rest_client = RestClient::new("https://test.deribit.com", client, rate_limiter);

        // Test that we can create requests - this doesn't actually call the API
        let _normal_request = TestRequest {
            expected_result: None,
        };
        let _exception_request = TestRequest {
            expected_result: Some("exception".to_string()),
        };

        // Test that rate limiting works for this endpoint type
        let result = rest_client
            .rate_limiter
            .check_limits(EndpointType::NonMatchingEngine)
            .await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_json_rpc_compliance() {
        // Test that the response follows JSON-RPC 2.0 specification
        let response_json = json!({
            "id": 789,
            "jsonrpc": "2.0",
            "result": {
                "version": "2.1.1"
            }
        });

        let response: TestResponse = serde_json::from_value(response_json).unwrap();

        // Verify JSON-RPC 2.0 compliance
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.id, 789);

        // Verify result is present and correct structure
        assert!(!response.result.version.is_empty());
    }

    #[test]
    fn test_request_serialization_with_none_expected_result() {
        let request = TestRequest {
            expected_result: None,
        };

        let json_value = serde_json::to_value(&request).unwrap();
        assert!(json_value.as_object().unwrap().is_empty());
    }

    #[test]
    fn test_request_serialization_with_some_expected_result() {
        let request = TestRequest {
            expected_result: Some("exception".to_string()),
        };

        let json_value = serde_json::to_value(&request).unwrap();
        assert_eq!(json_value["expected_result"], "exception");
    }

    #[test]
    fn test_constructor_methods() {
        // Test direct struct construction
        let normal_request = TestRequest {
            expected_result: None,
        };
        assert!(normal_request.expected_result.is_none());

        // Test direct struct construction with exception
        let exception_request = TestRequest {
            expected_result: Some("exception".to_string()),
        };
        assert_eq!(
            exception_request.expected_result,
            Some("exception".to_string())
        );
    }
}
