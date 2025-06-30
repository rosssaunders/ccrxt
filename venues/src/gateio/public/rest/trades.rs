use serde::{Deserialize, Serialize};

use super::RestClient;

/// Request parameters for retrieving trades
#[derive(Debug, Clone, Serialize, Default)]
pub struct TradesRequest {
    /// Currency pair to query trades for
    pub currency_pair: String,

    /// Maximum number of trades to return (default: 100, max: 1000)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    /// Page number for pagination (starts from 1)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,

    /// Start time for trade range (Unix timestamp in seconds)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,

    /// End time for trade range (Unix timestamp in seconds)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,
}

/// Trade information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trade {
    /// Trade ID
    pub id: String,

    /// Trading time (Unix timestamp in seconds)
    pub create_time: String,

    /// Trading time (Unix timestamp in milliseconds)
    pub create_time_ms: String,

    /// Currency pair
    pub currency_pair: String,

    /// Trade side (buy/sell)
    pub side: String,

    /// Trade role (taker/maker)
    pub role: String,

    /// Trade amount
    pub amount: String,

    /// Trade price
    pub price: String,

    /// Order ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,

    /// Trade fee
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee: Option<String>,

    /// Fee currency
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_currency: Option<String>,

    /// Point fee
    #[serde(skip_serializing_if = "Option::is_none")]
    pub point_fee: Option<String>,

    /// GT fee
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gt_fee: Option<String>,
}

impl RestClient {
    /// Retrieve recent trades for a currency pair
    ///
    /// This endpoint returns recent trades for the specified currency pair.
    /// You can filter by time range and limit the number of results.
    pub async fn get_trades(&self, params: TradesRequest) -> crate::gateio::Result<Vec<Trade>> {
        self.get_with_query("/spot/trades", Some(&params)).await
    }
}
