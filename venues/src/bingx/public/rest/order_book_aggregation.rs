use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::bingx::{EndpointType, RestResult};

const ORDER_BOOK_AGGREGATION_ENDPOINT: &str = "/openApi/spot/v1/market/depth";

/// Request for the order book endpoint
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetOrderBookAggregationRequest {
    /// Trading pair, e.g., BTC-USDT (required)
    pub symbol: String,

    /// Default 20, max 1000 (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,

    /// Request valid time window value (optional)
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<i64>,

    /// Request valid time window value (required)
    pub timestamp: i64,
}

/// Response from the order book aggregation endpoint
#[derive(Debug, Clone, Deserialize)]
pub struct GetOrderBookAggregationResponse {
    /// Buy depth, where the first element of the array is the price and the second element is the quantity
    pub bids: Vec<[String; 2]>,
    /// Sell depth, where the first element of the array is the price and the second element is the quantity
    pub asks: Vec<[String; 2]>,
    /// Timestamp
    pub ts: i64,
}

impl RestClient {
    /// Get order book
    ///
    /// Get order book depth for a trading pair.
    ///
    /// # Arguments
    /// * `request` - The order book request parameters
    ///
    /// # Returns
    /// Order book response containing bids, asks, and timestamp
    ///
    /// # Rate Limit
    /// - IP: 100 requests per 10 seconds (Group 1)
    ///
    /// # API Documentation
    /// - Endpoint: GET /openApi/spot/v1/market/depth
    /// - Content-Type: request body(application/json)
    ///
    /// https://bingx-api.github.io/docs/#/en-us/spot/market-api.html#Order%20Book
    pub async fn get_order_book_aggregation(
        &self,
        request: &GetOrderBookAggregationRequest,
    ) -> RestResult<GetOrderBookAggregationResponse> {
        self.send_request(
            ORDER_BOOK_AGGREGATION_ENDPOINT,
            Some(request),
            EndpointType::PublicMarket,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use reqwest::Client;

    use super::*;
    use crate::bingx::RateLimiter;

    #[test]
    fn test_order_book_aggregation_request_creation() {
        let symbol = "BTC-USDT".to_string();
        let limit = Some(20);
        let recv_window = Some(5000);
        let timestamp = 1640995200000i64;
        let request = GetOrderBookAggregationRequest {
            symbol: symbol.clone(),
            limit,
            recv_window,
            timestamp,
        };

        assert_eq!(request.symbol, symbol);
        assert_eq!(request.limit, limit);
        assert_eq!(request.recv_window, recv_window);
        assert_eq!(request.timestamp, timestamp);
    }

    #[test]
    fn test_order_book_aggregation_request_serialization() {
        let request = GetOrderBookAggregationRequest {
            symbol: "BTC-USDT".to_string(),
            limit: Some(20),
            recv_window: Some(5000),
            timestamp: 1640995200000i64,
        };
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"symbol\":\"BTC-USDT\""));
        assert!(json.contains("\"limit\":20"));
        assert!(json.contains("\"recvWindow\":5000"));
        assert!(json.contains("\"timestamp\":1640995200000"));
    }

    #[test]
    fn test_order_book_aggregation_response_deserialization() {
        let json = r#"{
            "bids": [["45000.0", "1.5"], ["44999.0", "2.0"]],
            "asks": [["45001.0", "1.2"], ["45002.0", "0.8"]],
            "ts": 1640995200000
        }"#;

        let response: GetOrderBookAggregationResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.bids.len(), 2);
        assert_eq!(response.asks.len(), 2);
        assert_eq!(response.ts, 1640995200000);
        let bid0 = response.bids.first().expect("Expected at least one bid");
        assert_eq!(*bid0.first().expect("Missing price in bid0"), "45000.0");
        assert_eq!(*bid0.get(1).expect("Missing amount in bid0"), "1.5");
        let ask0 = response.asks.first().expect("Expected at least one ask");
        assert_eq!(*ask0.first().expect("Missing price in ask0"), "45001.0");
        assert_eq!(*ask0.get(1).expect("Missing amount in ask0"), "1.2");
    }

    #[tokio::test]
    async fn test_get_order_book_aggregation_method_exists() {
        let client = RestClient::new(
            "http://127.0.0.1:0", // Invalid URL to guarantee error
            Client::new(),
            RateLimiter::new(),
        );

        let request = GetOrderBookAggregationRequest {
            symbol: "BTC-USDT".to_string(),
            limit: Some(20),
            recv_window: None,
            timestamp: 1640995200000i64,
        };

        // Test that the method exists and can be called
        // Note: This will fail with network error since we're not making real requests
        assert!(client.get_order_book_aggregation(&request).await.is_err());
    }
}
