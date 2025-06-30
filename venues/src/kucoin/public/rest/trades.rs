use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::kucoin::{ResponseHeaders, RestResponse, Result, TradeSide};

use super::RestClient;

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
    /// # Example
    /// ```rust,no_run
    /// use venues::kucoin::public::rest::{RestClient, GetTradesRequest};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = RestClient::new_default();
    ///     let request = GetTradesRequest {
    ///         symbol: "BTC-USDT".to_string(),
    ///     };
    ///     let (response, _headers) = client.get_trades(request).await?;
    ///     println!("Found {} trades", response.len());
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_trades(
        &self,
        request: GetTradesRequest,
    ) -> Result<(Vec<Trade>, ResponseHeaders)> {
        let mut params = HashMap::new();
        params.insert("symbol".to_string(), request.symbol);

        let (response, headers): (RestResponse<Vec<Trade>>, ResponseHeaders) =
            self.get("/api/v1/market/histories", Some(params)).await?;

        Ok((response.data, headers))
    }
}
