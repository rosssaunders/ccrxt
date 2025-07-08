use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::binance::spot::RestResult;

/// Request parameters for symbol order book ticker
#[derive(Debug, Clone, Serialize, Default)]
pub struct TickerBookRequest {
    /// Single symbol
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,

    /// Multiple symbols (JSON array format)
    #[serde(rename = "symbols", skip_serializing_if = "Option::is_none")]
    pub symbols: Option<String>,
}

/// Symbol order book ticker
#[derive(Debug, Clone, Deserialize)]
pub struct TickerBook {
    /// Symbol
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Best bid price
    #[serde(rename = "bidPrice")]
    pub bid_price: Decimal,

    /// Best bid quantity
    #[serde(rename = "bidQty")]
    pub bid_qty: Decimal,

    /// Best ask price
    #[serde(rename = "askPrice")]
    pub ask_price: Decimal,

    /// Best ask quantity
    #[serde(rename = "askQty")]
    pub ask_qty: Decimal,
}

impl RestClient {
    /// Get symbol order book ticker
    ///
    /// Best price/qty on the order book for a symbol or symbols.
    ///
    /// See: [API Documentation](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/public-api-endpoints#symbol-order-book-ticker)
    /// Method: GET /api/v3/ticker/bookTicker
    /// Weight: 2 for single symbol, 4 for multiple symbols
    /// Security: None
    pub async fn get_book_ticker(
        &self,
        params: Option<TickerBookRequest>,
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
            "/api/v3/ticker/bookTicker",
            reqwest::Method::GET,
            query_string.as_deref(),
            weight,
        )
        .await
    }
}
