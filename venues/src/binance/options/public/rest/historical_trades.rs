use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::binance::options::RestResult;

use super::client::RestClient;

/// Request parameters for historical trades
#[derive(Debug, Clone, Serialize)]
pub struct HistoricalTradesRequest {
    /// Option trading pair, e.g BTC-200730-9000-C
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// The UniqueId ID from which to return. The latest deal record is returned by default
    #[serde(rename = "fromId", skip_serializing_if = "Option::is_none")]
    pub from_id: Option<u64>,

    /// Number of records (Default: 100, Max: 500)
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

/// Historical trade information
#[derive(Debug, Clone, Deserialize)]
pub struct HistoricalTrade {
    /// UniqueId
    #[serde(rename = "id")]
    pub id: String,

    /// TradeId
    #[serde(rename = "tradeId")]
    pub trade_id: String,

    /// Completed trade price
    #[serde(rename = "price")]
    pub price: Decimal,

    /// Completed trade quantity
    #[serde(rename = "qty")]
    pub qty: Decimal,

    /// Completed trade amount
    #[serde(rename = "quoteQty")]
    pub quote_qty: Decimal,

    /// Completed trade direction (-1 Sell, 1 Buy)
    #[serde(rename = "side")]
    pub side: i32,

    /// Time
    #[serde(rename = "time")]
    pub time: u64,
}

impl RestClient {
    /// Get old trades lookup
    ///
    /// Returns older market historical trades.
    ///
    /// See: [API Documentation](https://developers.binance.com/docs/derivatives/option/market-data/Old-Trades-Lookup)
    /// Method: GET /eapi/v1/historicalTrades
    /// Weight: 20
    /// Security: MARKET_DATA
    pub async fn get_historical_trades(
        &self,
        params: HistoricalTradesRequest,
    ) -> RestResult<Vec<HistoricalTrade>> {
        let query_string = serde_urlencoded::to_string(&params).map_err(|e| {
            crate::binance::options::Errors::Error(format!("URL encoding error: {e}"))
        })?;

        self.send_request(
            "/eapi/v1/historicalTrades",
            reqwest::Method::GET,
            Some(&query_string),
            None,
            20,
        )
        .await
    }
}
