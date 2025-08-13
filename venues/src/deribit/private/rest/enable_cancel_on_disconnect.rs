use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::deribit::{EndpointType, JsonRpcResult, RestResult};

/// REST API endpoint constant
const ENABLE_CANCEL_ON_DISCONNECT_ENDPOINT: &str = "private/enable_cancel_on_disconnect";

/// Scope for Cancel On Disconnect operations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum CancelOnDisconnectScope {
    /// Change applies to current connection only (websocket only)
    Connection,

    /// Change applies to the entire account
    Account,
}

/// Request parameters for enable cancel on disconnect
#[derive(Debug, Clone, Serialize)]
pub struct EnableCancelOnDisconnectRequest {
    /// Specifies if Cancel On Disconnect change should be applied for the current connection or the account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<CancelOnDisconnectScope>,
}

/// Response for enable cancel on disconnect endpoint
pub type EnableCancelOnDisconnectResponse = JsonRpcResult<String>;

impl RestClient {
    /// Enable Cancel On Disconnect for the connection
    ///
    /// After enabling Cancel On Disconnect all orders created by the connection will be removed
    /// when the connection is closed. This endpoint requires account:read_write scope.
    ///
    /// [docs](https://docs.deribit.com/v2/#private-enable_cancel_on_disconnect)
    ///
    /// Rate limit: 500 credits per request (non-matching engine)
    /// Scope: account:read_write
    ///
    /// # Arguments
    /// * `scope` - Optional scope specification (connection or account, defaults to connection)
    ///
    /// # Returns
    /// Result containing "ok" string on success
    pub async fn enable_cancel_on_disconnect(
        &self,
        request: EnableCancelOnDisconnectRequest,
    ) -> RestResult<EnableCancelOnDisconnectResponse> {
        self.send_signed_request(
            ENABLE_CANCEL_ON_DISCONNECT_ENDPOINT,
            &request,
            EndpointType::MatchingEngine,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use rest::secrets::{ExposableSecret, SecretString};
    /// REST API endpoint constant
    use serde_json::{Value, json};

    use super::*;
    use crate::deribit::{AccountTier, private::rest::credentials::Credentials};

    #[test]
    fn test_scope_serialization() {
        assert_eq!(
            serde_json::to_string(&CancelOnDisconnectScope::Connection).unwrap(),
            "\"connection\""
        );
        assert_eq!(
            serde_json::to_string(&CancelOnDisconnectScope::Account).unwrap(),
            "\"account\""
        );
    }

    #[test]
    fn test_scope_deserialization() {
        let connection: CancelOnDisconnectScope = serde_json::from_str("\"connection\"").unwrap();
        assert_eq!(connection, CancelOnDisconnectScope::Connection);

        let account: CancelOnDisconnectScope = serde_json::from_str("\"account\"").unwrap();
        assert_eq!(account, CancelOnDisconnectScope::Account);
    }

    #[test]
    fn test_request_parameters_serialization_minimal() {
        let request = EnableCancelOnDisconnectRequest { scope: None };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert!(json_value.get("scope").is_none());
    }

    #[test]
    fn test_request_parameters_serialization_with_connection_scope() {
        let request = EnableCancelOnDisconnectRequest {
            scope: Some(CancelOnDisconnectScope::Connection),
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("scope").unwrap(), "connection");
    }

    #[test]
    fn test_request_parameters_serialization_with_account_scope() {
        let request = EnableCancelOnDisconnectRequest {
            scope: Some(CancelOnDisconnectScope::Account),
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("scope").unwrap(), "account");
    }

    #[test]
    fn test_response_structures_deserialization() {
        let response_json = json!({
            "id": 1,
            "jsonrpc": "2.0",
            "result": "ok"
        });

        let response: EnableCancelOnDisconnectResponse =
            serde_json::from_value(response_json).unwrap();

        assert_eq!(response.id, 1);
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.result, "ok");
    }

    #[tokio::test]
    async fn test_enable_cancel_on_disconnect_method_exists() {
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
        let _ = RestClient::enable_cancel_on_disconnect;

        // Verify the client exists
        let _ = &rest_client;

        println!("enable_cancel_on_disconnect method is accessible and properly typed");
    }

    #[test]
    fn test_all_scope_variants() {
        let scopes = vec![
            CancelOnDisconnectScope::Connection,
            CancelOnDisconnectScope::Account,
        ];

        // Test serialization/deserialization for all variants
        for scope in scopes {
            let serialized = serde_json::to_string(&scope).unwrap();
            let deserialized: CancelOnDisconnectScope = serde_json::from_str(&serialized).unwrap();
            assert_eq!(scope, deserialized);
        }

        println!("All scope variants are properly supported");
    }
}
