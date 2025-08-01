use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::bybit::{EndpointType, RestResult, enums::*};

/// Endpoint URL path for orderbook
const ENDPOINT_PATH: &str = "/v5/market/orderbook";

/// Request parameters for getting orderbook data
#[derive(Debug, Clone, Serialize)]
pub struct GetOrderbookRequest {
    /// Product type
    pub category: Category,

    /// Symbol name (e.g., "BTCUSDT")
    pub symbol: String,

    /// Limit size for each bid and ask. Spot: [1, 200]. Linear&Inverse: [1, 500]. Option: [1, 25]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

/// Orderbook level containing price and size
#[derive(Debug, Clone)]
pub struct OrderbookLevel {
    /// Price level
    pub price: String,

    /// Size at this price level
    pub size: String,
}

impl<'de> Deserialize<'de> for OrderbookLevel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let arr: Vec<String> = Vec::deserialize(deserializer)?;
        if arr.len() != 2 {
            return Err(serde::de::Error::custom(
                "Expected 2 elements in orderbook level array",
            ));
        }
        #[allow(clippy::indexing_slicing)]
        Ok(OrderbookLevel {
            price: arr[0].clone(),
            size: arr[1].clone(),
        })
    }
}

/// Orderbook data
#[derive(Debug, Clone, Deserialize)]
pub struct GetOrderbookData {
    /// Symbol name
    pub s: String,

    /// Bid array, sorted by price in descending order
    pub b: Vec<OrderbookLevel>,

    /// Ask array, sorted by price in ascending order
    pub a: Vec<OrderbookLevel>,

    /// Timestamp (ms) when system generates the data
    pub ts: u64,

    /// Update ID, always in sequence
    pub u: u64,

    /// Cross sequence (compare different levels orderbook data)
    pub seq: u64,

    /// Timestamp from matching engine when data is produced
    #[serde(rename = "cts")]
    pub create_time: u64,
}

/// Response from the orderbook endpoint
#[derive(Debug, Clone, Deserialize)]
pub struct GetOrderbookResponse {
    /// Success/Error code (0: success, 1: error)
    #[serde(rename = "retCode")]
    pub ret_code: i32,

    /// Success/Error message
    #[serde(rename = "retMsg")]
    pub ret_msg: String,

    /// Business data result
    pub result: GetOrderbookData,

    /// Extended information
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: serde_json::Value,

    /// Current timestamp in milliseconds
    pub time: u64,
}

impl RestClient {
    /// Get orderbook data
    ///
    /// Query for orderbook depth data.
    /// - Contract: 500-level orderbook data
    /// - Spot: 200-level orderbook data  
    /// - Option: 25-level orderbook data
    ///
    /// [API Documentation](https://bybit-exchange.github.io/docs/v5/market/orderbook)
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `request` - The orderbook request parameters including:
    ///   - `category`: Product type
    ///   - `symbol`: Symbol name
    ///   - `limit`: Optional depth limit
    ///
    /// # Returns
    /// A result containing the orderbook response with bid/ask levels or an error
    pub async fn get_orderbook(
        &self,
        request: GetOrderbookRequest,
    ) -> RestResult<GetOrderbookResponse> {
        self.send_public_request(ENDPOINT_PATH, Some(&request), EndpointType::Market)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_orderbook_level_deserialization() {
        let json = r#"["65485.47", "47.081829"]"#;

        let level: OrderbookLevel = serde_json::from_str(json).unwrap();
        assert_eq!(level.price, "65485.47");
        assert_eq!(level.size, "47.081829");
    }

    #[test]
    fn test_get_orderbook_request_direct_construction() {
        let request = GetOrderbookRequest {
            category: Category::Linear,
            symbol: "BTCUSDT".to_string(),
            limit: Some(100),
        };

        assert_eq!(request.category, Category::Linear);
        assert_eq!(request.symbol, "BTCUSDT");
        assert_eq!(request.limit, Some(100));
    }

    #[test]
    fn test_orderbook_response_deserialization() {
        let json = r#"{
            "retCode": 0,
            "retMsg": "OK",
            "result": {
                "s": "BTCUSDT",
                "b": [["65485.47", "47.081829"]],
                "a": [["65557.7", "16.606555"]],
                "ts": 1716863719031,
                "u": 230704,
                "seq": 1432604333,
                "cts": 1716863718905
            },
            "retExtInfo": {},
            "time": 1716863719031
        }"#;

        let response: GetOrderbookResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.ret_code, 0);
        assert_eq!(response.result.s, "BTCUSDT");
        assert_eq!(response.result.b.len(), 1);
        assert_eq!(response.result.a.len(), 1);
        assert_eq!(response.result.b[0].price, "65485.47");
        assert_eq!(response.result.a[0].price, "65557.7");
    }
}
