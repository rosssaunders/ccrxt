use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::gateio::delivery::enums::CandlestickInterval;

const DELIVERY_CANDLESTICKS_ENDPOINT: &str = "/delivery/{}/candlesticks";

/// Request parameters for delivery candlesticks
#[derive(Debug, Clone, Serialize, Default)]
pub struct DeliveryCandlesticksRequest {
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

/// Delivery candlestick data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryCandlestick {
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
    /// Get delivery candlesticks
    ///
    /// Retrieves candlestick data for a specific delivery contract.
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#get-futures-candlesticks-2>
    /// Supports mark price and index price with prefixes `mark_` and `index_`.
    pub async fn get_delivery_candlesticks(
        &self,
        params: DeliveryCandlesticksRequest,
    ) -> crate::gateio::delivery::Result<Vec<DeliveryCandlestick>> {
        let endpoint = DELIVERY_CANDLESTICKS_ENDPOINT.replace("{}", &params.settle);
        self.get_with_query(&endpoint, Some(&params)).await
    }

    /// Get delivery mark price candlesticks
    ///
    /// Retrieves mark price candlestick data for a specific delivery contract.
    pub async fn get_delivery_mark_price_candlesticks(
        &self,
        params: DeliveryCandlesticksRequest,
    ) -> crate::gateio::delivery::Result<Vec<DeliveryCandlestick>> {
        let mut mark_params = params;
        mark_params.contract = format!("mark_{}", mark_params.contract);
        let endpoint = DELIVERY_CANDLESTICKS_ENDPOINT.replace("{}", &mark_params.settle);
        self.get_with_query(&endpoint, Some(&mark_params)).await
    }

    /// Get delivery index price candlesticks
    ///
    /// Retrieves index price candlestick data for a specific delivery contract.
    pub async fn get_delivery_index_price_candlesticks(
        &self,
        params: DeliveryCandlesticksRequest,
    ) -> crate::gateio::delivery::Result<Vec<DeliveryCandlestick>> {
        let mut index_params = params;
        index_params.contract = format!("index_{}", index_params.contract);
        let endpoint = DELIVERY_CANDLESTICKS_ENDPOINT.replace("{}", &index_params.settle);
        self.get_with_query(&endpoint, Some(&index_params)).await
    }
}
