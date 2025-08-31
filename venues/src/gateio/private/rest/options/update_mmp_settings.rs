use super::{
    RestClient, RestResult,
    mmp_settings::{MMPSettings, UpdateMMPRequest},
};

const UPDATE_MMP_SETTINGS_ENDPOINT: &str = "/options/mmp";

impl RestClient {
    /// Update MMP settings
    ///
    /// This endpoint updates Market Maker Protection settings.
    ///
    /// [docs](https://www.gate.io/docs/apiv4/en/index.html#update-mmp-settings)
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `request` - The MMP settings update request parameters
    ///
    /// # Returns
    /// Updated MMP settings
    pub async fn update_mmp_settings(&self, request: UpdateMMPRequest) -> RestResult<MMPSettings> {
        self.post(UPDATE_MMP_SETTINGS_ENDPOINT, &request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_mmp_request_minimal() {
        let request = UpdateMMPRequest {
            underlying: "BTC_USDT".to_string(),
            enable: None,
            window: None,
            freeze_time: None,
            trade_limit: None,
            delta_limit: None,
            vega_limit: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["underlying"], "BTC_USDT");

        // Ensure optional fields are not serialized when None
        let obj = json.as_object().unwrap();
        assert_eq!(obj.len(), 1); // Only underlying
    }

    #[test]
    fn test_update_mmp_request_enable_only() {
        let request = UpdateMMPRequest {
            underlying: "ETH_USDT".to_string(),
            enable: Some(true),
            window: None,
            freeze_time: None,
            trade_limit: None,
            delta_limit: None,
            vega_limit: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["underlying"], "ETH_USDT");
        assert!(json["enable"].as_bool().unwrap_or(false));

        let obj = json.as_object().unwrap();
        assert_eq!(obj.len(), 2); // underlying and enable
    }

    #[test]
    fn test_update_mmp_request_partial_update() {
        let request = UpdateMMPRequest {
            underlying: "BTC_USDT".to_string(),
            enable: Some(false),
            window: Some(120),
            freeze_time: Some(60),
            trade_limit: None,
            delta_limit: None,
            vega_limit: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["underlying"], "BTC_USDT");
        assert!(!json["enable"].as_bool().unwrap_or(true));
        assert_eq!(json["window"], 120);
        assert_eq!(json["freeze_time"], 60);
        assert!(json.get("trade_limit").is_none());
        assert!(json.get("delta_limit").is_none());
        assert!(json.get("vega_limit").is_none());
    }

    #[test]
    fn test_update_mmp_request_full() {
        let request = UpdateMMPRequest {
            underlying: "ETH_USDT".to_string(),
            enable: Some(true),
            window: Some(90),
            freeze_time: Some(45),
            trade_limit: Some(150),
            delta_limit: Some("1500.75".to_string()),
            vega_limit: Some("750.25".to_string()),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["underlying"], "ETH_USDT");
        assert!(json["enable"].as_bool().unwrap_or(false));
        assert_eq!(json["window"], 90);
        assert_eq!(json["freeze_time"], 45);
        assert_eq!(json["trade_limit"], 150);
        assert_eq!(json["delta_limit"], "1500.75");
        assert_eq!(json["vega_limit"], "750.25");

        let obj = json.as_object().unwrap();
        assert_eq!(obj.len(), 7); // All fields
    }

    #[test]
    fn test_update_mmp_request_zero_values() {
        let request = UpdateMMPRequest {
            underlying: "ETH_USDT".to_string(),
            enable: Some(false),
            window: Some(0),
            freeze_time: Some(0),
            trade_limit: Some(0),
            delta_limit: Some("0.0".to_string()),
            vega_limit: Some("0.0".to_string()),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["underlying"], "ETH_USDT");
        assert!(!json["enable"].as_bool().unwrap_or(true));
        assert_eq!(json["window"], 0);
        assert_eq!(json["freeze_time"], 0);
        assert_eq!(json["trade_limit"], 0);
        assert_eq!(json["delta_limit"], "0.0");
        assert_eq!(json["vega_limit"], "0.0");
    }

    #[test]
    fn test_update_mmp_request_high_precision_limits() {
        let request = UpdateMMPRequest {
            underlying: "BTC_USDT".to_string(),
            enable: Some(true),
            window: Some(60),
            freeze_time: Some(30),
            trade_limit: Some(100),
            delta_limit: Some("1234.56789012".to_string()),
            vega_limit: Some("5678.90123456".to_string()),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["delta_limit"], "1234.56789012");
        assert_eq!(json["vega_limit"], "5678.90123456");
    }

    #[test]
    fn test_update_mmp_request_disable_mmp() {
        let request = UpdateMMPRequest {
            underlying: "BTC_USDT".to_string(),
            enable: Some(false),
            window: None,
            freeze_time: None,
            trade_limit: None,
            delta_limit: None,
            vega_limit: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["underlying"], "BTC_USDT");
        assert!(!json["enable"].as_bool().unwrap_or(true));

        let obj = json.as_object().unwrap();
        assert_eq!(obj.len(), 2); // Only underlying and enable
    }

    #[test]
    fn test_update_mmp_request_different_underlyings() {
        let underlyings = vec!["BTC_USDT", "ETH_USDT", "BNB_USDT", "SOL_USDT", "ADA_USDT"];

        for underlying in underlyings {
            let request = UpdateMMPRequest {
                underlying: underlying.to_string(),
                enable: Some(true),
                window: Some(60),
                freeze_time: Some(30),
                trade_limit: Some(100),
                delta_limit: Some("1000.0".to_string()),
                vega_limit: Some("500.0".to_string()),
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["underlying"], underlying);
            assert!(json["enable"].as_bool().unwrap_or(false));
        }
    }
}
