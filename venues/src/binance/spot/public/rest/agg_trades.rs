use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::binance::spot::RestResult;

use super::client::RestClient;

/// Request parameters for aggregate trades
#[derive(Debug, Clone, Serialize, Default)]
pub struct AggTradesRequest {
    /// Trading pair symbol
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// ID to get aggregate trades from INCLUSIVE
    #[serde(rename = "fromId", skip_serializing_if = "Option::is_none")]
    pub from_id: Option<u64>,

    /// Timestamp in ms to get aggregate trades from INCLUSIVE
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,

    /// Timestamp in ms to get aggregate trades until INCLUSIVE
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,

    /// Number of trades to return. Default 500, Max 1000
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

/// Aggregate trade information
#[derive(Debug, Clone, Deserialize)]
pub struct AggTrade {
    /// Aggregate trade ID
    #[serde(rename = "a")]
    pub agg_trade_id: u64,

    /// Trade price
    #[serde(rename = "p")]
    pub price: Decimal,

    /// Trade quantity
    #[serde(rename = "q")]
    pub quantity: Decimal,

    /// First trade ID
    #[serde(rename = "f")]
    pub first_trade_id: u64,

    /// Last trade ID
    #[serde(rename = "l")]
    pub last_trade_id: u64,

    /// Trade time
    #[serde(rename = "T")]
    pub timestamp: u64,

    /// Was the buyer the maker?
    #[serde(rename = "m")]
    pub is_buyer_maker: bool,

    /// Was this trade the best price match?
    #[serde(rename = "M")]
    pub is_best_match: bool,
}

impl RestClient {
    /// Get compressed/aggregate trades
    ///
    /// Trades that fill at the time, from the same order, with the same price will have the quantity aggregated.
    ///
    /// See: [API Documentation](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/public-api-endpoints#compressed-aggregate-trades-list)
    /// Method: GET /api/v3/aggTrades
    /// Weight: 2
    /// Security: None
    pub async fn get_agg_trades(&self, params: AggTradesRequest) -> RestResult<Vec<AggTrade>> {
        let query_string = serde_urlencoded::to_string(&params)
            .map_err(|e| crate::binance::spot::Errors::Error(format!("URL encoding error: {e}")))?;

        self.send_request(
            "/api/v3/aggTrades",
            reqwest::Method::GET,
            Some(&query_string),
            2,
        )
        .await
    }
}
