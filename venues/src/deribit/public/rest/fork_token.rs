//! Request and response structs for public/fork_token endpoint
//!
//! Generates a token for a new named session. This method can be used only with
//! session scoped tokens.

use super::client::RestClient;
use crate::deribit::{EndpointType, RestResult};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// Request parameters for the public/fork_token endpoint.
///
/// Generates a token for a new named session. This method can be used only with
/// session scoped tokens.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForkTokenRequest {
    /// JSON-RPC version (always "2.0")
    #[serde(rename = "jsonrpc")]
    pub jsonrpc: Cow<'static, str>,

    /// Unique request ID
    #[serde(rename = "id")]
    pub id: u64,

    /// Method name (always "public/fork_token")
    #[serde(rename = "method")]
    pub method: Cow<'static, str>,

    /// Request parameters
    #[serde(rename = "params")]
    pub params: ForkTokenParams,
}

/// Parameters for the fork_token request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForkTokenParams {
    /// Refresh token
    #[serde(rename = "refresh_token")]
    pub refresh_token: String,

    /// New session name
    #[serde(rename = "session_name")]
    pub session_name: String,
}

impl ForkTokenRequest {
    /// Create a new fork_token request
    pub fn new(refresh_token: String, session_name: String, id: u64) -> Self {
        Self {
            jsonrpc: "2.0".into(),
            id,
            method: "public/fork_token".into(),
            params: ForkTokenParams {
                refresh_token,
                session_name,
            },
        }
    }
}

/// Response for public/fork_token endpoint.
#[derive(Debug, Clone, Deserialize)]
pub struct ForkTokenResponse {
    /// The ID that was sent in the request
    #[serde(rename = "id")]
    pub id: u64,

    /// The JSON-RPC version (2.0)
    #[serde(rename = "jsonrpc")]
    pub jsonrpc: String,

    /// Result data for the token
    #[serde(rename = "result")]
    pub result: TokenResult,
}

/// Result data for the token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenResult {
    /// Access token
    #[serde(rename = "access_token")]
    pub access_token: String,

    /// Token lifetime in seconds
    #[serde(rename = "expires_in")]
    pub expires_in: u64,

    /// Can be used to request a new token (with a new lifetime)
    #[serde(rename = "refresh_token")]
    pub refresh_token: String,

    /// Type of the access for assigned token
    #[serde(rename = "scope")]
    pub scope: String,

    /// Optional Session id
    #[serde(rename = "sid")]
    pub sid: Option<String>,

    /// Authorization type, allowed value - "bearer"
    #[serde(rename = "token_type")]
    pub token_type: String,
}

