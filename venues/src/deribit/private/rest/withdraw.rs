use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::deribit::{Currency, EndpointType, RestResult, WithdrawalPriority, WithdrawalState};

/// Request parameters for withdrawal
#[derive(Debug, Clone, Serialize)]
pub struct WithdrawRequest {
    /// The currency symbol
    pub currency: Currency,
    /// Address in currency format, must be in address book
    pub address: String,
    /// Amount of funds to be withdrawn
    pub amount: f64,
    /// Withdrawal priority, optional for BTC, default: high
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<WithdrawalPriority>,
}

/// Withdrawal data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WithdrawalData {
    /// Address in proper format for currency
    pub address: String,
    /// Amount of funds in given currency
    pub amount: f64,
    /// The timestamp (milliseconds since the Unix epoch) of withdrawal confirmation, null when not confirmed
    pub confirmed_timestamp: Option<i64>,
    /// The timestamp (milliseconds since the Unix epoch)
    pub created_timestamp: i64,
    /// Currency, i.e "BTC", "ETH", "USDC"
    pub currency: String,
    /// Fee in currency
    pub fee: f64,
    /// Withdrawal id in Deribit system
    pub id: i64,
    /// Id of priority level
    pub priority: f64,
    /// Withdrawal state
    pub state: WithdrawalState,
    /// Transaction id in proper format for currency, null if id is not available
    pub transaction_id: Option<String>,
    /// The timestamp (milliseconds since the Unix epoch)
    pub updated_timestamp: i64,
}

/// Response for withdrawal endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WithdrawResponse {
    /// The id that was sent in the request
    pub id: i64,
    /// The JSON-RPC version (2.0)
    pub jsonrpc: String,
    /// Withdrawal result data
    pub result: WithdrawalData,
}

impl RestClient {
    /// Creates a new withdrawal request
    ///
    /// Creates a new withdrawal request for transferring funds to an external address.
    /// The address must be in the address book and the request requires wallet:read_write scope.
    ///
    /// See: <https://docs.deribit.com/v2/#private-withdraw>
    ///
    /// Rate limit: 500 credits per request (non-matching engine)
    /// Scope: wallet:read_write and mainaccount
    ///
    /// # Arguments
    /// * `currency` - The currency symbol (BTC, ETH, USDC, USDT, EURR)
    /// * `address` - Address in currency format, must be in address book
    /// * `amount` - Amount of funds to be withdrawn
    /// * `priority` - Withdrawal priority (optional for BTC, default: high)
    ///
    /// # Returns
    /// Withdrawal result with complete withdrawal information
    pub async fn withdraw(&self, currency: Currency, address: &str, amount: f64, priority: Option<WithdrawalPriority>) -> RestResult<WithdrawResponse> {
        let request = WithdrawRequest {
            currency,
            address: address.to_string(),
            amount,
            priority,
        };
        self.send_signed_request(
            "private/withdraw",
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
        let request = WithdrawRequest {
            currency: Currency::BTC,
            address: "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh".to_string(),
            amount: 0.001,
            priority: Some(WithdrawalPriority::High),
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("currency").unwrap(), "BTC");
        assert_eq!(
            json_value.get("address").unwrap(),
            "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh"
        );
        assert_eq!(json_value.get("amount").unwrap(), 0.001);
        assert_eq!(json_value.get("priority").unwrap(), "high");
    }

    #[test]
    fn test_request_parameters_serialization_without_priority() {
        let request = WithdrawRequest {
            currency: Currency::ETH,
            address: "0x742d35Cc6634C0532925a3b8D05c4ae5e34D7b1c".to_string(),
            amount: 0.5,
            priority: None,
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("currency").unwrap(), "ETH");
        assert_eq!(
            json_value.get("address").unwrap(),
            "0x742d35Cc6634C0532925a3b8D05c4ae5e34D7b1c"
        );
        assert_eq!(json_value.get("amount").unwrap(), 0.5);
        assert!(json_value.get("priority").is_none());
    }

    #[test]
    fn test_response_structures_deserialization() {
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
                "state": "unconfirmed",
                "transaction_id": null,
                "updated_timestamp": 1640995210000i64
            }
        });

        let response: WithdrawResponse = serde_json::from_value(response_json).unwrap();

        assert_eq!(response.id, 1);
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(
            response.result.address,
            "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh"
        );
        assert_eq!(response.result.amount, 0.001);
        assert_eq!(response.result.currency, "BTC");
        assert_eq!(response.result.fee, 0.0001);
        assert_eq!(response.result.state, WithdrawalState::Unconfirmed);
        assert_eq!(response.result.confirmed_timestamp, None);
        assert_eq!(response.result.transaction_id, None);
    }

    #[test]
    fn test_response_with_confirmed_withdrawal() {
        let response_json = json!({
            "id": 1,
            "jsonrpc": "2.0",
            "result": {
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
        });

        let response: WithdrawResponse = serde_json::from_value(response_json).unwrap();

        assert_eq!(response.result.currency, "ETH");
        assert_eq!(response.result.state, WithdrawalState::Completed);
        assert_eq!(response.result.confirmed_timestamp, Some(1640995300000i64));
        assert_eq!(
            response.result.transaction_id,
            Some("0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef".to_string())
        );
    }

    #[tokio::test]
    async fn test_withdraw_method_exists() {
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
        let _ = RestClient::withdraw;

        // Verify the client exists
        let _ = &rest_client;

        println!("withdraw method is accessible and properly typed");
    }
}
