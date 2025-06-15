/// Integration tests for Deribit private REST API
///
/// These tests verify the complete integration of the Deribit private REST client,
/// including request/response handling, rate limiting, and error cases.

use crate::deribit::{
    PrivateRestClient, VerifyBlockTradeRequest, Trade, TradeRole, TradeDirection,
    RateLimiter, AccountTier, Errors
};
use rest::secrets::ExposableSecret;
use reqwest::Client;
use serde_json::json;

#[derive(Clone)]
struct TestSecret {
    secret: String,
}

impl TestSecret {
    fn new(secret: String) -> Self {
        Self { secret }
    }
}

impl ExposableSecret for TestSecret {
    fn expose_secret(&self) -> String {
        self.secret.clone()
    }
}

/// Creates a test client for integration testing
fn create_test_client() -> PrivateRestClient {
    let api_key = Box::new(TestSecret::new("test_api_key".to_string())) as Box<dyn ExposableSecret>;
    let api_secret = Box::new(TestSecret::new("test_api_secret".to_string())) as Box<dyn ExposableSecret>;
    let client = Client::new();
    let rate_limiter = RateLimiter::new(AccountTier::Tier4);

    PrivateRestClient::new(
        api_key,
        api_secret,
        "https://test.deribit.com",
        rate_limiter,
        client,
    )
}

#[test]
fn test_complete_request_structure() {
    // Test creating a complete verify_block_trade request
    let request = VerifyBlockTradeRequest {
        timestamp: 1672534800000,
        nonce: "unique_nonce_12345".to_string(),
        role: TradeRole::Maker,
        trades: vec![
            Trade {
                instrument_name: "BTC-PERPETUAL".to_string(),
                price: 50000.0,
                amount: Some(0.1),
                direction: TradeDirection::Buy,
            },
            Trade {
                instrument_name: "ETH-PERPETUAL".to_string(),
                price: 3000.0,
                amount: Some(1.0),
                direction: TradeDirection::Sell,
            },
        ],
    };

    // Verify request serializes correctly
    let serialized = serde_json::to_value(&request).unwrap();
    
    assert_eq!(serialized["timestamp"], 1672534800000_i64);
    assert_eq!(serialized["nonce"], "unique_nonce_12345");
    assert_eq!(serialized["role"], "maker");
    
    let trades = serialized["trades"].as_array().unwrap();
    assert_eq!(trades.len(), 2);
    
    // Verify first trade
    assert_eq!(trades[0]["instrument_name"], "BTC-PERPETUAL");
    assert_eq!(trades[0]["price"], 50000.0);
    assert_eq!(trades[0]["amount"], 0.1);
    assert_eq!(trades[0]["direction"], "buy");
    
    // Verify second trade
    assert_eq!(trades[1]["instrument_name"], "ETH-PERPETUAL");
    assert_eq!(trades[1]["price"], 3000.0);
    assert_eq!(trades[1]["amount"], 1.0);
    assert_eq!(trades[1]["direction"], "sell");
}

#[test]
fn test_multi_trade_scenarios() {
    // Test various trade combinations
    let scenarios = vec![
        // Single trade
        vec![Trade {
            instrument_name: "BTC-PERPETUAL".to_string(),
            price: 50000.0,
            amount: Some(0.1),
            direction: TradeDirection::Buy,
        }],
        // Multiple trades with mixed directions
        vec![
            Trade {
                instrument_name: "BTC-PERPETUAL".to_string(),
                price: 50000.0,
                amount: Some(0.1),
                direction: TradeDirection::Buy,
            },
            Trade {
                instrument_name: "ETH-PERPETUAL".to_string(),
                price: 3000.0,
                amount: None, // No amount specified
                direction: TradeDirection::Sell,
            },
        ],
        // Options and futures mix
        vec![
            Trade {
                instrument_name: "BTC-28JUN24-60000-C".to_string(),
                price: 1500.0,
                amount: Some(5.0),
                direction: TradeDirection::Buy,
            },
            Trade {
                instrument_name: "ETH-PERPETUAL".to_string(),
                price: 3000.0,
                amount: Some(10.0),
                direction: TradeDirection::Sell,
            },
        ],
    ];

    for (i, trades) in scenarios.into_iter().enumerate() {
        let request = VerifyBlockTradeRequest {
            timestamp: 1672534800000 + i as i64,
            nonce: format!("test_nonce_{}", i),
            role: if i % 2 == 0 { TradeRole::Maker } else { TradeRole::Taker },
            trades,
        };

        // Verify each scenario serializes without error
        let serialized = serde_json::to_value(&request);
        assert!(serialized.is_ok(), "Scenario {} failed to serialize", i);
        
        let json = serialized.unwrap();
        assert!(json["trades"].is_array());
        assert!(!json["trades"].as_array().unwrap().is_empty());
    }
}

