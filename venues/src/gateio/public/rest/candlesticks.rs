use crate::gateio::CandlestickInterval;
use serde::{Deserialize, Serialize};

use super::RestClient;

/// Request parameters for retrieving candlestick data
#[derive(Debug, Clone, Serialize, Default)]
pub struct CandlesticksRequest {
    /// Currency pair to query candlesticks for
    pub currency_pair: String,

    /// Candlestick interval (e.g., 1m, 5m, 1h, 1d)
    pub interval: CandlestickInterval,

    /// Maximum number of candlesticks to return (default: 100, max: 1000)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    /// Start time for candlestick range (Unix timestamp in seconds)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,

    /// End time for candlestick range (Unix timestamp in seconds)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,
}

/// Candlestick data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Candlestick {
    /// Unix timestamp in seconds
    #[serde(rename = "0")]
    pub timestamp: String,

    /// Trading volume (quote currency)
    #[serde(rename = "1")]
    pub volume: String,

    /// Close price
    #[serde(rename = "2")]
    pub close: String,

    /// Highest price
    #[serde(rename = "3")]
    pub high: String,

    /// Lowest price
    #[serde(rename = "4")]
    pub low: String,

    /// Open price
    #[serde(rename = "5")]
    pub open: String,

    /// Trading volume (base currency)
    #[serde(rename = "6")]
    pub base_volume: String,
}



impl RestClient {
    /// Get candlestick data for a currency pair
    ///
    /// This endpoint returns OHLCV candlestick data for the specified currency pair and interval.
    /// You can filter by time range and limit the number of results.
    pub async fn get_candlesticks(
        &self,
        params: CandlesticksRequest,
    ) -> crate::gateio::Result<Vec<Vec<String>>> {
        self.get_with_query("/spot/candlesticks", Some(&params))
            .await
    }
}
