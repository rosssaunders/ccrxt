use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::bitmart::RestResult;
use crate::bitmart::rate_limit::EndpointType;

/// Request parameters for getting spot wallet balance (no parameters required)
#[derive(Debug, Serialize, Default)]
pub struct GetSpotWalletBalanceRequest {}

/// Spot wallet balance information for a specific currency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpotWalletBalance {
    /// Cryptocurrency abbreviation
    pub id: String,
    /// Full name
    pub name: String,
    /// Available balance
    pub available: String,
    /// Frozen balance
    pub frozen: String,
}

/// Response for spot wallet balance endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSpotWalletBalanceResponse {
    /// Array of spot wallet balance data
    pub wallet: Vec<SpotWalletBalance>,
}

impl RestClient {
    /// Get spot wallet balance
    ///
    /// Get the user's wallet balance for all currencies
    ///
    /// See: https://raw.githubusercontent.com/rosssaunders/coincise/refs/heads/main/docs/bitmart/spot/funding_account.md
    ///
    /// Rate limit: 12 times/2 sec per API key
    ///
    /// # Arguments
    /// * `request` - The request parameters (empty struct)
    ///
    /// # Returns
    /// Spot wallet balance information
    pub async fn get_spot_wallet_balance(&self, request: GetSpotWalletBalanceRequest) -> RestResult<GetSpotWalletBalanceResponse> {
        self.send_request(
            "/spot/v1/wallet",
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
    fn test_get_spot_wallet_balance_request_default() {
        let request = GetSpotWalletBalanceRequest::default();
        // This is an empty struct, just verify it can be created
        let _serialized = serde_json::to_string(&request).unwrap();
    }

    #[test]
    fn test_spot_wallet_balance_structure() {
        let balance = SpotWalletBalance {
            id: "BTC".to_string(),
            name: "Bitcoin".to_string(),
            available: "10.000000".to_string(),
            frozen: "10.000000".to_string(),
        };

        assert_eq!(balance.id, "BTC");
        assert_eq!(balance.name, "Bitcoin");
        assert_eq!(balance.available, "10.000000");
        assert_eq!(balance.frozen, "10.000000");
    }

    #[test]
    fn test_spot_wallet_balance_serialization_roundtrip() {
        let balance = SpotWalletBalance {
            id: "ETH".to_string(),
            name: "Ethereum".to_string(),
            available: "0.50000000".to_string(),
            frozen: "0.00000000".to_string(),
        };

        let serialized = serde_json::to_string(&balance).unwrap();
        let deserialized: SpotWalletBalance = serde_json::from_str(&serialized).unwrap();

        assert_eq!(balance.id, deserialized.id);
        assert_eq!(balance.name, deserialized.name);
        assert_eq!(balance.available, deserialized.available);
        assert_eq!(balance.frozen, deserialized.frozen);
    }

    #[test]
    fn test_get_spot_wallet_balance_response_structure() {
        let response = GetSpotWalletBalanceResponse {
            wallet: vec![
                SpotWalletBalance {
                    id: "BTC".to_string(),
                    name: "Bitcoin".to_string(),
                    available: "10.000000".to_string(),
                    frozen: "10.000000".to_string(),
                },
                SpotWalletBalance {
                    id: "ETH".to_string(),
                    name: "Ethereum".to_string(),
                    available: "5.000000".to_string(),
                    frozen: "0.000000".to_string(),
                },
            ],
        };

        assert_eq!(response.wallet.len(), 2);
        assert_eq!(response.wallet[0].id, "BTC");
        assert_eq!(response.wallet[1].id, "ETH");
    }

    #[test]
    fn test_response_json_parsing() {
        let json = r#"{
            "wallet": [
                {
                    "id": "BTC",
                    "available": "10.000000",
                    "name": "Bitcoin",
                    "frozen": "10.000000"
                }
            ]
        }"#;

        let response: GetSpotWalletBalanceResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.wallet.len(), 1);
        assert_eq!(response.wallet[0].id, "BTC");
        assert_eq!(response.wallet[0].name, "Bitcoin");
        assert_eq!(response.wallet[0].available, "10.000000");
        assert_eq!(response.wallet[0].frozen, "10.000000");
    }
}