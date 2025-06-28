use serde::{Deserialize, Serialize};

use super::RestClient;
pub use super::enable_cancel_on_disconnect::CancelOnDisconnectScope;
use crate::deribit::{EndpointType, RestResult};

/// Request parameters for disable cancel on disconnect
#[derive(Debug, Clone, Serialize)]
pub struct DisableCancelOnDisconnectRequest {
    /// Specifies if Cancel On Disconnect change should be applied for the current connection or the account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<CancelOnDisconnectScope>,
}

/// Response for disable cancel on disconnect endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisableCancelOnDisconnectResponse {
    /// The id that was sent in the request
    pub id: i64,
    /// The JSON-RPC version (2.0)
    pub jsonrpc: String,
    /// Result of method execution. "ok" in case of success
    pub result: String,
}

impl RestClient {
    /// Disable Cancel On Disconnect for the connection
    ///
    /// When change is applied for the account, every newly opened connection will start
    /// with inactive Cancel on Disconnect. This endpoint requires account:read_write scope.
    ///
    /// See: <https://docs.deribit.com/v2/#private-disable_cancel_on_disconnect>
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
            "private/disable_cancel_on_disconnect",
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
        let api_key =
            Box::new(PlainTextSecret::new("test_key".to_string())) as Box<dyn ExposableSecret>;
        let api_secret =
            Box::new(PlainTextSecret::new("test_secret".to_string())) as Box<dyn ExposableSecret>;
        let client = reqwest::Client::new();
        let rate_limiter = crate::deribit::RateLimiter::new(AccountTier::Tier4);

        let rest_client = RestClient::new(
            api_key,
            api_secret,
            "https://test.deribit.com",
            rate_limiter,
            client,
        );

        // Test that we can get a function reference to the method
        let _ = RestClient::disable_cancel_on_disconnect;

        // Verify the client exists
        let _ = &rest_client;

        println!("disable_cancel_on_disconnect method is accessible and properly typed");
    }
}
