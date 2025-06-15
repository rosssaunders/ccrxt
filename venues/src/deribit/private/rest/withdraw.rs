use super::client::RestClient;
use crate::deribit::RestResult;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

/// Request parameters for private/withdraw endpoint
#[derive(Debug, Clone, Serialize)]
pub struct WithdrawRequest {
    /// The currency symbol (BTC, ETH, USDC, USDT, EURR)
    pub currency: String,
    /// Address in currency format, it must be in address book
    pub address: String,
    /// Amount of funds to be withdrawn
    pub amount: f64,
    /// Withdrawal priority (optional for BTC, default: high)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,
}

/// Response for private/withdraw endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WithdrawResponse {
    /// The id that was sent in the request
    pub id: u64,
    /// The JSON-RPC version (2.0)
    pub jsonrpc: String,
    /// The withdrawal result data
    pub result: WithdrawResult,
}

/// Withdrawal result data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WithdrawResult {
    /// Address in proper format for currency
    pub address: String,
    /// Amount of funds in given currency
    pub amount: f64,
    /// The timestamp (milliseconds since the Unix epoch) of withdrawal confirmation, null when not confirmed
    pub confirmed_timestamp: Option<u64>,
    /// The timestamp (milliseconds since the Unix epoch)
    pub created_timestamp: u64,
    /// Currency, i.e "BTC", "ETH", "USDC"
    pub currency: String,
    /// Fee in currency
    pub fee: f64,
    /// Withdrawal id in Deribit system
    pub id: u64,
    /// Id of priority level
    pub priority: f64,
    /// Withdrawal state
    pub state: String,
    /// Transaction id in proper format for currency, null if id is not available
    pub transaction_id: Option<String>,
    /// The timestamp (milliseconds since the Unix epoch)
    pub updated_timestamp: u64,
}

