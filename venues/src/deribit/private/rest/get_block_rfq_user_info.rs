use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::deribit::{EndpointType, RestResult};

/// Request parameters for get Block RFQ user info
#[derive(Debug, Clone, Serialize)]
pub struct GetBlockRfqUserInfoRequest {
    // This endpoint takes no parameters
}

/// Parent identity information representing the overall account group
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParentIdentity {
    /// Group-level alias identifying the account group as a whole
    pub identity: String,
    /// Indicates whether the Parent Identity has maker scope
    pub is_maker: bool,
}

/// User-specific identity and rating information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    /// Specific alias identifying this account individually
    pub identity: String,
    /// Indicates whether this account has maker scope
    pub is_maker: bool,
    /// Taker rating associated with this account, if available
    pub taker_rating: f64,
    /// Unique user identifier
    pub user_id: i64,
}

/// Result data containing identity and rating information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetBlockRfqUserInfoResult {
    /// Parent Identity (group alias), representing the overall account group (main + subaccounts)
    pub parent: ParentIdentity,
    /// Array of user-specific identity and rating information
    pub users: Vec<UserInfo>,
}

/// Response for get Block RFQ user info endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetBlockRfqUserInfoResponse {
    /// The id that was sent in the request
    pub id: i64,
    /// The JSON-RPC version (2.0)
    pub jsonrpc: String,
    /// Result containing identity and rating information
    pub result: GetBlockRfqUserInfoResult,
}

impl RestClient {
    /// Get identity and rating information for the requesting account and its subaccounts
    ///
    /// This method returns identity and rating information for the requesting account and its
    /// subaccounts. Includes both group-level and individual user-level alias data, if available.
    /// This endpoint requires block_rfq:read scope.
    ///
    /// See: <https://docs.deribit.com/v2/#private-get_block_rfq_user_info>
    ///
    /// Rate limit: Depends on endpoint type (matching engine)
    /// Scope: block_rfq:read
    ///
    /// # Returns
    /// Result containing identity and rating information for the account and subaccounts
    pub async fn get_block_rfq_user_info(&self) -> RestResult<GetBlockRfqUserInfoResponse> {
        let request = GetBlockRfqUserInfoRequest {};
        self.send_signed_request(
            "private/get_block_rfq_user_info",
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
        fn new(secret: impl Into<String>) -> Self {
            Self {
                secret: secret.into(),
            }
        }
    }

    impl ExposableSecret for PlainTextSecret {
        fn expose_secret(&self) -> String {
            self.secret.clone()
        }
    }

    #[test]
    fn test_request_serialization() {
        let request = GetBlockRfqUserInfoRequest {};
        
        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();
        
        // Should serialize to an empty object since no parameters
        assert_eq!(json_value, json!({}));
    }

    #[test]
    fn test_response_structure() {
        let response_json = json!({
            "id": 123,
            "jsonrpc": "2.0",
            "result": {
                "parent": {
                    "identity": "group_alias_123",
                    "is_maker": true
                },
                "users": [
                    {
                        "identity": "user_alias_456",
                        "is_maker": true,
                        "taker_rating": 85.5,
                        "user_id": 12345
                    },
                    {
                        "identity": "user_alias_789",
                        "is_maker": false,
                        "taker_rating": 72.3,
                        "user_id": 67890
                    }
                ]
            }
        });

        let response: GetBlockRfqUserInfoResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.id, 123);
        assert_eq!(response.jsonrpc, "2.0");
        
        // Test parent identity
        assert_eq!(response.result.parent.identity, "group_alias_123");
        assert!(response.result.parent.is_maker);
        
        // Test users array
        assert_eq!(response.result.users.len(), 2);
        
        // Test first user
        let first_user = &response.result.users[0];
        assert_eq!(first_user.identity, "user_alias_456");
        assert!(first_user.is_maker);
        assert_eq!(first_user.taker_rating, 85.5);
        assert_eq!(first_user.user_id, 12345);
        
        // Test second user
        let second_user = &response.result.users[1];
        assert_eq!(second_user.identity, "user_alias_789");
        assert!(!second_user.is_maker);
        assert_eq!(second_user.taker_rating, 72.3);
        assert_eq!(second_user.user_id, 67890);
    }

