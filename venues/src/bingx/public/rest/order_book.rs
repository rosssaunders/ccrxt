use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::bingx::{EndpointType, RestResult};

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



/// Response from the order book endpoint
#[derive(Debug, Clone, Deserialize)]
pub struct GetOrderBookResponse {
    /// Buy depth, where the first element of the array is the price and the second element is the quantity
    pub bids: Vec<[f64; 2]>,
    /// Sell depth, where the first element of the array is the price and the second element is the quantity
    pub asks: Vec<[f64; 2]>,
    /// Timestamp of depth, Unit: milliseconds
    pub ts: i64,
}

impl RestClient {
    /// Get order book depth
    ///
    /// Get current order book depth for a symbol.
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
    use reqwest::Client;

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
            "bids": [[45000.0, 1.5], [44999.0, 2.0]],
            "asks": [[45001.0, 1.2], [45002.0, 0.8]],
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
            "https://open-api.bingx.com",
            Client::new(),
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
