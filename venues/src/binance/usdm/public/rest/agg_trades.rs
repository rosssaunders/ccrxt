//! Compressed/Aggregate Trades List endpoint for Binance USDM REST API.
//!
//! Implements GET /fapi/v1/aggTrades
//!
//! [Binance API docs](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Compressed-Aggregate-Trades-List)

use super::RestClient;
use crate::binance::usdm::RestResult;

use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// Request parameters for aggregate trades list.
#[derive(Debug, Clone, Serialize, Default)]
pub struct AggTradesRequest {
    /// Trading pair symbol (e.g., "BTCUSDT").
    pub symbol: Cow<'static, str>,
    /// ID to get aggregate trades from INCLUSIVE.
    #[serde(rename = "fromId")]
    pub from_id: Option<u64>,
    /// Timestamp in ms to get aggregate trades from INCLUSIVE.
    #[serde(rename = "startTime")]
    pub start_time: Option<u64>,
    /// Timestamp in ms to get aggregate trades until INCLUSIVE.
    #[serde(rename = "endTime")]
    pub end_time: Option<u64>,
    /// Number of trades to return. Default 500; max 1000.
    pub limit: Option<u16>,
}

/// Represents a single aggregate trade.
#[derive(Debug, Clone, Deserialize)]
pub struct AggTrade {
    /// Aggregate tradeId.
    #[serde(rename = "a")]
    pub agg_trade_id: u64,
    /// Price as a string.
    #[serde(rename = "p")]
    pub price: String,
    /// Quantity as a string.
    #[serde(rename = "q")]
    pub qty: String,
    /// First tradeId.
    #[serde(rename = "f")]
    pub first_trade_id: u64,
    /// Last tradeId.
    #[serde(rename = "l")]
    pub last_trade_id: u64,
    /// Timestamp.
    #[serde(rename = "T")]
    pub time: u64,
    /// Was the buyer the maker?
    #[serde(rename = "m")]
    pub is_buyer_maker: bool,
}

impl RestClient {
    /// Get compressed, aggregate market trades (GET /fapi/v1/aggTrades)
    ///
    /// [API docs](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Compressed-Aggregate-Trades-List)
    pub async fn get_agg_trades(&self, params: AggTradesRequest) -> RestResult<Vec<AggTrade>> {
        let mut query = format!("symbol={}", params.symbol);
        if let Some(from_id) = params.from_id {
            query.push_str(&format!("&fromId={}", from_id));
        }
        if let Some(start_time) = params.start_time {
            query.push_str(&format!("&startTime={}", start_time));
        }
        if let Some(end_time) = params.end_time {
            query.push_str(&format!("&endTime={}", end_time));
        }
        if let Some(limit) = params.limit {
            query.push_str(&format!("&limit={}", limit));
        }
        self.send_request(
            "/fapi/v1/aggTrades",
            reqwest::Method::GET,
            Some(&query),
            None,
            20,
        )
        .await
    }
}
