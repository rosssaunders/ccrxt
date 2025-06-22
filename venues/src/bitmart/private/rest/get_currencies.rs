use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::bitmart::RestResult;
use crate::bitmart::rate_limit::EndpointType;

/// Request parameters for getting currencies
#[derive(Debug, Serialize, Default)]
pub struct GetCurrenciesRequest {
    /// Single query, such as BTC; multiple queries, such as BTC,ETH,BMX, can have a maximum of 20.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currencies: Option<String>,
}

/// Currency information for a specific asset
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Currency {
    /// Token symbol, e.g., 'BTC'
    pub currency: String,
    /// Token name, e.g., 'Bitcoin'
    pub name: String,
    /// Contract address (null for native coins)
    pub contract_address: Option<String>,
    /// Network, e.g., 'ERC20'
    pub network: String,
    /// Availability to withdraw
    /// - `true` = available
    /// - `false` = not available
    pub withdraw_enabled: bool,
    /// Availability to deposit
    /// - `true` = available
    /// - `false` = not available
    pub deposit_enabled: bool,
    /// Minimum withdrawal amount
    pub withdraw_minsize: Option<String>,
    /// Minimum withdrawal fee (After 2025-05-18, the field will be removed)
    pub withdraw_minfee: Option<String>,
    /// Withdrawal fee. The unit corresponds to the currency
    pub withdraw_fee: String,
    /// Withdrawal fee estimate. The unit is USD.
    pub withdraw_fee_estimate: String,
}

/// Response for currencies endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCurrenciesResponse {
    /// Array of currency data
    pub currencies: Vec<Currency>,
}

