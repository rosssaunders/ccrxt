use super::client::RestClient;
use crate::okx::{EndpointType, RestResult};
use serde::{Deserialize, Serialize};

/// Request parameters for getting index candlesticks history
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetHistoryIndexCandlesRequest {
    /// Index (e.g., "BTC-USD"). Required.
    #[serde(rename = "instId")]
    pub inst_id: String,

    /// Pagination of data to return records earlier than the requested ts. Optional.
    #[serde(rename = "after", skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,

    /// Pagination of data to return records newer than the requested ts. Optional.
    #[serde(rename = "before", skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,

    /// Bar size, the default is "1m". Optional.
    /// e.g. [1m/3m/5m/15m/30m/1H/2H/4H]
    /// Hong Kong time opening price k-line: [6H/12H/1D/1W/1M]
    /// UTC time opening price k-line: [6Hutc/12Hutc/1Dutc/1Wutc/1Mutc]
    #[serde(rename = "bar", skip_serializing_if = "Option::is_none")]
    pub bar: Option<String>,

    /// Number of results per request. The maximum is 100; The default is 100. Optional.
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

/// Individual candlestick data
/// Array format: [ts, o, h, l, c, confirm]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexCandle(pub Vec<String>);

impl IndexCandle {
    /// Get the opening time of the candlestick (Unix timestamp in milliseconds)
    pub fn timestamp(&self) -> Option<&String> {
        self.0.first()
    }

    /// Get the open price
    pub fn open(&self) -> Option<&String> {
        self.0.get(1)
    }

    /// Get the high price
    pub fn high(&self) -> Option<&String> {
        self.0.get(2)
    }

    /// Get the low price
    pub fn low(&self) -> Option<&String> {
        self.0.get(3)
    }

    /// Get the close price
    pub fn close(&self) -> Option<&String> {
        self.0.get(4)
    }

    /// Get the confirm status
    /// "0" represents that it is uncompleted, "1" represents that it is completed
    pub fn confirm(&self) -> Option<&String> {
        self.0.get(5)
    }
}

/// Response for getting index candlesticks history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetHistoryIndexCandlesResponse {
    /// Response code ("0" for success)
    pub code: String,
    /// Response message
    pub msg: String,
    /// Candlestick data
    pub data: Vec<IndexCandle>,
}

