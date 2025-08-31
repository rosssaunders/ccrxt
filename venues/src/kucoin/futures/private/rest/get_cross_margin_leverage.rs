use serde::{Deserialize, Serialize};

use crate::kucoin::futures::{ResponseHeaders, RestResponse, Result, private_client::RestClient};

/// Endpoint URL for get cross margin leverage
const GET_CROSS_MARGIN_LEVERAGE_ENDPOINT: &str = "/api/v2/getCrossUserLeverage";

/// Request parameters for getting cross margin leverage.
#[derive(Debug, Clone, Serialize)]
pub struct GetCrossMarginLeverageRequest {
    /// Symbol of the contract (e.g., "XBTUSDTM"). Required parameter.
    pub symbol: String,
}

/// Response from the get cross margin leverage endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCrossMarginLeverageResponse {
    /// Contract symbol (e.g., "XBTUSDTM").
    pub symbol: String,

    /// Current leverage value (e.g., "3", "10", "20").
    pub leverage: String,
}

impl RestClient {
    /// Get Cross Margin Leverage
    ///
    /// Get the current cross margin leverage for a specific symbol.
    ///
    /// [docs](https://www.kucoin.com/docs-new/rest/futures-trading/positions/get-cross-margin-leverage)
    ///
    /// Rate limit: 9
    ///
    /// # Arguments
    /// * `request` - The get cross margin leverage request parameters
    ///
    /// # Returns
    /// Current cross margin leverage information for the symbol
    pub async fn get_cross_margin_leverage(
        &self,
        request: GetCrossMarginLeverageRequest,
    ) -> Result<(
        RestResponse<GetCrossMarginLeverageResponse>,
        ResponseHeaders,
    )> {
        self.get_with_request(GET_CROSS_MARGIN_LEVERAGE_ENDPOINT, &request)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_cross_margin_leverage_request_creation() {
        let request = GetCrossMarginLeverageRequest {
            symbol: "XBTUSDTM".to_string(),
        };
        assert_eq!(request.symbol, "XBTUSDTM");
    }

    #[test]
    fn test_get_cross_margin_leverage_response_deserialization() {
        let json = r#"{
            "symbol": "XBTUSDTM",
            "leverage": "3"
        }"#;

        let response: GetCrossMarginLeverageResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.symbol, "XBTUSDTM");
        assert_eq!(response.leverage, "3");
    }

    #[test]
    fn test_request_serialization() {
        let request = GetCrossMarginLeverageRequest {
            symbol: "ETHUSDTM".to_string(),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["symbol"], "ETHUSDTM");
        assert!(json["symbol"].is_string());
    }

    #[test]
    fn test_response_with_various_leverage_values() {
        let leverage_values = ["1", "2", "3", "5", "10", "20", "50", "100"];

        for leverage in leverage_values.iter() {
            let json = format!(
                r#"{{
                "symbol": "XBTUSDTM",
                "leverage": "{}"
            }}"#,
                leverage
            );

            let response: GetCrossMarginLeverageResponse = serde_json::from_str(&json).unwrap();
            assert_eq!(response.symbol, "XBTUSDTM");
            assert_eq!(response.leverage, *leverage);
        }
    }

    #[test]
    fn test_various_symbols() {
        let symbols = ["XBTUSDTM", "ETHUSDTM", "ADAUSDTM", "DOTUSDTM"];

        for symbol in symbols.iter() {
            let request = GetCrossMarginLeverageRequest {
                symbol: symbol.to_string(),
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["symbol"], *symbol);
        }
    }

    #[test]
    fn test_response_field_types() {
        let json = r#"{
            "symbol": "XBTUSDTM",
            "leverage": "10"
        }"#;

        let response: GetCrossMarginLeverageResponse = serde_json::from_str(json).unwrap();
        let serialized = serde_json::to_value(&response).unwrap();

        assert!(serialized["symbol"].is_string());
        assert!(serialized["leverage"].is_string());
    }

    #[test]
    fn test_leverage_string_format() {
        // Test that leverage is always returned as string
        let json = r#"{
            "symbol": "XBTUSDTM",
            "leverage": "25"
        }"#;

        let response: GetCrossMarginLeverageResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.leverage, "25");

        // Verify it's a string, not a number
        let parsed_value: serde_json::Value = serde_json::from_str(json).unwrap();
        assert!(parsed_value["leverage"].is_string());
    }

    #[test]
    fn test_request_with_empty_symbol() {
        let request = GetCrossMarginLeverageRequest {
            symbol: String::new(),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"symbol\":\"\""));
    }

    #[test]
    fn test_response_structure_completeness() {
        let json = r#"{
            "symbol": "XBTUSDTM",
            "leverage": "5"
        }"#;

        let response: GetCrossMarginLeverageResponse = serde_json::from_str(json).unwrap();
        let serialized = serde_json::to_value(&response).unwrap();

        // Should have exactly 2 fields
        assert_eq!(serialized.as_object().unwrap().len(), 2);
        assert!(serialized.get("symbol").is_some());
        assert!(serialized.get("leverage").is_some());
    }

    #[test]
    fn test_high_leverage_values() {
        let high_leverages = ["50", "75", "100", "125"];

        for leverage in high_leverages.iter() {
            let json = format!(
                r#"{{
                "symbol": "XBTUSDTM",
                "leverage": "{}"
            }}"#,
                leverage
            );

            let response: GetCrossMarginLeverageResponse = serde_json::from_str(&json).unwrap();
            assert_eq!(response.leverage, *leverage);
        }
    }

    #[test]
    fn test_decimal_leverage_values() {
        let decimal_leverages = ["1.5", "2.5", "3.3", "4.7"];

        for leverage in decimal_leverages.iter() {
            let json = format!(
                r#"{{
                "symbol": "XBTUSDTM",
                "leverage": "{}"
            }}"#,
                leverage
            );

            let response: GetCrossMarginLeverageResponse = serde_json::from_str(&json).unwrap();
            assert_eq!(response.leverage, *leverage);
        }
    }

    #[test]
    fn test_endpoint_constant() {
        assert_eq!(
            GET_CROSS_MARGIN_LEVERAGE_ENDPOINT,
            "/api/v2/getCrossUserLeverage"
        );
    }

    #[test]
    fn test_request_serialization_format() {
        let request = GetCrossMarginLeverageRequest {
            symbol: "XBTUSDTM".to_string(),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert_eq!(json, r#"{"symbol":"XBTUSDTM"}"#);
    }

    #[test]
    fn test_long_symbol_names() {
        let long_symbols = [
            "VERYLONGSYMBOLNAME",
            "ANOTHERLONGSYMBOL123",
            "EXTREMELYLONGSYMBOLNAMEHERE",
        ];

        for symbol in long_symbols.iter() {
            let request = GetCrossMarginLeverageRequest {
                symbol: symbol.to_string(),
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["symbol"], *symbol);
        }
    }
}
