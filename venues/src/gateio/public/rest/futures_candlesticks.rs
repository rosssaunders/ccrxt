use serde::{Deserialize, Serialize};
use crate::gateio::enums::CandlestickInterval;

use super::RestClient;

/// Request parameters for futures candlesticks
#[derive(Debug, Clone, Serialize, Default)]
pub struct FuturesCandlesticksRequest {
    /// Settlement currency
    pub settle: String,
    /// Contract name
    pub contract: String,
    /// Interval time between data points
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<CandlestickInterval>,
    /// Start time for the query
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,
    /// End time for the query
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,
    /// Maximum number of records to return (1-1000, default 100)  
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

/// Futures candlestick data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuturesCandlestick {
    /// Unix timestamp in seconds
    pub t: i64,
    
    /// Trading volume (in quote currency)
    pub v: i64,
    
    /// Close price
    pub c: String,
    
    /// Highest price
    pub h: String,
    
    /// Lowest price
    pub l: String,
    
    /// Open price
    pub o: String,
    
    /// Trading volume (in base currency)
    pub sum: String,
}

impl RestClient {
    /// Get futures candlesticks
    ///
    /// Retrieves candlestick data for a specific futures contract.
    /// Supports mark price and index price with prefixes `mark_` and `index_`.
    pub async fn get_futures_candlesticks(&self, params: FuturesCandlesticksRequest) -> crate::gateio::Result<Vec<FuturesCandlestick>> {
        let endpoint = format!("/futures/{}/candlesticks", params.settle);
        self.get_with_query(&endpoint, Some(&params)).await
    }
    
    /// Get futures mark price candlesticks
    ///
    /// Retrieves mark price candlestick data for a specific futures contract.
    pub async fn get_futures_mark_price_candlesticks(&self, params: FuturesCandlesticksRequest) -> crate::gateio::Result<Vec<FuturesCandlestick>> {
        let mut mark_params = params;
        mark_params.contract = format!("mark_{}", mark_params.contract);
        let endpoint = format!("/futures/{}/candlesticks", mark_params.settle);
        self.get_with_query(&endpoint, Some(&mark_params)).await
    }
    
    /// Get futures index price candlesticks
    ///
    /// Retrieves index price candlestick data for a specific futures contract.
    pub async fn get_futures_index_price_candlesticks(&self, params: FuturesCandlesticksRequest) -> crate::gateio::Result<Vec<FuturesCandlestick>> {
        let mut index_params = params;
        index_params.contract = format!("index_{}", index_params.contract);
        let endpoint = format!("/futures/{}/candlesticks", index_params.settle);
        self.get_with_query(&endpoint, Some(&index_params)).await
    }
}