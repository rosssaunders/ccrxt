use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::bingx::spot::{EndpointType, RestResult};

const TICKER_24HR_ENDPOINT: &str = "/openApi/spot/v1/ticker/24hr";

/// Request for the 24hr ticker price change statistics endpoint
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Get24hrTickerRequest {
    /// Trading pairs, such as: BTC-USDT, will return all symbol data when no parameters are entered (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// The timestamp of the request, in milliseconds (required)
    pub timestamp: i64,
    /// Request valid time window value, unit: millisecond (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<i64>,
}

/// Response from the 24hr ticker endpoint
pub type Get24hrTickerResponse = Vec<Ticker24hr>;

/// 24-hour ticker price change statistics
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ticker24hr {
    /// Trading pair, for example: BTC-USDT
    pub symbol: String,
    /// Opening price in the last 24 hours
    pub open_price: f64,
    /// The highest price in the last 24 hours
    pub high_price: f64,
    /// The lowest price in the last 24 hours
    pub low_price: f64,
    /// Latest price
    pub last_price: f64,
    /// Price change amount
    #[serde(default)]
    pub price_change: Option<f64>,
    /// Total trading volume (base asset)
    pub volume: f64,
    /// Total quote volume (quote asset)
    pub quote_volume: f64,
    /// The start time of the ticker interval
    pub open_time: i64,
    /// End time of the ticker interval
    pub close_time: i64,
    /// The number of transactions within the statistical time
    #[serde(default)]
    pub count: Option<i32>,
    /// Bid price
    pub ask_price: f64,
    /// Bid quantity
    pub ask_qty: f64,
    /// Ask price
    pub bid_price: f64,
    /// Ask quantity
    pub bid_qty: f64,
    /// Price change percentage field
    pub price_change_percent: String,
}

impl RestClient {
    /// Get 24hr ticker price change statistics
    ///
    /// Get price change statistics for symbols over a rolling 24hr period.
    ///
    /// # Arguments
    /// * `request` - The 24hr ticker request parameters
    ///
    /// # Returns
    /// Vector of 24hr price change statistics for requested symbols
    ///
    /// # Rate Limit
    /// - IP: 100 requests per 10 seconds (Group 1)
    ///
    /// # API Documentation
    /// - Endpoint: GET /openApi/spot/v1/ticker/24hr
    /// - Content-Type: request body(application/json)
    /// - [docs]: https://bingx-api.github.io/docs/#/en-us/spot/market-api.html#24hr%20Ticker%20Price%20Change%20Statistics
    pub async fn get_24hr_ticker(
        &self,
        request: &Get24hrTickerRequest,
    ) -> RestResult<Get24hrTickerResponse> {
        self.send_request(
            TICKER_24HR_ENDPOINT,
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
    fn test_24hr_ticker_request_creation() {
        let timestamp = 1640995200000;
        let request = Get24hrTickerRequest {
            timestamp,
            symbol: None,
            recv_window: None,
        };

        assert_eq!(request.timestamp, timestamp);
        assert!(request.symbol.is_none());
        assert!(request.recv_window.is_none());
    }

    #[test]
    fn test_24hr_ticker_request_for_symbol() {
        let timestamp = 1640995200000;
        let symbol = "BTC-USDT".to_string();
        let request = Get24hrTickerRequest {
            timestamp,
            symbol: Some(symbol.clone()),
            recv_window: None,
        };

        assert_eq!(request.timestamp, timestamp);
        assert_eq!(request.symbol, Some(symbol));
        assert!(request.recv_window.is_none());
    }

    #[test]
    fn test_24hr_ticker_request_with_recv_window() {
        let timestamp = 1640995200000;
        let recv_window = 5000;
        let request = Get24hrTickerRequest {
            timestamp,
            symbol: None,
            recv_window: Some(recv_window),
        };

        assert_eq!(request.timestamp, timestamp);
        assert_eq!(request.recv_window, Some(recv_window));
    }

    #[test]
    fn test_24hr_ticker_request_serialization() {
        let request = Get24hrTickerRequest {
            timestamp: 1640995200000,
            symbol: None,
            recv_window: None,
        };
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"timestamp\":1640995200000"));
    }

    #[test]
    fn test_ticker_24hr_deserialization() {
        let json = r#"{
            "symbol": "BTC-USDT",
            "openPrice": 44000.00,
            "highPrice": 46000.00,
            "lowPrice": 43000.00,
            "lastPrice": 45000.00,
            "volume": 1000.50,
            "quoteVolume": 45002250.00,
            "openTime": 1640995200000,
            "closeTime": 1641081599999,
            "count": 12345,
            "bidPrice": 44999.0,
            "bidQty": 1.5,
            "askPrice": 45001.0,
            "askQty": 2.0,
            "priceChangePercent": "2.27"
        }"#;

        let ticker: Ticker24hr = serde_json::from_str(json).unwrap();
        assert_eq!(ticker.symbol, "BTC-USDT");
        assert_eq!(ticker.open_price, 44000.0);
        assert_eq!(ticker.high_price, 46000.0);
        assert_eq!(ticker.low_price, 43000.0);
        assert_eq!(ticker.last_price, 45000.0);
        assert_eq!(ticker.volume, 1000.5);
        assert_eq!(ticker.quote_volume, 45002250.0);
        assert_eq!(ticker.open_time, 1640995200000);
        assert_eq!(ticker.close_time, 1641081599999);
        assert_eq!(ticker.count, Some(12345));
        assert_eq!(ticker.bid_price, 44999.0);
        assert_eq!(ticker.bid_qty, 1.5);
        assert_eq!(ticker.ask_price, 45001.0);
        assert_eq!(ticker.ask_qty, 2.0);
        assert_eq!(ticker.price_change_percent, "2.27");
    }

    #[tokio::test]
    async fn test_get_24hr_ticker_method_exists() {
        let client = RestClient::new(
            "https://open-api.bingx.com",
            std::sync::Arc::new(rest::native::NativeHttpClient::default()),
            RateLimiter::new(),
        );

        let request = Get24hrTickerRequest {
            timestamp: 1640995200000,
            symbol: None,
            recv_window: None,
        };

        // Test that the method exists and can be called
        // Note: This will fail with network error since we're not making real requests
        assert!(client.get_24hr_ticker(&request).await.is_err());
    }
}
