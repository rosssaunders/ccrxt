use serde::{Deserialize, Serialize};

use super::client::RestClient;
use super::submit_transfer_to_user::TransferData;
use crate::deribit::{Currency, EndpointType, RestResult};

/// Request parameters for get transfers
#[derive(Debug, Clone, Serialize)]
pub struct GetTransfersRequest {
    /// The currency symbol
    pub currency: Currency,
    /// Number of requested items, default - 10
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    /// The offset for pagination, default - 0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
}

/// Result data for get transfers endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTransfersResult {
    /// Total number of results available
    pub count: i32,
    /// Array of transfer entries
    pub data: Vec<TransferData>,
}

/// Response for get transfers endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTransfersResponse {
    /// The id that was sent in the request
    pub id: i64,
    /// The JSON-RPC version (2.0)
    pub jsonrpc: String,
    /// Transfer result data
    pub result: GetTransfersResult,
}

impl RestClient {
    /// Retrieve the user's transfers list
    ///
    /// This endpoint retrieves transfer history for the authenticated user.
    /// The endpoint requires wallet:read scope.
    ///
    /// See: <https://docs.deribit.com/v2/#private-get_transfers>
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
    /// Result containing transfer history with count and data array
    pub async fn get_transfers(&self, currency: Currency, count: Option<i32>, offset: Option<i32>) -> RestResult<GetTransfersResponse> {
        let request = GetTransfersRequest {
            currency,
            count,
            offset,
        };
        self.send_signed_request(
            "private/get_transfers",
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
    fn test_request_parameters_serialization_required_only() {
        let request = GetTransfersRequest {
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
        let request = GetTransfersRequest {
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
        let request = GetTransfersRequest {
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
        let request = GetTransfersRequest {
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

        let response: GetTransfersResponse = serde_json::from_value(response_json).unwrap();

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
                        "amount": 0.001,
                        "created_timestamp": 1640995200000i64,
                        "currency": "BTC",
                        "direction": "payment",
                        "id": 12345,
                        "other_side": "subaccount_01",
                        "state": "confirmed",
                        "type": "subaccount",
                        "updated_timestamp": 1640995210000i64
                    },
                    {
                        "amount": 0.5,
                        "created_timestamp": 1640995300000i64,
                        "currency": "ETH",
                        "direction": "payment",
                        "id": 12346,
                        "other_side": "user_wallet_address",
                        "state": "prepared",
                        "type": "user",
                        "updated_timestamp": 1640995400000i64
                    }
                ]
            }
        });

        let response: GetTransfersResponse = serde_json::from_value(response_json).unwrap();

        assert_eq!(response.id, 1);
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.result.count, 2);
        assert_eq!(response.result.data.len(), 2);

        // Check first transfer
        let first_transfer = &response.result.data[0];
        assert_eq!(first_transfer.amount, 0.001);
        assert_eq!(first_transfer.created_timestamp, 1640995200000);
        assert_eq!(first_transfer.currency, "BTC");
        assert_eq!(first_transfer.direction, "payment");
        assert_eq!(first_transfer.id, 12345);
        assert_eq!(first_transfer.other_side, "subaccount_01");
        assert_eq!(first_transfer.state, "confirmed");
        assert_eq!(first_transfer.transfer_type, "subaccount");
        assert_eq!(first_transfer.updated_timestamp, 1640995210000);

        // Check second transfer
        let second_transfer = &response.result.data[1];
        assert_eq!(second_transfer.amount, 0.5);
        assert_eq!(second_transfer.created_timestamp, 1640995300000);
        assert_eq!(second_transfer.currency, "ETH");
        assert_eq!(second_transfer.direction, "payment");
        assert_eq!(second_transfer.id, 12346);
        assert_eq!(second_transfer.other_side, "user_wallet_address");
        assert_eq!(second_transfer.state, "prepared");
        assert_eq!(second_transfer.transfer_type, "user");
        assert_eq!(second_transfer.updated_timestamp, 1640995400000);
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
            let request = GetTransfersRequest {
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
    fn test_transfer_states() {
        let states = vec![
            "prepared",
            "confirmed",
            "cancelled",
            "waiting_for_admin",
            "insufficient_funds",
            "withdrawal_limit",
        ];

        for state in states {
            let response_json = json!({
                "id": 1,
                "jsonrpc": "2.0",
                "result": {
                    "count": 1,
                    "data": [
                        {
                            "amount": 0.001,
                            "created_timestamp": 1640995200000i64,
                            "currency": "BTC",
                            "direction": "payment",
                            "id": 12345,
                            "other_side": "subaccount_01",
                            "state": state,
                            "type": "subaccount",
                            "updated_timestamp": 1640995210000i64
                        }
                    ]
                }
            });

            let response: GetTransfersResponse = serde_json::from_value(response_json).unwrap();
            assert_eq!(response.result.data[0].state, state);
        }
    }

    #[test]
    fn test_transfer_types() {
        let types = vec![("user", "user"), ("subaccount", "subaccount")];

        for (type_value, expected) in types {
            let response_json = json!({
                "id": 1,
                "jsonrpc": "2.0",
                "result": {
                    "count": 1,
                    "data": [
                        {
                            "amount": 0.001,
                            "created_timestamp": 1640995200000i64,
                            "currency": "BTC",
                            "direction": "payment",
                            "id": 12345,
                            "other_side": "test_destination",
                            "state": "confirmed",
                            "type": type_value,
                            "updated_timestamp": 1640995210000i64
                        }
                    ]
                }
            });

            let response: GetTransfersResponse = serde_json::from_value(response_json).unwrap();
            assert_eq!(response.result.data[0].transfer_type, expected);
        }
    }

    #[tokio::test]
    async fn test_get_transfers_method_exists() {
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
        let _ = RestClient::get_transfers;

        // Verify the client exists
        let _ = &rest_client;

        println!("get_transfers method is accessible and properly typed");
    }

    #[test]
    fn test_pagination_edge_cases() {
        // Test with zero count
        let request = GetTransfersRequest {
            currency: Currency::BTC,
            count: Some(0),
            offset: Some(0),
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();
        assert_eq!(json_value.get("count").unwrap(), 0);

        // Test with large offset
        let request = GetTransfersRequest {
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
                        "amount": 0.001,
                        "created_timestamp": 1640995200000i64,
                        "currency": "BTC",
                        "direction": "payment",
                        "id": 12345,
                        "other_side": "subaccount_01",
                        "state": "confirmed",
                        "type": "subaccount",
                        "updated_timestamp": 1640995210000i64
                    }
                ]
            }
        });

        let response: GetTransfersResponse = serde_json::from_value(response_json).unwrap();

        // Verify JSON-RPC 2.0 compliance
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.id, 789);

        // Verify result is present and correct structure
        assert_eq!(response.result.count, 1);
        assert_eq!(response.result.data.len(), 1);
    }
}
