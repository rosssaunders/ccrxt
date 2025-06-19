use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::deribit::{Currency, EndpointType, RestResult};

// Reuse existing TransferData from submit_transfer_to_user.rs
use super::submit_transfer_to_user::TransferData;

/// Request parameters for canceling a transfer
#[derive(Debug, Clone, Serialize)]
pub struct CancelTransferByIdRequest {
    /// The currency symbol
    pub currency: Currency,
    /// Id of transfer
    pub id: i64,
}

/// Response for cancel transfer by id endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelTransferByIdResponse {
    /// The id that was sent in the request
    pub id: i64,
    /// The JSON-RPC version (2.0)
    pub jsonrpc: String,
    /// Transfer result data
    pub result: TransferData,
}

impl RestClient {
    /// Cancel transfer
    ///
    /// Cancels a transfer request identified by the transfer ID.
    /// This is a private method; it can only be used after authentication.
    ///
    /// See: <https://docs.deribit.com/v2/#private-cancel_transfer_by_id>
    ///
    /// Rate limit: 500 credits per request (non-matching engine)
    /// Scope: wallet:read_write
    ///
    /// # Arguments
    /// * `currency` - The currency symbol (BTC, ETH, USDC, USDT, EURR)
    /// * `id` - Id of transfer
    ///
    /// # Returns
    /// Cancel result with complete transfer information
    pub async fn cancel_transfer_by_id(&self, currency: Currency, id: i64) -> RestResult<CancelTransferByIdResponse> {
        let request = CancelTransferByIdRequest {
            currency,
            id,
        };
        self.send_signed_request(
            "private/cancel_transfer_by_id",
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
    fn test_request_parameters_serialization() {
        let request = CancelTransferByIdRequest {
            currency: Currency::BTC,
            id: 12345,
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("currency").unwrap(), "BTC");
        assert_eq!(json_value.get("id").unwrap(), 12345);
    }

    #[test]
    fn test_request_parameters_all_currencies() {
        let currencies = vec![
            (Currency::BTC, "BTC"),
            (Currency::ETH, "ETH"),
            (Currency::USDC, "USDC"),
            (Currency::USDT, "USDT"),
            (Currency::EURR, "EURR"),
        ];

        for (currency, expected_str) in currencies {
            let request = CancelTransferByIdRequest {
                currency,
                id: 12345,
            };

            let json_str = serde_json::to_string(&request).unwrap();
            let json_value: Value = serde_json::from_str(&json_str).unwrap();

            assert_eq!(json_value.get("currency").unwrap(), expected_str);
            assert_eq!(json_value.get("id").unwrap(), 12345);
        }
    }

    #[test]
    fn test_response_deserialization_cancelled_state() {
        let response_json = json!({
            "id": 1,
            "jsonrpc": "2.0",
            "result": {
                "amount": 0.001,
                "created_timestamp": 1640995200000i64,
                "currency": "BTC",
                "direction": "payment",
                "id": 12345,
                "other_side": "subaccount_01",
                "state": "cancelled",
                "type": "subaccount",
                "updated_timestamp": 1640995300000i64
            }
        });

        let response: CancelTransferByIdResponse = serde_json::from_value(response_json).unwrap();

        assert_eq!(response.id, 1);
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.result.amount, 0.001);
        assert_eq!(response.result.created_timestamp, 1640995200000);
        assert_eq!(response.result.currency, "BTC");
        assert_eq!(response.result.direction, "payment");
        assert_eq!(response.result.id, 12345);
        assert_eq!(response.result.other_side, "subaccount_01");
        assert_eq!(response.result.state, "cancelled");
        assert_eq!(response.result.transfer_type, "subaccount");
        assert_eq!(response.result.updated_timestamp, 1640995300000);
        // Verify updated_timestamp is later than created_timestamp
        assert!(response.result.updated_timestamp > response.result.created_timestamp);
    }

    #[test]
    fn test_response_deserialization_user_transfer() {
        let response_json = json!({
            "id": 2,
            "jsonrpc": "2.0",
            "result": {
                "amount": 0.5,
                "created_timestamp": 1640995300000i64,
                "currency": "ETH",
                "direction": "payment",
                "id": 67890,
                "other_side": "user_wallet_address",
                "state": "cancelled",
                "type": "user",
                "updated_timestamp": 1640995400000i64
            }
        });

        let response: CancelTransferByIdResponse = serde_json::from_value(response_json).unwrap();

        assert_eq!(response.id, 2);
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.result.amount, 0.5);
        assert_eq!(response.result.currency, "ETH");
        assert_eq!(response.result.direction, "payment");
        assert_eq!(response.result.id, 67890);
        assert_eq!(response.result.other_side, "user_wallet_address");
        assert_eq!(response.result.state, "cancelled");
        assert_eq!(response.result.transfer_type, "user");
    }