impl RestClient {
    /// Creates a new withdrawal request
    ///
    /// This is a private method; it can only be used after authentication.
    /// Requires scope: `wallet:read_write` and mainaccount
    ///
    /// # Arguments
    /// * `currency` - The currency symbol (BTC, ETH, USDC, USDT, EURR)
    /// * `address` - Address in currency format, it must be in address book
    /// * `amount` - Amount of funds to be withdrawn
    /// * `priority` - Withdrawal priority (optional for BTC, default: high)
    ///               Valid values: insane, extreme_high, very_high, high, mid, low, very_low
    ///
    /// # Returns
    /// A result containing the withdrawal response or an error
    pub async fn withdraw(
        &self,
        currency: &str,
        address: &str,
        amount: f64,
        priority: Option<&str>,
    ) -> RestResult<WithdrawResult> {
        let mut params = json!({
            "currency": currency,
            "address": address,
            "amount": amount
        });

        if let Some(p) = priority {
            params["priority"] = Value::String(p.to_string());
        }

        self.send_private_request("private/withdraw", params).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_withdraw_request_structure() {
        let request = WithdrawRequest {
            currency: "BTC".to_string(),
            address: "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa".to_string(),
            amount: 0.001,
            priority: Some("high".to_string()),
        };

        let json_value = serde_json::to_value(&request).unwrap();
        assert_eq!(json_value.get("currency").unwrap(), "BTC");
        assert_eq!(json_value.get("address").unwrap(), "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa");
        assert_eq!(json_value.get("amount").unwrap(), 0.001);
        assert_eq!(json_value.get("priority").unwrap(), "high");
    }

    #[test]
    fn test_withdraw_request_without_priority() {
        let request = WithdrawRequest {
            currency: "ETH".to_string(),
            address: "0x742d35Cc6634C0532925a3b8D43C67C3b4BF9B7E".to_string(),
            amount: 0.1,
            priority: None,
        };

        let json_value = serde_json::to_value(&request).unwrap();
        assert_eq!(json_value.get("currency").unwrap(), "ETH");
        assert_eq!(json_value.get("address").unwrap(), "0x742d35Cc6634C0532925a3b8D43C67C3b4BF9B7E");
        assert_eq!(json_value.get("amount").unwrap(), 0.1);
        assert!(json_value.get("priority").is_none());
    }

    #[test]
    fn test_withdraw_result_deserialization() {
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

        let result: WithdrawResult = serde_json::from_value(result_json).unwrap();
        assert_eq!(result.address, "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa");
        assert_eq!(result.amount, 0.001);
        assert_eq!(result.confirmed_timestamp, None);
        assert_eq!(result.created_timestamp, 1609459200000);
        assert_eq!(result.currency, "BTC");
        assert_eq!(result.fee, 0.0005);
        assert_eq!(result.id, 12345);
        assert_eq!(result.priority, 2.0);
        assert_eq!(result.state, "unconfirmed");
        assert_eq!(result.transaction_id, None);
        assert_eq!(result.updated_timestamp, 1609459200000);
    }

    #[test]
    fn test_withdraw_result_with_confirmation() {
        let result_json = json!({
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
        });

        let result: WithdrawResult = serde_json::from_value(result_json).unwrap();
        assert_eq!(result.address, "bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4");
        assert_eq!(result.amount, 0.005);
        assert_eq!(result.confirmed_timestamp, Some(1609462800000));
        assert_eq!(result.created_timestamp, 1609459200000);
        assert_eq!(result.currency, "BTC");
        assert_eq!(result.fee, 0.0002);
        assert_eq!(result.id, 67890);
        assert_eq!(result.priority, 3.0);
        assert_eq!(result.state, "completed");
        assert_eq!(result.transaction_id, Some("a1b2c3d4e5f6789012345678901234567890123456789012345678901234567890".to_string()));
        assert_eq!(result.updated_timestamp, 1609462800000);
    }

    #[test]
    fn test_withdraw_response_structure() {
        let response_json = json!({
            "id": 1,
            "jsonrpc": "2.0",
            "result": {
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
            }
        });

        let response: WithdrawResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.id, 1);
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.result.address, "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa");
        assert_eq!(response.result.currency, "BTC");
        assert_eq!(response.result.state, "unconfirmed");
    }

    #[test]
    fn test_withdrawal_states() {
        let states = vec![
            "unconfirmed",
            "confirmed", 
            "cancelled",
            "completed",
            "interrupted",
            "rejected"
        ];

        for state in states {
            let result_json = json!({
                "address": "test_address",
                "amount": 1.0,
                "confirmed_timestamp": null,
                "created_timestamp": 1609459200000_u64,
                "currency": "BTC",
                "fee": 0.0005,
                "id": 12345,
                "priority": 2.0,
                "state": state,
                "transaction_id": null,
                "updated_timestamp": 1609459200000_u64
            });

            let result: WithdrawResult = serde_json::from_value(result_json).unwrap();
            assert_eq!(result.state, state);
        }
    }

    #[test]
    fn test_supported_currencies() {
        let currencies = vec!["BTC", "ETH", "USDC", "USDT", "EURR"];

        for currency in currencies {
            let request = WithdrawRequest {
                currency: currency.to_string(),
                address: "test_address".to_string(),
                amount: 1.0,
                priority: None,
            };

            let json_value = serde_json::to_value(&request).unwrap();
            assert_eq!(json_value.get("currency").unwrap(), currency);
        }
    }

    #[test]
    fn test_priority_levels() {
        let priorities = vec![
            "insane",
            "extreme_high", 
            "very_high",
            "high",
            "mid",
            "low",
            "very_low"
        ];

        for priority in priorities {
            let request = WithdrawRequest {
                currency: "BTC".to_string(),
                address: "test_address".to_string(),
                amount: 1.0,
                priority: Some(priority.to_string()),
            };

            let json_value = serde_json::to_value(&request).unwrap();
            assert_eq!(json_value.get("priority").unwrap(), priority);
        }
    }
}