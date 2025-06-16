use super::client::RestClient;
use crate::bitmart::RestResult;
use crate::bitmart::rate_limit::EndpointType;
use serde::{Deserialize, Serialize};

/// Request parameters for getting account balance
#[derive(Debug, Serialize, Default)]
pub struct GetAccountBalanceRequest {
    /// Currency filter (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Whether to return USD valuation (optional)
    #[serde(rename = "needUsdValuation", skip_serializing_if = "Option::is_none")]
    pub need_usd_valuation: Option<bool>,
}

/// Wallet balance information for a specific currency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletBalance {
    /// Token symbol, e.g., 'BTC'
    pub currency: String,
    /// Token name, e.g., 'Bitcoin'
    pub name: String,
    /// Available balance
    pub available: String,
    /// Available balance USD valuation (only present if needUsdValuation=true)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_usd_valuation: Option<String>,
    /// Trading frozen balance
    pub frozen: String,
    /// Trading frozen balance + Other frozen balance
    #[serde(rename = "unAvailable")]
    pub unavailable: String,
}

/// Response for account balance endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAccountBalanceResponse {
    /// Array of wallet balance data
    pub wallet: Vec<WalletBalance>,
}

impl RestClient {
    /// Get account balance
    ///
    /// Gets the user's wallet balance. Only assets with a balance greater than 0 will be returned.
    ///
    /// See: https://raw.githubusercontent.com/rosssaunders/coincise/refs/heads/main/docs/bitmart/spot/funding_account.md
    ///
    /// Rate limit: 12 times/2 sec per API key
    ///
    /// # Arguments
    /// * `request` - The request parameters
    ///
    /// # Returns
    /// Account balance information
    pub async fn get_account_balance(&self, request: GetAccountBalanceRequest) -> RestResult<GetAccountBalanceResponse> {
        self.send_request(
            "/account/v1/wallet",
            reqwest::Method::GET,
            Some(&request),
            EndpointType::FundingAccount,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_account_balance_request_default() {
        let request = GetAccountBalanceRequest::default();
        assert!(request.currency.is_none());
        assert!(request.need_usd_valuation.is_none());
    }

    #[test]
    fn test_get_account_balance_request_with_currency() {
        let request = GetAccountBalanceRequest {
            currency: Some("USDT".to_string()),
            need_usd_valuation: None,
        };
        assert_eq!(request.currency, Some("USDT".to_string()));
        assert!(request.need_usd_valuation.is_none());
    }

    #[test]
    fn test_get_account_balance_request_with_usd_valuation() {
        let request = GetAccountBalanceRequest {
            currency: None,
            need_usd_valuation: Some(true),
        };
        assert!(request.currency.is_none());
        assert_eq!(request.need_usd_valuation, Some(true));
    }

    #[test]
    fn test_wallet_balance_structure() {
        let balance = WalletBalance {
            currency: "USDT".to_string(),
            name: "Tether USD".to_string(),
            available: "1000.00000000".to_string(),
            available_usd_valuation: Some("1002.00000000".to_string()),
            frozen: "0.00000000".to_string(),
            unavailable: "0.00000000".to_string(),
        };

        assert_eq!(balance.currency, "USDT");
        assert_eq!(balance.name, "Tether USD");
        assert_eq!(balance.available, "1000.00000000");
        assert_eq!(
            balance.available_usd_valuation,
            Some("1002.00000000".to_string())
        );
        assert_eq!(balance.frozen, "0.00000000");
        assert_eq!(balance.unavailable, "0.00000000");
    }

    #[test]
    fn test_wallet_balance_serialization_roundtrip() {
        let balance = WalletBalance {
            currency: "BTC".to_string(),
            name: "Bitcoin".to_string(),
            available: "0.50000000".to_string(),
            available_usd_valuation: None,
            frozen: "0.00000000".to_string(),
            unavailable: "0.00000000".to_string(),
        };

        let serialized = serde_json::to_string(&balance).unwrap();
        let deserialized: WalletBalance = serde_json::from_str(&serialized).unwrap();

        assert_eq!(balance.currency, deserialized.currency);
        assert_eq!(balance.name, deserialized.name);
        assert_eq!(balance.available, deserialized.available);
        assert_eq!(
            balance.available_usd_valuation,
            deserialized.available_usd_valuation
        );
        assert_eq!(balance.frozen, deserialized.frozen);
        assert_eq!(balance.unavailable, deserialized.unavailable);
    }

    #[test]
    fn test_get_account_balance_response_structure() {
        let response = GetAccountBalanceResponse {
            wallet: vec![
                WalletBalance {
                    currency: "USDT".to_string(),
                    name: "Tether USD".to_string(),
                    available: "1000.00000000".to_string(),
                    available_usd_valuation: Some("1002.00000000".to_string()),
                    frozen: "0.00000000".to_string(),
                    unavailable: "0.00000000".to_string(),
                },
                WalletBalance {
                    currency: "BTC".to_string(),
                    name: "Bitcoin".to_string(),
                    available: "0.05000000".to_string(),
                    available_usd_valuation: None,
                    frozen: "0.00000000".to_string(),
                    unavailable: "0.00000000".to_string(),
                },
            ],
        };

        assert_eq!(response.wallet.len(), 2);
        assert_eq!(response.wallet[0].currency, "USDT");
        assert_eq!(response.wallet[1].currency, "BTC");
    }

    #[test]
    fn test_response_json_parsing() {
        let json = r#"{
            "wallet": [
                {
                    "currency": "USDT",
                    "name": "Tether USD",
                    "available": "1000.00000000",
                    "available_usd_valuation": "1002.00000000",
                    "frozen": "0.00000000",
                    "unAvailable": "0.00000000"
                }
            ]
        }"#;

        let response: GetAccountBalanceResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.wallet.len(), 1);
        assert_eq!(response.wallet[0].currency, "USDT");
        assert_eq!(
            response.wallet[0].available_usd_valuation,
            Some("1002.00000000".to_string())
        );
    }
}
