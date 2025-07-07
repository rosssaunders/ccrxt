use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::bingx::{EndpointType, RestResult};

const SYMBOL_ORDER_BOOK_TICKER_ENDPOINT: &str = "/openApi/spot/v1/ticker/bookTicker";

/// Request for the symbol order book ticker endpoint
#[derive(Debug, Clone, Serialize)]
pub struct GetSymbolOrderBookTickerRequest {
    /// Trading pair, such as: BTC_USDT (required)
    pub symbol: String,
}

/// Response from the symbol order book ticker endpoint
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSymbolOrderBookTickerResponse {
    /// Data type
    pub event_type: String,
    /// Trading pair, such as: BTC_USDT
    pub symbol: String,
    /// Best bid price
    pub bid_price: String,
    /// Best bid volume
    pub bid_volume: String,
    /// Best ask price
    pub ask_price: String,
    /// Best ask volume
    pub ask_volume: String,
}

impl RestClient {
    /// Get symbol order book ticker
    ///
    /// Get the best bid and ask prices and volumes for a symbol.
    ///
    /// # Arguments
    /// * `request` - The symbol order book ticker request parameters
    ///
    /// # Returns
    /// Symbol order book ticker response containing best bid/ask prices and volumes
    ///
    /// # Rate Limit
    /// - IP: 100 requests per 10 seconds (Group 1)
    ///
    /// # API Documentation
    /// - Endpoint: GET /openApi/spot/v1/ticker/bookTicker
    /// - Content-Type: request body(application/json)
    pub async fn get_symbol_order_book_ticker(
        &self,
        request: &GetSymbolOrderBookTickerRequest,
    ) -> RestResult<GetSymbolOrderBookTickerResponse> {
        self.send_request(
            SYMBOL_ORDER_BOOK_TICKER_ENDPOINT,
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
    fn test_symbol_order_book_ticker_request_creation() {
        let symbol = "BTC_USDT".to_string();
        let request = GetSymbolOrderBookTickerRequest {
            symbol: symbol.clone(),
        };

        assert_eq!(request.symbol, symbol);
    }

    #[test]
    fn test_symbol_order_book_ticker_request_serialization() {
        let request = GetSymbolOrderBookTickerRequest {
            symbol: "BTC_USDT".to_string(),
        };
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"symbol\":\"BTC_USDT\""));
    }

    #[test]
    fn test_symbol_order_book_ticker_response_deserialization() {
        let json = r#"{
            "eventType": "bookTicker",
            "symbol": "BTC_USDT",
            "bidPrice": "44999.50",
            "bidVolume": "1.5",
            "askPrice": "45001.00",
            "askVolume": "2.0"
        }"#;

        let response: GetSymbolOrderBookTickerResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.event_type, "bookTicker");
        assert_eq!(response.symbol, "BTC_USDT");
        assert_eq!(response.bid_price, "44999.50");
        assert_eq!(response.bid_volume, "1.5");
        assert_eq!(response.ask_price, "45001.00");
        assert_eq!(response.ask_volume, "2.0");
    }

    #[tokio::test]
    async fn test_get_symbol_order_book_ticker_method_exists() {
        let client = RestClient::new(
            "https://open-api.bingx.com",
            Client::new(),
            RateLimiter::new(),
        );

        let request = GetSymbolOrderBookTickerRequest {
            symbol: "BTC_USDT".to_string(),
        };

        // Test that the method exists and can be called
        // Note: This will fail with network error since we're not making real requests
        assert!(client.get_symbol_order_book_ticker(&request).await.is_err());
    }
}
