use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::kucoin::spot::{ResponseHeaders, RestResponse, Result};

/// Endpoint URL for cross margin symbols configuration
const GET_MARGIN_SYMBOLS_ENDPOINT: &str = "/api/v3/margin/symbols";

/// Request for getting cross margin symbols
#[derive(Debug, Clone, Serialize)]
pub struct GetMarginSymbolsRequest {
    /// Optional symbol to query (e.g., "BTC-USDT")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
}

/// Cross margin symbol information
#[derive(Debug, Clone, Deserialize)]
pub struct MarginSymbolInfo {
    pub symbol: String,

    pub name: String,

    #[serde(rename = "enableTrading")]
    pub enable_trading: bool,

    pub market: String,

    #[serde(rename = "baseCurrency")]
    pub base_currency: String,

    #[serde(rename = "quoteCurrency")]
    pub quote_currency: String,

    #[serde(rename = "baseIncrement")]
    pub base_increment: String,

    #[serde(rename = "baseMinSize")]
    pub base_min_size: String,

    #[serde(rename = "quoteIncrement")]
    pub quote_increment: String,

    #[serde(rename = "quoteMinSize")]
    pub quote_min_size: String,

    #[serde(rename = "baseMaxSize")]
    pub base_max_size: String,
}

/// Response for getting cross margin symbols
#[derive(Debug, Clone, Deserialize)]
pub struct GetMarginSymbolsResponse {
    pub timestamp: i64,

    pub items: Vec<MarginSymbolInfo>,
}

impl RestClient {
    /// Get Cross Margin Symbols Configuration
    ///
    /// Query the configuration of cross margin symbols. If `symbol` is provided,
    /// only the specified symbol will be queried.
    ///
    /// - [docs](https://www.kucoin.com/docs-new/rest/margin-trading/market-data/get-symbols-cross-margin)
    ///
    /// Rate limit: weight 3 (Public)
    ///
    /// # Arguments
    /// * `request` - Optional symbol for filtering
    ///
    /// # Returns
    /// Cross margin symbols configuration and response headers
    pub async fn get_margin_symbols(
        &self,
        request: GetMarginSymbolsRequest,
    ) -> Result<(GetMarginSymbolsResponse, ResponseHeaders)> {
        let (response, headers): (RestResponse<GetMarginSymbolsResponse>, ResponseHeaders) = self
            .get_with_request(GET_MARGIN_SYMBOLS_ENDPOINT, &request)
            .await?;
        Ok((response.data, headers))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_endpoint_constant() {
        assert_eq!(GET_MARGIN_SYMBOLS_ENDPOINT, "/api/v3/margin/symbols");
    }

    #[test]
    fn test_get_margin_symbols_request_creation() {
        let req = GetMarginSymbolsRequest {
            symbol: Some("BTC-USDT".to_string()),
        };
        assert_eq!(req.symbol, Some("BTC-USDT".to_string()));
    }

    #[test]
    fn test_margin_symbol_info_fields() {
        let info = MarginSymbolInfo {
            symbol: "BTC-USDT".to_string(),
            name: "BTC/USDT".to_string(),
            enable_trading: true,
            market: "spot".to_string(),
            base_currency: "BTC".to_string(),
            quote_currency: "USDT".to_string(),
            base_increment: "0.0000001".to_string(),
            base_min_size: "0.001".to_string(),
            quote_increment: "0.000001".to_string(),
            quote_min_size: "1".to_string(),
            base_max_size: "1000".to_string(),
        };
        assert_eq!(info.symbol, "BTC-USDT");
        assert!(info.enable_trading);
    }

    #[test]
    fn test_response_deserialization_sample() {
        let json = r#"{
            "code": "200000",
            "data": {
                "timestamp": 1700000000000,
                "items": [
                    {
                        "symbol": "BTC-USDT",
                        "name": "BTC/USDT",
                        "enableTrading": true,
                        "market": "spot",
                        "baseCurrency": "BTC",
                        "quoteCurrency": "USDT",
                        "baseIncrement": "0.0000001",
                        "baseMinSize": "0.001",
                        "quoteIncrement": "0.000001",
                        "quoteMinSize": "1",
                        "baseMaxSize": "1000"
                    }
                ]
            }
        }"#;

        let resp: RestResponse<GetMarginSymbolsResponse> = serde_json::from_str(json).unwrap();
        assert_eq!(resp.code, "200000");
        assert_eq!(resp.data.items.len(), 1);
        assert_eq!(resp.data.items[0].symbol, "BTC-USDT");
        assert!(resp.data.items[0].enable_trading);
    }
}
