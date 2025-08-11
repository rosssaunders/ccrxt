use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::bingx::spot::{EndpointType, RestResult};

const SYMBOL_ORDER_BOOK_TICKER_ENDPOINT: &str = "/openApi/spot/v1/ticker/bookTicker";

/// Request for the symbol order book ticker endpoint
#[derive(Debug, Clone, Serialize)]
pub struct GetSymbolOrderBookTickerRequest {
    /// Trading pair, such as: BTC_USDT (required)
    pub symbol: String,
}

/// Response from the symbol order book ticker endpoint
pub type GetSymbolOrderBookTickerResponse = Vec<OrderBookTicker>;

/// Order book ticker data
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderBookTicker {
    /// Data type
    pub event_type: String,
    /// Event time
    pub time: i64,
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
    /// - [docs]: https://bingx-api.github.io/docs/#/en-us/spot/market-api.html#Symbol%20Order%20Book%20Ticker
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

    use super::*;
    use crate::bingx::spot::RateLimiter;

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
        let json = r#"[{
            "eventType": "bookTicker",
            "time": 1640995200000,
            "symbol": "BTC_USDT",
            "bidPrice": "44999.50",
            "bidVolume": "1.5",
            "askPrice": "45001.00",
            "askVolume": "2.0"
        }]"#;

        let response: GetSymbolOrderBookTickerResponse = serde_json::from_str(json).unwrap();
        let ticker = response.first().expect("Expected at least one ticker");
        assert_eq!(ticker.event_type, "bookTicker");
        assert_eq!(ticker.symbol, "BTC_USDT");
        assert_eq!(ticker.bid_price, "44999.50");
        assert_eq!(ticker.bid_volume, "1.5");
        assert_eq!(ticker.ask_price, "45001.00");
        assert_eq!(ticker.ask_volume, "2.0");
    }

    #[tokio::test]
    async fn test_get_symbol_order_book_ticker_method_exists() {
        let client = RestClient::new(
            "http://127.0.0.1:0", // Invalid URL to guarantee error
            std::sync::Arc::new(rest::native::NativeHttpClient::default()),
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
