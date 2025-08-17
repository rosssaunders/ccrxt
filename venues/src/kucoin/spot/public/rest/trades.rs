use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::kucoin::spot::{ResponseHeaders, RestResponse, Result, TradeSide};

const TRADES_ENDPOINT: &str = "/api/v1/market/histories";

/// Request for getting recent trades
#[derive(Debug, Clone, Serialize)]
pub struct GetTradesRequest {
    /// Trading symbol (e.g., "BTC-USDT")
    pub symbol: String,
}

/// Trade information
#[derive(Debug, Clone, Deserialize)]
pub struct Trade {
    /// Trade sequence number
    pub sequence: String,

    /// Trade price
    pub price: String,

    /// Trade size/quantity
    pub size: String,

    /// Trade side (buy/sell)
    pub side: TradeSide,

    /// Trade timestamp (milliseconds)
    pub time: i64,
}

impl RestClient {
    /// Get recent trades for a symbol
    ///
    /// [docs](https://docs.kucoin.com/#get-trade-histories)
    pub async fn get_trades(
        &self,
        request: GetTradesRequest,
    ) -> Result<(Vec<Trade>, ResponseHeaders)> {
        let (response, headers): (RestResponse<Vec<Trade>>, ResponseHeaders) =
            self.get_with_request(TRADES_ENDPOINT, &request).await?;

        Ok((response.data, headers))
    }
}
