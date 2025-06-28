use serde::{Deserialize, Serialize};

use crate::binance::coinm::RestResult;
use crate::binance::coinm::public::rest::RestClient;

/// Request parameters for the historical trades endpoint.
#[derive(Debug, Clone, Serialize, Default)]
pub struct HistoricalTradesRequest {
    /// Trading symbol (e.g., "BTCUSD_PERP").
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Default 100; max 500.
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    /// TradeId to fetch from. Default gets most recent trades.
    #[serde(rename = "fromId", skip_serializing_if = "Option::is_none")]
    pub from_id: Option<u64>,
}

/// Represents a single historical trade.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoricalTrade {
    /// Trade ID.
    pub id: u64,

    /// Price of the trade.
    pub price: String,

    /// Quantity of the trade.
    pub qty: String,

    /// Base asset quantity.
    pub base_qty: String,

    /// Trade timestamp in milliseconds.
    pub time: u64,

    /// Whether the buyer was the maker.
    pub is_buyer_maker: bool,
}

impl RestClient {
    /// Get older market historical trades.
    ///
    /// Market trades means trades filled in the order book. Only market trades will be
    /// returned, which means the insurance fund trades and ADL trades won't be
    /// returned.
    ///
    /// [Official API docs](https://developers.binance.com/docs/derivatives/coin-margined-futures/market-data/rest-api/Old-Trades-Lookup)
    ///
    /// Weight: 20
    pub async fn get_historical_trades(
        &self,
        params: HistoricalTradesRequest,
    ) -> RestResult<Vec<HistoricalTrade>> {
        self.send_request(
            "/dapi/v1/historicalTrades",
            reqwest::Method::GET,
            Some(params),
            20,
        )
        .await
    }
}
