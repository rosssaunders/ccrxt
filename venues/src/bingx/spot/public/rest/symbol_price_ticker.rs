use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::bingx::spot::{EndpointType, RestResult};

const SYMBOL_PRICE_TICKER_ENDPOINT: &str = "/openApi/spot/v1/ticker/price";

/// Request for the symbol price ticker endpoint
#[derive(Debug, Clone, Serialize)]
pub struct GetSymbolPriceTickerRequest {
    /// Trading pair, such as: BTC_USDT (required)
    pub symbol: String,
}

/// Response from the symbol price ticker endpoint
pub type GetSymbolPriceTickerResponse = Vec<SymbolTicker>;

/// Symbol ticker data
#[derive(Debug, Clone, Deserialize)]
pub struct SymbolTicker {
    /// Trading pair, such as: BTC_USDT
    pub symbol: String,
    /// Trade data array
    pub trades: Vec<TradeData>,
}

/// Trade data within ticker response
#[derive(Debug, Clone, Deserialize)]
pub struct TradeData {
    /// Trade timestamp
    pub timestamp: i64,
    /// Trade ID
    #[serde(rename = "tradeId")]
    pub trade_id: String,
    /// Trade price
    pub price: String,
    /// Amount (may be empty)
    #[serde(default)]
    pub amount: String,
    /// Trade type
    #[serde(rename = "type")]
    pub trade_type: i32,
    /// Volume
    pub volume: String,
}

impl RestClient {
    /// Get symbol price ticker
    ///
    /// Get the latest price for a symbol.
    ///
    /// # Arguments
    /// * `request` - The symbol price ticker request parameters
    ///
    /// # Returns
    /// Symbol price ticker response containing latest price and timestamp
    ///
    /// # Rate Limit
    /// - IP: 100 requests per 10 seconds (Group 1)
    ///
    /// # API Documentation
    /// - Endpoint: GET /openApi/spot/v1/ticker/price
    /// - Content-Type: request body(application/json)
    /// - [docs]: https://bingx-api.github.io/docs/#/en-us/spot/market-api.html#Symbol%20Price%20Ticker
    pub async fn get_symbol_price_ticker(
        &self,
        request: &GetSymbolPriceTickerRequest,
    ) -> RestResult<GetSymbolPriceTickerResponse> {
        self.send_request(
            SYMBOL_PRICE_TICKER_ENDPOINT,
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
    fn test_symbol_price_ticker_request_creation() {
        let symbol = "BTC_USDT".to_string();
        let request = GetSymbolPriceTickerRequest {
            symbol: symbol.clone(),
        };

        assert_eq!(request.symbol, symbol);
    }

    #[test]
    fn test_symbol_price_ticker_request_serialization() {
        let request = GetSymbolPriceTickerRequest {
            symbol: "BTC_USDT".to_string(),
        };
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"symbol\":\"BTC_USDT\""));
    }

    #[test]
    fn test_symbol_price_ticker_response_deserialization() {
        let json = r#"[{
            "symbol": "BTC_USDT",
            "trades": [{
                "timestamp": 1640995200000,
                "tradeId": "123456",
                "price": "45000.50",
                "amount": "",
                "type": 1,
                "volume": "0.1"
            }]
        }]"#;

        let response: GetSymbolPriceTickerResponse = serde_json::from_str(json).unwrap();
        let ticker = response.first().expect("Expected at least one ticker");
        assert_eq!(ticker.symbol, "BTC_USDT");
        assert_eq!(ticker.trades.len(), 1);
        let trade = ticker.trades.first().expect("Expected at least one trade");
        assert_eq!(trade.price, "45000.50");
        assert_eq!(trade.timestamp, 1640995200000);
    }

    #[tokio::test]
    async fn test_get_symbol_price_ticker_method_exists() {
        let client = RestClient::new(
            "http://127.0.0.1:0", // Invalid URL to guarantee error
            Client::new(),
            RateLimiter::new(),
        );

        let request = GetSymbolPriceTickerRequest {
            symbol: "BTC_USDT".to_string(),
        };

        // Test that the method exists and can be called
        // Note: This will fail with network error since we're not making real requests
        assert!(client.get_symbol_price_ticker(&request).await.is_err());
    }
}
