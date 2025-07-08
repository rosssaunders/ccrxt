//! Recent Trades List endpoint for Binance USDM REST API.
//!
//! Implements GET /fapi/v1/trades
//!
//! [Binance API docs](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Recent-Trades-List)
use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::binance::usdm::RestResult;

/// Request parameters for recent trades list.
#[derive(Debug, Clone, Serialize, Default)]
pub struct RecentTradesRequest {
    /// Trading pair symbol (e.g., "BTCUSDT").
    pub symbol: Cow<'static, str>,
    /// Number of trades to return. Default 500; max 1000.
    pub limit: Option<u16>,
}

/// Represents a single recent trade.
#[derive(Debug, Clone, Deserialize)]
pub struct RecentTrade {
    /// Trade ID.
    pub id: u64,
    /// Price as a string.
    pub price: String,
    /// Quantity as a string.
    pub qty: String,
    /// Quote quantity as a string.
    #[serde(rename = "quoteQty")]
    pub quote_qty: String,
    /// Trade time (ms since epoch).
    pub time: u64,
    /// True if buyer is the maker.
    #[serde(rename = "isBuyerMaker")]
    pub is_buyer_maker: bool,
}

impl RestClient {
    /// Get recent market trades (GET /fapi/v1/trades)
    ///
    /// [API docs](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Recent-Trades-List)
    pub async fn get_recent_trades(
        &self,
        params: RecentTradesRequest,
    ) -> RestResult<Vec<RecentTrade>> {
        let mut query = format!("symbol={}", params.symbol);
        if let Some(limit) = params.limit {
            query.push_str(&format!("&limit={}", limit));
        }
        self.send_request(
            "/fapi/v1/trades",
            reqwest::Method::GET,
            Some(&query),
            None,
            5,
        )
        .await
    }
}
