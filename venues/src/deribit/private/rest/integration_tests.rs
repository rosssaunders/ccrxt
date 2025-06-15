use crate::deribit::private::RestClient;
use rest::secrets::ExposableSecret;

/// A plain text implementation of ExposableSecret for testing purposes.
#[derive(Clone)]
#[allow(dead_code)]
struct PlainTextSecret {
    secret: String,
}

impl ExposableSecret for PlainTextSecret {
    fn expose_secret(&self) -> String {
        self.secret.clone()
    }
}

impl PlainTextSecret {
    #[allow(dead_code)]
    fn new(secret: String) -> Self {
        Self { secret }
    }
}

#[tokio::test]
async fn test_deribit_private_endpoints_compile() {
    // Test that all the new private endpoints compile and are accessible
    let client_id =
        Box::new(PlainTextSecret::new("test_client_id".to_string())) as Box<dyn ExposableSecret>;
    let client_secret =
        Box::new(PlainTextSecret::new("test_client_secret".to_string())) as Box<dyn ExposableSecret>;
    let client = reqwest::Client::new();

    let rest_client = RestClient::new(client_id, client_secret, "https://test.deribit.com", client);

    // Test that methods exist by verifying we can get function references to them
    // This proves they compile and are accessible without needing to call them
    let _ = RestClient::authenticate;
    let _ = RestClient::withdraw;

    // Verify RestClient itself compiles
    let _ = &rest_client;

    println!("All Deribit private endpoint methods are accessible and properly typed");
}

#[test]
fn test_withdraw_request_parameters_serialization() {
    use crate::deribit::{WithdrawRequest};
    use serde_json;

    // Test basic withdrawal request
    let withdraw_params = WithdrawRequest {
        currency: "BTC".to_string(),
        address: "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa".to_string(),
        amount: 0.001,
        priority: Some("high".to_string()),
    };
    let json_value = serde_json::to_value(withdraw_params).unwrap();
    assert_eq!(json_value.get("currency").unwrap(), "BTC");
    assert_eq!(json_value.get("address").unwrap(), "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa");
    assert_eq!(json_value.get("amount").unwrap(), 0.001);
    assert_eq!(json_value.get("priority").unwrap(), "high");

    // Test withdrawal request without priority
    let withdraw_params_no_priority = WithdrawRequest {
        currency: "ETH".to_string(),
        address: "0x742d35Cc6634C0532925a3b8D43C67C3b4BF9B7E".to_string(),
        amount: 0.1,
        priority: None,
    };
    let json_value = serde_json::to_value(withdraw_params_no_priority).unwrap();
    assert_eq!(json_value.get("currency").unwrap(), "ETH");
    assert!(json_value.get("priority").is_none());
}

#[test]
fn test_withdraw_response_structures_deserialization() {
    use crate::deribit::{WithdrawResult, WithdrawResponse};
    use serde_json::json;

    // Test WithdrawResult deserialization
    let result_json = json!({
        "address": "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa",
        "amount": 0.001,
        "confirmed_timestamp": null,
        "created_timestamp": 1609459200000_u64,
        "currency": "BTC",
        "fee": 0.0005,
        "id": 12345,
        "priority": 2.0,
        "state": "unconfirmed",
        "transaction_id": null,
        "updated_timestamp": 1609459200000_u64
    });
    let _result: WithdrawResult = serde_json::from_value(result_json).unwrap();

    // Test full WithdrawResponse deserialization
    let response_json = json!({
        "id": 1,
        "jsonrpc": "2.0",
        "result": {
            "address": "bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4",
            "amount": 0.005,
            "confirmed_timestamp": 1609462800000_u64,
            "created_timestamp": 1609459200000_u64,
            "currency": "BTC",
            "fee": 0.0002,
            "id": 67890,
            "priority": 3.0,
            "state": "completed",
            "transaction_id": "a1b2c3d4e5f6789012345678901234567890123456789012345678901234567890",
            "updated_timestamp": 1609462800000_u64
        }
    });
    let _response: WithdrawResponse = serde_json::from_value(response_json).unwrap();

    println!("All Deribit withdraw response structures deserialize correctly from JSON");
}