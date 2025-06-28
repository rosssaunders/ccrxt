use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::bingx::{EndpointType, RestResult};

/// Request for the historical K-line endpoint
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetHistoricalKlineRequest {
    /// Trading pair, e.g., BTC-USDT, please use uppercase letters (required)
    pub symbol: String,
    /// Time interval, reference field description (required)
    /// Valid values: 1m, 3m, 5m, 15m, 30m, 1h, 2h, 4h, 6h, 8h, 12h, 1d, 3d, 1w, 1M
    pub interval: String,
    /// Start time, unit: milliseconds (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    /// End time, unit: milliseconds (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    /// Default value: 500 Maximum value: 500 (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

impl GetHistoricalKlineRequest {
    /// Create a new request for historical K-line data
    pub fn new(symbol: String, interval: String) -> Self {
        Self {
            symbol,
            interval,
            start_time: None,
            end_time: None,
            limit: None,
        }
    }

    /// Set the start time
    pub fn with_start_time(mut self, start_time: i64) -> Self {
        self.start_time = Some(start_time);
        self
    }

    /// Set the end time
    pub fn with_end_time(mut self, end_time: i64) -> Self {
        self.end_time = Some(end_time);
        self
    }

    /// Set the limit for number of klines to return
    pub fn with_limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
}

/// Response from the historical K-line endpoint
#[derive(Debug, Clone, Deserialize)]
pub struct GetHistoricalKlineResponse {
    /// K-line array
    pub klines: Vec<HistoricalKline>,
}

/// Historical K-line data
/// [open_time, open, high, low, close, volume, close_time, quote_asset_volume, count, taker_buy_base_asset_volume, taker_buy_quote_asset_volume, ignore]
pub type HistoricalKline = [f64; 12];

impl RestClient {
    /// Get historical K-line data
    ///
    /// Query historical K-line data for transaction prices.
    ///
    /// # Arguments
    /// * `request` - The historical K-line request parameters
    ///
    /// # Returns
    /// Response containing historical K-line data array
    ///
    /// # Rate Limit
    /// - IP: 100 requests per 10 seconds (Group 1)
    ///
    /// # API Documentation
    /// - Endpoint: GET /openApi/market/his/v1/kline
    /// - Content-Type: request body(application/json)
    ///
    /// # Time Rules
    /// - If startTime and endTime are not sent, the latest K-line data is returned by default
    /// - If startTime and endTime are sent, the latest K-line data up to endTime is returned by default
    /// - If startTime is sent but endTime is not sent, the latest K-line data starting from startTime is returned by default
    /// - If startTime is not sent but endTime is sent, the latest K-line data up to endTime is returned by default
    pub async fn get_historical_kline(
        &self,
        request: &GetHistoricalKlineRequest,
    ) -> RestResult<GetHistoricalKlineResponse> {
        self.send_request(
            "/openApi/market/his/v1/kline",
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
    fn test_historical_kline_request_creation() {
        let symbol = "BTC-USDT".to_string();
        let interval = "1h".to_string();
        let request = GetHistoricalKlineRequest::new(symbol.clone(), interval.clone());

        assert_eq!(request.symbol, symbol);
        assert_eq!(request.interval, interval);
        assert!(request.start_time.is_none());
        assert!(request.end_time.is_none());
        assert!(request.limit.is_none());
    }

    #[test]
    fn test_historical_kline_request_with_time_range() {
        let symbol = "BTC-USDT".to_string();
        let interval = "1h".to_string();
        let start_time = 1640995200000;
        let end_time = 1641081600000;

        let request = GetHistoricalKlineRequest::new(symbol.clone(), interval.clone())
            .with_start_time(start_time)
            .with_end_time(end_time);

        assert_eq!(request.symbol, symbol);
        assert_eq!(request.interval, interval);
        assert_eq!(request.start_time, Some(start_time));
        assert_eq!(request.end_time, Some(end_time));
    }

    #[test]
    fn test_historical_kline_request_with_limit() {
        let symbol = "BTC-USDT".to_string();
        let interval = "1h".to_string();
        let limit = 100;

        let request =
            GetHistoricalKlineRequest::new(symbol.clone(), interval.clone()).with_limit(limit);

        assert_eq!(request.symbol, symbol);
        assert_eq!(request.interval, interval);
        assert_eq!(request.limit, Some(limit));
    }

    #[test]
    fn test_historical_kline_request_serialization() {
        let request = GetHistoricalKlineRequest::new("BTC-USDT".to_string(), "1h".to_string());
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"symbol\":\"BTC-USDT\""));
        assert!(json.contains("\"interval\":\"1h\""));
    }

    #[test]
    fn test_historical_kline_response_deserialization() {
        let json = r#"{
            "klines": [
                [1640995200000.0, 45000.0, 46000.0, 44000.0, 45500.0, 1000.0, 1640998799999.0, 45250000.0, 500.0, 600.0, 27150000.0, 0.0]
            ]
        }"#;

        let response: GetHistoricalKlineResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.klines.len(), 1);
        let kline = response
            .klines
            .first()
            .expect("Expected at least one kline entry");
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
    async fn test_get_historical_kline_method_exists() {
        let client = RestClient::new(
            "https://open-api.bingx.com",
            Client::new(),
            RateLimiter::new(),
        );

        let request = GetHistoricalKlineRequest::new("BTC-USDT".to_string(), "1h".to_string());

        // Test that the method exists and can be called
        // Note: This will fail with network error since we're not making real requests
        assert!(client.get_historical_kline(&request).await.is_err());
    }
}
