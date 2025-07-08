use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::bitmart::{RestResult, rate_limit::EndpointType};

const CURRENCY_LIST_ENDPOINT: &str = "/spot/v1/currencies";

/// Request parameters for getting currency list
#[derive(Debug, Serialize, Default)]
pub struct GetCurrencyListRequest {
    // No parameters needed for this endpoint
}

/// Currency information from the public API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Currency {
    /// Currency abbreviation, such as BTC
    pub id: String,
    /// Currency full name, such as Bitcoin
    pub name: String,
    /// Whether this currency can be withdrawn on the platform
    /// - `true` = can
    /// - `false` = no
    pub withdraw_enabled: bool,
    /// Whether this currency can be deposited on the platform
    /// - `true` = can
    /// - `false` = no
    pub deposit_enabled: bool,
}

/// Response for currency list endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCurrencyListResponse {
    /// Array of currency data
    pub currencies: Vec<Currency>,
}

impl RestClient {
    /// Get Currency List (V1)
    ///
    /// Get a list of all cryptocurrencies on the platform
    ///
    /// See: https://raw.githubusercontent.com/rosssaunders/coincise/refs/heads/main/docs/bitmart/spot/public_market_data.md
    ///
    /// Rate limit: 10 times/2sec per IP
    ///
    /// # Returns
    /// List of all cryptocurrencies on the platform
    pub async fn get_currency_list(
        &self,
        _request: GetCurrencyListRequest,
    ) -> RestResult<GetCurrencyListResponse> {
        self.send_request(
            CURRENCY_LIST_ENDPOINT,
            reqwest::Method::GET,
            Option::<&()>::None, // No query parameters
            EndpointType::SpotPublicMarket,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_currency_list_request_default() {
        let request = GetCurrencyListRequest::default();
        // Request has no fields to check
        let _ = request;
    }

    #[test]
    fn test_currency_structure() {
        let currency = Currency {
            id: "BTC".to_string(),
            name: "Bitcoin".to_string(),
            withdraw_enabled: true,
            deposit_enabled: true,
        };

        assert_eq!(currency.id, "BTC");
        assert_eq!(currency.name, "Bitcoin");
        assert!(currency.withdraw_enabled);
        assert!(currency.deposit_enabled);
    }

    #[test]
    fn test_currency_serialization_roundtrip() {
        let currency = Currency {
            id: "ETH".to_string(),
            name: "Ethereum".to_string(),
            withdraw_enabled: false,
            deposit_enabled: true,
        };

        let serialized = serde_json::to_string(&currency).expect("Failed to serialize currency");
        let deserialized: Currency =
            serde_json::from_str(&serialized).expect("Failed to deserialize currency");

        assert_eq!(currency.id, deserialized.id);
        assert_eq!(currency.name, deserialized.name);
        assert_eq!(currency.withdraw_enabled, deserialized.withdraw_enabled);
        assert_eq!(currency.deposit_enabled, deserialized.deposit_enabled);
    }

    #[test]
    fn test_get_currency_list_response_structure() {
        let response = GetCurrencyListResponse {
            currencies: vec![
                Currency {
                    id: "BTC".to_string(),
                    name: "Bitcoin".to_string(),
                    withdraw_enabled: true,
                    deposit_enabled: true,
                },
                Currency {
                    id: "ETH".to_string(),
                    name: "Ethereum".to_string(),
                    withdraw_enabled: true,
                    deposit_enabled: true,
                },
            ],
        };

        assert_eq!(response.currencies.len(), 2);
        assert_eq!(response.currencies[0].id, "BTC");
        assert_eq!(response.currencies[1].id, "ETH");
    }

    #[test]
    fn test_response_json_parsing() {
        let json = r#"{
            "currencies": [
                {
                    "id": "BTC",
                    "name": "Bitcoin",
                    "withdraw_enabled": true,
                    "deposit_enabled": true
                },
                {
                    "id": "ETH",
                    "name": "Ethereum",
                    "withdraw_enabled": true,
                    "deposit_enabled": true
                }
            ]
        }"#;

        let response: GetCurrencyListResponse =
            serde_json::from_str(json).expect("Failed to deserialize response");
        assert_eq!(response.currencies.len(), 2);
        assert_eq!(response.currencies[0].id, "BTC");
        assert_eq!(response.currencies[0].name, "Bitcoin");
        assert!(response.currencies[0].withdraw_enabled);
        assert!(response.currencies[0].deposit_enabled);
        assert_eq!(response.currencies[1].id, "ETH");
        assert_eq!(response.currencies[1].name, "Ethereum");
    }
}
