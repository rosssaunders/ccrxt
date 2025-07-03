use serde::{Deserialize, Serialize, Serializer};

use super::RestClient;
use crate::bingx::{DepthType, EndpointType, RestResult};

/// Serialize depth type enum as string
fn serialize_depth_type<S>(depth_type: &DepthType, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(depth_type.as_str())
}

/// Request for the order book aggregation endpoint
#[derive(Debug, Clone, Serialize)]
pub struct GetOrderBookAggregationRequest {
    /// Trading pair, such as: BTC_USDT (required)
    pub symbol: String,
    /// Query depth (required)
    pub depth: i64,
    /// Precision type: step0 default precision, step1 to step5 are 10 to 100000 times precision respectively (required)
    #[serde(rename = "type", serialize_with = "serialize_depth_type")]
    pub type_: DepthType,
}

impl GetOrderBookAggregationRequest {
    /// Create a new request for order book aggregation
    pub fn new(symbol: String, depth: i64, type_: DepthType) -> Self {
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
    pub async fn get_order_book_aggregation(
        &self,
        request: &GetOrderBookAggregationRequest,
    ) -> RestResult<GetOrderBookAggregationResponse> {
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
    use reqwest::Client;

    use super::*;
    use crate::bingx::RateLimiter;

    #[test]
    fn test_order_book_aggregation_request_creation() {
        let symbol = "BTC_USDT".to_string();
        let depth = 20;
        let type_ = DepthType::Step0;
        let request = GetOrderBookAggregationRequest::new(symbol.clone(), depth, type_);

        assert_eq!(request.symbol, symbol);
        assert_eq!(request.depth, depth);
        assert_eq!(request.type_, type_);
    }

    #[test]
    fn test_order_book_aggregation_request_serialization() {
        let request =
            GetOrderBookAggregationRequest::new("BTC_USDT".to_string(), 20, DepthType::Step0);
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
        let bid0 = response.bids.first().expect("Expected at least one bid");
        assert_eq!(*bid0.first().expect("Missing price in bid0"), 45000.0);
        assert_eq!(*bid0.get(1).expect("Missing amount in bid0"), 1.5);
        let ask0 = response.asks.first().expect("Expected at least one ask");
        assert_eq!(*ask0.first().expect("Missing price in ask0"), 45001.0);
        assert_eq!(*ask0.get(1).expect("Missing amount in ask0"), 1.2);
    }

    #[tokio::test]
    async fn test_get_order_book_aggregation_method_exists() {
        let client = RestClient::new(
            "https://open-api.bingx.com",
            Client::new(),
            RateLimiter::new(),
        );

        let request =
            GetOrderBookAggregationRequest::new("BTC_USDT".to_string(), 20, DepthType::Step0);

        // Test that the method exists and can be called
        // Note: This will fail with network error since we're not making real requests
        assert!(client.get_order_book_aggregation(&request).await.is_err());
    }
}
