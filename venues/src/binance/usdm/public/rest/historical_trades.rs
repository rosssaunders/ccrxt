//! Old Trades Lookup endpoint for Binance USDM REST API.
//!
//! Implements GET /fapi/v1/historicalTrades
//!
//! [Binance API docs](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Old-Trades-Lookup)

use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use super::RestClient;
use crate::binance::usdm::RestResult;

/// Request parameters for old trades lookup.
#[derive(Debug, Clone, Serialize, Default)]
pub struct HistoricalTradesRequest {
    /// Trading pair symbol (e.g., "BTCUSDT").
    pub symbol: Cow<'static, str>,
    /// Number of trades to return. Default 100; max 500.
    pub limit: Option<u16>,
    /// TradeId to fetch from. Default gets most recent trades.
    #[serde(rename = "fromId")]
    pub from_id: Option<u64>,
}

/// Represents a single historical trade.
#[derive(Debug, Clone, Deserialize)]
pub struct HistoricalTrade {
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
    /// Get older market historical trades (GET /fapi/v1/historicalTrades)
    ///
    /// [API docs](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Old-Trades-Lookup)
    pub async fn get_historical_trades(&self, params: HistoricalTradesRequest) -> RestResult<Vec<HistoricalTrade>> {
        let mut query = format!("symbol={}", params.symbol);
        if let Some(limit) = params.limit {
            query.push_str(&format!("&limit={}", limit));
        }
        if let Some(from_id) = params.from_id {
            query.push_str(&format!("&fromId={}", from_id));
        }
        self.send_request(
            "/fapi/v1/historicalTrades",
            reqwest::Method::GET,
            Some(&query),
            None,
            20,
        )
        .await
    }
}
