use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::deribit::{ClearanceState, Currency, DepositState, EndpointType, RestResult};

/// Deposit entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepositData {
    /// Address in proper format for currency
    pub address: String,
    /// Amount of funds in given currency
    pub amount: f64,
    /// Clearance state
    pub clearance_state: ClearanceState,
    /// Currency, i.e "BTC", "ETH", "USDC"
    pub currency: String,
    /// Note
    pub note: String,
    /// The timestamp (milliseconds since the Unix epoch)
    pub received_timestamp: i64,
    /// Transaction id in proper format for currency, null if id is not available
    pub refund_transaction_id: Option<String>,
    /// Address in proper format for currency
    pub source_address: String,
    /// Deposit state
    pub state: DepositState,
    /// Transaction id in proper format for currency, null if id is not available
    pub transaction_id: Option<String>,
    /// The timestamp (milliseconds since the Unix epoch)
    pub updated_timestamp: i64,
}

/// Request parameters for get deposits
#[derive(Debug, Clone, Serialize)]
pub struct GetDepositsRequest {
    /// The currency symbol
    pub currency: Currency,
    /// Number of requested items, default - 10
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    /// The offset for pagination, default - 0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
}

/// Result data for get deposits endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDepositsResult {
    /// Total number of results available
    pub count: i32,
    /// Array of deposit entries
    pub data: Vec<DepositData>,
}

/// Response for get deposits endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDepositsResponse {
    /// The id that was sent in the request
    pub id: i64,
    /// The JSON-RPC version (2.0)
    pub jsonrpc: String,
    /// Deposit result data
    pub result: GetDepositsResult,
}

