use serde::{Deserialize, Serialize};

use crate::bingx::{EndpointType, RestResult};
use super::RestClient;

/// Request for the recent trades list endpoint
#[derive(Debug, Clone, Serialize)]
pub struct GetRecentTradesRequest {
    /// Trading pair, e.g., BTC-USDT (required)
    pub symbol: String,
    /// Default 100, max 500 (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// Timestamp of initiating the request, Unit: milliseconds (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<i64>,
    /// Request valid time window value, Unit: milliseconds (required)
    pub timestamp: i64,
}

impl GetRecentTradesRequest {
    /// Create a new request for recent trades
    pub fn new(symbol: String, timestamp: i64) -> Self {
        Self {
            symbol,
            limit: None,
            recv_window: None,
            timestamp,
        }
    }

    /// Set the limit for number of trades to return
    pub fn with_limit(mut self, limit: i32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Set the receive window
    pub fn with_recv_window(mut self, recv_window: i64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }
}

/// Response from the recent trades list endpoint
pub type GetRecentTradesResponse = Vec<Trade>;

/// Trade information
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Trade {
    /// Transaction id
    pub id: i64,
    /// Price
    pub price: f64,
    /// Quantity
    pub qty: f64,
    /// Time
    pub time: i64,
    /// Buyer or not
    pub buyer_maker: bool,
}

impl RestClient {
    /// Get recent trades list
    ///
    /// Get the most recent trades for a symbol.
    ///
    /// # Arguments
    /// * `request` - The recent trades request parameters
    ///
    /// # Returns
    /// Vector of recent trades for the specified symbol
    ///
    /// # Rate Limit
    /// - IP: 100 requests per 10 seconds (Group 1)
    ///
    /// # API Documentation
    /// - Endpoint: GET /openApi/spot/v1/market/trades
    /// - Content-Type: request body(application/json)
    pub async fn get_recent_trades(&self, request: &GetRecentTradesRequest) -> RestResult<GetRecentTradesResponse> {
        self.send_request(
            "/openApi/spot/v1/market/trades",
            Some(request),
            EndpointType::PublicMarket,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::Client;
    use crate::bingx::RateLimiter;

    #[test]
    fn test_recent_trades_request_creation() {
        let symbol = "BTC-USDT".to_string();
        let timestamp = 1640995200000;
        let request = GetRecentTradesRequest::new(symbol.clone(), timestamp);
        
        assert_eq!(request.symbol, symbol);
        assert_eq!(request.timestamp, timestamp);
        assert!(request.limit.is_none());
        assert!(request.recv_window.is_none());
    }

    #[test]
    fn test_recent_trades_request_with_limit() {
        let symbol = "BTC-USDT".to_string();
        let timestamp = 1640995200000;
        let limit = 50;
        let request = GetRecentTradesRequest::new(symbol.clone(), timestamp).with_limit(limit);
        
        assert_eq!(request.symbol, symbol);
        assert_eq!(request.timestamp, timestamp);
        assert_eq!(request.limit, Some(limit));
    }

    #[test]
    fn test_recent_trades_request_with_recv_window() {
        let symbol = "BTC-USDT".to_string();
        let timestamp = 1640995200000;
        let recv_window = 5000;
        let request = GetRecentTradesRequest::new(symbol.clone(), timestamp).with_recv_window(recv_window);
        
        assert_eq!(request.symbol, symbol);
        assert_eq!(request.timestamp, timestamp);
        assert_eq!(request.recv_window, Some(recv_window));
    }

    #[test]
    fn test_recent_trades_request_serialization() {
        let request = GetRecentTradesRequest::new("BTC-USDT".to_string(), 1640995200000);
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"symbol\":\"BTC-USDT\""));
        assert!(json.contains("\"timestamp\":1640995200000"));
    }

    #[test]
    fn test_trade_deserialization() {
        let json = r#"{
            "id": 123456,
            "price": 45000.50,
            "qty": 0.1,
            "time": 1640995200000,
            "buyerMaker": true
        }"#;
        
        let trade: Trade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.id, 123456);
        assert_eq!(trade.price, 45000.50);
        assert_eq!(trade.qty, 0.1);
        assert_eq!(trade.time, 1640995200000);
        assert!(trade.buyer_maker);
    }

    #[tokio::test]
    async fn test_get_recent_trades_method_exists() {
        let client = RestClient::new(
            "https://open-api.bingx.com",
            Client::new(),
            RateLimiter::new(),
        );

        let request = GetRecentTradesRequest::new("BTC-USDT".to_string(), 1640995200000);
        
        // Test that the method exists and can be called
        // Note: This will fail with network error since we're not making real requests
        assert!(client.get_recent_trades(&request).await.is_err());
    }
}