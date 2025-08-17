use serde::Serialize;

use super::{RestClient, withdraw::WithdrawResponse};
use crate::deribit::{Currency, EndpointType, RestResult};

/// REST API endpoint constant
const CANCEL_WITHDRAWAL_ENDPOINT: &str = "private/cancel_withdrawal";

/// Request parameters for canceling a withdrawal
#[derive(Debug, Clone, Serialize)]
pub struct CancelWithdrawalRequest {
    /// The currency symbol
    pub currency: Currency,

    /// The withdrawal id
    pub id: i64,
}

/// Response for cancel withdrawal endpoint (reuses WithdrawResponse)
pub type CancelWithdrawalResponse = WithdrawResponse;

impl RestClient {
    /// Cancels withdrawal request
    ///
    /// Cancels a pending withdrawal request identified by the withdrawal ID.
    /// This is a private method; it can only be used after authentication.
    ///
    /// [docs](https://docs.deribit.com/v2/#private-cancel_withdrawal)
    ///
    /// Rate limit: Non-matching engine endpoint
    /// Scope: wallet:read_write
    ///
    /// # Arguments
    /// * `currency` - The currency symbol (BTC, ETH, USDC, USDT, EURR)
    /// * `id` - The withdrawal id
    ///
    /// # Returns
    /// Cancel result with complete withdrawal information
    pub async fn cancel_withdrawal(
        &self,
        request: CancelWithdrawalRequest,
    ) -> RestResult<CancelWithdrawalResponse> {
        self.send_signed_request(
            CANCEL_WITHDRAWAL_ENDPOINT,
            &request,
            EndpointType::NonMatchingEngine,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use rest::secrets::SecretString;
    /// REST API endpoint constant
    use serde_json::{Value, json};

    use super::*;
    use crate::deribit::{AccountTier, WithdrawalState, private::rest::credentials::Credentials};

    #[test]
    fn test_cancel_withdrawal_request_serialization() {
        let request = CancelWithdrawalRequest {
            currency: Currency::BTC,
            id: 12345,
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("currency").unwrap(), "BTC");
        assert_eq!(json_value.get("id").unwrap(), 12345);
    }

    #[test]
    fn test_cancel_withdrawal_request_different_currencies() {
        let currencies = vec![
            Currency::BTC,
            Currency::ETH,
            Currency::USDC,
            Currency::USDT,
            Currency::EURR,
        ];

        for currency in currencies {
            let request = CancelWithdrawalRequest {
                currency: currency.clone(),
                id: 54321,
            };

            let json_str = serde_json::to_string(&request).unwrap();
            let json_value: Value = serde_json::from_str(&json_str).unwrap();

            assert_eq!(json_value.get("currency").unwrap(), &currency.to_string());
            assert_eq!(json_value.get("id").unwrap(), 54321);
        }
    }

    #[test]
    fn test_cancel_withdrawal_response_deserialization() {
        let response_json = json!({
            "id": 1,
            "jsonrpc": "2.0",
            "result": {
                "address": "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh",
                "amount": 0.001,
                "confirmed_timestamp": null,
                "created_timestamp": 1640995200000i64,
                "currency": "BTC",
                "fee": 0.0001,
                "id": 12345,
                "priority": 2.0,
                "state": "cancelled",
                "transaction_id": null,
                "updated_timestamp": 1640995300000i64
            }
        });

        let response: CancelWithdrawalResponse = serde_json::from_value(response_json).unwrap();

        assert_eq!(response.id, 1);
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(
            response.result.address,
            "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh"
        );
        assert_eq!(response.result.amount, 0.001);
        assert_eq!(response.result.currency, "BTC");
        assert_eq!(response.result.fee, 0.0001);
        assert_eq!(response.result.id, 12345);
        assert_eq!(response.result.state, WithdrawalState::Cancelled);
        assert_eq!(response.result.confirmed_timestamp, None);
        assert_eq!(response.result.transaction_id, None);
        // Verify updated_timestamp is later than created_timestamp
        assert!(response.result.updated_timestamp > response.result.created_timestamp);
    }

    #[test]
    fn test_cancel_withdrawal_response_with_different_states() {
        // Test various withdrawal states that might be returned
        let states = vec![
            ("cancelled", WithdrawalState::Cancelled),
            ("unconfirmed", WithdrawalState::Unconfirmed),
            ("confirmed", WithdrawalState::Confirmed),
            ("completed", WithdrawalState::Completed),
            ("interrupted", WithdrawalState::Interrupted),
            ("rejected", WithdrawalState::Rejected),
        ];

        for (state_str, expected_state) in states {
            let response_json = json!({
                "id": 1,
                "jsonrpc": "2.0",
                "result": {
                    "address": "0x742d35Cc6634C0532925a3b8D05c4ae5e34D7b1c",
                    "amount": 0.5,
                    "confirmed_timestamp": null,
                    "created_timestamp": 1640995200000i64,
                    "currency": "ETH",
                    "fee": 0.002,
                    "id": 67890,
                    "priority": 1.0,
                    "state": state_str,
                    "transaction_id": null,
                    "updated_timestamp": 1640995300000i64
                }
            });

            let response: CancelWithdrawalResponse = serde_json::from_value(response_json).unwrap();
            assert_eq!(response.result.state, expected_state);
        }
    }

    #[tokio::test]
    async fn test_cancel_withdrawal_method_exists() {
        // Test that the method exists and compiles without needing to call it
        let credentials = Credentials {
            api_key: SecretString::from("test_key".to_string()),
            api_secret: SecretString::from("test_secret".to_string()),
        };
        let http_client = Arc::new(rest::native::NativeHttpClient::default());
        let rate_limiter = crate::deribit::RateLimiter::new(AccountTier::Tier4);

        let rest_client = RestClient::new(
            credentials,
            "https://test.deribit.com",
            rate_limiter,
            http_client,
        );

        // Test that we can get a function reference to the method
        let _ = RestClient::cancel_withdrawal;

        // Verify the client exists
        let _ = &rest_client;

        println!("cancel_withdrawal method is accessible and properly typed");
    }

    #[test]
    fn test_large_withdrawal_ids() {
        // Test with large withdrawal IDs to ensure proper handling
        let large_id = 9223372036854775807i64; // i64::MAX
        let request = CancelWithdrawalRequest {
            currency: Currency::USDC,
            id: large_id,
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("currency").unwrap(), "USDC");
        assert_eq!(json_value.get("id").unwrap(), large_id);
    }

    #[test]
    fn test_json_rpc_compliance() {
        // Verify the response structure follows JSON-RPC 2.0 specification
        let response_json = json!({
            "id": 42,
            "jsonrpc": "2.0",
            "result": {
                "address": "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh",
                "amount": 0.001,
                "confirmed_timestamp": null,
                "created_timestamp": 1640995200000i64,
                "currency": "BTC",
                "fee": 0.0001,
                "id": 12345,
                "priority": 2.0,
                "state": "cancelled",
                "transaction_id": null,
                "updated_timestamp": 1640995300000i64
            }
        });

        let response: CancelWithdrawalResponse = serde_json::from_value(response_json).unwrap();

        // Verify JSON-RPC compliance
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.id, 42);

        // Verify result structure
        assert!(!response.result.address.is_empty());
        assert!(response.result.amount > 0.0);
        assert!(!response.result.currency.is_empty());
        assert!(response.result.id > 0);
    }
}