    #[test]
    fn test_response_empty_users() {
        let response_json = json!({
            "id": 456,
            "jsonrpc": "2.0",
            "result": {
                "parent": {
                    "identity": "group_only",
                    "is_maker": false
                },
                "users": []
            }
        });

        let response: GetBlockRfqUserInfoResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.id, 456);
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.result.parent.identity, "group_only");
        assert!(!response.result.parent.is_maker);
        assert!(response.result.users.is_empty());
    }

    #[test]
    fn test_endpoint_method_signature() {
        // Test that we can create a mock client
        let rate_limiter = crate::deribit::RateLimiter::new(AccountTier::Tier1);
        let client = RestClient::new(
            Box::new(PlainTextSecret::new("test_key")),
            Box::new(PlainTextSecret::new("test_secret")),
            "https://test.deribit.com",
            rate_limiter,
            reqwest::Client::new(),
        );

        // Test method signature - this ensures the method compiles correctly
        // We can't actually call it without a real connection, but we can verify the signature
        let _future = client.get_block_rfq_user_info();
    }

    #[test]
    fn test_json_rpc_compliance() {
        let response_json = json!({
            "id": 999,
            "jsonrpc": "2.0",
            "result": {
                "parent": {
                    "identity": "test_group",
                    "is_maker": true
                },
                "users": [
                    {
                        "identity": "test_user",
                        "is_maker": false,
                        "taker_rating": 50.0,
                        "user_id": 1
                    }
                ]
            }
        });

        let response: GetBlockRfqUserInfoResponse = serde_json::from_value(response_json).unwrap();
        
        // Verify JSON-RPC 2.0 compliance
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.id, 999);
        assert!(!response.result.users.is_empty());
    }

    #[test]
    fn test_response_deserialization_edge_cases() {
        // Test with various identity formats and rating values
        let response_json = json!({
            "id": 1,
            "jsonrpc": "2.0",
            "result": {
                "parent": {
                    "identity": "PARENT-WITH_SPECIAL.CHARS123",
                    "is_maker": false
                },
                "users": [
                    {
                        "identity": "simple_user",
                        "is_maker": true,
                        "taker_rating": 0.0,
                        "user_id": 1
                    },
                    {
                        "identity": "user-with-dashes",
                        "is_maker": false,
                        "taker_rating": 100.0,
                        "user_id": 999999
                    },
                    {
                        "identity": "user.with.dots",
                        "is_maker": true,
                        "taker_rating": 45.75,
                        "user_id": 12345678
                    }
                ]
            }
        });

        let response: GetBlockRfqUserInfoResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.result.parent.identity, "PARENT-WITH_SPECIAL.CHARS123");
        assert!(!response.result.parent.is_maker);
        
        assert_eq!(response.result.users.len(), 3);
        assert_eq!(response.result.users[0].identity, "simple_user");
        assert_eq!(response.result.users[0].taker_rating, 0.0);
        assert_eq!(response.result.users[1].identity, "user-with-dashes");
        assert_eq!(response.result.users[1].taker_rating, 100.0);
        assert_eq!(response.result.users[2].identity, "user.with.dots");
        assert_eq!(response.result.users[2].taker_rating, 45.75);
    }

    #[test]
    fn test_parent_identity_structure() {
        let parent_json = json!({
            "identity": "test_parent",
            "is_maker": true
        });

        let parent: ParentIdentity = serde_json::from_value(parent_json).unwrap();
        assert_eq!(parent.identity, "test_parent");
        assert!(parent.is_maker);
    }

    #[test]
    fn test_user_info_structure() {
        let user_json = json!({
            "identity": "test_user",
            "is_maker": false,
            "taker_rating": 87.25,
            "user_id": 54321
        });

        let user: UserInfo = serde_json::from_value(user_json).unwrap();
        assert_eq!(user.identity, "test_user");
        assert!(!user.is_maker);
        assert_eq!(user.taker_rating, 87.25);
        assert_eq!(user.user_id, 54321);
    }
}