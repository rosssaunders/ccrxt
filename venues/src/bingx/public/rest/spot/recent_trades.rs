use serde::{Deserialize, Serialize};

use crate::bingx::{EndpointType, PublicRestClient as RestClient, RestResult};

const RECENT_TRADES_ENDPOINT: &str = "/openApi/spot/v1/market/trades";

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
    /// [docs](https://bingx-api.github.io/docs/#/en-us/spot/market-api.html#Recent%20Trades%20List)
    ///
    /// # Arguments
    /// * `request` - The recent trades request parameters
    ///
    /// # Returns
    /// Vector of recent trades for the specified symbol
    ///
    /// # Rate Limit
    /// - IP: 100 requests per 10 seconds (Group 1)
    pub async fn get_recent_trades(
        &self,
        request: &GetRecentTradesRequest,
    ) -> RestResult<GetRecentTradesResponse> {
        self.send_request(
            RECENT_TRADES_ENDPOINT,
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
    fn test_recent_trades_request_creation() {
        let symbol = "BTC-USDT".to_string();
        let timestamp = 1640995200000;
        let request = GetRecentTradesRequest {
            symbol: symbol.clone(),
            timestamp,
            limit: None,
            recv_window: None,
        };

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
        let request = GetRecentTradesRequest {
            symbol: symbol.clone(),
            timestamp,
            limit: Some(limit),
            recv_window: None,
        };

        assert_eq!(request.symbol, symbol);
        assert_eq!(request.timestamp, timestamp);
        assert_eq!(request.limit, Some(limit));
    }

    #[test]
    fn test_recent_trades_request_with_recv_window() {
        let symbol = "BTC-USDT".to_string();
        let timestamp = 1640995200000;
        let recv_window = 5000;
        let request = GetRecentTradesRequest {
            symbol: symbol.clone(),
            timestamp,
            limit: None,
            recv_window: Some(recv_window),
        };

        assert_eq!(request.symbol, symbol);
        assert_eq!(request.timestamp, timestamp);
        assert_eq!(request.recv_window, Some(recv_window));
    }

    #[test]
    fn test_recent_trades_request_serialization() {
        let request = GetRecentTradesRequest {
            symbol: "BTC-USDT".to_string(),
            timestamp: 1640995200000,
            limit: None,
            recv_window: None,
        };
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
            "http://127.0.0.1:0", // Invalid URL to guarantee error
            std::sync::Arc::new(rest::native::NativeHttpClient::default()),
            RateLimiter::new(),
        );

        let request = GetRecentTradesRequest {
            symbol: "BTC-USDT".to_string(),
            timestamp: 1640995200000,
            limit: None,
            recv_window: None,
        };

        // Test that the method exists and can be called
        // Note: This will fail with network error since we're not making real requests
        assert!(client.get_recent_trades(&request).await.is_err());
    }
}
