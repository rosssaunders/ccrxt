//! Request and response structs for public/exchange_token endpoint
//!
//! This endpoint generates a token for a new subject id and can be used to switch
//! between subaccounts.

use serde::{Deserialize, Serialize};

use crate::deribit::{EndpointType, RestResult};

/// Request parameters for the public/exchange_token endpoint.
#[derive(Debug, Serialize)]
pub struct ExchangeTokenRequest {
    /// Refresh token (required)
    pub refresh_token: String,
    /// New subject id (required)
    pub subject_id: i64,
    /// Optional scope override for the new session.
    /// Cannot exceed caller's permissions.
    /// Supports "session" scope for direct session creation during token exchange.
    pub scope: Option<String>,
}

/// Response for public/exchange_token
#[derive(Debug, Deserialize)]
pub struct ExchangeTokenResponse {
    /// Access token
    pub access_token: String,
    /// Token lifetime in seconds
    pub expires_in: i64,
    /// Can be used to request a new token (with a new lifetime)
    pub refresh_token: String,
    /// Type of the access for assigned token
    pub scope: String,
    /// Optional Session id
    pub sid: Option<String>,
    /// Authorization type, allowed value - "bearer"
    pub token_type: String,
}

impl super::RestClient {
    /// Calls the public/exchange_token endpoint.
    ///
    /// Generates a token for a new subject id. This method can be used to switch
    /// between subaccounts.
    ///
    /// # Arguments
    /// * `params` - The exchange token request parameters
    ///
    /// # Returns
    /// The exchange token response containing the new tokens and session information
    pub async fn exchange_token(
        &self,
        params: &ExchangeTokenRequest,
    ) -> RestResult<ExchangeTokenResponse> {
        let endpoint_type = EndpointType::from_path("public/exchange_token");
        self.send_request(
            "public/exchange_token",
            reqwest::Method::POST,
            Some(params),
            endpoint_type,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exchange_token_request_serialization() {
        let request = ExchangeTokenRequest {
            refresh_token: "test_refresh_token".to_string(),
            subject_id: 12345,
            scope: Some("session".to_string()),
        };

        let json_value = serde_json::to_value(&request).unwrap();
        assert_eq!(json_value["refresh_token"], "test_refresh_token");
        assert_eq!(json_value["subject_id"], 12345);
        assert_eq!(json_value["scope"], "session");
    }

    #[test]
    fn test_exchange_token_request_without_scope() {
        let request = ExchangeTokenRequest {
            refresh_token: "test_refresh_token".to_string(),
            subject_id: 12345,
            scope: None,
        };

        let json_value = serde_json::to_value(&request).unwrap();
        assert_eq!(json_value["refresh_token"], "test_refresh_token");
        assert_eq!(json_value["subject_id"], 12345);
        assert!(json_value["scope"].is_null());
    }

    #[test]
    fn test_exchange_token_response_deserialization() {
        let json_response = r#"{
            "access_token": "test_access_token",
            "expires_in": 3600,
            "refresh_token": "test_new_refresh_token",
            "scope": "account:read",
            "sid": "session_123",
            "token_type": "bearer"
        }"#;

        let response: ExchangeTokenResponse = serde_json::from_str(json_response).unwrap();
        assert_eq!(response.access_token, "test_access_token");
        assert_eq!(response.expires_in, 3600);
        assert_eq!(response.refresh_token, "test_new_refresh_token");
        assert_eq!(response.scope, "account:read");
        assert_eq!(response.sid, Some("session_123".to_string()));
        assert_eq!(response.token_type, "bearer");
    }

    #[test]
    fn test_exchange_token_response_without_sid() {
        let json_response = r#"{
            "access_token": "test_access_token",
            "expires_in": 3600,
            "refresh_token": "test_new_refresh_token",
            "scope": "account:read",
            "sid": null,
            "token_type": "bearer"
        }"#;

        let response: ExchangeTokenResponse = serde_json::from_str(json_response).unwrap();
        assert_eq!(response.access_token, "test_access_token");
        assert_eq!(response.expires_in, 3600);
        assert_eq!(response.refresh_token, "test_new_refresh_token");
        assert_eq!(response.scope, "account:read");
        assert_eq!(response.sid, None);
        assert_eq!(response.token_type, "bearer");
    }
}