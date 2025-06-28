use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::binance::options::RestResult;

use super::client::RestClient;

/// Request parameters for recent block trades
#[derive(Debug, Clone, Serialize, Default)]
pub struct BlockTradesRequest {
    /// Option trading pair, e.g. BTC-200730-9000-C
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,

    /// Number of records (Default: 100, Max: 500)
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

/// Recent block trade information
#[derive(Debug, Clone, Deserialize)]
pub struct BlockTrade {
    /// ID
    #[serde(rename = "id")]
    pub id: u64,

    /// Trade ID
    #[serde(rename = "tradeId")]
    pub trade_id: u64,

    /// Symbol
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Price
    #[serde(rename = "price")]
    pub price: Decimal,

    /// Quantity
    #[serde(rename = "qty")]
    pub qty: Decimal,

    /// Quote quantity
    #[serde(rename = "quoteQty")]
    pub quote_qty: Decimal,

    /// Side (-1 Sell, 1 Buy)
    #[serde(rename = "side")]
    pub side: i32,

    /// Time
    #[serde(rename = "time")]
    pub time: u64,
}

impl RestClient {
    /// Get recent block trades list
    ///
    /// Returns recent block trades.
    ///
    /// See: [API Documentation](https://developers.binance.com/docs/derivatives/option/market-data/Recent-Block-Trade-List)
    /// Method: GET /eapi/v1/blockTrades
    /// Weight: 5
    /// Security: None
    pub async fn get_block_trades(
        &self,
        params: BlockTradesRequest,
    ) -> RestResult<Vec<BlockTrade>> {
        let query_string = if params.symbol.is_some() || params.limit.is_some() {
            Some(serde_urlencoded::to_string(&params).map_err(|e| {
                crate::binance::options::Errors::Error(format!("URL encoding error: {e}"))
            })?)
        } else {
            None
        };

        self.send_request(
            "/eapi/v1/blockTrades",
            reqwest::Method::GET,
            query_string.as_deref(),
            None,
            5,
        )
        .await
    }
}
