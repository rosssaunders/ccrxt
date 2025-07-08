use serde::{Deserialize, Serialize};

use super::RestClient;

/// Request parameters for futures trades
#[derive(Debug, Clone, Serialize, Default)]
pub struct FuturesTradesRequest {
    /// Settlement currency
    pub settle: String,
    /// Contract name
    pub contract: String,
    /// Maximum number of records to return (1-1000, default 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// Specify list offset (default 0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// Specify the starting point for this list
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_id: Option<String>,
    /// Specify starting time in Unix seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,
    /// Specify ending time in Unix seconds  
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,
}

/// Futures trade entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuturesTrade {
    /// Trade ID
    pub id: i64,

    /// Trading time
    pub create_time: f64,

    /// Trading contract
    pub contract: String,

    /// Trading size
    pub size: i64,

    /// Trading price
    pub price: String,

    /// Whether internal trade
    pub is_internal: bool,
}

impl RestClient {
    /// Get futures trading history
    ///
    /// Retrieves recent trades for a specific futures contract.
    /// Maximum of 1000 records can be returned per request.
    pub async fn get_futures_trades(
        &self,
        params: FuturesTradesRequest,
    ) -> crate::gateio::Result<Vec<FuturesTrade>> {
        let endpoint = format!("/futures/{}/trades", params.settle);
        self.get_with_query(&endpoint, Some(&params)).await
    }
}
