use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::binance::options::RestResult;

use super::client::RestClient;

/// Request parameters for symbol price ticker
#[derive(Debug, Clone, Serialize)]
pub struct SymbolPriceTickerRequest {
    /// Spot pair (Option contract underlying asset, e.g BTCUSDT)
    #[serde(rename = "underlying")]
    pub underlying: String,
}

/// Symbol price ticker response
#[derive(Debug, Clone, Deserialize)]
pub struct SymbolPriceTickerResponse {
    /// Time
    #[serde(rename = "time")]
    pub time: u64,

    /// Current spot index price
    #[serde(rename = "indexPrice")]
    pub index_price: Decimal,
}

impl RestClient {
    /// Get symbol price ticker
    ///
    /// Returns spot index price for option underlying.
    ///
    /// See: [API Documentation](https://developers.binance.com/docs/derivatives/option/market-data/Symbol-Price-Ticker)
    /// Method: GET /eapi/v1/index
    /// Weight: 1
    /// Security: None
    pub async fn get_symbol_price_ticker(
        &self,
        params: SymbolPriceTickerRequest,
    ) -> RestResult<SymbolPriceTickerResponse> {
        let query_string = serde_urlencoded::to_string(&params).map_err(|e| {
            crate::binance::options::Errors::Error(format!("URL encoding error: {e}"))
        })?;

        self.send_request(
            "/eapi/v1/index",
            reqwest::Method::GET,
            Some(&query_string),
            None,
            1,
        )
        .await
    }
}
