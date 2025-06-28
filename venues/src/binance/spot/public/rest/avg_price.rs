use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::binance::spot::RestResult;

use super::client::RestClient;

/// Request parameters for average price
#[derive(Debug, Clone, Serialize)]
pub struct AvgPriceRequest {
    /// Trading pair symbol
    #[serde(rename = "symbol")]
    pub symbol: String,
}

/// Average price response
#[derive(Debug, Clone, Deserialize)]
pub struct AvgPriceResponse {
    /// Minutes for which the average price is calculated
    #[serde(rename = "mins")]
    pub mins: u32,

    /// Average price
    #[serde(rename = "price")]
    pub price: Decimal,

    /// Close time of the interval
    #[serde(rename = "closeTime")]
    pub close_time: u64,
}

impl RestClient {
    /// Get current average price
    ///
    /// Current average price for a symbol.
    ///
    /// See: [API Documentation](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/public-api-endpoints#current-average-price)
    /// Method: GET /api/v3/avgPrice
    /// Weight: 2
    /// Security: None
    pub async fn get_avg_price(&self, params: AvgPriceRequest) -> RestResult<AvgPriceResponse> {
        let query_string = serde_urlencoded::to_string(&params)
            .map_err(|e| crate::binance::spot::Errors::Error(format!("URL encoding error: {e}")))?;

        self.send_request(
            "/api/v3/avgPrice",
            reqwest::Method::GET,
            Some(&query_string),
            2,
        )
        .await
    }
}
