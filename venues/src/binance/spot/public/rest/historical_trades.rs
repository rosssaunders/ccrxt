use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::binance::spot::RestResult;

use super::client::RestClient;

/// Request parameters for historical trades
#[derive(Debug, Clone, Serialize)]
pub struct HistoricalTradesRequest {
    /// Trading pair symbol
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Number of trades to return. Default 500, Max 1000
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    /// TradeId to fetch from. Default gets most recent trades
    #[serde(rename = "fromId", skip_serializing_if = "Option::is_none")]
    pub from_id: Option<u64>,
}

/// Historical trade information
#[derive(Debug, Clone, Deserialize)]
pub struct HistoricalTrade {
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
    /// Get historical trades
    ///
    /// Get older market trades.
    ///
    /// See: [API Documentation](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/public-api-endpoints#old-trade-lookup)
    /// Method: GET /api/v3/historicalTrades
    /// Weight: 25
    /// Security: None
    pub async fn get_historical_trades(
        &self,
        params: HistoricalTradesRequest,
    ) -> RestResult<Vec<HistoricalTrade>> {
        let query_string = serde_urlencoded::to_string(&params)
            .map_err(|e| crate::binance::spot::Errors::Error(format!("URL encoding error: {e}")))?;

        self.send_request(
            "/api/v3/historicalTrades",
            reqwest::Method::GET,
            Some(&query_string),
            25,
        )
        .await
    }
}
