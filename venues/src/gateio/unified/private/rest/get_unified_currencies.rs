use super::{
    RestClient,
    unified_mode::{UnifiedCurrenciesRequest, UnifiedCurrency},
};

const UNIFIED_CURRENCIES_ENDPOINT: &str = "/unified/currencies";

impl RestClient {
    /// Get unified currencies
    ///
    /// This endpoint returns currency information for unified accounts.
    ///
    /// # API Documentation
    /// <https://www.gate.io/docs/developers/apiv4/#get-unified-currencies>
    pub async fn get_unified_currencies(
        &self,
        params: UnifiedCurrenciesRequest,
    ) -> crate::gateio::unified::Result<Vec<UnifiedCurrency>> {
        self.get_with_query(UNIFIED_CURRENCIES_ENDPOINT, &params)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unified_currencies_request_serialization() {
        let request = UnifiedCurrenciesRequest {
            currency: Some("BTC".to_string()),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["currency"], "BTC");
    }

    #[test]
    fn test_unified_currencies_request_default() {
        let request = UnifiedCurrenciesRequest::default();

        let json = serde_json::to_value(&request).unwrap();
        assert!(!json.as_object().unwrap().contains_key("currency"));
    }

    #[test]
    fn test_unified_currency_deserialization() {
        let json = r#"{
            "currency": "BTC",
            "name": "Bitcoin",
            "delisted": false,
            "withdraw_disabled": false,
            "withdraw_delayed": false,
            "deposit_disabled": false,
            "trade_disabled": false,
            "fixed_rate": "0",
            "cross_margin": true,
            "lendable": true,
            "borrowable": true
        }"#;

        let currency: UnifiedCurrency = serde_json::from_str(json).unwrap();
        assert_eq!(currency.currency, "BTC");
        assert_eq!(currency.name, "Bitcoin");
        assert!(!currency.delisted);
        assert!(!currency.withdraw_disabled);
        assert!(!currency.withdraw_delayed);
        assert!(!currency.deposit_disabled);
        assert!(!currency.trade_disabled);
        assert_eq!(currency.fixed_rate, "0");
        assert!(currency.cross_margin);
        assert!(currency.lendable);
        assert!(currency.borrowable);
    }

    #[test]
    fn test_unified_currencies_array_deserialization() {
        let json = r#"[
            {
                "currency": "BTC",
                "name": "Bitcoin",
                "delisted": false,
                "withdraw_disabled": false,
                "withdraw_delayed": false,
                "deposit_disabled": false,
                "trade_disabled": false,
                "fixed_rate": "0",
                "cross_margin": true,
                "lendable": true,
                "borrowable": true
            },
            {
                "currency": "ETH",
                "name": "Ethereum",
                "delisted": false,
                "withdraw_disabled": false,
                "withdraw_delayed": false,
                "deposit_disabled": false,
                "trade_disabled": false,
                "fixed_rate": "0",
                "cross_margin": true,
                "lendable": true,
                "borrowable": true
            }
        ]"#;

        let currencies: Vec<UnifiedCurrency> = serde_json::from_str(json).unwrap();
        assert_eq!(currencies.len(), 2);
        assert_eq!(currencies[0].currency, "BTC");
        assert_eq!(currencies[1].currency, "ETH");
    }

    #[test]
    fn test_unified_currencies_endpoint() {
        assert_eq!(UNIFIED_CURRENCIES_ENDPOINT, "/unified/currencies");
    }

    #[test]
    fn test_unified_currency_disabled_features() {
        let json = r#"{
            "currency": "XYZ",
            "name": "XYZ Token",
            "delisted": true,
            "withdraw_disabled": true,
            "withdraw_delayed": true,
            "deposit_disabled": true,
            "trade_disabled": true,
            "fixed_rate": "1",
            "cross_margin": false,
            "lendable": false,
            "borrowable": false
        }"#;

        let currency: UnifiedCurrency = serde_json::from_str(json).unwrap();
        assert!(currency.delisted);
        assert!(currency.withdraw_disabled);
        assert!(currency.withdraw_delayed);
        assert!(currency.deposit_disabled);
        assert!(currency.trade_disabled);
        assert!(!currency.cross_margin);
        assert!(!currency.lendable);
        assert!(!currency.borrowable);
    }
}
