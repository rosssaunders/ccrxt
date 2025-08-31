use serde::{Deserialize, Serialize};

use crate::kucoin::futures::{
    MarginMode, ResponseHeaders, RestResponse, Result, private_client::RestClient,
};

/// Endpoint URL for get margin mode
const GET_MARGIN_MODE_ENDPOINT: &str = "/api/v2/position/getMarginMode";

/// Request parameters for getting margin mode.
#[derive(Debug, Clone, Serialize)]
pub struct GetMarginModeRequest {
    /// Trading symbol (e.g., "XBTUSDTM"). Required parameter.
    pub symbol: String,
}

/// Response from the get margin mode endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarginModeResponse {
    /// Trading symbol (e.g., "XBTUSDTM").
    pub symbol: String,

    /// Current margin mode (CROSS_MARGIN or ISOLATED_MARGIN).
    pub margin_mode: MarginMode,

    /// Cross margin leverage value (e.g., "10").
    pub cross_margin_leverage: String,

    /// Isolated margin leverage value (e.g., "20").
    pub isolated_margin_leverage: String,
}

impl RestClient {
    /// Get Margin Mode
    ///
    /// Get the margin mode of a symbol and check the leverage of cross margin
    /// and isolated margin.
    ///
    /// [docs](https://www.kucoin.com/docs-new/rest/futures-trading/positions/get-margin-mode)
    ///
    /// Rate limit: 9
    ///
    /// # Arguments
    /// * `request` - The get margin mode request parameters
    ///
    /// # Returns
    /// Margin mode information including current mode and leverage values
    pub async fn get_margin_mode(
        &self,
        request: GetMarginModeRequest,
    ) -> Result<(MarginModeResponse, ResponseHeaders)> {
        let (response, headers): (RestResponse<MarginModeResponse>, ResponseHeaders) = self
            .get_with_request(GET_MARGIN_MODE_ENDPOINT, &request)
            .await?;

        Ok((response.data, headers))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_margin_mode_request_creation() {
        let request = GetMarginModeRequest {
            symbol: "XBTUSDTM".to_string(),
        };
        assert_eq!(request.symbol, "XBTUSDTM");
    }

    #[test]
    fn test_margin_mode_response_deserialization_cross() {
        let json = r#"{
            "symbol": "XBTUSDTM",
            "marginMode": "CROSS_MARGIN",
            "crossMarginLeverage": "10",
            "isolatedMarginLeverage": "20"
        }"#;

        let response: MarginModeResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.symbol, "XBTUSDTM");
        assert_eq!(response.margin_mode, MarginMode::CrossMargin);
        assert_eq!(response.cross_margin_leverage, "10");
        assert_eq!(response.isolated_margin_leverage, "20");
    }

    #[test]
    fn test_margin_mode_response_deserialization_isolated() {
        let json = r#"{
            "symbol": "ETHUSDTM",
            "marginMode": "ISOLATED_MARGIN",
            "crossMarginLeverage": "5",
            "isolatedMarginLeverage": "15"
        }"#;

        let response: MarginModeResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.symbol, "ETHUSDTM");
        assert_eq!(response.margin_mode, MarginMode::IsolatedMargin);
        assert_eq!(response.cross_margin_leverage, "5");
        assert_eq!(response.isolated_margin_leverage, "15");
    }

    #[test]
    fn test_get_margin_mode_endpoint() {
        assert_eq!(GET_MARGIN_MODE_ENDPOINT, "/api/v2/position/getMarginMode");
    }

    #[test]
    fn test_request_serialization() {
        let request = GetMarginModeRequest {
            symbol: "ADAUSDTM".to_string(),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["symbol"], "ADAUSDTM");
        assert!(json["symbol"].is_string());
    }

    #[test]
    fn test_various_symbols() {
        let symbols = ["XBTUSDTM", "ETHUSDTM", "ADAUSDTM", "DOTUSDTM"];

        for symbol in symbols.iter() {
            let request = GetMarginModeRequest {
                symbol: symbol.to_string(),
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["symbol"], *symbol);
        }
    }

    #[test]
    fn test_margin_mode_variants() {
        let modes = [
            ("CROSS_MARGIN", MarginMode::CrossMargin),
            ("ISOLATED_MARGIN", MarginMode::IsolatedMargin),
        ];

        for (mode_str, expected_mode) in modes.iter() {
            let json = format!(
                r#"{{
                "symbol": "XBTUSDTM",
                "marginMode": "{}",
                "crossMarginLeverage": "10",
                "isolatedMarginLeverage": "20"
            }}"#,
                mode_str
            );

            let response: MarginModeResponse = serde_json::from_str(&json).unwrap();
            assert_eq!(response.margin_mode, *expected_mode);
        }
    }

    #[test]
    fn test_leverage_string_values() {
        let leverage_values = ["1", "3", "5", "10", "20", "50", "100"];

        for leverage in leverage_values.iter() {
            let json = format!(
                r#"{{
                "symbol": "XBTUSDTM",
                "marginMode": "CROSS_MARGIN",
                "crossMarginLeverage": "{}",
                "isolatedMarginLeverage": "{}"
            }}"#,
                leverage, leverage
            );

            let response: MarginModeResponse = serde_json::from_str(&json).unwrap();
            assert_eq!(response.cross_margin_leverage, *leverage);
            assert_eq!(response.isolated_margin_leverage, *leverage);
        }
    }

    #[test]
    fn test_response_field_types() {
        let json = r#"{
            "symbol": "XBTUSDTM",
            "marginMode": "CROSS_MARGIN",
            "crossMarginLeverage": "10",
            "isolatedMarginLeverage": "20"
        }"#;

        let response: MarginModeResponse = serde_json::from_str(json).unwrap();
        let serialized = serde_json::to_value(&response).unwrap();

        assert!(serialized["symbol"].is_string());
        assert!(serialized["marginMode"].is_string());
        assert!(serialized["crossMarginLeverage"].is_string());
        assert!(serialized["isolatedMarginLeverage"].is_string());
    }

    #[test]
    fn test_camel_case_conversion() {
        let json = r#"{
            "symbol": "XBTUSDTM",
            "marginMode": "CROSS_MARGIN",
            "crossMarginLeverage": "10",
            "isolatedMarginLeverage": "20"
        }"#;

        let response: MarginModeResponse = serde_json::from_str(json).unwrap();
        let serialized = serde_json::to_value(&response).unwrap();

        // Verify camelCase field names
        assert!(serialized.get("marginMode").is_some());
        assert!(serialized.get("crossMarginLeverage").is_some());
        assert!(serialized.get("isolatedMarginLeverage").is_some());

        // Verify snake_case fields don't exist
        assert!(serialized.get("margin_mode").is_none());
        assert!(serialized.get("cross_margin_leverage").is_none());
        assert!(serialized.get("isolated_margin_leverage").is_none());
    }

    #[test]
    fn test_decimal_leverage_values() {
        let json = r#"{
            "symbol": "XBTUSDTM",
            "marginMode": "ISOLATED_MARGIN",
            "crossMarginLeverage": "3.5",
            "isolatedMarginLeverage": "7.2"
        }"#;

        let response: MarginModeResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.cross_margin_leverage, "3.5");
        assert_eq!(response.isolated_margin_leverage, "7.2");
    }

    #[test]
    fn test_response_structure_completeness() {
        let json = r#"{
            "symbol": "XBTUSDTM",
            "marginMode": "CROSS_MARGIN",
            "crossMarginLeverage": "10",
            "isolatedMarginLeverage": "20"
        }"#;

        let response: MarginModeResponse = serde_json::from_str(json).unwrap();
        let serialized = serde_json::to_value(&response).unwrap();

        // Should have exactly 4 fields
        assert_eq!(serialized.as_object().unwrap().len(), 4);
        assert!(serialized.get("symbol").is_some());
        assert!(serialized.get("marginMode").is_some());
        assert!(serialized.get("crossMarginLeverage").is_some());
        assert!(serialized.get("isolatedMarginLeverage").is_some());
    }

    #[test]
    fn test_empty_symbol_handling() {
        let request = GetMarginModeRequest {
            symbol: String::new(),
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
    fn test_high_leverage_values() {
        let json = r#"{
            "symbol": "XBTUSDTM",
            "marginMode": "CROSS_MARGIN",
            "crossMarginLeverage": "125",
            "isolatedMarginLeverage": "100"
        }"#;

        let response: MarginModeResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.cross_margin_leverage, "125");
        assert_eq!(response.isolated_margin_leverage, "100");
    }

    #[test]
    fn test_request_serialization_format() {
        let request = GetMarginModeRequest {
            symbol: "XBTUSDTM".to_string(),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert_eq!(json, r#"{"symbol":"XBTUSDTM"}"#);
    }
}
