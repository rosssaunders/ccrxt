use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::bingx::spot::{EndpointType, RestResult};

const OLD_TRADE_LOOKUP_ENDPOINT: &str = "/openApi/market/his/v1/trade";

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

/// Response from the old trade lookup endpoint
pub type GetOldTradeResponse = Vec<OldTrade>;

/// Old trade information
#[derive(Debug, Clone, Deserialize)]
pub struct OldTrade {
    /// Trade id
    pub tid: String,
    /// Trade time
    pub t: i64,
    /// Market side (1=buy, 2=sell)
    pub ms: i32,
    /// Symbol
    pub s: String,
    /// Price
    pub p: f64,
    /// Volume/Quantity
    pub v: f64,
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
    pub async fn get_old_trade(
        &self,
        request: &GetOldTradeRequest,
    ) -> RestResult<GetOldTradeResponse> {
        self.send_request(
            OLD_TRADE_LOOKUP_ENDPOINT,
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
    use crate::bingx::spot::RateLimiter;

    #[test]
    fn test_old_trade_request_creation() {
        let symbol = "BTC-USDT".to_string();
        let request = GetOldTradeRequest {
            symbol: symbol.clone(),
            limit: None,
            from_id: None,
        };

        assert_eq!(request.symbol, symbol);
        assert!(request.limit.is_none());
        assert!(request.from_id.is_none());
    }

    #[test]
    fn test_old_trade_request_with_limit() {
        let symbol = "BTC-USDT".to_string();
        let limit = 50;
        let request = GetOldTradeRequest {
            symbol: symbol.clone(),
            limit: Some(limit),
            from_id: None,
        };

        assert_eq!(request.symbol, symbol);
        assert_eq!(request.limit, Some(limit));
    }

    #[test]
    fn test_old_trade_request_with_from_id() {
        let symbol = "BTC-USDT".to_string();
        let from_id = "12345".to_string();
        let request = GetOldTradeRequest {
            symbol: symbol.clone(),
            limit: None,
            from_id: Some(from_id.clone()),
        };

        assert_eq!(request.symbol, symbol);
        assert_eq!(request.from_id, Some(from_id));
    }

    #[test]
    fn test_old_trade_request_serialization() {
        let request = GetOldTradeRequest {
            symbol: "BTC-USDT".to_string(),
            limit: None,
            from_id: None,
        };
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"symbol\":\"BTC-USDT\""));
    }

    #[test]
    fn test_old_trade_deserialization() {
        let json = r#"{
            "tid": "123456",
            "t": 1640995200000,
            "ms": 1,
            "s": "BTC-USDT",
            "p": 45000.50,
            "v": 0.1
        }"#;

        let trade: OldTrade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.tid, "123456");
        assert_eq!(trade.t, 1640995200000);
        assert_eq!(trade.ms, 1);
        assert_eq!(trade.s, "BTC-USDT");
        assert_eq!(trade.p, 45000.50);
        assert_eq!(trade.v, 0.1);
    }

    #[tokio::test]
    async fn test_get_old_trade_method_exists() {
        let client = RestClient::new(
            "http://127.0.0.1:0", // Invalid URL to guarantee error
            Client::new(),
            RateLimiter::new(),
        );

        let request = GetOldTradeRequest {
            symbol: "BTC-USDT".to_string(),
            limit: None,
            from_id: None,
        };

        // Test that the method exists and can be called
        // Note: This will fail with network error since we're not making real requests
        assert!(client.get_old_trade(&request).await.is_err());
    }
}
