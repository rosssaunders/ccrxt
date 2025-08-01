use serde::{Serialize, Serializer};

use super::RestClient;
use crate::bingx::spot::{EndpointType, Interval, RestResult};

const KLINE_ENDPOINT: &str = "/openApi/spot/v2/market/kline";

/// Serialize interval enum as string
fn serialize_interval<S>(interval: &Interval, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(interval.as_str())
}

/// Request for the kline/candlestick data endpoint
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetKlineRequest {
    /// Trading pair, e.g., BTC-USDT (required)
    pub symbol: String,
    /// Time interval (required)
    #[serde(serialize_with = "serialize_interval")]
    pub interval: Interval,
    /// Start time, unit: milliseconds (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    /// End time, unit: milliseconds (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    /// Default value: 500 Maximum value: 1440 (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Timestamp of initiating the request, Unit: milliseconds (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<i64>,
    /// Request valid time window value, Unit: milliseconds (required)
    pub timestamp: i64,
}

/// Response from the kline/candlestick data endpoint
pub type GetKlineResponse = Vec<Kline>;

/// Candlestick/K-line data
/// [open_time, open, high, low, close, volume, close_time, quote_asset_volume]
pub type Kline = [f64; 8];

impl RestClient {
    /// Get kline/candlestick data
    ///
    /// Get candlestick chart data for a symbol within specified time intervals.
    ///
    /// # Arguments
    /// * `request` - The kline request parameters
    ///
    /// # Returns
    /// Response containing candlestick chart data array
    ///
    /// # Rate Limit
    /// - IP: 100 requests per 10 seconds (Group 1)
    ///
    /// # API Documentation
    /// - Endpoint: GET /openApi/spot/v2/market/kline
    /// - Content-Type: request body(application/json)
    /// - [docs]: https://bingx-api.github.io/docs/#/en-us/spot/market-api.html#Kline/Candlestick%20Data
    ///
    /// # Time Interval Rules
    /// - If startTime and endTime are not provided, the latest candlestick chart data will be returned by default
    /// - If startTime and endTime are provided, the latest candlestick chart data up to endTime will be returned by default
    /// - If startTime is provided and endTime is not provided, the latest candlestick chart data starting from startTime will be returned by default
    /// - If startTime is not provided and endTime is provided, the latest candlestick chart data up to endTime will be returned by default
    pub async fn get_kline(&self, request: GetKlineRequest) -> RestResult<GetKlineResponse> {
        self.send_request(KLINE_ENDPOINT, Some(&request), EndpointType::PublicMarket)
            .await
    }
}

#[cfg(test)]
mod tests {
    use reqwest::Client;

    use super::*;
    use crate::bingx::spot::RateLimiter;

    #[test]
    fn test_kline_request_creation() {
        let symbol = "BTC-USDT".to_string();
        let interval = Interval::OneHour;
        let timestamp = 1640995200000;
        let request = GetKlineRequest {
            symbol: symbol.clone(),
            interval,
            start_time: None,
            end_time: None,
            limit: None,
            recv_window: None,
            timestamp,
        };

        assert_eq!(request.symbol, symbol);
        assert_eq!(request.interval, interval);
        assert_eq!(request.timestamp, timestamp);
        assert!(request.start_time.is_none());
        assert!(request.end_time.is_none());
        assert!(request.limit.is_none());
        assert!(request.recv_window.is_none());
    }

    #[test]
    fn test_kline_request_with_time_range() {
        let symbol = "BTC-USDT".to_string();
        let interval = Interval::OneHour;
        let timestamp = 1640995200000;
        let start_time = 1640995200000;
        let end_time = 1641081600000;

        let request = GetKlineRequest {
            symbol: symbol.clone(),
            interval,
            start_time: Some(start_time),
            end_time: Some(end_time),
            limit: None,
            recv_window: None,
            timestamp,
        };

        assert_eq!(request.symbol, symbol);
        assert_eq!(request.interval, interval);
        assert_eq!(request.timestamp, timestamp);
        assert_eq!(request.start_time, Some(start_time));
        assert_eq!(request.end_time, Some(end_time));
    }

    #[test]
    fn test_kline_request_with_limit() {
        let symbol = "BTC-USDT".to_string();
        let interval = Interval::OneHour;
        let timestamp = 1640995200000;
        let limit = 100;

        let request = GetKlineRequest {
            symbol: symbol.clone(),
            interval,
            start_time: None,
            end_time: None,
            limit: Some(limit),
            recv_window: None,
            timestamp,
        };

        assert_eq!(request.symbol, symbol);
        assert_eq!(request.interval, interval);
        assert_eq!(request.timestamp, timestamp);
        assert_eq!(request.limit, Some(limit));
    }

    #[test]
    fn test_kline_request_serialization() {
        let request = GetKlineRequest {
            symbol: "BTC-USDT".to_string(),
            interval: Interval::OneHour,
            start_time: None,
            end_time: None,
            limit: None,
            recv_window: None,
            timestamp: 1640995200000,
        };
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"symbol\":\"BTC-USDT\""));
        assert!(json.contains("\"interval\":\"1h\""));
        assert!(json.contains("\"timestamp\":1640995200000"));
    }

    #[test]
    fn test_kline_response_deserialization() {
        let json = r#"[
            [1640995200000.0, 45000.0, 46000.0, 44000.0, 45500.0, 1000.0, 1640998799999.0, 45250000.0]
        ]"#;

        let response: GetKlineResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.len(), 1);
        let kline = response.first().expect("Expected at least one kline entry");
        assert_eq!(
            *kline.first().expect("Missing open_time in kline"),
            1640995200000.0
        ); // open_time
        assert_eq!(*kline.get(1).expect("Missing open in kline"), 45000.0); // open
        assert_eq!(*kline.get(2).expect("Missing high in kline"), 46000.0); // high
        assert_eq!(*kline.get(3).expect("Missing low in kline"), 44000.0); // low
        assert_eq!(*kline.get(4).expect("Missing close in kline"), 45500.0); // close
        assert_eq!(*kline.get(5).expect("Missing volume in kline"), 1000.0); // volume
    }

    #[tokio::test]
    async fn test_get_kline_method_exists() {
        let client = RestClient::new(
            "http://127.0.0.1:0", // Invalid URL to guarantee error
            Client::new(),
            RateLimiter::new(),
        );

        let request = GetKlineRequest {
            symbol: "BTC-USDT".to_string(),
            interval: Interval::OneHour,
            start_time: None,
            end_time: None,
            limit: None,
            recv_window: None,
            timestamp: 1640995200000,
        };

        // Test that the method exists and can be called
        // Note: This will fail with network error since we're not making real requests
        assert!(client.get_kline(&request).await.is_err());
    }
}
