use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::binance::options::RestResult;

/// Request parameters for recent trades
#[derive(Debug, Clone, Serialize)]
pub struct RecentTradesRequest {
    /// Option trading pair, e.g BTC-200730-9000-C
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Number of records (Default: 100, Max: 500)
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

/// Recent trade information
#[derive(Debug, Clone, Deserialize)]
pub struct RecentTrade {
    /// Trade ID
    #[serde(rename = "id")]
    pub id: String,

    /// Symbol
    #[serde(rename = "symbol")]
    pub symbol: String,

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
    /// Get recent trades list
    ///
    /// Returns recent market trades.
    ///
    /// See: [API Documentation](https://developers.binance.com/docs/derivatives/option/market-data/Recent-Trades-List)
    /// Method: GET /eapi/v1/trades
    /// Weight: 5
    /// Security: None
    pub async fn get_recent_trades(
        &self,
        params: RecentTradesRequest,
    ) -> RestResult<Vec<RecentTrade>> {
        let query_string = serde_urlencoded::to_string(&params).map_err(|e| {
            crate::binance::options::Errors::Error(format!("URL encoding error: {e}"))
        })?;

        self.send_request(
            "/eapi/v1/trades",
            reqwest::Method::GET,
            Some(&query_string),
            None,
            5,
        )
        .await
    }
}
