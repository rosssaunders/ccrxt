use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::kucoin::spot::{Market, ResponseHeaders, RestResponse, Result};

const ALL_SYMBOLS_ENDPOINT: &str = "/api/v1/symbols";

/// Request for getting all symbols
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetAllSymbolsRequest {
    /// Market filter (optional)
    pub market: Option<Market>,
}

/// Symbol information response
#[derive(Debug, Clone, Deserialize)]
pub struct SymbolInfo {
    /// Symbol name
    pub symbol: String,

    /// Symbol name for display
    pub name: String,

    /// Base currency
    #[serde(rename = "baseCurrency")]
    pub base_currency: String,

    /// Quote currency
    #[serde(rename = "quoteCurrency")]
    pub quote_currency: String,

    /// Fee currency
    #[serde(rename = "feeCurrency")]
    pub fee_currency: String,

    /// Market (e.g., "BTC", "ETH", "USDT")
    pub market: String,

    /// Base minimum size
    #[serde(rename = "baseMinSize")]
    pub base_min_size: String,

    /// Quote minimum size
    #[serde(rename = "quoteMinSize")]
    pub quote_min_size: String,

    /// Base maximum size
    #[serde(rename = "baseMaxSize")]
    pub base_max_size: String,

    /// Quote maximum size
    #[serde(rename = "quoteMaxSize")]
    pub quote_max_size: String,

    /// Base increment (minimum order size increment)
    #[serde(rename = "baseIncrement")]
    pub base_increment: String,

    /// Quote increment (minimum price increment)
    #[serde(rename = "quoteIncrement")]
    pub quote_increment: String,

    /// Price increment (tick size)
    #[serde(rename = "priceIncrement")]
    pub price_increment: String,

    /// Price limit rate
    #[serde(rename = "priceLimitRate")]
    pub price_limit_rate: String,

    /// Whether trading is enabled
    #[serde(rename = "enableTrading")]
    pub enable_trading: bool,

    /// Whether this is a margin trading symbol
    #[serde(rename = "isMarginEnabled")]
    pub is_margin_enabled: bool,

    /// Fee category (optional as it might not always be present)
    #[serde(rename = "feeCategory")]
    pub fee_category: Option<i32>,
}

impl RestClient {
    /// Get information for all available symbols
    ///
    /// Reference: https://docs.kucoin.com/#get-symbols-list
    pub async fn get_all_symbols(
        &self,
        request: GetAllSymbolsRequest,
    ) -> Result<(Vec<SymbolInfo>, ResponseHeaders)> {
        let (response, headers): (RestResponse<Vec<SymbolInfo>>, ResponseHeaders) = self
            .get_with_request(ALL_SYMBOLS_ENDPOINT, &request)
            .await?;

        Ok((response.data, headers))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_symbols_request_default() {
        let request = GetAllSymbolsRequest::default();
        assert!(request.market.is_none());
    }
}
