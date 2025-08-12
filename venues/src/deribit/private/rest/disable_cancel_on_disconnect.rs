use serde::Serialize;

use super::RestClient;
pub use super::enable_cancel_on_disconnect::CancelOnDisconnectScope;
use crate::deribit::{EndpointType, JsonRpcResult, RestResult};

/// REST API endpoint constant
const DISABLE_CANCEL_ON_DISCONNECT_ENDPOINT: &str = "private/disable_cancel_on_disconnect";

/// Request parameters for disable cancel on disconnect
#[derive(Debug, Clone, Serialize)]
pub struct DisableCancelOnDisconnectRequest {
    /// Specifies if Cancel On Disconnect change should be applied for the current connection or the account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<CancelOnDisconnectScope>,
}

/// Response for disable cancel on disconnect endpoint
pub type DisableCancelOnDisconnectResponse = JsonRpcResult<String>;

impl RestClient {
    /// Disable Cancel On Disconnect for the connection
    ///
    /// When change is applied for the account, every newly opened connection will start
    /// with inactive Cancel on Disconnect. This endpoint requires account:read_write scope.
    ///
    /// [docs]: https://docs.deribit.com/v2/#private-disable_cancel_on_disconnect
    ///
    /// Rate limit: 500 credits per request (non-matching engine)
    /// Scope: account:read_write
    ///
    /// # Arguments
    /// * `scope` - Optional scope specification (connection or account, defaults to connection)
    ///
    /// # Returns
    /// Result containing "ok" string on success
    pub async fn disable_cancel_on_disconnect(
        &self,
        request: DisableCancelOnDisconnectRequest,
    ) -> RestResult<DisableCancelOnDisconnectResponse> {
        self.send_signed_request(
            DISABLE_CANCEL_ON_DISCONNECT_ENDPOINT,
            &request,
            EndpointType::MatchingEngine,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
/// REST API endpoint constant
    use serde_json::{Value, json};

    use super::*;
    use crate::deribit::AccountTier;

    
    use crate::deribit::private::rest::credentials::Credentials;
    use rest::secrets::SecretString;


    #[test]
    fn test_request_parameters_serialization_minimal() {
        let request = DisableCancelOnDisconnectRequest { scope: None };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert!(json_value.get("scope").is_none());
    }

    #[test]
    fn test_request_parameters_serialization_with_connection_scope() {
        let request = DisableCancelOnDisconnectRequest {
            scope: Some(CancelOnDisconnectScope::Connection),
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("scope").unwrap(), "connection");
    }

    #[test]
    fn test_request_parameters_serialization_with_account_scope() {
        let request = DisableCancelOnDisconnectRequest {
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

        let response: DisableCancelOnDisconnectResponse =
            serde_json::from_value(response_json).unwrap();

        assert_eq!(response.id, 1);
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.result, "ok");
    }

    #[tokio::test]
    async fn test_disable_cancel_on_disconnect_method_exists() {
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
        let _ = RestClient::disable_cancel_on_disconnect;

        // Verify the client exists
        let _ = &rest_client;

        println!("disable_cancel_on_disconnect method is accessible and properly typed");
    }
}
