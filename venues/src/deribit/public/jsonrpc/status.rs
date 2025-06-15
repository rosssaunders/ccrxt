//! Implementation of the public/status endpoint for Deribit

use super::client::JsonRpcClient;
use crate::deribit::{DeribitResult, EndpointType};
use serde::{Deserialize, Serialize};

/// Status result data structure
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StatusResult {
    /// Platform lock status
    /// - "true": platform is locked in all currencies
    /// - "partial": some currencies are locked  
    /// - "false": no currencies are locked
    pub locked: String,
    
    /// List of currency indices that are locked platform-wise
    pub locked_indices: Vec<u32>,
}

impl JsonRpcClient {
    /// Get platform status information
    ///
    /// Returns information about locked currencies and the overall platform lock status.
    /// This method takes no parameters.
    ///
    /// See: [Deribit API Documentation](https://docs.deribit.com/v2/#public-status)
    ///
    /// Rate limit: Consumes 500 credits (non-matching engine endpoint)
    ///
    /// # Returns
    /// Response containing the platform lock status and list of locked currency indices
    ///
    /// # Example
    /// ```
    /// # use venues::deribit::public::jsonrpc::{JsonRpcClient, StatusResult};
    /// # use venues::deribit::{RateLimiter, AccountTier};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = reqwest::Client::new();
    /// let rate_limiter = RateLimiter::new(AccountTier::Tier4);
    /// let deribit_client = JsonRpcClient::new("https://www.deribit.com/api/v2", client, rate_limiter);
    ///
    /// let status = deribit_client.get_status().await?;
    /// println!("Platform locked: {}", status.locked);
    /// println!("Locked indices: {:?}", status.locked_indices);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_status(&self) -> DeribitResult<StatusResult> {
        self.send_request::<StatusResult, ()>(
            "public/status",
            None,
            EndpointType::PublicStatus,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::deribit::AccountTier;
    use serde_json::json;

    #[test]
    fn test_status_result_structure() {
        let status_json = json!({
            "locked": "false",
            "locked_indices": []
        });

        let status: StatusResult = serde_json::from_value(status_json).unwrap();
        assert_eq!(status.locked, "false");
        assert_eq!(status.locked_indices.len(), 0);
    }

    #[test]
    fn test_status_result_with_locked_currencies() {
        let status_json = json!({
            "locked": "partial",
            "locked_indices": [1, 2, 5]
        });

        let status: StatusResult = serde_json::from_value(status_json).unwrap();
        assert_eq!(status.locked, "partial");
        assert_eq!(status.locked_indices, vec![1, 2, 5]);
    }

    #[test]
    fn test_status_result_fully_locked() {
        let status_json = json!({
            "locked": "true",
            "locked_indices": [0, 1, 2, 3, 4, 5]
        });

        let status: StatusResult = serde_json::from_value(status_json).unwrap();
        assert_eq!(status.locked, "true");
        assert_eq!(status.locked_indices, vec![0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_status_result_serialization_roundtrip() {
        let original = StatusResult {
            locked: "partial".to_string(),
            locked_indices: vec![1, 3, 5],
        };

        let serialized = serde_json::to_value(&original).unwrap();
        let deserialized: StatusResult = serde_json::from_value(serialized).unwrap();

        assert_eq!(original, deserialized);
    }

    #[tokio::test]
    async fn test_get_status_method_compilation() {
        // This test ensures the get_status method compiles and is accessible
        // without needing to make an actual HTTP request
        use crate::deribit::RateLimiter;
        let client = reqwest::Client::new();
        let rate_limiter = RateLimiter::new(AccountTier::Tier4);
        let jsonrpc_client = JsonRpcClient::new("https://www.deribit.com/api/v2", client, rate_limiter);

        // Verify the method exists and is properly typed
        let _ = JsonRpcClient::get_status;
        let _ = &jsonrpc_client;

        // This proves the method signature is correct without calling it
        println!("get_status method is accessible and properly typed");
    }

    #[test]
    fn test_locked_values() {
        // Test the three possible values for the locked field
        let values = ["true", "partial", "false"];
        
        for value in values {
            let status_json = json!({
                "locked": value,
                "locked_indices": []
            });
            
            let status: StatusResult = serde_json::from_value(status_json).unwrap();
            assert_eq!(status.locked, value);
        }
    }

    #[test]
    fn test_empty_locked_indices() {
        let status_json = json!({
            "locked": "false",
            "locked_indices": []
        });

        let status: StatusResult = serde_json::from_value(status_json).unwrap();
        assert!(status.locked_indices.is_empty());
    }

    #[test]
    fn test_large_locked_indices() {
        let indices: Vec<u32> = (0..100).collect();
        let status_json = json!({
            "locked": "true",
            "locked_indices": indices
        });

        let status: StatusResult = serde_json::from_value(status_json).unwrap();
        assert_eq!(status.locked_indices.len(), 100);
        assert_eq!(status.locked_indices[0], 0);
        assert_eq!(status.locked_indices[99], 99);
    }
}