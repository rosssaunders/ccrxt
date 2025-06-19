use serde::{Deserialize, Serialize};

use crate::bingx::{EndpointType, RestResult};
use super::RestClient;

/// Request for the order book aggregation endpoint
#[derive(Debug, Clone, Serialize)]
pub struct GetOrderBookAggregationRequest {
    /// Trading pair, such as: BTC_USDT (required)
    pub symbol: String,
    /// Query depth (required)
    pub depth: i64,
    /// step0 default precision, step1 to step5 are 10 to 100000 times precision respectively (required)
    /// Valid values: step0, step1, step2, step3, step4, step5
    #[serde(rename = "type")]
    pub type_: String,
}

impl GetOrderBookAggregationRequest {
    /// Create a new request for order book aggregation
    pub fn new(symbol: String, depth: i64, type_: String) -> Self {
        Self {
            symbol,
            depth,
            type_,
        }
    }
}

/// Response from the order book aggregation endpoint
#[derive(Debug, Clone, Deserialize)]
pub struct GetOrderBookAggregationResponse {
    /// Buy depth, where the first element of the array is the price and the second element is the quantity
    pub bids: Vec<[f64; 2]>,
    /// Sell depth, where the first element of the array is the price and the second element is the quantity
    pub asks: Vec<[f64; 2]>,
    /// Timestamp
    pub ts: i64,
}

impl RestClient {
    /// Get order book aggregation
    ///
    /// Get order book depth with different precision levels.
    ///
    /// # Arguments
    /// * `request` - The order book aggregation request parameters
    ///
    /// # Returns
    /// Order book aggregation response containing bids, asks, and timestamp
    ///
    /// # Rate Limit
    /// - IP: 100 requests per 10 seconds (Group 1)
    ///
    /// # API Documentation
    /// - Endpoint: GET /openApi/spot/v2/market/depth
    /// - Content-Type: request body(application/json)
    pub async fn get_order_book_aggregation(&self, request: &GetOrderBookAggregationRequest) -> RestResult<GetOrderBookAggregationResponse> {
        self.send_request(
            "/openApi/spot/v2/market/depth",
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
    fn test_order_book_aggregation_request_creation() {
        let symbol = "BTC_USDT".to_string();
        let depth = 20;
        let type_ = "step0".to_string();
        let request = GetOrderBookAggregationRequest::new(symbol.clone(), depth, type_.clone());
        
        assert_eq!(request.symbol, symbol);
        assert_eq!(request.depth, depth);
        assert_eq!(request.type_, type_);
    }

    #[test]
    fn test_order_book_aggregation_request_serialization() {
        let request = GetOrderBookAggregationRequest::new("BTC_USDT".to_string(), 20, "step0".to_string());
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"symbol\":\"BTC_USDT\""));
        assert!(json.contains("\"depth\":20"));
        assert!(json.contains("\"type\":\"step0\""));
    }

    #[test]
    fn test_order_book_aggregation_response_deserialization() {
        let json = r#"{
            "bids": [[45000.0, 1.5], [44999.0, 2.0]],
            "asks": [[45001.0, 1.2], [45002.0, 0.8]],
            "ts": 1640995200000
        }"#;
        
        let response: GetOrderBookAggregationResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.bids.len(), 2);
        assert_eq!(response.asks.len(), 2);
        assert_eq!(response.ts, 1640995200000);
        assert_eq!(response.bids[0][0], 45000.0);
        assert_eq!(response.bids[0][1], 1.5);
        assert_eq!(response.asks[0][0], 45001.0);
        assert_eq!(response.asks[0][1], 1.2);
    }

    #[tokio::test]
    async fn test_get_order_book_aggregation_method_exists() {
        let client = RestClient::new(
            "https://open-api.bingx.com",
            Client::new(),
            RateLimiter::new(),
        );

        let request = GetOrderBookAggregationRequest::new("BTC_USDT".to_string(), 20, "step0".to_string());
        
        // Test that the method exists and can be called
        // Note: This will fail with network error since we're not making real requests
        assert!(client.get_order_book_aggregation(&request).await.is_err());
    }
}