impl RestClient {
    /// Get currencies
    ///
    /// Gets the currency of the asset for withdrawal
    ///
    /// See: https://raw.githubusercontent.com/rosssaunders/coincise/refs/heads/main/docs/bitmart/spot/funding_account.md
    ///
    /// Rate limit: 12 times/2 sec per API key
    ///
    /// # Arguments
    /// * `request` - The request parameters
    ///
    /// # Returns
    /// Currency information
    pub async fn get_currencies(&self, request: GetCurrenciesRequest) -> RestResult<GetCurrenciesResponse> {
        self.send_request(
            "/account/v1/currencies",
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
    fn test_get_currencies_request_default() {
        let request = GetCurrenciesRequest::default();
        assert!(request.currencies.is_none());
    }

    #[test]
    fn test_get_currencies_request_with_single_currency() {
        let request = GetCurrenciesRequest {
            currencies: Some("BTC".to_string()),
        };
        assert_eq!(request.currencies, Some("BTC".to_string()));
    }

    #[test]
    fn test_get_currencies_request_with_multiple_currencies() {
        let request = GetCurrenciesRequest {
            currencies: Some("BTC,ETH,BMX".to_string()),
        };
        assert_eq!(request.currencies, Some("BTC,ETH,BMX".to_string()));
    }

    #[test]
    fn test_currency_structure() {
        let currency = Currency {
            currency: "USDT-TRC20".to_string(),
            name: "USDT-TRC20".to_string(),
            contract_address: Some("TR7NHqjeKQxGTCi8q8ZY4pL8otSzgjLj6t".to_string()),
            network: "TRC20".to_string(),
            withdraw_enabled: true,
            deposit_enabled: true,
            withdraw_minsize: Some("10".to_string()),
            withdraw_minfee: None,
            withdraw_fee: "10".to_string(),
            withdraw_fee_estimate: "10.3".to_string(),
        };

        assert_eq!(currency.currency, "USDT-TRC20");
        assert_eq!(currency.name, "USDT-TRC20");
        assert_eq!(
            currency.contract_address,
            Some("TR7NHqjeKQxGTCi8q8ZY4pL8otSzgjLj6t".to_string())
        );
        assert_eq!(currency.network, "TRC20");
        assert!(currency.withdraw_enabled);
        assert!(currency.deposit_enabled);
        assert_eq!(currency.withdraw_minsize, Some("10".to_string()));
        assert_eq!(currency.withdraw_minfee, None);
        assert_eq!(currency.withdraw_fee, "10");
        assert_eq!(currency.withdraw_fee_estimate, "10.3");
    }

    #[test]
    fn test_currency_serialization_roundtrip() {
        let currency = Currency {
            currency: "BTC".to_string(),
            name: "Bitcoin".to_string(),
            contract_address: None,
            network: "BTC".to_string(),
            withdraw_enabled: false,
            deposit_enabled: true,
            withdraw_minsize: None,
            withdraw_minfee: None,
            withdraw_fee: "0.0005".to_string(),
            withdraw_fee_estimate: "15.5".to_string(),
        };

        let serialized = serde_json::to_string(&currency).unwrap();
        let deserialized: Currency = serde_json::from_str(&serialized).unwrap();

        assert_eq!(currency.currency, deserialized.currency);
        assert_eq!(currency.name, deserialized.name);
        assert_eq!(currency.contract_address, deserialized.contract_address);
        assert_eq!(currency.network, deserialized.network);
        assert_eq!(currency.withdraw_enabled, deserialized.withdraw_enabled);
        assert_eq!(currency.deposit_enabled, deserialized.deposit_enabled);
        assert_eq!(currency.withdraw_minsize, deserialized.withdraw_minsize);
        assert_eq!(currency.withdraw_minfee, deserialized.withdraw_minfee);
        assert_eq!(currency.withdraw_fee, deserialized.withdraw_fee);
        assert_eq!(
            currency.withdraw_fee_estimate,
            deserialized.withdraw_fee_estimate
        );
    }

    #[test]
    fn test_get_currencies_response_structure() {
        let response = GetCurrenciesResponse {
            currencies: vec![
                Currency {
                    currency: "USDT".to_string(),
                    name: "Tether USD".to_string(),
                    contract_address: None,
                    network: "OMNI".to_string(),
                    withdraw_enabled: false,
                    deposit_enabled: false,
                    withdraw_minsize: None,
                    withdraw_minfee: None,
                    withdraw_fee: "10".to_string(),
                    withdraw_fee_estimate: "10.3".to_string(),
                },
                Currency {
                    currency: "USDT-TRC20".to_string(),
                    name: "USDT-TRC20".to_string(),
                    contract_address: Some("TR7NHqjeKQxGTCi8q8ZY4pL8otSzgjLj6t".to_string()),
                    network: "TRC20".to_string(),
                    withdraw_enabled: true,
                    deposit_enabled: true,
                    withdraw_minsize: Some("10".to_string()),
                    withdraw_minfee: None,
                    withdraw_fee: "10".to_string(),
                    withdraw_fee_estimate: "10.3".to_string(),
                },
            ],
        };

        assert_eq!(response.currencies.len(), 2);
        assert_eq!(response.currencies[0].currency, "USDT");
        assert_eq!(response.currencies[1].currency, "USDT-TRC20");
    }

    #[test]
    fn test_response_json_parsing() {
        let json = r#"{
            "currencies": [
                {
                    "currency": "USDT",
                    "name": "Tether USD",
                    "contract_address": null,
                    "network": "OMNI",
                    "withdraw_enabled": false,
                    "deposit_enabled": false,
                    "withdraw_minsize": null,
                    "withdraw_minfee": null,
                    "withdraw_fee": "10",
                    "withdraw_fee_estimate": "10.3"
                },
                {
                    "currency": "USDT-TRC20",
                    "name": "USDT-TRC20",
                    "contract_address": "TR7NHqjeKQxGTCi8q8ZY4pL8otSzgjLj6t",
                    "network": "TRC20",
                    "withdraw_enabled": true,
                    "deposit_enabled": true,
                    "withdraw_minsize": "10",
                    "withdraw_minfee": null,
                    "withdraw_fee": "10",
                    "withdraw_fee_estimate": "10.3"
                }
            ]
        }"#;

        let response: GetCurrenciesResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.currencies.len(), 2);
        assert_eq!(response.currencies[0].currency, "USDT");
        assert_eq!(response.currencies[0].contract_address, None);
        assert_eq!(response.currencies[1].currency, "USDT-TRC20");
        assert_eq!(
            response.currencies[1].contract_address,
            Some("TR7NHqjeKQxGTCi8q8ZY4pL8otSzgjLj6t".to_string())
        );
    }
}
