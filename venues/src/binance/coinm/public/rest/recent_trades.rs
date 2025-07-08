use serde::{Deserialize, Serialize};

use crate::binance::coinm::{RestResult, public::rest::RestClient};

/// Request parameters for the recent trades endpoint.
#[derive(Debug, Clone, Serialize, Default)]
pub struct RecentTradesRequest {
    /// Trading symbol (e.g., "BTCUSD_PERP").
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Default 500; max 1000.
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

/// Represents a single trade in the recent trades response.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Trade {
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
    /// Get recent market trades.
    ///
    /// Market trades means trades filled in the order book. Only market trades will be
    /// returned, which means the insurance fund trades and ADL trades won't be
    /// returned.
    ///
    /// [Official API docs](https://developers.binance.com/docs/derivatives/coin-margined-futures/market-data/rest-api/Recent-Trades-List)
    ///
    /// Weight: 5
    pub async fn get_recent_trades(&self, params: RecentTradesRequest) -> RestResult<Vec<Trade>> {
        self.send_request("/dapi/v1/trades", reqwest::Method::GET, Some(params), 5)
            .await
    }
}
