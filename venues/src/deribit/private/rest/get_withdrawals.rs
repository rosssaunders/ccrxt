use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::deribit::{Currency, EndpointType, RestResult};

// Re-export WithdrawalData from withdraw.rs to maintain consistency
pub use super::withdraw::WithdrawalData;

/// Request parameters for get withdrawals
#[derive(Debug, Clone, Serialize)]
pub struct GetWithdrawalsRequest {
    /// The currency symbol
    pub currency: Currency,
    /// Number of requested items, default - 10
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    /// The offset for pagination, default - 0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
}

/// Result data for get withdrawals endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetWithdrawalsResult {
    /// Total number of results available
    pub count: i32,
    /// Array of withdrawal entries
    pub data: Vec<WithdrawalData>,
}

/// Response for get withdrawals endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetWithdrawalsResponse {
    /// The id that was sent in the request
    pub id: i64,
    /// The JSON-RPC version (2.0)
    pub jsonrpc: String,
    /// Withdrawal result data
    pub result: GetWithdrawalsResult,
}

impl RestClient {
    /// Retrieve the latest users withdrawals
    ///
    /// This endpoint retrieves withdrawal history for the authenticated user.
    /// The endpoint requires wallet:read scope.
    ///
    /// See: <https://docs.deribit.com/v2/#private-get_withdrawals>
    ///
    /// Rate limit: 500 credits per request (non-matching engine)
    /// Scope: wallet:read
    ///
    /// # Arguments
    /// * `currency` - The currency symbol (BTC, ETH, USDC, USDT, EURR)
    /// * `count` - Number of requested items, default - 10
    /// * `offset` - The offset for pagination, default - 0
    ///
    /// # Returns
    /// Result containing withdrawal history with count and data array
    pub async fn get_withdrawals(
        &self,
        currency: Currency,
        count: Option<i32>,
        offset: Option<i32>,
    ) -> RestResult<GetWithdrawalsResponse> {
        let request = GetWithdrawalsRequest {
            currency,
            count,
            offset,
        };
        self.send_signed_request(
            "private/get_withdrawals",
            &request,
            EndpointType::NonMatchingEngine,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use rest::secrets::ExposableSecret;
    use serde_json::{Value, json};

    use super::*;
    use crate::deribit::{AccountTier, WithdrawalState};

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
    fn test_request_parameters_serialization_required_only() {
        let request = GetWithdrawalsRequest {
            currency: Currency::BTC,
            count: None,
            offset: None,
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("currency").unwrap(), "BTC");
        assert!(json_value.get("count").is_none());
        assert!(json_value.get("offset").is_none());
    }

    #[test]
    fn test_request_parameters_serialization_with_count() {
        let request = GetWithdrawalsRequest {
            currency: Currency::ETH,
            count: Some(20),
            offset: None,
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("currency").unwrap(), "ETH");
        assert_eq!(json_value.get("count").unwrap(), 20);
        assert!(json_value.get("offset").is_none());
    }

    #[test]
    fn test_request_parameters_serialization_with_offset() {
        let request = GetWithdrawalsRequest {
            currency: Currency::USDC,
            count: None,
            offset: Some(50),
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("currency").unwrap(), "USDC");
        assert!(json_value.get("count").is_none());
        assert_eq!(json_value.get("offset").unwrap(), 50);
    }

    #[test]
    fn test_request_parameters_serialization_all_parameters() {
        let request = GetWithdrawalsRequest {
            currency: Currency::USDT,
            count: Some(5),
            offset: Some(10),
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("currency").unwrap(), "USDT");
        assert_eq!(json_value.get("count").unwrap(), 5);
        assert_eq!(json_value.get("offset").unwrap(), 10);
    }

    #[test]
    fn test_response_structures_deserialization_empty() {
        let response_json = json!({
            "id": 1,
            "jsonrpc": "2.0",
            "result": {
                "count": 0,
                "data": []
            }
        });

        let response: GetWithdrawalsResponse = serde_json::from_value(response_json).unwrap();

        assert_eq!(response.id, 1);
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.result.count, 0);
        assert!(response.result.data.is_empty());
    }

    #[test]
    fn test_response_structures_deserialization_with_data() {
        let response_json = json!({
            "id": 1,
            "jsonrpc": "2.0",
            "result": {
                "count": 2,
                "data": [
                    {
                        "address": "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh",
                        "amount": 0.001,
                        "confirmed_timestamp": null,
                        "created_timestamp": 1640995200000i64,
                        "currency": "BTC",
                        "fee": 0.0001,
                        "id": 12345,
                        "priority": 2.0,
                        "state": "unconfirmed",
                        "transaction_id": null,
                        "updated_timestamp": 1640995210000i64
                    },
                    {
                        "address": "0x742d35Cc6634C0532925a3b8D05c4ae5e34D7b1c",
                        "amount": 0.5,
                        "confirmed_timestamp": 1640995300000i64,
                        "created_timestamp": 1640995200000i64,
                        "currency": "ETH",
                        "fee": 0.002,
                        "id": 12346,
                        "priority": 1.0,
                        "state": "completed",
                        "transaction_id": "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
                        "updated_timestamp": 1640995400000i64
                    }
                ]
            }
        });

        let response: GetWithdrawalsResponse = serde_json::from_value(response_json).unwrap();

        assert_eq!(response.id, 1);
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.result.count, 2);
        assert_eq!(response.result.data.len(), 2);

        // Check first withdrawal
        let first_withdrawal = &response.result.data[0];
        assert_eq!(first_withdrawal.address, "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh");
        assert_eq!(first_withdrawal.amount, 0.001);
        assert_eq!(first_withdrawal.currency, "BTC");
        assert_eq!(first_withdrawal.fee, 0.0001);
        assert_eq!(first_withdrawal.id, 12345);
        assert_eq!(first_withdrawal.priority, 2.0);
        assert_eq!(first_withdrawal.state, WithdrawalState::Unconfirmed);
        assert_eq!(first_withdrawal.confirmed_timestamp, None);
        assert_eq!(first_withdrawal.transaction_id, None);

        // Check second withdrawal
        let second_withdrawal = &response.result.data[1];
        assert_eq!(second_withdrawal.address, "0x742d35Cc6634C0532925a3b8D05c4ae5e34D7b1c");
        assert_eq!(second_withdrawal.amount, 0.5);
        assert_eq!(second_withdrawal.currency, "ETH");
        assert_eq!(second_withdrawal.fee, 0.002);
        assert_eq!(second_withdrawal.id, 12346);
        assert_eq!(second_withdrawal.priority, 1.0);
        assert_eq!(second_withdrawal.state, WithdrawalState::Completed);
        assert_eq!(second_withdrawal.confirmed_timestamp, Some(1640995300000i64));
        assert_eq!(
            second_withdrawal.transaction_id,
            Some("0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef".to_string())
        );
    }