#[tokio::test]
async fn test_client_initialization_and_id_generation() {
    let client = create_test_client();
    
    // Test that the client is properly initialized
    assert_eq!(client.base_url, "https://test.deribit.com");
    
    // Test request ID generation is sequential and unique
    let mut ids = vec![];
    for _ in 0..10 {
        ids.push(client.next_request_id().await);
    }
    
    // Verify IDs are sequential and unique
    for (i, &id) in ids.iter().enumerate() {
        assert_eq!(id, (i + 1) as u64);
    }
    
    // Verify no duplicates
    let mut sorted_ids = ids.clone();
    sorted_ids.sort();
    sorted_ids.dedup();
    assert_eq!(sorted_ids.len(), ids.len());
}

#[test]
fn test_error_handling_scenarios() {
    // Test various JSON-RPC error response formats
    let error_responses = vec![
        // Standard API error
        json!({
            "jsonrpc": "2.0",
            "id": 1,
            "error": {
                "code": 10001,
                "message": "Invalid API key"
            }
        }),
        // Error with additional data
        json!({
            "jsonrpc": "2.0",
            "id": "test-id",
            "error": {
                "code": 11030,
                "message": "Invalid signature",
                "data": {
                    "reason": "signature_verification_failed"
                }
            }
        }),
        // Authentication error
        json!({
            "jsonrpc": "2.0",
            "id": null,
            "error": {
                "code": 13009,
                "message": "Access denied"
            }
        }),
    ];

    for (i, error_json) in error_responses.into_iter().enumerate() {
        let error_response: Result<crate::deribit::ErrorResponse, _> = 
            serde_json::from_value(error_json);
        
        assert!(error_response.is_ok(), "Error response {} failed to parse", i);
        
        let error = error_response.unwrap();
        assert_eq!(error.jsonrpc, "2.0");
        assert!(error.error.code > 0);
        assert!(!error.error.message.is_empty());
        
        // Test conversion to our error type
        let api_error: Errors = error.into();
        match api_error {
            Errors::ApiError { code, message } => {
                assert!(code > 0);
                assert!(!message.is_empty());
            }
            _ => panic!("Expected ApiError for response {}", i),
        }
    }
}

#[test]
fn test_edge_cases_and_validation() {
    // Test edge cases for trade parameters
    
    // Minimum values
    let min_trade = Trade {
        instrument_name: "X".to_string(), // Minimal instrument name
        price: 0.01, // Small price
        amount: Some(0.001), // Minimal amount
        direction: TradeDirection::Buy,
    };
    
    // Maximum realistic values
    let max_trade = Trade {
        instrument_name: "BTC-29MAR24-100000-C".to_string(), // Long instrument name
        price: 1000000.0, // High price
        amount: Some(1000.0), // Large amount
        direction: TradeDirection::Sell,
    };
    
    // No amount specified
    let no_amount_trade = Trade {
        instrument_name: "ETH-PERPETUAL".to_string(),
        price: 3000.0,
        amount: None,
        direction: TradeDirection::Buy,
    };
    
    let request = VerifyBlockTradeRequest {
        timestamp: 1672534800000,
        nonce: "edge_case_test".to_string(),
        role: TradeRole::Taker,
        trades: vec![min_trade, max_trade, no_amount_trade],
    };
    
    // Verify serialization handles edge cases
    let serialized = serde_json::to_value(&request).unwrap();
    let trades = serialized["trades"].as_array().unwrap();
    
    assert_eq!(trades.len(), 3);
    
    // Verify no amount is properly omitted
    assert!(!trades[2].as_object().unwrap().contains_key("amount"));
    
    // Verify large values are preserved
    assert_eq!(trades[1]["price"], 1000000.0);
    assert_eq!(trades[1]["amount"], 1000.0);
}

#[test]
fn test_json_rpc_compliance() {
    // Test JSON-RPC 2.0 compliance for responses
    let valid_responses = vec![
        // Successful response
        json!({
            "jsonrpc": "2.0",
            "id": 1,
            "result": {
                "signature": "abcdef123456789"
            }
        }),
        // Response with string ID
        json!({
            "jsonrpc": "2.0",
            "id": "request-123",
            "result": {
                "signature": "xyz789abcdef"
            }
        }),
        // Response with null ID
        json!({
            "jsonrpc": "2.0",
            "id": null,
            "result": {
                "signature": "nullid123"
            }
        }),
    ];

    for (i, response_json) in valid_responses.into_iter().enumerate() {
        let response: Result<crate::deribit::VerifyBlockTradeResponse, _> = 
            serde_json::from_value(response_json);
        
        assert!(response.is_ok(), "Valid response {} failed to parse", i);
        
        let parsed = response.unwrap();
        assert_eq!(parsed.jsonrpc, "2.0");
        assert!(!parsed.result.signature.is_empty());
    }
}