impl RestClient {
    /// Get index candlesticks history
    ///
    /// Retrieve the candlestick charts of the index from recent years.
    ///
    /// See: https://www.okx.com/docs-v5/en/#rest-api-market-data-get-index-candlesticks-history
    ///
    /// Rate limit: 10 requests per 2 seconds (IP-based)
    ///
    /// # Arguments
    /// * `request` - The index candlesticks history request parameters
    ///
    /// # Returns
    /// Response containing the index candlestick data
    pub async fn get_history_index_candles(
        &self,
        request: &GetHistoryIndexCandlesRequest,
    ) -> RestResult<GetHistoryIndexCandlesResponse> {
        self.send_request(
            "api/v5/market/history-index-candles",
            reqwest::Method::GET,
            Some(request),
            EndpointType::PublicMarketData,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_get_history_index_candles_request_structure() {
        let request = GetHistoryIndexCandlesRequest {
            inst_id: "BTC-USD".to_string(),
            after: Some("1597026383085".to_string()),
            before: None,
            bar: Some("1H".to_string()),
            limit: Some("50".to_string()),
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(
            serialized.get("instId").and_then(|v| v.as_str()),
            Some("BTC-USD")
        );
        assert_eq!(
            serialized.get("after").and_then(|v| v.as_str()),
            Some("1597026383085")
        );
        assert_eq!(serialized.get("before"), None);
        assert_eq!(
            serialized.get("bar").and_then(|v| v.as_str()),
            Some("1H")
        );
        assert_eq!(
            serialized.get("limit").and_then(|v| v.as_str()),
            Some("50")
        );
    }

    #[test]
    fn test_get_history_index_candles_minimal_request() {
        let request = GetHistoryIndexCandlesRequest {
            inst_id: "BTC-USD".to_string(),
            after: None,
            before: None,
            bar: None,
            limit: None,
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(
            serialized.get("instId").and_then(|v| v.as_str()),
            Some("BTC-USD")
        );
        assert_eq!(serialized.get("after"), None);
        assert_eq!(serialized.get("before"), None);
        assert_eq!(serialized.get("bar"), None);
        assert_eq!(serialized.get("limit"), None);
    }

    #[test]
    fn test_index_candle_accessors() {
        let candle_data = vec![
            "1597026383085".to_string(),
            "11811.7".to_string(),
            "11811.7".to_string(),
            "11811.7".to_string(),
            "11811.7".to_string(),
            "1".to_string(),
        ];
        let candle = IndexCandle(candle_data);

        assert_eq!(candle.timestamp(), Some(&"1597026383085".to_string()));
        assert_eq!(candle.open(), Some(&"11811.7".to_string()));
        assert_eq!(candle.high(), Some(&"11811.7".to_string()));
        assert_eq!(candle.low(), Some(&"11811.7".to_string()));
        assert_eq!(candle.close(), Some(&"11811.7".to_string()));
        assert_eq!(candle.confirm(), Some(&"1".to_string()));
    }

    #[test]
    fn test_get_history_index_candles_response_structure() {
        let response_json = json!({
            "code": "0",
            "msg": "",
            "data": [
                ["1597026383085", "11811.7", "11813.7", "11811.4", "11812.7", "1"],
                ["1597026323085", "11810.5", "11815.2", "11809.8", "11811.7", "1"]
            ]
        });

        let response: GetHistoryIndexCandlesResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.msg, "");
        assert_eq!(response.data.len(), 2);

        let first_candle = &response.data[0];
        assert_eq!(first_candle.timestamp(), Some(&"1597026383085".to_string()));
        assert_eq!(first_candle.open(), Some(&"11811.7".to_string()));
        assert_eq!(first_candle.high(), Some(&"11813.7".to_string()));
        assert_eq!(first_candle.low(), Some(&"11811.4".to_string()));
        assert_eq!(first_candle.close(), Some(&"11812.7".to_string()));
        assert_eq!(first_candle.confirm(), Some(&"1".to_string()));
    }

    #[test]
    fn test_request_serialization_roundtrip() {
        let original = GetHistoryIndexCandlesRequest {
            inst_id: "ETH-USD".to_string(),
            after: Some("1597026383085".to_string()),
            before: Some("1597026484085".to_string()),
            bar: Some("5m".to_string()),
            limit: Some("25".to_string()),
        };

        let serialized = serde_json::to_value(&original).unwrap();
        let deserialized: GetHistoryIndexCandlesRequest = serde_json::from_value(serialized).unwrap();

        assert_eq!(original.inst_id, deserialized.inst_id);
        assert_eq!(original.after, deserialized.after);
        assert_eq!(original.before, deserialized.before);
        assert_eq!(original.bar, deserialized.bar);
        assert_eq!(original.limit, deserialized.limit);
    }

    #[test]
    fn test_empty_candle_data() {
        let candle = IndexCandle(vec![]);
        assert_eq!(candle.timestamp(), None);
        assert_eq!(candle.open(), None);
        assert_eq!(candle.high(), None);
        assert_eq!(candle.low(), None);
        assert_eq!(candle.close(), None);
        assert_eq!(candle.confirm(), None);
    }

    #[test]
    fn test_partial_candle_data() {
        let candle = IndexCandle(vec![
            "1597026383085".to_string(),
            "11811.7".to_string(),
            "11813.7".to_string(),
        ]);
        assert_eq!(candle.timestamp(), Some(&"1597026383085".to_string()));
        assert_eq!(candle.open(), Some(&"11811.7".to_string()));
        assert_eq!(candle.high(), Some(&"11813.7".to_string()));
        assert_eq!(candle.low(), None);
        assert_eq!(candle.close(), None);
        assert_eq!(candle.confirm(), None);
    }
}