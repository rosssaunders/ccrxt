use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::cryptocom::RestResult;

const CURRENCY_NETWORKS_ENDPOINT: &str = "private/get-currency-networks";

/// Network information for a currency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInfo {
    /// The network ID, can be used in create-withdrawal
    pub network_id: String,

    /// Whether withdrawals are enabled for this network
    pub withdraw_enabled: bool,

    /// Whether deposits are enabled for this network
    pub deposit_enabled: bool,

    /// Withdrawal fee for this network
    pub withdrawal_fee: Option<f64>,

    /// Minimum withdrawal amount for this network
    pub min_withdrawal_amount: f64,

    /// Confirmation blocks count required
    pub confirmation_required: u32,
}

/// Currency network mapping information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrencyInfo {
    /// Full name of the currency e.g. "SHIBA INU"
    pub full_name: String,

    /// Default network if not provided in create-withdrawal
    pub default_network: Option<String>,

    /// List of available networks for this currency
    pub network_list: Vec<NetworkInfo>,
}

/// Response for get currency networks endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCurrencyNetworksResponse {
    /// Last update timestamp
    pub update_time: u64,

    /// Map of currency symbol to currency information
    pub currency_map: HashMap<String, CurrencyInfo>,
}

impl RestClient {
    /// Get the symbol network mapping
    ///
    /// Returns the symbol network mapping for all supported currencies.
    /// Works for master account only, not for sub-accounts.
    ///
    /// [docs](https://exchange-docs.crypto.com/exchange/v1/rest-ws/index.html#private-get-currency-networks)
    ///
    /// # Returns
    /// Currency network mapping information for all supported currencies
    pub async fn get_currency_networks(&self) -> RestResult<GetCurrencyNetworksResponse> {
        // Empty struct to represent request with no parameters
        #[derive(Debug, Clone, Serialize)]
        struct EmptyRequest {}

        self.send_signed_request(CURRENCY_NETWORKS_ENDPOINT, EmptyRequest {})
            .await
    }
}

#[cfg(test)]
mod tests {
    use rest::secrets::ExposableSecret;
    use serde_json::json;

    use super::*;

    /// A plain text implementation of ExposableSecret for testing purposes.
    #[derive(Clone)]
    #[allow(dead_code)]
    struct PlainTextSecret {
        secret: String,
    }

    impl ExposableSecret for PlainTextSecret {
        fn expose_secret(&self) -> String {
            self.secret.clone()
        }
    }

    impl PlainTextSecret {
        #[allow(dead_code)]
        fn new(secret: String) -> Self {
            Self { secret }
        }
    }

    #[test]
    fn test_network_info_structure() {
        let network_json = json!({
            "network_id": "ETH",
            "withdrawal_fee": 20.0,
            "withdraw_enabled": true,
            "min_withdrawal_amount": 40.0,
            "deposit_enabled": true,
            "confirmation_required": 12
        });

        let network: NetworkInfo = serde_json::from_value(network_json).unwrap();
        assert_eq!(network.network_id, "ETH");
        assert_eq!(network.withdrawal_fee, Some(20.0));
        assert!(network.withdraw_enabled);
        assert_eq!(network.min_withdrawal_amount, 40.0);
        assert!(network.deposit_enabled);
        assert_eq!(network.confirmation_required, 12);
    }

    #[test]
    fn test_network_info_with_null_fee() {
        let network_json = json!({
            "network_id": "ETH",
            "withdrawal_fee": null,
            "withdraw_enabled": true,
            "min_withdrawal_amount": 10.0,
            "deposit_enabled": true,
            "confirmation_required": 12
        });

        let network: NetworkInfo = serde_json::from_value(network_json).unwrap();
        assert_eq!(network.network_id, "ETH");
        assert_eq!(network.withdrawal_fee, None);
        assert!(network.withdraw_enabled);
        assert_eq!(network.min_withdrawal_amount, 10.0);
        assert!(network.deposit_enabled);
        assert_eq!(network.confirmation_required, 12);
    }

