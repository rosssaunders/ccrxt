use serde::{Deserialize, Serialize};

use super::RestClient;

const CURRENCIES_ENDPOINT: &str = "/spot/currencies";

/// Currency information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Currency {
    /// Currency name
    pub currency: String,

    /// Whether currency is delisted
    pub delisted: bool,

    /// Whether deposits are disabled
    pub withdraw_disabled: bool,

    /// Whether withdrawals are disabled
    pub withdraw_delayed: bool,

    /// Whether deposits are disabled
    pub deposit_disabled: bool,

    /// Whether trading is disabled
    pub trade_disabled: bool,

    /// Fixed fee rate for withdrawal
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_rate: Option<String>,

    /// Chain name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain: Option<String>,
}

impl RestClient {
    /// List all currencies
    ///
    /// This endpoint returns a list of all supported currencies with their
    /// trading status, withdrawal/deposit status, and fee information.
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#list-all-currencies-details>
    pub async fn list_currencies(&self) -> crate::gateio::spot::RestResult<Vec<Currency>> {
        self.get(CURRENCIES_ENDPOINT).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_currency_full_deserialization() {
        let json = r#"{
            "currency": "BTC",
            "delisted": false,
            "withdraw_disabled": false,
            "withdraw_delayed": false,
            "deposit_disabled": false,
            "trade_disabled": false,
            "fixed_rate": "0.0005",
            "chain": "BTC"
        }"#;

        let currency: Currency = serde_json::from_str(json).unwrap();
        assert_eq!(currency.currency, "BTC");
        assert_eq!(currency.delisted, false);
        assert_eq!(currency.withdraw_disabled, false);
        assert_eq!(currency.withdraw_delayed, false);
        assert_eq!(currency.deposit_disabled, false);
        assert_eq!(currency.trade_disabled, false);
        assert_eq!(currency.fixed_rate, Some("0.0005".to_string()));
        assert_eq!(currency.chain, Some("BTC".to_string()));
    }

    #[test]
    fn test_currency_minimal_deserialization() {
        let json = r#"{
            "currency": "ETH",
            "delisted": false,
            "withdraw_disabled": false,
            "withdraw_delayed": false,
            "deposit_disabled": false,
            "trade_disabled": false
        }"#;

