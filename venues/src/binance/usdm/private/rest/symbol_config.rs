use serde::{Deserialize, Serialize};

use super::UsdmClient;
use crate::binance::usdm::RestResult;
use crate::binance::usdm::enums::MarginType;

/// Endpoint path for symbol configuration.
const SYMBOL_CONFIG_ENDPOINT: &str = "/fapi/v1/symbolConfig";

/// Request parameters for the Symbol Config endpoint.
///
/// Used to query symbol configuration for a specific symbol or all symbols.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetSymbolConfigRequest {
    /// Trading symbol to query (optional).
    /// If not provided, returns configuration for all symbols.
    /// Must match a valid trading symbol (e.g., "BTCUSDT").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,

    /// Optional window for request validity in milliseconds.
    /// If not provided, default is 5000ms.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Request timestamp in milliseconds since epoch.
    pub timestamp: u64,
}

/// Symbol configuration response for a single symbol.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SymbolConfigResponse {
    /// Trading symbol.
    pub symbol: String,

    /// Margin type for the symbol.
    pub margin_type: MarginType,

    /// Whether auto add margin is enabled for the symbol.
    pub is_auto_add_margin: bool,

    /// Current leverage for the symbol.
    pub leverage: u32,

    /// Maximum notional value allowed for the symbol.
    pub max_notional_value: String,
}

impl UsdmClient {
    /// Symbol Configuration (USER_DATA)
    ///
    /// Get current account symbol configuration.
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/usds-margined-futures/account/rest-api/Symbol-Config
    ///
    /// Rate limit: 5
    ///
    /// # Arguments
    /// * `params` - The request parameters for symbol config
    ///
    /// # Returns
    /// Returns `Vec<SymbolConfigResponse>` containing symbol configurations.
    pub async fn get_symbol_config(
        &self,
        params: GetSymbolConfigRequest,
    ) -> RestResult<Vec<SymbolConfigResponse>> {
        self.send_signed_request(
            SYMBOL_CONFIG_ENDPOINT,
            reqwest::Method::GET,
            Some(&params),
            5,
            false,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_request_serialization() {
        let req = GetSymbolConfigRequest {
            symbol: Some("BTCUSDT".to_string()),
            recv_window: Some(5000),
            timestamp: 1640995200000,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("BTCUSDT"));
        assert!(json.contains("recvWindow"));
        assert!(json.contains("timestamp"));
    }

    #[test]
    fn test_response_deserialization() {
        let json = r#"{
            \"symbol\": \"BTCUSDT\",
            \"marginType\": \"CROSSED\",
            \"isAutoAddMargin\": true,
            \"leverage\": 21,
            \"maxNotionalValue\": \"1000000\"
        }"#;
        let resp: SymbolConfigResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.symbol, "BTCUSDT");
        assert_eq!(resp.leverage, 21);
        assert_eq!(resp.max_notional_value, "1000000");
    }
}
