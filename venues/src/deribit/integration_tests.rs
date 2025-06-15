//! Integration tests for Deribit public REST API endpoints
//!
//! These tests verify that the API structures are correctly defined and accessible.

use reqwest::Client;
use crate::deribit::{
    public::rest::{ExchangeTokenRequest, ExchangeTokenResponse, RestClient},
    AccountTier, RateLimiter,
};

#[tokio::test]
async fn test_exchange_token_endpoint_structure() {
    // Test that all the endpoint structures compile and are accessible
    let client = Client::new();
    let rate_limiter = RateLimiter::new(AccountTier::Tier4);
    let rest_client = RestClient::new("https://test.deribit.com", client, rate_limiter);

    // Test that the exchange_token method exists and is accessible
    let _ = RestClient::exchange_token;

    // Test that we can create request structures
    let request = ExchangeTokenRequest {
        refresh_token: "test_refresh_token".to_string(),
        subject_id: 12345,
        scope: Some("session".to_string()),
    };

    // Verify the request serializes correctly
    let json_value = serde_json::to_value(&request).unwrap();
    assert_eq!(json_value["refresh_token"], "test_refresh_token");
    assert_eq!(json_value["subject_id"], 12345);
    assert_eq!(json_value["scope"], "session");

    // Test response structure deserialization
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

    // Verify RestClient itself compiles and has correct base URL
    assert_eq!(rest_client.base_url, "https://test.deribit.com");

    println!("Exchange token endpoint structures are correctly implemented and accessible");
}

#[test]
fn test_exchange_token_request_validation() {
    // Test required fields
    let request = ExchangeTokenRequest {
        refresh_token: "refresh_123".to_string(),
        subject_id: 98765,
        scope: None,
    };

    let json_value = serde_json::to_value(&request).unwrap();
    assert_eq!(json_value["refresh_token"], "refresh_123");
    assert_eq!(json_value["subject_id"], 98765);
    assert!(json_value["scope"].is_null());

    // Test with scope
    let request_with_scope = ExchangeTokenRequest {
        refresh_token: "refresh_456".to_string(),
        subject_id: 11111,
        scope: Some("account:read account:write".to_string()),
    };

    let json_value = serde_json::to_value(&request_with_scope).unwrap();
    assert_eq!(json_value["refresh_token"], "refresh_456");
    assert_eq!(json_value["subject_id"], 11111);
    assert_eq!(json_value["scope"], "account:read account:write");
}

#[test]
fn test_exchange_token_response_variations() {
    // Test response without optional sid
    let json_response = r#"{
        "access_token": "access_789",
        "expires_in": 7200,
        "refresh_token": "refresh_789",
        "scope": "session",
        "token_type": "bearer"
    }"#;

    let response: ExchangeTokenResponse = serde_json::from_str(json_response).unwrap();
    assert_eq!(response.access_token, "access_789");
    assert_eq!(response.expires_in, 7200);
    assert_eq!(response.refresh_token, "refresh_789");
    assert_eq!(response.scope, "session");
    assert_eq!(response.sid, None);
    assert_eq!(response.token_type, "bearer");

    // Test response with null sid
    let json_response_null_sid = r#"{
        "access_token": "access_000",
        "expires_in": 1800,
        "refresh_token": "refresh_000",
        "scope": "trading",
        "sid": null,
        "token_type": "bearer"
    }"#;

    let response: ExchangeTokenResponse = serde_json::from_str(json_response_null_sid).unwrap();
    assert_eq!(response.access_token, "access_000");
    assert_eq!(response.expires_in, 1800);
    assert_eq!(response.refresh_token, "refresh_000");
    assert_eq!(response.scope, "trading");
    assert_eq!(response.sid, None);
    assert_eq!(response.token_type, "bearer");
}