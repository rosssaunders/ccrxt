use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::binance::spot::RestResult;

use super::client::RestClient;

/// Request parameters for symbol price ticker
#[derive(Debug, Clone, Serialize, Default)]
pub struct TickerPriceRequest {
    /// Single symbol
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,

    /// Multiple symbols (JSON array format)
    #[serde(rename = "symbols", skip_serializing_if = "Option::is_none")]
    pub symbols: Option<String>,
}

/// Symbol price ticker
#[derive(Debug, Clone, Deserialize)]
pub struct TickerPrice {
    /// Symbol
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Price
    #[serde(rename = "price")]
    pub price: Decimal,
}

impl RestClient {
    /// Get symbol price ticker
    ///
    /// Latest price for a symbol or symbols.
    ///
    /// See: [API Documentation](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/public-api-endpoints#symbol-price-ticker)
    /// Method: GET /api/v3/ticker/price
    /// Weight: 2 for single symbol, 4 for multiple symbols
    /// Security: None
    pub async fn get_price_ticker(
        &self,
        params: Option<TickerPriceRequest>,
    ) -> RestResult<serde_json::Value> {
        let (query_string, weight) = if let Some(p) = params {
            let weight = if p.symbol.is_some() {
                2 // Single symbol
            } else {
                4 // Multiple symbols or default
            };

            let qs = serde_urlencoded::to_string(&p).map_err(|e| {
                crate::binance::spot::Errors::Error(format!("URL encoding error: {e}"))
            })?;
            (Some(qs), weight)
        } else {
            (None, 4) // All symbols
        };

        self.send_request(
            "/api/v3/ticker/price",
            reqwest::Method::GET,
            query_string.as_deref(),
            weight,
        )
        .await
    }
}
