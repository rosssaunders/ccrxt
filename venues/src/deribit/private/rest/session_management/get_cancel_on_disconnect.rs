use serde::{Deserialize, Serialize};

pub use super::enable_cancel_on_disconnect::CancelOnDisconnectScope;
use crate::deribit::{EndpointType, JsonRpcResult, PrivateRestClient, RestResult};

/// REST API endpoint constant
const GET_CANCEL_ON_DISCONNECT_ENDPOINT: &str = "private/get_cancel_on_disconnect";

/// Request parameters for get cancel on disconnect
#[derive(Debug, Clone, Serialize)]
pub struct GetCancelOnDisconnectRequest {
    /// Specifies if Cancel On Disconnect should be checked for the current connection or the account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<CancelOnDisconnectScope>,
}

/// Cancel On Disconnect configuration result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelOnDisconnectResult {
    /// Current configuration status
    pub enabled: bool,

    /// Informs if Cancel on Disconnect was checked for the current connection or the account
    pub scope: String,
}

/// Response for get cancel on disconnect endpoint
pub type GetCancelOnDisconnectResponse = JsonRpcResult<CancelOnDisconnectResult>;

impl PrivateRestClient {
    /// Read current Cancel On Disconnect configuration for the account
    ///
    /// This endpoint requires account:read scope and allows checking the current
    /// Cancel On Disconnect configuration for either the connection or account.
    ///
    /// [docs](https://docs.deribit.com/v2/#private-get_cancel_on_disconnect)
    ///
    /// Rate limit: 500 credits per request (non-matching engine)
    /// Scope: account:read
    ///
    /// # Arguments
    /// * `scope` - Optional scope specification (connection or account, defaults to connection)
    ///
    /// # Returns
    /// Result containing Cancel On Disconnect configuration with enabled status and scope
    pub async fn get_cancel_on_disconnect(
        &self,
        request: GetCancelOnDisconnectRequest,
    ) -> RestResult<GetCancelOnDisconnectResponse> {
        self.send_signed_request(
            GET_CANCEL_ON_DISCONNECT_ENDPOINT,
            &request,
            EndpointType::MatchingEngine,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use rest::secrets::SecretString;
    /// REST API endpoint constant
    use serde_json::{Value, json};

    use super::*;
    use crate::deribit::{AccountTier, credentials::Credentials};

    #[test]
    fn test_request_parameters_serialization_minimal() {
        let request = GetCancelOnDisconnectRequest { scope: None };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert!(json_value.get("scope").is_none());
    }

    #[test]
    fn test_request_parameters_serialization_with_connection_scope() {
        let request = GetCancelOnDisconnectRequest {
            scope: Some(CancelOnDisconnectScope::Connection),
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("scope").unwrap(), "connection");
    }

    #[test]
    fn test_request_parameters_serialization_with_account_scope() {
        let request = GetCancelOnDisconnectRequest {
            scope: Some(CancelOnDisconnectScope::Account),
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("scope").unwrap(), "account");
    }

    #[test]
    fn test_response_structures_deserialization_enabled() {
        let response_json = json!({
            "id": 1,
            "jsonrpc": "2.0",
            "result": {
                "enabled": true,
                "scope": "connection"
            }
        });

        let response: GetCancelOnDisconnectResponse =
            serde_json::from_value(response_json).unwrap();

        assert_eq!(response.id, 1);
        assert_eq!(response.jsonrpc, "2.0");
        assert!(response.result.enabled);
        assert_eq!(response.result.scope, "connection");
    }

    #[test]
    fn test_response_structures_deserialization_disabled() {
        let response_json = json!({
            "id": 2,
            "jsonrpc": "2.0",
            "result": {
                "enabled": false,
                "scope": "account"
            }
        });

        let response: GetCancelOnDisconnectResponse =
            serde_json::from_value(response_json).unwrap();

        assert_eq!(response.id, 2);
        assert_eq!(response.jsonrpc, "2.0");
        assert!(!response.result.enabled);
        assert_eq!(response.result.scope, "account");
    }

    #[test]
    fn test_result_structure_serialization() {
        let result = CancelOnDisconnectResult {
            enabled: true,
            scope: "connection".to_string(),
        };

        let json_str = serde_json::to_string(&result).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("enabled").unwrap(), true);
        assert_eq!(json_value.get("scope").unwrap(), "connection");
    }

    #[tokio::test]
    async fn test_get_cancel_on_disconnect_method_exists() {
        // Test that the method exists and compiles without needing to call it
        let credentials = Credentials {
            api_key: SecretString::from("test_key".to_string()),
            api_secret: SecretString::from("test_secret".to_string()),
        };
        let http_client = Arc::new(rest::native::NativeHttpClient::default());
        let rate_limiter = crate::deribit::RateLimiter::new(AccountTier::Tier4);

        let rest_client = PrivateRestClient::new(
            credentials,
            "https://test.deribit.com",
            rate_limiter,
            http_client,
        );

        // Test that we can get a function reference to the method
        let _ = PrivateRestClient::get_cancel_on_disconnect;

        // Verify the client exists
        let _ = &rest_client;

        println!("get_cancel_on_disconnect method is accessible and properly typed");
    }
}
