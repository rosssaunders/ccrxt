use serde::{Deserialize, Serialize};

use super::RestClient;

/// Request parameters for futures contract stats
#[derive(Debug, Clone, Serialize, Default)]
pub struct FuturesStatsRequest {
    /// Settlement currency
    pub settle: String,
    /// Contract name
    pub contract: String,
    /// Start time in Unix seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,
    /// Interval time between data points (default 5m)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    /// Maximum number of records to return (1-200, default 30)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

/// Futures contract statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuturesStats {
    /// Statistical timestamp
    pub time: i64,

    /// Long/short account number ratio
    pub lsr_taker: f64,

    /// Long/short position ratio  
    pub lsr_account: f64,

    /// Long liquidation size
    pub long_liq_size: serde_json::Value,

    /// Long liquidation amount
    pub long_liq_amount: serde_json::Value,

    /// Long liquidation volume (in USD)
    pub long_liq_usd: serde_json::Value,

    /// Short liquidation size
    pub short_liq_size: serde_json::Value,

    /// Short liquidation amount
    pub short_liq_amount: serde_json::Value,

    /// Short liquidation volume (in USD)
    pub short_liq_usd: serde_json::Value,

    /// Open interest
    pub open_interest: serde_json::Value,

    /// Mark price
    pub mark_price: serde_json::Value,

    /// Top trader long/short position ratio
    pub top_lsr_account: f64,

    /// Top trader long/short size ratio
    pub top_lsr_size: f64,
}

impl RestClient {
    /// Get futures contract stats
    ///
    /// Retrieves statistical data for a specific futures contract including
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#futures-stats>
    /// liquidation data, position ratios, and open interest.
    pub async fn get_futures_stats(
        &self,
        params: FuturesStatsRequest,
    ) -> crate::gateio::perpetual::Result<Vec<FuturesStats>> {
        let endpoint = format!("/futures/{}/contract_stats", params.settle);
        self.get_with_query(&endpoint, Some(&params)).await
    }
}
