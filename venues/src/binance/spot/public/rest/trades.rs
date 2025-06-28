use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::binance::spot::RestResult;

use super::client::RestClient;

/// Request parameters for recent trades
#[derive(Debug, Clone, Serialize)]
pub struct TradesRequest {
    /// Trading pair symbol
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Number of trades to return. Default 500, Max 1000
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

/// Trade information
#[derive(Debug, Clone, Deserialize)]
pub struct Trade {
    /// Trade ID
    #[serde(rename = "id")]
    pub id: u64,

    /// Trade price
    #[serde(rename = "price")]
    pub price: Decimal,

    /// Trade quantity
    #[serde(rename = "qty")]
    pub qty: Decimal,

    /// Quote quantity
    #[serde(rename = "quoteQty")]
    pub quote_qty: Decimal,

    /// Trade time
    #[serde(rename = "time")]
    pub time: u64,

    /// Was the buyer the maker?
    #[serde(rename = "isBuyerMaker")]
    pub is_buyer_maker: bool,

    /// Was this trade the best price match?
    #[serde(rename = "isBestMatch")]
    pub is_best_match: bool,
}

impl RestClient {
    /// Get recent trades
    ///
    /// Get recent trades for a given symbol.
    ///
    /// See: [API Documentation](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/public-api-endpoints#recent-trades-list)
    /// Method: GET /api/v3/trades
    /// Weight: 25
    /// Security: None
    pub async fn get_recent_trades(&self, params: TradesRequest) -> RestResult<Vec<Trade>> {
        let query_string = serde_urlencoded::to_string(&params)
            .map_err(|e| crate::binance::spot::Errors::Error(format!("URL encoding error: {e}")))?;

        self.send_request(
            "/api/v3/trades",
            reqwest::Method::GET,
            Some(&query_string),
            25,
        )
        .await
    }
}
