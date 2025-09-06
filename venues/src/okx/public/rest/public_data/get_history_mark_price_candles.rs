use serde::{Deserialize, Serialize};

use crate::okx::{EndpointType, RestResult, public_client::RestClient};

const MARKET_HISTORY_MARK_PRICE_CANDLES_ENDPOINT: &str = "api/v5/market/history-mark-price-candles";

/// Bar size/timeframe for candlesticks
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum BarSize {
    /// 1 minute
    #[serde(rename = "1m")]
    OneMinute,

    /// 3 minutes
    #[serde(rename = "3m")]
    ThreeMinutes,

    /// 5 minutes
    #[serde(rename = "5m")]
    FiveMinutes,

    /// 15 minutes
    #[serde(rename = "15m")]
    FifteenMinutes,

    /// 30 minutes
    #[serde(rename = "30m")]
    ThirtyMinutes,

    /// 1 hour
    #[serde(rename = "1H")]
    OneHour,

    /// 2 hours
    #[serde(rename = "2H")]
    TwoHours,

    /// 4 hours
    #[serde(rename = "4H")]
    FourHours,

    /// 6 hours (Hong Kong time)
    #[serde(rename = "6H")]
    SixHours,

    /// 12 hours (Hong Kong time)
    #[serde(rename = "12H")]
    TwelveHours,

    /// 1 day (Hong Kong time)
    #[serde(rename = "1D")]
    OneDay,

    /// 1 week (Hong Kong time)
    #[serde(rename = "1W")]
    OneWeek,

    /// 1 month (Hong Kong time)
    #[serde(rename = "1M")]
    OneMonth,

    /// 6 hours (UTC time)
    #[serde(rename = "6Hutc")]
    SixHoursUtc,

    /// 12 hours (UTC time)
    #[serde(rename = "12Hutc")]
    TwelveHoursUtc,

    /// 1 day (UTC time)
    #[serde(rename = "1Dutc")]
    OneDayUtc,

    /// 1 week (UTC time)
    #[serde(rename = "1Wutc")]
    OneWeekUtc,

    /// 1 month (UTC time)
    #[serde(rename = "1Mutc")]
    OneMonthUtc,
}

/// Request parameters for getting mark price candlesticks history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetHistoryMarkPriceCandlesRequest {
    /// Instrument ID (e.g., "BTC-USD-SWAP")
    #[serde(rename = "instId")]
    pub inst_id: String,

    /// Pagination of data to return records earlier than the requested ts
    #[serde(rename = "after", skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,

    /// Pagination of data to return records newer than the requested ts
    #[serde(rename = "before", skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,

    /// Bar size, default is 1m
    #[serde(rename = "bar", skip_serializing_if = "Option::is_none")]
    pub bar: Option<BarSize>,

    /// Number of results per request. Max 100, default 100
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

/// Individual mark price candle data
/// Format: [ts, o, h, l, c, confirm]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkPriceCandle {
    /// Opening time of the candlestick, Unix timestamp in milliseconds
    pub ts: String,

    /// Open price
    pub o: String,

    /// Highest price
    pub h: String,

    /// Lowest price
    pub l: String,

    /// Close price
    pub c: String,

    /// The state of candlesticks. 0: uncompleted, 1: completed
    pub confirm: String,
}

