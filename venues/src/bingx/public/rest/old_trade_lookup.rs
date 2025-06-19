use serde::{Deserialize, Serialize};

use crate::bingx::{EndpointType, RestResult};
use super::RestClient;

/// Request for the old trade lookup endpoint
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOldTradeRequest {
    /// Trading pair, e.g., BTC-USDT, please use uppercase letters (required)
    pub symbol: String,
    /// Default 100, maximum 500 (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// The last recorded tid (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_id: Option<String>,
}

impl GetOldTradeRequest {
    /// Create a new request for old trade lookup
    pub fn new(symbol: String) -> Self {
        Self {
            symbol,
            limit: None,
            from_id: None,
        }
    }

    /// Set the limit for number of trades to return
    pub fn with_limit(mut self, limit: i32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Set the from_id to get trades from a specific trade ID
    pub fn with_from_id(mut self, from_id: String) -> Self {
        self.from_id = Some(from_id);
        self
    }
}

/// Response from the old trade lookup endpoint
pub type GetOldTradeResponse = Vec<OldTrade>;

/// Old trade information
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OldTrade {
    /// Trade id
    pub id: i64,
    /// Price
    pub price: f64,
    /// Quantity
    pub qty: f64,
    /// Time
    pub time: i64,
    /// Buyer maker
    pub buyer_maker: bool,
}

impl RestClient {
    /// Get old trade lookup
    ///
    /// Get historical trade data for a symbol.
    ///
    /// # Arguments
    /// * `request` - The old trade lookup request parameters
    ///
    /// # Returns
    /// Vector of historical trades for the specified symbol
    ///
    /// # Rate Limit
    /// - IP: 100 requests per 10 seconds (Group 1)
    ///
    /// # API Documentation
    /// - Endpoint: GET /openApi/market/his/v1/trade
    /// - Content-Type: request body(application/json)
    pub async fn get_old_trade(&self, request: &GetOldTradeRequest) -> RestResult<GetOldTradeResponse> {
        self.send_request(
            "/openApi/market/his/v1/trade",
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
    fn test_old_trade_request_creation() {
        let symbol = "BTC-USDT".to_string();
        let request = GetOldTradeRequest::new(symbol.clone());
        
        assert_eq!(request.symbol, symbol);
        assert!(request.limit.is_none());
        assert!(request.from_id.is_none());
    }

    #[test]
    fn test_old_trade_request_with_limit() {
        let symbol = "BTC-USDT".to_string();
        let limit = 50;
        let request = GetOldTradeRequest::new(symbol.clone()).with_limit(limit);
        
        assert_eq!(request.symbol, symbol);
        assert_eq!(request.limit, Some(limit));
    }

    #[test]
    fn test_old_trade_request_with_from_id() {
        let symbol = "BTC-USDT".to_string();
        let from_id = "12345".to_string();
        let request = GetOldTradeRequest::new(symbol.clone()).with_from_id(from_id.clone());
        
        assert_eq!(request.symbol, symbol);
        assert_eq!(request.from_id, Some(from_id));
    }

    #[test]
    fn test_old_trade_request_serialization() {
        let request = GetOldTradeRequest::new("BTC-USDT".to_string());
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"symbol\":\"BTC-USDT\""));
    }

    #[test]
    fn test_old_trade_deserialization() {
        let json = r#"{
            "id": 123456,
            "price": 45000.50,
            "qty": 0.1,
            "time": 1640995200000,
            "buyerMaker": true
        }"#;
        
        let trade: OldTrade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.id, 123456);
        assert_eq!(trade.price, 45000.50);
        assert_eq!(trade.qty, 0.1);
        assert_eq!(trade.time, 1640995200000);
        assert!(trade.buyer_maker);
    }

    #[tokio::test]
    async fn test_get_old_trade_method_exists() {
        let client = RestClient::new(
            "https://open-api.bingx.com",
            Client::new(),
            RateLimiter::new(),
        );

        let request = GetOldTradeRequest::new("BTC-USDT".to_string());
        
        // Test that the method exists and can be called
        // Note: This will fail with network error since we're not making real requests
        assert!(client.get_old_trade(&request).await.is_err());
    }
}