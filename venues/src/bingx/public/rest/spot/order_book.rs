use serde::{Deserialize, Deserializer, Serialize};

use crate::bingx::{EndpointType, PublicRestClient as RestClient, RestResult};

const ORDER_BOOK_ENDPOINT: &str = "/openApi/spot/v1/market/depth";

/// Request for the order book endpoint
#[derive(Debug, Clone, Serialize)]
pub struct GetOrderBookRequest {
    /// Trading pair, e.g., BTC-USDT (required)
    pub symbol: String,

    /// Default 20, max 1000 (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,

    /// Timestamp of initiating the request, Unit: milliseconds (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<i64>,

    /// Request valid time window value, Unit: milliseconds (required)
    pub timestamp: i64,
}

/// Custom deserializer for price-quantity pairs that come as string arrays
fn deserialize_price_qty_array<'de, D>(deserializer: D) -> Result<Vec<[f64; 2]>, D::Error>
where
    D: Deserializer<'de>,
{
    let string_arrays: Vec<[String; 2]> = Vec::deserialize(deserializer)?;
    let mut result = Vec::new();

    for arr in string_arrays {
        let price = arr[0].parse::<f64>().map_err(serde::de::Error::custom)?;
        let quantity = arr[1].parse::<f64>().map_err(serde::de::Error::custom)?;
        result.push([price, quantity]);
    }

    Ok(result)
}

/// Response from the order book endpoint
#[derive(Debug, Clone, Deserialize)]
pub struct GetOrderBookResponse {
    /// Buy depth, where the first element of the array is the price and the second element is the quantity
    #[serde(deserialize_with = "deserialize_price_qty_array")]
    pub bids: Vec<[f64; 2]>,
    /// Sell depth, where the first element of the array is the price and the second element is the quantity
    #[serde(deserialize_with = "deserialize_price_qty_array")]
    pub asks: Vec<[f64; 2]>,
    /// Timestamp of depth, Unit: milliseconds
    pub ts: i64,
    /// Last update ID (optional field from API response)
    #[serde(rename = "lastUpdateId", default)]
    pub last_update_id: Option<i64>,
}

impl RestClient {
    /// Get order book depth
    ///
    /// Get current order book depth for a symbol.
    ///
    /// [docs](https://bingx-api.github.io/docs/#/en-us/spot/market-api.html#Order%20Book)
    ///
    /// # Arguments
    /// * `request` - The order book request parameters
    ///
    /// # Returns
    /// Order book response containing bids, asks, and timestamp
    ///
    /// # Rate Limit
    /// - IP: 100 requests per 10 seconds (Group 1)
    pub async fn get_order_book(
        &self,
        request: &GetOrderBookRequest,
    ) -> RestResult<GetOrderBookResponse> {
        self.send_request(
            ORDER_BOOK_ENDPOINT,
            Some(request),
            EndpointType::PublicMarket,
        )
        .await
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::bingx::RateLimiter;

    #[test]
    fn test_order_book_request_creation() {
        let symbol = "BTC-USDT".to_string();
        let timestamp = 1640995200000;
        let request = GetOrderBookRequest {
            symbol: symbol.clone(),
            limit: None,
            recv_window: None,
            timestamp,
        };

        assert_eq!(request.symbol, symbol);
        assert_eq!(request.timestamp, timestamp);
        assert!(request.limit.is_none());
        assert!(request.recv_window.is_none());
    }

    #[test]
    fn test_order_book_request_with_limit() {
        let symbol = "BTC-USDT".to_string();
        let timestamp = 1640995200000;
        let limit = 50;
        let request = GetOrderBookRequest {
            symbol: symbol.clone(),
            limit: Some(limit),
            recv_window: None,
            timestamp,
        };

        assert_eq!(request.symbol, symbol);
        assert_eq!(request.timestamp, timestamp);
        assert_eq!(request.limit, Some(limit));
    }

    #[test]
    fn test_order_book_request_with_recv_window() {
        let symbol = "BTC-USDT".to_string();
        let timestamp = 1640995200000;
        let recv_window = 5000;
        let request = GetOrderBookRequest {
            symbol: symbol.clone(),
            limit: None,
            recv_window: Some(recv_window),
            timestamp,
        };

        assert_eq!(request.symbol, symbol);
        assert_eq!(request.timestamp, timestamp);
        assert_eq!(request.recv_window, Some(recv_window));
    }

    #[test]
    fn test_order_book_request_serialization() {
        let request = GetOrderBookRequest {
            symbol: "BTC-USDT".to_string(),
            limit: None,
            recv_window: None,
            timestamp: 1640995200000,
        };
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"symbol\":\"BTC-USDT\""));
        assert!(json.contains("\"timestamp\":1640995200000"));
    }

    #[test]
    fn test_order_book_response_deserialization() {
        let json = r#"{
            "bids": [["45000.0", "1.5"], ["44999.0", "2.0"]],
            "asks": [["45001.0", "1.2"], ["45002.0", "0.8"]],
            "ts": 1640995200000
        }"#;

        let response: GetOrderBookResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.bids.len(), 2);
        assert_eq!(response.asks.len(), 2);
        assert_eq!(response.ts, 1640995200000);
        let bid0 = response.bids.first().expect("Expected at least one bid");
        assert_eq!(*bid0.first().expect("Missing price in bid0"), 45000.0);
        assert_eq!(*bid0.get(1).expect("Missing amount in bid0"), 1.5);
        let ask0 = response.asks.first().expect("Expected at least one ask");
        assert_eq!(*ask0.first().expect("Missing price in ask0"), 45001.0);
        assert_eq!(*ask0.get(1).expect("Missing amount in ask0"), 1.2);
    }

    #[tokio::test]
    async fn test_get_order_book_method_exists() {
        let client = RestClient::new(
            "http://127.0.0.1:0", // Invalid URL to guarantee error
            std::sync::Arc::new(rest::native::NativeHttpClient::default()),
            RateLimiter::new(),
        );

        let request = GetOrderBookRequest {
            symbol: "BTC-USDT".to_string(),
            limit: None,
            recv_window: None,
            timestamp: 1640995200000,
        };

        // Test that the method exists and can be called
        // Note: This will fail with network error since we're not making real requests
        assert!(client.get_order_book(&request).await.is_err());
    }
}
