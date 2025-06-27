use crate::binance::coinm::{RestResult, RestResponse};
use crate::binance::coinm::public::rest::RestClient;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

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
    /// Price
    pub price: Decimal,
    /// Timestamp
    pub time: i64,
}

impl RestClient {
    /// Get symbol price ticker
    /// 
    /// Weight: 1 for a single symbol; 2 when the symbol parameter is omitted
    pub async fn get_ticker_price(&self, params: TickerPriceRequest) -> RestResult<Vec<TickerPrice>> {
        let weight = if params.symbol.is_some() { 1 } else { 2 };
        
        if params.symbol.is_some() {
            // Single ticker
            let response = self.send_request(
                "/dapi/v1/ticker/price",
                reqwest::Method::GET,
                Some(params),
                weight,
            )
            .await?;
            Ok(RestResponse {
                data: vec![response.data],
                request_duration: response.request_duration,
                headers: response.headers,
            })
        } else {
            // All tickers
            self.send_request(
                "/dapi/v1/ticker/price",
                reqwest::Method::GET,
                Some(params),
                weight,
            )
            .await
        }
    }
}