    #[test]
    fn test_response_with_different_states() {
        let states = vec![
            "prepared",
            "confirmed", 
            "cancelled",
            "waiting_for_admin",
            "insufficient_funds",
            "withdrawal_limit"
        ];

        for state_str in states {
            let response_json = json!({
                "id": 1,
                "jsonrpc": "2.0",
                "result": {
                    "amount": 0.001,
                    "created_timestamp": 1640995200000i64,
                    "currency": "BTC",
                    "direction": "payment",
                    "id": 12345,
                    "other_side": "subaccount_01",
                    "state": state_str,
                    "type": "subaccount",
                    "updated_timestamp": 1640995300000i64
                }
            });

            let response: CancelTransferByIdResponse = serde_json::from_value(response_json).unwrap();
            assert_eq!(response.result.state, state_str);
        }
    }

    #[test]
    fn test_response_with_different_currencies() {
        let currencies = vec!["BTC", "ETH", "USDC", "USDT", "EURR"];

        for currency in currencies {
            let response_json = json!({
                "id": 1,
                "jsonrpc": "2.0",
                "result": {
                    "amount": 0.001,
                    "created_timestamp": 1640995200000i64,
                    "currency": currency,
                    "direction": "payment",
                    "id": 12345,
                    "other_side": "subaccount_01",
                    "state": "cancelled",
                    "type": "subaccount",
                    "updated_timestamp": 1640995300000i64
                }
            });

            let response: CancelTransferByIdResponse = serde_json::from_value(response_json).unwrap();
            assert_eq!(response.result.currency, currency);
        }
    }

    #[test]
    fn test_response_with_different_types() {
        let types = vec![
            ("user", "user"),
            ("subaccount", "subaccount"),
        ];

        for (type_value, expected) in types {
            let response_json = json!({
                "id": 1,
                "jsonrpc": "2.0",
                "result": {
                    "amount": 0.001,
                    "created_timestamp": 1640995200000i64,
                    "currency": "BTC",
                    "direction": "payment",
                    "id": 12345,
                    "other_side": "test_destination",
                    "state": "cancelled",
                    "type": type_value,
                    "updated_timestamp": 1640995300000i64
                }
            });

            let response: CancelTransferByIdResponse = serde_json::from_value(response_json).unwrap();
            assert_eq!(response.result.transfer_type, expected);
        }
    }

    #[tokio::test]
    async fn test_cancel_transfer_by_id_method_exists() {
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
        let _ = RestClient::cancel_transfer_by_id;

        // Verify the client exists
        let _ = &rest_client;

        println!("cancel_transfer_by_id method is accessible and properly typed");
    }

    #[test]
    fn test_edge_case_large_ids() {
        // Test with large transfer IDs
        let request = CancelTransferByIdRequest {
            currency: Currency::BTC,
            id: i64::MAX,
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("currency").unwrap(), "BTC");
        assert_eq!(json_value.get("id").unwrap(), i64::MAX);
    }

    #[test]
    fn test_json_rpc_compliance() {
        // Test that the response follows JSON-RPC 2.0 specification
        let response_json = json!({
            "id": 789,
            "jsonrpc": "2.0",
            "result": {
                "amount": 0.001,
                "created_timestamp": 1640995200000i64,
                "currency": "BTC",
                "direction": "payment",
                "id": 12345,
                "other_side": "subaccount_01",
                "state": "cancelled",
                "type": "subaccount",
                "updated_timestamp": 1640995300000i64
            }
        });

        let response: CancelTransferByIdResponse = serde_json::from_value(response_json).unwrap();
        
        // Verify JSON-RPC 2.0 compliance
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.id, 789);
        
        // Verify result is present and correct structure
        assert_eq!(response.result.amount, 0.001);
        assert_eq!(response.result.id, 12345);
    }
}