        let currency: Currency = serde_json::from_str(json).unwrap();
        assert_eq!(currency.currency, "ETH");
        assert_eq!(currency.delisted, false);
        assert_eq!(currency.withdraw_disabled, false);
        assert_eq!(currency.withdraw_delayed, false);
        assert_eq!(currency.deposit_disabled, false);
        assert_eq!(currency.trade_disabled, false);
        assert_eq!(currency.fixed_rate, None);
        assert_eq!(currency.chain, None);
    }

    #[test]
    fn test_currency_delisted() {
        let json = r#"{
            "currency": "LUNA",
            "delisted": true,
            "withdraw_disabled": true,
            "withdraw_delayed": false,
            "deposit_disabled": true,
            "trade_disabled": true
        }"#;

        let currency: Currency = serde_json::from_str(json).unwrap();
        assert_eq!(currency.currency, "LUNA");
        assert_eq!(currency.delisted, true);
        assert_eq!(currency.withdraw_disabled, true);
        assert_eq!(currency.deposit_disabled, true);
        assert_eq!(currency.trade_disabled, true);
    }

    #[test]
    fn test_currency_all_flags_true() {
        let json = r#"{
            "currency": "TEST",
            "delisted": true,
            "withdraw_disabled": true,
            "withdraw_delayed": true,
            "deposit_disabled": true,
            "trade_disabled": true,
            "fixed_rate": "0.1",
            "chain": "TEST"
        }"#;

        let currency: Currency = serde_json::from_str(json).unwrap();
        assert_eq!(currency.currency, "TEST");
        assert_eq!(currency.delisted, true);
        assert_eq!(currency.withdraw_disabled, true);
        assert_eq!(currency.withdraw_delayed, true);
        assert_eq!(currency.deposit_disabled, true);
        assert_eq!(currency.trade_disabled, true);
        assert_eq!(currency.fixed_rate, Some("0.1".to_string()));
        assert_eq!(currency.chain, Some("TEST".to_string()));
    }

    #[test]
    fn test_currency_different_chains() {
        let chains = vec![
            "BTC", "ETH", "BSC", "TRC20", "POLYGON", "ARBITRUM", "OPTIMISM",
        ];

        for chain in chains {
            let json = format!(
                r#"{{
                "currency": "USDT",
                "delisted": false,
                "withdraw_disabled": false,
                "withdraw_delayed": false,
                "deposit_disabled": false,
                "trade_disabled": false,
                "chain": "{}"
            }}"#,
                chain
            );

            let currency: Currency = serde_json::from_str(&json).unwrap();
            assert_eq!(currency.currency, "USDT");
            assert_eq!(currency.chain, Some(chain.to_string()));
        }
    }

    #[test]
    fn test_currency_different_fixed_rates() {
        let rates = vec![
            "0",
            "0.0001",
            "0.001",
            "0.01",
            "0.1",
            "1",
            "10",
            "100.123456",
        ];

        for rate in rates {
            let json = format!(
                r#"{{
                "currency": "TOKEN",
                "delisted": false,
                "withdraw_disabled": false,
                "withdraw_delayed": false,
                "deposit_disabled": false,
                "trade_disabled": false,
                "fixed_rate": "{}"
            }}"#,
                rate
            );

            let currency: Currency = serde_json::from_str(&json).unwrap();
            assert_eq!(currency.currency, "TOKEN");
            assert_eq!(currency.fixed_rate, Some(rate.to_string()));
        }
    }

    #[test]
    fn test_currency_array_deserialization() {
        let json = r#"[
            {
                "currency": "BTC",
                "delisted": false,
                "withdraw_disabled": false,
                "withdraw_delayed": false,
                "deposit_disabled": false,
                "trade_disabled": false,
                "fixed_rate": "0.0005",
                "chain": "BTC"
            },
            {
                "currency": "ETH",
                "delisted": false,
                "withdraw_disabled": false,
                "withdraw_delayed": false,
                "deposit_disabled": false,
                "trade_disabled": false,
                "fixed_rate": "0.008",
                "chain": "ETH"
            },
            {
                "currency": "USDT",
                "delisted": false,
                "withdraw_disabled": false,
                "withdraw_delayed": false,
                "deposit_disabled": false,
                "trade_disabled": false,
                "fixed_rate": "1",
                "chain": "TRC20"
            }
        ]"#;

        let currencies: Vec<Currency> = serde_json::from_str(json).unwrap();
        assert_eq!(currencies.len(), 3);

        assert_eq!(currencies[0].currency, "BTC");
        assert_eq!(currencies[0].fixed_rate, Some("0.0005".to_string()));
        assert_eq!(currencies[0].chain, Some("BTC".to_string()));

        assert_eq!(currencies[1].currency, "ETH");
        assert_eq!(currencies[1].fixed_rate, Some("0.008".to_string()));
        assert_eq!(currencies[1].chain, Some("ETH".to_string()));

        assert_eq!(currencies[2].currency, "USDT");
        assert_eq!(currencies[2].fixed_rate, Some("1".to_string()));
        assert_eq!(currencies[2].chain, Some("TRC20".to_string()));
    }

    #[test]
    fn test_currency_empty_array_deserialization() {
        let json = r#"[]"#;
        let currencies: Vec<Currency> = serde_json::from_str(json).unwrap();
        assert_eq!(currencies.len(), 0);
    }

    #[test]
    fn test_currency_serialization() {
        let currency = Currency {
            currency: "BTC".to_string(),
            delisted: false,
            withdraw_disabled: false,
            withdraw_delayed: false,
            deposit_disabled: false,
            trade_disabled: false,
            fixed_rate: Some("0.0005".to_string()),
            chain: Some("BTC".to_string()),
        };

        let json = serde_json::to_value(&currency).unwrap();
        assert_eq!(json["currency"], "BTC");
        assert_eq!(json["delisted"], false);
        assert_eq!(json["withdraw_disabled"], false);
        assert_eq!(json["withdraw_delayed"], false);
        assert_eq!(json["deposit_disabled"], false);
        assert_eq!(json["trade_disabled"], false);
        assert_eq!(json["fixed_rate"], "0.0005");
        assert_eq!(json["chain"], "BTC");
    }

    #[test]
    fn test_currency_serialization_with_none_fields() {
        let currency = Currency {
            currency: "TEST".to_string(),
            delisted: false,
            withdraw_disabled: false,
            withdraw_delayed: false,
            deposit_disabled: false,
            trade_disabled: false,
            fixed_rate: None,
            chain: None,
        };

        let json = serde_json::to_string(&currency).unwrap();
        assert!(!json.contains("fixed_rate"));
        assert!(!json.contains("chain"));
    }

    #[test]
    fn test_currency_serialization_round_trip() {
        let original = Currency {
            currency: "ETH".to_string(),
            delisted: false,
            withdraw_disabled: true,
            withdraw_delayed: false,
            deposit_disabled: false,
            trade_disabled: false,
            fixed_rate: Some("0.008".to_string()),
            chain: Some("ETH".to_string()),
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: Currency = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.currency, original.currency);
        assert_eq!(deserialized.delisted, original.delisted);
        assert_eq!(deserialized.withdraw_disabled, original.withdraw_disabled);
        assert_eq!(deserialized.withdraw_delayed, original.withdraw_delayed);
        assert_eq!(deserialized.deposit_disabled, original.deposit_disabled);
        assert_eq!(deserialized.trade_disabled, original.trade_disabled);
        assert_eq!(deserialized.fixed_rate, original.fixed_rate);
        assert_eq!(deserialized.chain, original.chain);
    }

    #[test]
    fn test_currency_special_names() {
        let special_names = vec!["USDT", "USDC", "DAI", "BUSD", "TUSD", "USDP", "GUSD"];

        for name in special_names {
            let json = format!(
                r#"{{
                "currency": "{}",
                "delisted": false,
                "withdraw_disabled": false,
                "withdraw_delayed": false,
                "deposit_disabled": false,
                "trade_disabled": false
            }}"#,
                name
            );

            let currency: Currency = serde_json::from_str(&json).unwrap();
            assert_eq!(currency.currency, name);
        }
    }

    #[test]
    fn test_currency_mixed_case_names() {
        let json = r#"{
            "currency": "wBTC",
            "delisted": false,
            "withdraw_disabled": false,
            "withdraw_delayed": false,
            "deposit_disabled": false,
            "trade_disabled": false,
            "chain": "ETH"
        }"#;

        let currency: Currency = serde_json::from_str(json).unwrap();
        assert_eq!(currency.currency, "wBTC");
        assert_eq!(currency.chain, Some("ETH".to_string()));
    }

    #[test]
    fn test_currency_realistic_scenarios() {
        // Active major currency
        let btc_json = r#"{
            "currency": "BTC",
            "delisted": false,
            "withdraw_disabled": false,
            "withdraw_delayed": false,
            "deposit_disabled": false,
            "trade_disabled": false,
            "fixed_rate": "0.0005",
            "chain": "BTC"
        }"#;

        let btc: Currency = serde_json::from_str(btc_json).unwrap();
        assert_eq!(btc.currency, "BTC");
        assert!(!btc.delisted);
        assert!(!btc.trade_disabled);

        // Maintenance scenario
        let maintenance_json = r#"{
            "currency": "ALGO",
            "delisted": false,
            "withdraw_disabled": true,
            "withdraw_delayed": true,
            "deposit_disabled": true,
            "trade_disabled": false,
            "fixed_rate": "0.1",
            "chain": "ALGO"
        }"#;

        let maintenance: Currency = serde_json::from_str(maintenance_json).unwrap();
        assert_eq!(maintenance.currency, "ALGO");
        assert!(!maintenance.delisted);
        assert!(!maintenance.trade_disabled); // Can still trade
        assert!(maintenance.withdraw_disabled); // But can't withdraw
        assert!(maintenance.deposit_disabled); // Or deposit
    }
}
