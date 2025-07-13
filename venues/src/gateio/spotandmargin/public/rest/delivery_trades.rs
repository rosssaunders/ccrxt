use serde::{Deserialize, Serialize};

use super::RestClient;

/// Request parameters for delivery trades
#[derive(Debug, Clone, Serialize, Default)]
pub struct DeliveryTradesRequest {
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

/// Delivery trade entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryTrade {
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
    /// Get delivery trading history
    ///
    /// Retrieves recent trades for a specific delivery contract.
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#futures-trading-history-2>
    /// Maximum of 1000 records can be returned per request.
    pub async fn get_delivery_trades(
        &self,
        params: DeliveryTradesRequest,
    ) -> crate::gateio::spotandmargin::Result<Vec<DeliveryTrade>> {
        let endpoint = format!("/delivery/{}/trades", params.settle);
        self.get_with_query(&endpoint, Some(&params)).await
    }
}