    #[test]
    fn test_currency_info_structure() {
        let currency_json = json!({
            "full_name": "Polygon",
            "default_network": "ETH",
            "network_list": [
                {
                    "network_id": "ETH",
                    "withdrawal_fee": 20.0,
                    "withdraw_enabled": true,
                    "min_withdrawal_amount": 40.0,
                    "deposit_enabled": true,
                    "confirmation_required": 0
                },
                {
                    "network_id": "MATIC",
                    "withdrawal_fee": 0.08,
                    "withdraw_enabled": true,
                    "min_withdrawal_amount": 0.16,
                    "deposit_enabled": true,
                    "confirmation_required": 0
                }
            ]
        });

        let currency: CurrencyInfo = serde_json::from_value(currency_json).unwrap();
        assert_eq!(currency.full_name, "Polygon");
        assert_eq!(currency.default_network, Some("ETH".to_string()));
        assert_eq!(currency.network_list.len(), 2);
        assert_eq!(currency.network_list.first().unwrap().network_id, "ETH");
        assert_eq!(currency.network_list.get(1).unwrap().network_id, "MATIC");
    }

    #[test]
    fn test_currency_info_with_no_default_network() {
        let currency_json = json!({
            "full_name": "Adventure Gold",
            "default_network": null,
            "network_list": [
                {
                    "network_id": "ETH",
                    "withdrawal_fee": null,
                    "withdraw_enabled": true,
                    "min_withdrawal_amount": 10.0,
                    "deposit_enabled": true,
                    "confirmation_required": 12
                }
            ]
        });

        let currency: CurrencyInfo = serde_json::from_value(currency_json).unwrap();
        assert_eq!(currency.full_name, "Adventure Gold");
        assert_eq!(currency.default_network, None);
        assert_eq!(currency.network_list.len(), 1);
        assert_eq!(currency.network_list.first().unwrap().network_id, "ETH");
        assert_eq!(currency.network_list.first().unwrap().withdrawal_fee, None);
    }

    #[test]
    fn test_get_currency_networks_response_structure() {
        let response_json = json!({
            "update_time": 1641151604000_u64,
            "currency_map": {
                "AGLD": {
                    "full_name": "Adventure Gold",
                    "default_network": null,
                    "network_list": [
                        {
                            "network_id": "ETH",
                            "withdrawal_fee": null,
                            "withdraw_enabled": true,
                            "min_withdrawal_amount": 10.0,
                            "deposit_enabled": true,
                            "confirmation_required": 12
                        }
                    ]
                },
                "MATIC": {
                    "full_name": "Polygon",
                    "default_network": "ETH",
                    "network_list": [
                        {
                            "network_id": "BNB",
                            "withdrawal_fee": 0.8,
                            "withdraw_enabled": true,
                            "min_withdrawal_amount": 1.6,
                            "deposit_enabled": true,
                            "confirmation_required": 0
                        },
                        {
                            "network_id": "ETH",
                            "withdrawal_fee": 20.0,
                            "withdraw_enabled": true,
                            "min_withdrawal_amount": 40.0,
                            "deposit_enabled": true,
                            "confirmation_required": 0
                        }
                    ]
                }
            }
        });

        let response: GetCurrencyNetworksResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.update_time, 1641151604000);
        assert_eq!(response.currency_map.len(), 2);

        let agld = response.currency_map.get("AGLD").unwrap();
        assert_eq!(agld.full_name, "Adventure Gold");
        assert_eq!(agld.default_network, None);
        assert_eq!(agld.network_list.len(), 1);

        let matic = response.currency_map.get("MATIC").unwrap();
        assert_eq!(matic.full_name, "Polygon");
        assert_eq!(matic.default_network, Some("ETH".to_string()));
        assert_eq!(matic.network_list.len(), 2);
    }
}
