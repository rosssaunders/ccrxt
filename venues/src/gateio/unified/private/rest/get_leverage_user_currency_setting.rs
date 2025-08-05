use super::{RestClient, leverage::LeverageConfig};

const LEVERAGE_USER_CURRENCY_SETTING_ENDPOINT: &str = "/unified/leverage/user_currency_setting";

impl RestClient {
    /// Get current leverage setting
    ///
    /// This endpoint returns the current leverage setting for a currency.
    ///
    /// # API Documentation
    /// <https://www.gate.io/docs/developers/apiv4/#get-leverage-user-currency-setting>
    pub async fn get_leverage_user_currency_setting(
        &self,
        currency: &str,
    ) -> crate::gateio::unified::RestResult<LeverageConfig> {
        let endpoint = format!(
            "{}?currency={}",
            LEVERAGE_USER_CURRENCY_SETTING_ENDPOINT, currency
        );
        self.get(&endpoint).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leverage_setting_endpoint() {
        let currency = "BTC";
        let endpoint = format!(
            "{}?currency={}",
            LEVERAGE_USER_CURRENCY_SETTING_ENDPOINT, currency
        );
        assert_eq!(
            endpoint,
            "/unified/leverage/user_currency_setting?currency=BTC"
        );
    }

    #[test]
    fn test_leverage_setting_different_currencies() {
        let currencies = vec!["BTC", "ETH", "USDT", "BNB", "SOL"];

        for currency in currencies {
            let endpoint = format!(
                "{}?currency={}",
                LEVERAGE_USER_CURRENCY_SETTING_ENDPOINT, currency
            );
            assert!(endpoint.contains(currency));
            assert!(endpoint.starts_with("/unified/leverage/user_currency_setting?currency="));
        }
    }

    #[test]
    fn test_leverage_config_response_deserialization() {
        let json = r#"{
            "currency": "BTC",
            "max_leverage": "10",
            "min_size": "0.001",
            "max_size": "100",
            "maintenance_rate": "0.005"
        }"#;

        let config: LeverageConfig = serde_json::from_str(json).unwrap();
        assert_eq!(config.currency, "BTC");
        assert_eq!(config.max_leverage, "10");
    }
}
