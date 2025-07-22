use serde::{Deserialize, Serialize};

/// Market Maker Protection (MMP) settings (common struct used by multiple endpoints)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MMPSettings {
    /// User ID
    pub user: i64,

    /// Underlying asset
    pub underlying: String,

    /// Enable MMP
    pub enable: bool,

    /// Window size in seconds
    pub window: i32,

    /// Freeze time in seconds
    pub freeze_time: i32,

    /// Trade limit
    pub trade_limit: i32,

    /// Delta limit
    pub delta_limit: String,

    /// Vega limit
    pub vega_limit: String,
}

/// Request to update MMP settings
#[derive(Debug, Clone, Serialize)]
pub struct UpdateMMPRequest {
    /// Underlying asset
    pub underlying: String,

    /// Enable MMP
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,

    /// Window size in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window: Option<i32>,

    /// Freeze time in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub freeze_time: Option<i32>,

    /// Trade limit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_limit: Option<i32>,

    /// Delta limit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delta_limit: Option<String>,

    /// Vega limit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vega_limit: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mmp_settings_deserialization() {
        let json = r#"{
            "user": 12345,
            "underlying": "BTC_USDT",
            "enable": true,
            "window": 60,
            "freeze_time": 30,
            "trade_limit": 100,
            "delta_limit": "1000.0",
            "vega_limit": "500.0"
        }"#;

        let settings: MMPSettings = serde_json::from_str(json).unwrap();
        assert_eq!(settings.user, 12345);
        assert_eq!(settings.underlying, "BTC_USDT");
        assert_eq!(settings.enable, true);
        assert_eq!(settings.window, 60);
        assert_eq!(settings.freeze_time, 30);
        assert_eq!(settings.trade_limit, 100);
        assert_eq!(settings.delta_limit, "1000.0");
        assert_eq!(settings.vega_limit, "500.0");
    }

    #[test]
    fn test_mmp_settings_disabled_deserialization() {
        let json = r#"{
            "user": 67890,
            "underlying": "ETH_USDT",
            "enable": false,
            "window": 120,
            "freeze_time": 60,
            "trade_limit": 50,
            "delta_limit": "0.0",
            "vega_limit": "0.0"
        }"#;

        let settings: MMPSettings = serde_json::from_str(json).unwrap();
        assert_eq!(settings.user, 67890);
        assert_eq!(settings.underlying, "ETH_USDT");
        assert_eq!(settings.enable, false);
        assert_eq!(settings.window, 120);
        assert_eq!(settings.freeze_time, 60);
        assert_eq!(settings.trade_limit, 50);
        assert_eq!(settings.delta_limit, "0.0");
        assert_eq!(settings.vega_limit, "0.0");
    }

    #[test]
    fn test_mmp_settings_extreme_values() {
        let json = r#"{
            "user": 9223372036854775807,
            "underlying": "BTC_USDT",
            "enable": true,
            "window": 2147483647,
            "freeze_time": 2147483647,
            "trade_limit": 2147483647,
            "delta_limit": "99999999.99999999",
            "vega_limit": "99999999.99999999"
        }"#;

        let settings: MMPSettings = serde_json::from_str(json).unwrap();
        assert_eq!(settings.user, 9223372036854775807);
        assert_eq!(settings.underlying, "BTC_USDT");
        assert_eq!(settings.enable, true);
        assert_eq!(settings.window, 2147483647);
        assert_eq!(settings.freeze_time, 2147483647);
        assert_eq!(settings.trade_limit, 2147483647);
        assert_eq!(settings.delta_limit, "99999999.99999999");
        assert_eq!(settings.vega_limit, "99999999.99999999");
    }

    #[test]
    fn test_update_mmp_request_minimal_serialization() {
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
        assert!(json.get("enable").is_none());
        assert!(json.get("window").is_none());
        assert!(json.get("freeze_time").is_none());
        assert!(json.get("trade_limit").is_none());
        assert!(json.get("delta_limit").is_none());
        assert!(json.get("vega_limit").is_none());
    }

    #[test]
    fn test_update_mmp_request_full_update() {
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
        assert_eq!(json["enable"], true);
        assert_eq!(json["window"], 90);
        assert_eq!(json["freeze_time"], 45);
        assert_eq!(json["trade_limit"], 150);
        assert_eq!(json["delta_limit"], "1500.75");
        assert_eq!(json["vega_limit"], "750.25");
    }

    #[test]
    fn test_mmp_settings_different_underlyings() {
        let underlyings = vec!["BTC_USDT", "ETH_USDT", "BNB_USDT", "SOL_USDT", "ADA_USDT"];
        
        for underlying in underlyings {
            let json = format!(r#"{{
                "user": 12345,
                "underlying": "{}",
                "enable": true,
                "window": 60,
                "freeze_time": 30,
                "trade_limit": 100,
                "delta_limit": "1000.0",
                "vega_limit": "500.0"
            }}"#, underlying);

            let settings: MMPSettings = serde_json::from_str(&json).unwrap();
            assert_eq!(settings.underlying, underlying);
            assert_eq!(settings.user, 12345);
        }
    }
}