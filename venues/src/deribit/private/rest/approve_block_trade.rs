use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::deribit::{EndpointType, JsonRpcResult, RestResult};

/// REST API endpoint constant
const APPROVE_BLOCK_TRADE_ENDPOINT: &str = "private/approve_block_trade";

/// Role enum for block trade approval
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    /// Maker role
    Maker,
    /// Taker role
    Taker,
}

/// Request parameters for approve block trade
#[derive(Debug, Clone, Serialize)]
pub struct ApproveBlockTradeRequest {
    /// Timestamp, shared with other party (milliseconds since the UNIX epoch)
    pub timestamp: i64,

    /// Nonce, shared with other party
    pub nonce: String,

    /// Describes if user wants to be maker or taker of trades
    pub role: Role,
}

/// Response for approve block trade endpoint
pub type ApproveBlockTradeResponse = JsonRpcResult<String>;

impl RestClient {
    /// Approve a pending block trade
    ///
    /// Used to approve a pending block trade. `nonce` and `timestamp` are used to
    /// identify the block trade while `role` should be opposite to the trading
    /// counterparty.
    ///
    /// To use a block trade approval feature the additional API key setting feature
    /// called: `enabled_features: block_trade_approval` is required. This key has to be
    /// given to broker/registered partner who performs the trades on behalf of the user
    /// for the feature to be active. If the user wants to approve the trade, he has to
    /// approve it from different API key with doesn't have this feature enabled.
    ///
    /// [docs](https://docs.deribit.com/v2/#private-approve_block_trade)
    ///
    /// Rate limit: 500 credits per request (non-matching engine)
    /// Scope: block_trade:read_write
    ///
    /// # Arguments
    /// * `timestamp` - Timestamp, shared with other party (milliseconds since the UNIX epoch)
    /// * `nonce` - Nonce, shared with other party
    /// * `role` - Describes if user wants to be maker or taker of trades
    ///
    /// # Returns
    /// Result with "ok" string in case of success
    pub async fn approve_block_trade(
        &self,
        request: ApproveBlockTradeRequest,
    ) -> RestResult<ApproveBlockTradeResponse> {
        self.send_signed_request(
            APPROVE_BLOCK_TRADE_ENDPOINT,
            &request,
            EndpointType::NonMatchingEngine,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

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
    fn test_role_serialization() {
        let maker_role = Role::Maker;
        let taker_role = Role::Taker;

        let maker_json = serde_json::to_string(&maker_role).unwrap();
        let taker_json = serde_json::to_string(&taker_role).unwrap();

        assert_eq!(maker_json, "\"maker\"");
        assert_eq!(taker_json, "\"taker\"");
    }

    #[test]
    fn test_role_deserialization() {
        let maker_role: Role = serde_json::from_str("\"maker\"").unwrap();
        let taker_role: Role = serde_json::from_str("\"taker\"").unwrap();

        matches!(maker_role, Role::Maker);
        matches!(taker_role, Role::Taker);
    }

    #[test]
    fn test_request_parameters_serialization() {
        let request = ApproveBlockTradeRequest {
            timestamp: 1672738134824,
            nonce: "test_nonce_123".to_string(),
            role: Role::Maker,
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("timestamp").unwrap(), 1672738134824i64);
        assert_eq!(json_value.get("nonce").unwrap(), "test_nonce_123");
        assert_eq!(json_value.get("role").unwrap(), "maker");
    }

    #[test]
    fn test_request_parameters_serialization_taker() {
        let request = ApproveBlockTradeRequest {
            timestamp: 1672738134824,
            nonce: "another_nonce_456".to_string(),
            role: Role::Taker,
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("timestamp").unwrap(), 1672738134824i64);
        assert_eq!(json_value.get("nonce").unwrap(), "another_nonce_456");
        assert_eq!(json_value.get("role").unwrap(), "taker");
    }

    #[test]
    fn test_response_structures_deserialization() {
        let response_json = json!({
            "id": 1,
            "jsonrpc": "2.0",
            "result": "ok"
        });

        let response: ApproveBlockTradeResponse = serde_json::from_value(response_json).unwrap();

        assert_eq!(response.id, 1);
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.result, "ok");
    }

    #[tokio::test]
    async fn test_approve_block_trade_method_exists() {
        // Test that the method exists and compiles without needing to call it
        let api_key =
            Box::new(PlainTextSecret::new("test_key".to_string())) as Box<dyn ExposableSecret>;
        let api_secret =
            Box::new(PlainTextSecret::new("test_secret".to_string())) as Box<dyn ExposableSecret>;
        let http_client = Arc::new(rest::native::NativeHttpClient::default());
        let rate_limiter = crate::deribit::RateLimiter::new(AccountTier::Tier4);

        let rest_client = RestClient::new(
            api_key,
            api_secret,
            "https://test.deribit.com",
            rate_limiter,
            http_client,
        );

        // Test that we can get a function reference to the method
        let _ = RestClient::approve_block_trade;

        // Verify the client exists
        let _ = &rest_client;

        println!("approve_block_trade method is accessible and properly typed");
    }
}
