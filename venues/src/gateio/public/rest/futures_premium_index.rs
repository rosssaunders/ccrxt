use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::gateio::enums::CandlestickInterval;

/// Request parameters for futures premium index
#[derive(Debug, Clone, Serialize, Default)]
pub struct FuturesPremiumIndexRequest {
    /// Settlement currency
    pub settle: String,
    /// Contract name
    pub contract: String,
    /// Start time in Unix seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,
    /// End time in Unix seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,
    /// Interval time between data points
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<CandlestickInterval>,
    /// Maximum number of records to return (1-1000, default 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

/// Premium index K-line data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuturesPremiumIndex {
    /// Unix timestamp in seconds
    pub t: i64,

    /// Close price
    pub c: String,

    /// Highest price
    pub h: String,

    /// Lowest price
    pub l: String,

    /// Open price
    pub o: String,
}

impl RestClient {
    /// Get premium index K-line
    ///
    /// Retrieves premium index candlestick data for a specific futures contract.
    /// Premium index tracks the difference between mark price and index price.
    pub async fn get_futures_premium_index(
        &self,
        params: FuturesPremiumIndexRequest,
    ) -> crate::gateio::Result<Vec<FuturesPremiumIndex>> {
        let endpoint = format!("/futures/{}/premium_index", params.settle);
        self.get_with_query(&endpoint, Some(&params)).await
    }
}
