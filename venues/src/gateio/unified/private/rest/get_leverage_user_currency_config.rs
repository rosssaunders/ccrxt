use super::{RestClient, leverage::LeverageConfig};

const LEVERAGE_USER_CURRENCY_CONFIG_ENDPOINT: &str = "/unified/leverage/user_currency_config";

impl RestClient {
    /// Get leverage configuration
    ///
    /// This endpoint returns leverage configuration for currencies.
    ///
    /// # API Documentation
    /// <https://www.gate.io/docs/developers/apiv4/#get-leverage-user-currency-config>
    pub async fn get_leverage_user_currency_config(
        &self,
        currency: Option<&str>,
    ) -> crate::gateio::unified::Result<Vec<LeverageConfig>> {
        let mut endpoint = LEVERAGE_USER_CURRENCY_CONFIG_ENDPOINT.to_string();
        if let Some(currency) = currency {
            endpoint.push_str(&format!("?currency={}", currency));
        }
        self.get(&endpoint).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leverage_config_deserialization() {
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
        assert_eq!(config.min_size, "0.001");
        assert_eq!(config.max_size, "100");
        assert_eq!(config.maintenance_rate, "0.005");
    }

    #[test]
    fn test_leverage_config_array_deserialization() {
        let json = r#"[
            {
                "currency": "BTC",
                "max_leverage": "10",
                "min_size": "0.001",
                "max_size": "100",
                "maintenance_rate": "0.005"
            },
            {
                "currency": "ETH",
                "max_leverage": "20",
                "min_size": "0.01",
                "max_size": "1000",
                "maintenance_rate": "0.003"
            }
        ]"#;

        let configs: Vec<LeverageConfig> = serde_json::from_str(json).unwrap();
        assert_eq!(configs.len(), 2);
        assert_eq!(configs[0].currency, "BTC");
        assert_eq!(configs[1].currency, "ETH");
    }

    #[test]
    fn test_leverage_config_endpoint_without_currency() {
        let endpoint = LEVERAGE_USER_CURRENCY_CONFIG_ENDPOINT;
        assert_eq!(endpoint, "/unified/leverage/user_currency_config");
    }

    #[test]
    fn test_leverage_config_endpoint_with_currency() {
        let currency = "BTC";
        let endpoint = format!(
            "{}?currency={}",
            LEVERAGE_USER_CURRENCY_CONFIG_ENDPOINT, currency
        );
        assert_eq!(
            endpoint,
            "/unified/leverage/user_currency_config?currency=BTC"
        );
    }
}