impl RestClient {
    /// Calls the public/fork_token endpoint.
    ///
    /// Generates a token for a new named session. This method can be used only with
    /// session scoped tokens.
    ///
    /// # Arguments
    /// * `refresh_token` - The refresh token
    /// * `session_name` - The new session name
    /// * `request_id` - Unique request ID for JSON-RPC
    ///
    /// # Returns
    /// A result containing the token response or an error
    ///
    /// [Official API docs](https://docs.deribit.com/)
    pub async fn fork_token(
        &self,
        refresh_token: String,
        session_name: String,
        request_id: u64,
    ) -> RestResult<ForkTokenResponse> {
        let request = ForkTokenRequest::new(refresh_token, session_name, request_id);
        
        self.send_request(
            "public/fork_token",
            reqwest::Method::POST,
            Some(&request),
            EndpointType::PublicForkToken,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_fork_token_request_creation() {
        let request = ForkTokenRequest::new(
            "test_refresh_token".to_string(),
            "test_session".to_string(),
            123,
        );

        assert_eq!(request.jsonrpc, "2.0");
        assert_eq!(request.id, 123);
        assert_eq!(request.method, "public/fork_token");
        assert_eq!(request.params.refresh_token, "test_refresh_token");
        assert_eq!(request.params.session_name, "test_session");
    }

    #[test]
    fn test_fork_token_request_serialization() {
        let request = ForkTokenRequest::new(
            "test_refresh_token".to_string(),
            "test_session".to_string(),
            123,
        );

        let json = serde_json::to_string(&request).expect("Should serialize");
        let parsed: serde_json::Value = serde_json::from_str(&json).expect("Should parse");

        assert_eq!(parsed["jsonrpc"], "2.0");
        assert_eq!(parsed["id"], 123);
        assert_eq!(parsed["method"], "public/fork_token");
        assert_eq!(parsed["params"]["refresh_token"], "test_refresh_token");
        assert_eq!(parsed["params"]["session_name"], "test_session");
    }

    #[test]
    fn test_fork_token_response_deserialization() {
        let json = r#"{
            "id": 123,
            "jsonrpc": "2.0",
            "result": {
                "access_token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiIsImp0aSI6Ijc2ZTQ4ZGI4LTcwZjMtNGVkNi04ZjYyLWM4ZTJhYTQyNjY4MSIsImN0eSI6Im9wYXF1ZSJ9",
                "expires_in": 604800,
                "refresh_token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiIsImp0aSI6IjIyYWI5NDJmLTQ3ODItNGNjNy05ZmNmLWVhNTVhMWJlNDM1NyIsImN0eSI6Im9wYXF1ZSJ9",
                "scope": "session:test_session",
                "sid": "test_session_id",
                "token_type": "bearer"
            }
        }"#;

        let response: ForkTokenResponse = serde_json::from_str(json).expect("Should deserialize");

        assert_eq!(response.id, 123);
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.result.access_token, "eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiIsImp0aSI6Ijc2ZTQ4ZGI4LTcwZjMtNGVkNi04ZjYyLWM4ZTJhYTQyNjY4MSIsImN0eSI6Im9wYXF1ZSJ9");
        assert_eq!(response.result.expires_in, 604800);
        assert_eq!(response.result.refresh_token, "eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiIsImp0aSI6IjIyYWI5NDJmLTQ3ODItNGNjNy05ZmNmLWVhNTVhMWJlNDM1NyIsImN0eSI6Im9wYXF1ZSJ9");
        assert_eq!(response.result.scope, "session:test_session");
        assert_eq!(response.result.sid, Some("test_session_id".to_string()));
        assert_eq!(response.result.token_type, "bearer");
    }

    #[test]
    fn test_fork_token_response_without_sid() {
        let json = r#"{
            "id": 456,
            "jsonrpc": "2.0",
            "result": {
                "access_token": "test_token",
                "expires_in": 3600,
                "refresh_token": "test_refresh",
                "scope": "session:test",
                "token_type": "bearer"
            }
        }"#;

        let response: ForkTokenResponse = serde_json::from_str(json).expect("Should deserialize");

        assert_eq!(response.result.sid, None);
    }

    #[test]
    fn test_fork_token_params_structure() {
        let params = ForkTokenParams {
            refresh_token: "test_token".to_string(),
            session_name: "test_session".to_string(),
        };

        let json = serde_json::to_string(&params).expect("Should serialize");
        let parsed: ForkTokenParams = serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(parsed.refresh_token, "test_token");
        assert_eq!(parsed.session_name, "test_session");
    }

    #[test]
    fn test_token_result_serialization_roundtrip() {
        let token_result = TokenResult {
            access_token: "test_access_token".to_string(),
            expires_in: 7200,
            refresh_token: "test_refresh_token".to_string(),
            scope: "session:test_scope".to_string(),
            sid: Some("test_sid".to_string()),
            token_type: "bearer".to_string(),
        };

        let json = serde_json::to_string(&token_result).expect("Should serialize");
        let parsed: TokenResult = serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(parsed.access_token, token_result.access_token);
        assert_eq!(parsed.expires_in, token_result.expires_in);
        assert_eq!(parsed.refresh_token, token_result.refresh_token);
        assert_eq!(parsed.scope, token_result.scope);
        assert_eq!(parsed.sid, token_result.sid);
        assert_eq!(parsed.token_type, token_result.token_type);
    }
}