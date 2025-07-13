use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::kucoin::spot::{ResponseHeaders, RestResponse, Result};

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
    /// Get cross margin symbols configuration
    ///
    /// This endpoint allows querying the configuration of cross margin symbols.
    /// If `symbol` is provided, only the specified symbol will be queried.
    pub async fn get_margin_symbols(
        &self,
        request: GetMarginSymbolsRequest,
    ) -> Result<(GetMarginSymbolsResponse, ResponseHeaders)> {
        let mut params = HashMap::new();
        if let Some(symbol) = request.symbol {
            params.insert("symbol".to_string(), symbol);
        }
        let (response, headers): (RestResponse<GetMarginSymbolsResponse>, ResponseHeaders) = self
            .get(
                "/api/v3/margin/symbols",
                if params.is_empty() {
                    None
                } else {
                    Some(params)
                },
            )
            .await?;
        Ok((response.data, headers))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
