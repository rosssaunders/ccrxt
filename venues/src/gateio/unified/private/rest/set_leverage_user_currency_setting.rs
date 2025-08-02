use super::{
    RestClient,
    leverage::{LeverageConfig, SetLeverageConfigRequest},
};

const LEVERAGE_USER_CURRENCY_SETTING_ENDPOINT: &str = "/unified/leverage/user_currency_setting";

impl RestClient {
    /// Set leverage for currency
    ///
    /// This endpoint sets the leverage for a specific currency.
    ///
    /// # API Documentation
    /// <https://www.gate.io/docs/developers/apiv4/#set-leverage-user-currency-setting>
    pub async fn set_leverage_user_currency_setting(
        &self,
        request: SetLeverageConfigRequest,
    ) -> crate::gateio::unified::Result<LeverageConfig> {
        self.post(LEVERAGE_USER_CURRENCY_SETTING_ENDPOINT, &request)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_leverage_request_serialization() {
        let request = SetLeverageConfigRequest {
            currency: "BTC".to_string(),
            leverage: "5".to_string(),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["currency"], "BTC");
        assert_eq!(json["leverage"], "5");
    }

    #[test]
    fn test_set_leverage_different_leverages() {
        let leverages = vec!["1", "2", "5", "10", "20", "50", "100"];

        for leverage in leverages {
            let request = SetLeverageConfigRequest {
                currency: "BTC".to_string(),
                leverage: leverage.to_string(),
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["leverage"], leverage);
        }
    }

    #[test]
    fn test_set_leverage_different_currencies() {
        let currencies = vec!["BTC", "ETH", "USDT", "BNB", "SOL"];

        for currency in currencies {
            let request = SetLeverageConfigRequest {
                currency: currency.to_string(),
                leverage: "10".to_string(),
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["currency"], currency);
        }
    }

    #[test]
    fn test_leverage_user_currency_setting_endpoint() {
        assert_eq!(
            LEVERAGE_USER_CURRENCY_SETTING_ENDPOINT,
            "/unified/leverage/user_currency_setting"
        );
    }
}
