use serde::{Deserialize, Serialize};

use crate::kucoin::futures::{
    MarginMode, ResponseHeaders, RestResponse, Result, private_client::RestClient,
};

/// Endpoint URL for change margin mode
const CHANGE_MARGIN_MODE_ENDPOINT: &str = "/api/v2/position/changeMarginMode";

/// Request parameters for changing margin mode.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeMarginModeRequest {
    /// Trading symbol (e.g., "XBTUSDTM"). Required parameter.
    pub symbol: String,

    /// Target margin mode (CROSS_MARGIN or ISOLATED_MARGIN). Required parameter.
    pub margin_mode: MarginMode,
}

/// Response from the change margin mode endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeMarginModeResponse {
    /// Success flag indicating if the margin mode change was successful.
    pub result: bool,
}

impl RestClient {
    /// Switch Margin Mode
    ///
    /// Switch the margin mode of a symbol between cross margin and isolated margin.
    ///
    /// [docs](https://www.kucoin.com/docs-new/rest/futures-trading/positions/switch-margin-mode)
    ///
    /// Rate limit: 20
    ///
    /// # Arguments
    /// * `request` - The change margin mode request parameters
    ///
    /// # Returns
    /// Success flag indicating if the margin mode change was successful
    pub async fn change_margin_mode(
        &self,
        request: ChangeMarginModeRequest,
    ) -> Result<(RestResponse<ChangeMarginModeResponse>, ResponseHeaders)> {
        self.post_with_request(CHANGE_MARGIN_MODE_ENDPOINT, &request)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_change_margin_mode_request_serialization_cross() {
        let request = ChangeMarginModeRequest {
            symbol: "XBTUSDTM".to_string(),
            margin_mode: MarginMode::CrossMargin,
        };

        let json = serde_json::to_string(&request).unwrap();
        let expected = r#"{"symbol":"XBTUSDTM","marginMode":"CROSS_MARGIN"}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn test_change_margin_mode_request_serialization_isolated() {
        let request = ChangeMarginModeRequest {
            symbol: "ETHUSDTM".to_string(),
            margin_mode: MarginMode::IsolatedMargin,
        };

        let json = serde_json::to_string(&request).unwrap();
        let expected = r#"{"symbol":"ETHUSDTM","marginMode":"ISOLATED_MARGIN"}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn test_change_margin_mode_response_deserialization_success() {
        let json = r#"{"result":true}"#;
        let response: ChangeMarginModeResponse = serde_json::from_str(json).unwrap();
        assert!(response.result);
    }

    #[test]
    fn test_change_margin_mode_response_deserialization_failure() {
        let json = r#"{"result":false}"#;
        let response: ChangeMarginModeResponse = serde_json::from_str(json).unwrap();
        assert!(!response.result);
    }

    #[test]
    fn test_change_margin_mode_endpoint() {
        assert_eq!(
            CHANGE_MARGIN_MODE_ENDPOINT,
            "/api/v2/position/changeMarginMode"
        );
    }

    #[test]
    fn test_margin_mode_variants() {
        // Test cross margin serialization
        let cross_json = serde_json::to_string(&MarginMode::CrossMargin).unwrap();
        assert_eq!(cross_json, "\"CROSS_MARGIN\"");

        // Test isolated margin serialization
        let isolated_json = serde_json::to_string(&MarginMode::IsolatedMargin).unwrap();
        assert_eq!(isolated_json, "\"ISOLATED_MARGIN\"");
    }

    #[test]
    fn test_request_with_various_symbols() {
        let symbols = ["XBTUSDTM", "ETHUSDTM", "ADAUSDTM", "DOTUSDTM"];

        for symbol in symbols.iter() {
            let request = ChangeMarginModeRequest {
                symbol: symbol.to_string(),
                margin_mode: MarginMode::CrossMargin,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["symbol"], *symbol);
            assert_eq!(json["marginMode"], "CROSS_MARGIN");
        }
    }

    #[test]
    fn test_change_margin_mode_request_field_types() {
        let request = ChangeMarginModeRequest {
            symbol: "XBTUSDTM".to_string(),
            margin_mode: MarginMode::IsolatedMargin,
        };

        // Verify field types through serialization
        let json = serde_json::to_value(&request).unwrap();

        assert!(json["symbol"].is_string());
        assert!(json["marginMode"].is_string());
        assert_eq!(json["symbol"], "XBTUSDTM");
        assert_eq!(json["marginMode"], "ISOLATED_MARGIN");
    }

    #[test]
    fn test_response_field_types() {
        let json = r#"{"result":true}"#;
        let response: ChangeMarginModeResponse = serde_json::from_str(json).unwrap();

        // Verify the field is boolean
        let serialized = serde_json::to_value(&response).unwrap();
        assert!(serialized["result"].is_boolean());
        assert!(serialized["result"].as_bool().unwrap_or(false));
    }

    #[test]
    fn test_camel_case_serialization() {
        let request = ChangeMarginModeRequest {
            symbol: "XBTUSDTM".to_string(),
            margin_mode: MarginMode::CrossMargin,
        };

        let json = serde_json::to_value(&request).unwrap();

        // Verify camelCase conversion
        assert!(json.get("marginMode").is_some());
        assert!(json.get("margin_mode").is_none()); // Should not exist
    }

    #[test]
    fn test_empty_symbol_handling() {
        let request = ChangeMarginModeRequest {
            symbol: String::new(),
            margin_mode: MarginMode::CrossMargin,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"symbol\":\"\""));
    }

    #[test]
    fn test_margin_mode_round_trip() {
        let modes = [MarginMode::CrossMargin, MarginMode::IsolatedMargin];

        for mode in modes.iter() {
            let serialized = serde_json::to_string(mode).unwrap();
            let deserialized: MarginMode = serde_json::from_str(&serialized).unwrap();
            assert_eq!(*mode, deserialized);
        }
    }

    #[test]
    fn test_request_serialization_completeness() {
        let request = ChangeMarginModeRequest {
            symbol: "XBTUSDTM".to_string(),
            margin_mode: MarginMode::IsolatedMargin,
        };

        let json = serde_json::to_value(&request).unwrap();

        // Ensure all fields are present
        assert_eq!(json.as_object().unwrap().len(), 2);
        assert!(json.get("symbol").is_some());
        assert!(json.get("marginMode").is_some());
    }
}