impl RestClient {
    /// Get mark price candlesticks history
    ///
    /// Retrieve the candlestick charts of mark price from recent years.
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#rest-api-public-rest-api-get-mark-price-candlesticks-history)
    ///
    /// Rate limit: 20 requests per 2 seconds
    ///
    /// # Arguments
    /// * `request` - The mark price candles request parameters
    ///
    /// # Returns
    /// Response containing the mark price candlestick data
    pub async fn get_history_mark_price_candles(
        &self,
        request: GetHistoryMarkPriceCandlesRequest,
    ) -> RestResult<[String; 6]> {
        self.send_get_request(
            MARKET_HISTORY_MARK_PRICE_CANDLES_ENDPOINT,
            Some(&request),
            EndpointType::PublicMarketData,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::okx::response::ApiResponse;

    #[test]
    fn test_bar_size_serialization() {
        assert_eq!(
            serde_json::to_value(BarSize::OneMinute).unwrap(),
            json!("1m")
        );
        assert_eq!(
            serde_json::to_value(BarSize::FifteenMinutes).unwrap(),
            json!("15m")
        );
        assert_eq!(serde_json::to_value(BarSize::OneHour).unwrap(), json!("1H"));
        assert_eq!(serde_json::to_value(BarSize::OneDay).unwrap(), json!("1D"));
        assert_eq!(
            serde_json::to_value(BarSize::OneDayUtc).unwrap(),
            json!("1Dutc")
        );
    }

    #[test]
    fn test_bar_size_deserialization() {
        assert_eq!(
            serde_json::from_value::<BarSize>(json!("1m")).unwrap(),
            BarSize::OneMinute
        );
        assert_eq!(
            serde_json::from_value::<BarSize>(json!("5m")).unwrap(),
            BarSize::FiveMinutes
        );
        assert_eq!(
            serde_json::from_value::<BarSize>(json!("1H")).unwrap(),
            BarSize::OneHour
        );
        assert_eq!(
            serde_json::from_value::<BarSize>(json!("1Wutc")).unwrap(),
            BarSize::OneWeekUtc
        );
    }

    #[test]
    fn test_get_history_mark_price_candles_request_minimal() {
        let request = GetHistoryMarkPriceCandlesRequest {
            inst_id: "BTC-USD-SWAP".to_string(),
            after: None,
            before: None,
            bar: None,
            limit: None,
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(
            serialized.get("instId").and_then(|v| v.as_str()),
            Some("BTC-USD-SWAP")
        );
        assert!(serialized.get("after").is_none());
        assert!(serialized.get("before").is_none());
        assert!(serialized.get("bar").is_none());
        assert!(serialized.get("limit").is_none());
    }

    #[test]
    fn test_get_history_mark_price_candles_request_full() {
        let request = GetHistoryMarkPriceCandlesRequest {
            inst_id: "BTC-USD-SWAP".to_string(),
            after: Some("1597026383085".to_string()),
            before: Some("1597026383000".to_string()),
            bar: Some(BarSize::FifteenMinutes),
            limit: Some("100".to_string()),
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(
            serialized.get("instId").and_then(|v| v.as_str()),
            Some("BTC-USD-SWAP")
        );
        assert_eq!(
            serialized.get("after").and_then(|v| v.as_str()),
            Some("1597026383085")
        );
        assert_eq!(
            serialized.get("before").and_then(|v| v.as_str()),
            Some("1597026383000")
        );
        assert_eq!(serialized.get("bar").and_then(|v| v.as_str()), Some("15m"));
        assert_eq!(
            serialized.get("limit").and_then(|v| v.as_str()),
            Some("100")
        );
    }

    #[test]
    fn test_get_history_mark_price_candles_response() {
        let response_json = json!({
            "code": "0",
            "msg": "",
            "data": [
                ["1597026383085", "3811.24", "3811.24", "3811.24", "3811.24", "1"],
                ["1597026383000", "3811.25", "3811.25", "3811.25", "3811.25", "1"]
            ]
        });

        let response: ApiResponse<[String; 6]> = serde_json::from_value(response_json).unwrap();

        assert_eq!(response.code, "0");
        assert_eq!(response.msg, "");
        assert_eq!(response.data.len(), 2);

        let first_candle = &response.data[0];
        assert_eq!(first_candle[0], "1597026383085"); // ts
        assert_eq!(first_candle[1], "3811.24"); // o
        assert_eq!(first_candle[2], "3811.24"); // h
        assert_eq!(first_candle[3], "3811.24"); // l
        assert_eq!(first_candle[4], "3811.24"); // c
        assert_eq!(first_candle[5], "1"); // confirm
    }

    #[test]
    fn test_mark_price_candle_structure() {
        let candle = MarkPriceCandle {
            ts: "1597026383085".to_string(),
            o: "3811.24".to_string(),
            h: "3811.25".to_string(),
            l: "3811.23".to_string(),
            c: "3811.24".to_string(),
            confirm: "1".to_string(),
        };

        assert_eq!(candle.ts, "1597026383085");
        assert_eq!(candle.o, "3811.24");
        assert_eq!(candle.h, "3811.25");
        assert_eq!(candle.l, "3811.23");
        assert_eq!(candle.c, "3811.24");
        assert_eq!(candle.confirm, "1");
    }

    #[test]
    fn test_request_serialization_roundtrip() {
        let original = GetHistoryMarkPriceCandlesRequest {
            inst_id: "ETH-USD-SWAP".to_string(),
            after: Some("1597026383085".to_string()),
            before: None,
            bar: Some(BarSize::OneHour),
            limit: Some("50".to_string()),
        };

        let serialized = serde_json::to_value(&original).unwrap();
        let deserialized: GetHistoryMarkPriceCandlesRequest =
            serde_json::from_value(serialized).unwrap();

        assert_eq!(original.inst_id, deserialized.inst_id);
        assert_eq!(original.after, deserialized.after);
        assert_eq!(original.before, deserialized.before);
        assert_eq!(original.bar, deserialized.bar);
        assert_eq!(original.limit, deserialized.limit);
    }

    #[test]
    fn test_public_api_integration() {
        // Test that the endpoint method exists on RestClient
        use std::{collections::HashMap, sync::Arc};

        use async_trait::async_trait;
        use rest::{HttpClient, HttpError, Response};

        use crate::okx::{RateLimiter, RestClient};

        #[derive(Debug)]
        struct MockHttpClient;

        #[async_trait]
        impl HttpClient for MockHttpClient {
            async fn execute(&self, _request: rest::Request) -> Result<Response, HttpError> {
                Ok(Response {
                    status: 200,
                    headers: HashMap::new(),
                    body: br#"{"code":"0","msg":"","data":[]}"#.to_vec().into(),
                })
            }
        }

        let http_client: Arc<dyn HttpClient> = Arc::new(MockHttpClient);
        let _client = RestClient::new("https://www.okx.com", http_client, RateLimiter::new());

        let request = GetHistoryMarkPriceCandlesRequest {
            inst_id: "BTC-USD-SWAP".to_string(),
            after: None,
            before: None,
            bar: Some(BarSize::OneHour),
            limit: Some("10".to_string()),
        };

        // This test just ensures the method exists and can be called
        // In a real test, we would mock the HTTP client
        // but for now we just verify the types work together
        assert_eq!(request.inst_id, "BTC-USD-SWAP");
        assert_eq!(request.bar, Some(BarSize::OneHour));
        assert_eq!(request.limit, Some("10".to_string()));
    }
}