impl RestClient {
    /// Retrieve the latest users deposits
    ///
    /// This endpoint retrieves deposit history for the authenticated user.
    /// The endpoint requires wallet:read scope.
    ///
    /// See: <https://docs.deribit.com/v2/#private-get_deposits>
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
    /// Result containing deposit history with count and data array
    pub async fn get_deposits(&self, request: GetDepositsRequest) -> RestResult<GetDepositsResponse> {
        self.send_signed_request(
            "private/get_deposits",
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
    fn test_request_parameters_serialization_required_only() {
        let request = GetDepositsRequest {
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
        let request = GetDepositsRequest {
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
        let request = GetDepositsRequest {
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
        let request = GetDepositsRequest {
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

        let response: GetDepositsResponse = serde_json::from_value(response_json).unwrap();

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
                        "clearance_state": "success",
                        "currency": "BTC",
                        "note": "Test deposit",
                        "received_timestamp": 1640995200000i64,
                        "refund_transaction_id": null,
                        "source_address": "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa",
                        "state": "completed",
                        "transaction_id": "1a2b3c4d5e6f7890abcdef1234567890abcdef1234567890abcdef1234567890",
                        "updated_timestamp": 1640995210000i64
                    },
                    {
                        "address": "0x742d35Cc6634C0532925a3b8D05c4ae5e34D7b1c",
                        "amount": 0.5,
                        "clearance_state": "in_progress",
                        "currency": "ETH",
                        "note": "ETH deposit",
                        "received_timestamp": 1640995200000i64,
                        "refund_transaction_id": null,
                        "source_address": "0x1234567890abcdef1234567890abcdef12345678",
                        "state": "pending",
                        "transaction_id": null,
                        "updated_timestamp": 1640995400000i64
                    }
                ]
            }
        });

        let response: GetDepositsResponse = serde_json::from_value(response_json).unwrap();

        assert_eq!(response.id, 1);
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.result.count, 2);
        assert_eq!(response.result.data.len(), 2);

        // Check first deposit
        let first_deposit = &response.result.data[0];
        assert_eq!(
            first_deposit.address,
            "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh"
        );
        assert_eq!(first_deposit.amount, 0.001);
        assert_eq!(first_deposit.clearance_state, ClearanceState::Success);
        assert_eq!(first_deposit.currency, "BTC");
        assert_eq!(first_deposit.note, "Test deposit");
        assert_eq!(first_deposit.received_timestamp, 1640995200000);
        assert_eq!(first_deposit.refund_transaction_id, None);
        assert_eq!(
            first_deposit.source_address,
            "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa"
        );
        assert_eq!(first_deposit.state, DepositState::Completed);
        assert_eq!(
            first_deposit.transaction_id,
            Some("1a2b3c4d5e6f7890abcdef1234567890abcdef1234567890abcdef1234567890".to_string())
        );
        assert_eq!(first_deposit.updated_timestamp, 1640995210000);

        // Check second deposit
        let second_deposit = &response.result.data[1];
        assert_eq!(
            second_deposit.address,
            "0x742d35Cc6634C0532925a3b8D05c4ae5e34D7b1c"
        );
        assert_eq!(second_deposit.amount, 0.5);
        assert_eq!(second_deposit.clearance_state, ClearanceState::InProgress);
        assert_eq!(second_deposit.currency, "ETH");
        assert_eq!(second_deposit.note, "ETH deposit");
        assert_eq!(second_deposit.received_timestamp, 1640995200000);
        assert_eq!(second_deposit.refund_transaction_id, None);
        assert_eq!(
            second_deposit.source_address,
            "0x1234567890abcdef1234567890abcdef12345678"
        );
        assert_eq!(second_deposit.state, DepositState::Pending);
        assert_eq!(second_deposit.transaction_id, None);
        assert_eq!(second_deposit.updated_timestamp, 1640995400000);
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
            let request = GetDepositsRequest {
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
    fn test_response_with_all_deposit_states() {
        let states = vec![
            ("pending", DepositState::Pending),
            ("completed", DepositState::Completed),
            ("rejected", DepositState::Rejected),
            ("replaced", DepositState::Replaced),
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
                            "clearance_state": "success",
                            "currency": "BTC",
                            "note": "Test deposit",
                            "received_timestamp": 1640995200000i64,
                            "refund_transaction_id": null,
                            "source_address": "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa",
                            "state": state_str,
                            "transaction_id": null,
                            "updated_timestamp": 1640995210000i64
                        }
                    ]
                }
            });

            let response: GetDepositsResponse = serde_json::from_value(response_json).unwrap();
            assert_eq!(response.result.data[0].state, expected_state);
        }
    }

    #[test]
    fn test_response_with_all_clearance_states() {
        let states = vec![
            ("in_progress", ClearanceState::InProgress),
            (
                "pending_admin_decision",
                ClearanceState::PendingAdminDecision,
            ),
            ("pending_user_input", ClearanceState::PendingUserInput),
            ("success", ClearanceState::Success),
            ("failed", ClearanceState::Failed),
            ("cancelled", ClearanceState::Cancelled),
            ("refund_initiated", ClearanceState::RefundInitiated),
            ("refunded", ClearanceState::Refunded),
        ];

        for (clearance_str, expected_clearance) in states {
            let response_json = json!({
                "id": 1,
                "jsonrpc": "2.0",
                "result": {
                    "count": 1,
                    "data": [
                        {
                            "address": "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh",
                            "amount": 0.001,
                            "clearance_state": clearance_str,
                            "currency": "BTC",
                            "note": "Test deposit",
                            "received_timestamp": 1640995200000i64,
                            "refund_transaction_id": null,
                            "source_address": "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa",
                            "state": "pending",
                            "transaction_id": null,
                            "updated_timestamp": 1640995210000i64
                        }
                    ]
                }
            });

            let response: GetDepositsResponse = serde_json::from_value(response_json).unwrap();
            assert_eq!(response.result.data[0].clearance_state, expected_clearance);
        }
    }

    #[tokio::test]
    async fn test_get_deposits_method_exists() {
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
        let _ = RestClient::get_deposits;

        // Verify the client exists
        let _ = &rest_client;

        println!("get_deposits method is accessible and properly typed");
    }

    #[test]
    fn test_pagination_edge_cases() {
        // Test with zero count
        let request = GetDepositsRequest {
            currency: Currency::BTC,
            count: Some(0),
            offset: Some(0),
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();
        assert_eq!(json_value.get("count").unwrap(), 0);

        // Test with large offset
        let request = GetDepositsRequest {
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
                        "clearance_state": "success",
                        "currency": "BTC",
                        "note": "Test deposit",
                        "received_timestamp": 1640995200000i64,
                        "refund_transaction_id": null,
                        "source_address": "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa",
                        "state": "completed",
                        "transaction_id": null,
                        "updated_timestamp": 1640995210000i64
                    }
                ]
            }
        });

        let response: GetDepositsResponse = serde_json::from_value(response_json).unwrap();

        // Verify JSON-RPC 2.0 compliance
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.id, 789);

        // Verify result is present and correct structure
        assert_eq!(response.result.count, 1);
        assert_eq!(response.result.data.len(), 1);
    }
}
