use serde::{Deserialize, Serialize};

use super::RestClient;

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
    /// Get specific currency details
    ///
    /// This endpoint returns details for a specific currency, including trading status, withdrawal/deposit status, and fee information.
    ///
    /// [docs](https://www.gate.com/docs/developers/apiv4/#get-details-of-a-specific-currency)
    pub async fn get_currency(&self, currency: &str) -> crate::gateio::spot::RestResult<Currency> {
        let endpoint = format!("/spot/currencies/{}", currency);
        self.get(&endpoint).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_currency_btc_deserialization() {
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
        assert!(!currency.delisted);
        assert!(!currency.withdraw_disabled);
        assert!(!currency.withdraw_delayed);
        assert!(!currency.deposit_disabled);
        assert!(!currency.trade_disabled);
        assert_eq!(currency.fixed_rate, Some("0.0005".to_string()));
        assert_eq!(currency.chain, Some("BTC".to_string()));
    }

    #[test]
    fn test_get_currency_eth_deserialization() {
        let json = r#"{
            "currency": "ETH",
            "delisted": false,
            "withdraw_disabled": false,
            "withdraw_delayed": false,
            "deposit_disabled": false,
            "trade_disabled": false,
            "fixed_rate": "0.008",
            "chain": "ETH"
        }"#;

        let currency: Currency = serde_json::from_str(json).unwrap();
        assert_eq!(currency.currency, "ETH");
        assert!(!currency.delisted);
        assert_eq!(currency.fixed_rate, Some("0.008".to_string()));
        assert_eq!(currency.chain, Some("ETH".to_string()));
    }

    #[test]
    fn test_get_currency_stablecoin_deserialization() {
        let json = r#"{
            "currency": "USDT",
            "delisted": false,
            "withdraw_disabled": false,
            "withdraw_delayed": false,
            "deposit_disabled": false,
            "trade_disabled": false,
            "fixed_rate": "1",
            "chain": "TRC20"
        }"#;

        let currency: Currency = serde_json::from_str(json).unwrap();
        assert_eq!(currency.currency, "USDT");
        assert_eq!(currency.fixed_rate, Some("1".to_string()));
        assert_eq!(currency.chain, Some("TRC20".to_string()));
    }

    #[test]
    fn test_get_currency_maintenance_deserialization() {
        let json = r#"{
            "currency": "ALGO",
            "delisted": false,
            "withdraw_disabled": true,
            "withdraw_delayed": true,
            "deposit_disabled": true,
            "trade_disabled": false,
            "fixed_rate": "0.1",
            "chain": "ALGO"
        }"#;

        let currency: Currency = serde_json::from_str(json).unwrap();
        assert_eq!(currency.currency, "ALGO");
        assert!(currency.withdraw_disabled);
        assert!(currency.withdraw_delayed);
        assert!(currency.deposit_disabled);
        assert!(!currency.trade_disabled); // Can still trade during maintenance
    }

    #[test]
    fn test_get_currency_delisted_deserialization() {
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
        assert!(currency.delisted);
        assert!(currency.withdraw_disabled);
        assert!(currency.deposit_disabled);
        assert!(currency.trade_disabled);
        assert_eq!(currency.fixed_rate, None);
        assert_eq!(currency.chain, None);
    }

    #[test]
    fn test_get_currency_clone() {
        let original = Currency {
            currency: "BTC".to_string(),
            delisted: false,
            withdraw_disabled: false,
            withdraw_delayed: false,
            deposit_disabled: false,
            trade_disabled: false,
            fixed_rate: Some("0.0005".to_string()),
            chain: Some("BTC".to_string()),
        };

        let cloned = original.clone();
        assert_eq!(cloned.currency, original.currency);
        assert_eq!(cloned.delisted, original.delisted);
        assert_eq!(cloned.withdraw_disabled, original.withdraw_disabled);
        assert_eq!(cloned.withdraw_delayed, original.withdraw_delayed);
        assert_eq!(cloned.deposit_disabled, original.deposit_disabled);
        assert_eq!(cloned.trade_disabled, original.trade_disabled);
        assert_eq!(cloned.fixed_rate, original.fixed_rate);
        assert_eq!(cloned.chain, original.chain);
    }

    #[test]
    fn test_get_currency_debug() {
        let currency = Currency {
            currency: "ETH".to_string(),
            delisted: false,
            withdraw_disabled: false,
            withdraw_delayed: false,
            deposit_disabled: false,
            trade_disabled: false,
            fixed_rate: Some("0.008".to_string()),
            chain: Some("ETH".to_string()),
        };

        let debug_str = format!("{:?}", currency);
        assert!(debug_str.contains("Currency"));
        assert!(debug_str.contains("ETH"));
        assert!(debug_str.contains("0.008"));
    }
}
