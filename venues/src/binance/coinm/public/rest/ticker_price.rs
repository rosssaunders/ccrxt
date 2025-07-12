use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::binance::coinm::{RestResult, public::rest::RestClient};

/// Parameters for Symbol Price Ticker
#[derive(Debug, Clone, Serialize, Default)]
pub struct TickerPriceRequest {
    /// Symbol name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// Pair name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pair: Option<String>,
}

/// Symbol price ticker
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TickerPrice {
    /// Symbol name
    pub symbol: String,
    /// Pair name
    pub ps: String,
    /// Price
    pub price: Decimal,
    /// Timestamp
    pub time: i64,
}

impl RestClient {
    /// Get symbol price ticker
    ///
    /// Weight: 1 for a single symbol; 2 when the symbol parameter is omitted
    pub async fn get_ticker_price(
        &self,
        params: TickerPriceRequest,
    ) -> RestResult<Vec<TickerPrice>> {
        let weight = if params.symbol.is_some() { 1 } else { 2 };

        // The API always returns an array, even for single symbols
        self.send_request(
            "/dapi/v1/ticker/price",
            reqwest::Method::GET,
            Some(params),
            weight,
        )
        .await
    }
}
