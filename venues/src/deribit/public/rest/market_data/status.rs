use serde::Deserialize;

use crate::deribit::{
    EndpointType, JsonRpcResult, PublicRestClient, RestResult, enums::PlatformLockStatus,
};

const STATUS_ENDPOINT: &str = "public/status";

/// Result object for the public/status endpoint.
#[derive(Debug, Clone, Deserialize)]
pub struct GetStatusResult {
    /// Platform lock status:
    /// - `AllLocked`: Platform is locked in all currencies ("true")
    /// - `PartialLocked`: Some currencies are locked ("partial")
    /// - `Unlocked`: No currencies are locked ("false")
    pub locked: PlatformLockStatus,

    /// List of currency indices locked platform-wise
    pub locked_indices: Option<Vec<String>>,
}

/// Response for public/status endpoint following Deribit JSON-RPC 2.0 format.
pub type GetStatusResponse = JsonRpcResult<GetStatusResult>;

impl PublicRestClient {
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
    /// [docs](https://docs.deribit.com/#public-status)
    pub async fn get_status(&self) -> RestResult<GetStatusResponse> {
        self.send_post_request(
            STATUS_ENDPOINT,
            None::<&()>,
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
        assert_eq!(response.result.locked, PlatformLockStatus::Open);
        assert_eq!(response.result.locked_indices.as_deref(), Some(&[][..]));
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
        assert_eq!(response.result.locked, PlatformLockStatus::PartialLocked);
        let indices = response.result.locked_indices.as_ref().unwrap();
        assert_eq!(indices.len(), 2);
        assert_eq!(indices[0], "BTC");
        assert_eq!(indices[1], "ETH");
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
        assert_eq!(response.result.locked, PlatformLockStatus::AllLocked);
        let indices = response.result.locked_indices.as_ref().unwrap();
        assert_eq!(indices.len(), 4);
        assert!(indices.contains(&"BTC".to_string()));
        assert!(indices.contains(&"ETH".to_string()));
        assert!(indices.contains(&"SOL".to_string()));
        assert!(indices.contains(&"USDC".to_string()));
    }

    #[test]
    fn test_locked_status_values() {
        // Test all possible values for the locked field
        let cases = vec![
            ("true", PlatformLockStatus::AllLocked),
            ("partial", PlatformLockStatus::PartialLocked),
            ("false", PlatformLockStatus::Open),
        ];
        for (locked_value, expected_enum) in cases {
            let response_json = json!({
                "id": 1,
                "jsonrpc": "2.0",
                "result": {
                    "locked": locked_value,
                    "locked_indices": []
                }
            });

            let response: GetStatusResponse = serde_json::from_value(response_json).unwrap();
            assert_eq!(response.result.locked, expected_enum);
        }
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
        assert_eq!(response.result.locked_indices, Some(vec![]));

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
        let indices = response.result.locked_indices.as_ref().unwrap();
        assert_eq!(indices.len(), 1);
        assert_eq!(indices[0], "BTC");
    }
}