    #[test]
    fn test_all_supported_currencies() {
        let currencies = vec![
            Currency::BTC,
            Currency::ETH,
            Currency::USDC,
            Currency::USDT,
            Currency::EURR,
        ];

        for currency in currencies {
            let request = GetWithdrawalsRequest {
                currency: currency.clone(),
                count: Some(10),
                offset: Some(0),
            };

            let json_str = serde_json::to_string(&request).unwrap();
            let json_value: Value = serde_json::from_str(&json_str).unwrap();

            // Verify currency serialization
            match currency {
                Currency::BTC => assert_eq!(json_value.get("currency").unwrap(), "BTC"),
                Currency::ETH => assert_eq!(json_value.get("currency").unwrap(), "ETH"),
                Currency::USDC => assert_eq!(json_value.get("currency").unwrap(), "USDC"),
                Currency::USDT => assert_eq!(json_value.get("currency").unwrap(), "USDT"),
                Currency::EURR => assert_eq!(json_value.get("currency").unwrap(), "EURR"),
                _ => {}
            }

            assert_eq!(json_value.get("count").unwrap(), 10);
            assert_eq!(json_value.get("offset").unwrap(), 0);
        }
    }

    #[test]
    fn test_response_with_all_withdrawal_states() {
        let states = vec![
            ("unconfirmed", WithdrawalState::Unconfirmed),
            ("confirmed", WithdrawalState::Confirmed),
            ("cancelled", WithdrawalState::Cancelled),
            ("completed", WithdrawalState::Completed),
            ("interrupted", WithdrawalState::Interrupted),
            ("rejected", WithdrawalState::Rejected),
        ];

        for (state_str, expected_state) in states {
            let response_json = json!({
                "id": 1,
                "jsonrpc": "2.0",
                "result": {
                    "count": 1,
                    "data": [
                        {
                            "address": "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh",
                            "amount": 0.001,
                            "confirmed_timestamp": null,
                            "created_timestamp": 1640995200000i64,
                            "currency": "BTC",
                            "fee": 0.0001,
                            "id": 12345,
                            "priority": 2.0,
                            "state": state_str,
                            "transaction_id": null,
                            "updated_timestamp": 1640995210000i64
                        }
                    ]
                }
            });

            let response: GetWithdrawalsResponse = serde_json::from_value(response_json).unwrap();
            assert_eq!(response.result.data[0].state, expected_state);
        }
    }

    #[tokio::test]
    async fn test_get_withdrawals_method_exists() {
        // Test that the method exists and compiles without needing to call it
        let api_key = Box::new(PlainTextSecret::new("test_key".to_string())) as Box<dyn ExposableSecret>;
        let api_secret = Box::new(PlainTextSecret::new("test_secret".to_string())) as Box<dyn ExposableSecret>;
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
        let _ = RestClient::get_withdrawals;

        // Verify the client exists
        let _ = &rest_client;

        println!("get_withdrawals method is accessible and properly typed");
    }

    #[test]
    fn test_pagination_edge_cases() {
        // Test with zero count
        let request = GetWithdrawalsRequest {
            currency: Currency::BTC,
            count: Some(0),
            offset: Some(0),
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();
        assert_eq!(json_value.get("count").unwrap(), 0);

        // Test with large offset
        let request = GetWithdrawalsRequest {
            currency: Currency::ETH,
            count: Some(100),
            offset: Some(9999),
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();
        assert_eq!(json_value.get("count").unwrap(), 100);
        assert_eq!(json_value.get("offset").unwrap(), 9999);
    }

    #[test]
    fn test_json_rpc_compliance() {
        // Test that the response follows JSON-RPC 2.0 specification
        let response_json = json!({
            "id": 789,
            "jsonrpc": "2.0",
            "result": {
                "count": 1,
                "data": [
                    {
                        "address": "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh",
                        "amount": 0.001,
                        "confirmed_timestamp": null,
                        "created_timestamp": 1640995200000i64,
                        "currency": "BTC",
                        "fee": 0.0001,
                        "id": 12345,
                        "priority": 2.0,
                        "state": "unconfirmed",
                        "transaction_id": null,
                        "updated_timestamp": 1640995210000i64
                    }
                ]
            }
        });

        let response: GetWithdrawalsResponse = serde_json::from_value(response_json).unwrap();
        
        // Verify JSON-RPC 2.0 compliance
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.id, 789);
        
        // Verify result is present and correct structure
        assert_eq!(response.result.count, 1);
        assert_eq!(response.result.data.len(), 1);
    }
}