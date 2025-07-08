use serde::{Deserialize, Serialize};

use crate::binance::coinm::{RestResult, public::rest::RestClient};

/// Request parameters for the aggregate trades endpoint.
#[derive(Debug, Clone, Serialize, Default)]
pub struct AggregateTradesRequest {
    /// Trading symbol (e.g., "BTCUSD_PERP").
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// ID to get aggregate trades from INCLUSIVE.
    #[serde(rename = "fromId", skip_serializing_if = "Option::is_none")]
    pub from_id: Option<u64>,

    /// Timestamp in ms to get aggregate trades from INCLUSIVE.
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,

    /// Timestamp in ms to get aggregate trades until INCLUSIVE.
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,

    /// Default 500; max 1000.
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

/// Represents a single aggregate trade.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AggregateTrade {
    /// Aggregate trade ID.
    #[serde(rename = "a")]
    pub agg_trade_id: u64,

    /// Price.
    #[serde(rename = "p")]
    pub price: String,

    /// Quantity.
    #[serde(rename = "q")]
    pub quantity: String,

    /// First trade ID.
    #[serde(rename = "f")]
    pub first_trade_id: u64,

    /// Last trade ID.
    #[serde(rename = "l")]
    pub last_trade_id: u64,

    /// Timestamp.
    #[serde(rename = "T")]
    pub timestamp: u64,

    /// Was the buyer the maker?
    #[serde(rename = "m")]
    pub is_buyer_maker: bool,
}

impl RestClient {
    /// Get compressed, aggregate trades.
    ///
    /// Market trades that fill in 100ms with the same price and the same taking side
    /// will have the quantity aggregated.
    ///
    /// Notes:
    /// - Support querying futures trade histories that are not older than one year
    /// - If both startTime and endTime are sent, time between startTime and endTime
    ///   must be less than 1 hour.
    /// - If fromId, startTime, and endTime are not sent, the most recent aggregate
    ///   trades will be returned.
    /// - Only market trades will be aggregated and returned, which means the insurance
    ///   fund trades and ADL trades won't be aggregated.
    /// - Sending both startTime/endTime and fromId might cause response timeout,
    ///   please send either fromId or startTime/endTime
    ///
    /// [Official API docs](https://developers.binance.com/docs/derivatives/coin-margined-futures/market-data/rest-api/Compressed-Aggregate-Trades-List)
    ///
    /// Weight: 20
    pub async fn get_aggregate_trades(
        &self,
        params: AggregateTradesRequest,
    ) -> RestResult<Vec<AggregateTrade>> {
        self.send_request("/dapi/v1/aggTrades", reqwest::Method::GET, Some(params), 20)
            .await
    }
}
