use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::okx::{Bar, EndpointType, RestResult};

const MARKET_INDEX_CANDLES_ENDPOINT: &str = "api/v5/market/index-candles";
/// Request parameters for getting index candlesticks
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetIndexCandlesRequest {
    /// Index instrument ID (e.g., "BTC-USD")
    #[serde(rename = "instId")]
    pub inst_id: String,

    /// Pagination of data to return records earlier than the requested ts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,

    /// Pagination of data to return records newer than the requested ts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,

    /// Bar size, default is 1m
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bar: Option<Bar>,

    /// Number of results per request. Maximum is 100, default is 100
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

/// Individual candlestick data point
/// Data is returned as array: [ts, o, h, l, c, confirm]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexCandle {
    /// Opening time of the candlestick, Unix timestamp format in milliseconds
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

/// Response for getting index candlesticks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetIndexCandlesResponse {
    /// Response code ("0" for success)
    pub code: String,
    /// Response message
    pub msg: String,
    /// Candlestick data - array format [ts,o,h,l,c,confirm]
    pub data: Vec<Vec<String>>,
}

impl RestClient {
    /// Get index candlesticks
    ///
    /// Retrieve the candlestick charts of the index. This endpoint can retrieve the
    /// latest 1,440 data entries. Charts are returned in groups based on the requested bar.
    ///
    /// [docs]: https://www.okx.com/docs-v5/en/#rest-api-public-rest-api-get-index-candlesticks
    ///
    /// Rate limit: 20 requests per 2 seconds
    ///
    /// # Arguments
    /// * `request` - The index candlesticks request parameters
    ///
    /// # Returns
    /// Response containing the candlestick data
    pub async fn get_index_candles(
        &self,
        request: GetIndexCandlesRequest,
    ) -> RestResult<GetIndexCandlesResponse> {
        self.send_request(
            MARKET_INDEX_CANDLES_ENDPOINT,
            reqwest::Method::GET,
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

    #[test]
    fn test_get_index_candles_request_structure() {
        let request = GetIndexCandlesRequest {
            inst_id: "BTC-USD".to_string(),
            after: None,
            before: None,
            bar: Some(Bar::M1),
            limit: Some("100".to_string()),
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(
            serialized.get("instId").and_then(|v| v.as_str()),
            Some("BTC-USD")
        );
        assert_eq!(serialized.get("bar").and_then(|v| v.as_str()), Some("1m"));
        assert_eq!(
            serialized.get("limit").and_then(|v| v.as_str()),
            Some("100")
        );
    }

    #[test]
    fn test_get_index_candles_request_minimal() {
        let request = GetIndexCandlesRequest {
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
        // Optional fields should not be present
        assert!(!serialized.as_object().unwrap().contains_key("after"));
        assert!(!serialized.as_object().unwrap().contains_key("before"));
        assert!(!serialized.as_object().unwrap().contains_key("bar"));
        assert!(!serialized.as_object().unwrap().contains_key("limit"));
    }

    #[test]
    fn test_get_index_candles_response_structure() {
        let response_json = json!({
            "code": "0",
            "msg": "",
            "data": [
                ["1597026383085", "3811.31", "3811.31", "3811.31", "3811.31", "1"],
                ["1597026440000", "3811.31", "3813.04", "3811.31", "3813.04", "1"]
            ]
        });

        let response: GetIndexCandlesResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 2);
        assert_eq!(response.data[0].len(), 6); // [ts,o,h,l,c,confirm]
        assert_eq!(response.data[0][0], "1597026383085"); // timestamp
        assert_eq!(response.data[0][1], "3811.31"); // open
        assert_eq!(response.data[0][5], "1"); // confirm
    }

    #[test]
    fn test_bar_enum_serialization() {
        assert_eq!(serde_json::to_value(Bar::M1).unwrap(), "1m");
        assert_eq!(serde_json::to_value(Bar::H1).unwrap(), "1H");
        assert_eq!(serde_json::to_value(Bar::D1).unwrap(), "1D");
        assert_eq!(serde_json::to_value(Bar::Month1Utc).unwrap(), "1Mutc");
    }

    #[test]
    fn test_get_index_candles_with_pagination() {
        let request = GetIndexCandlesRequest {
            inst_id: "BTC-USD".to_string(),
            after: Some("1597026383085".to_string()),
            before: Some("1597026440000".to_string()),
            bar: Some(Bar::H1),
            limit: Some("50".to_string()),
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(
            serialized.get("after").and_then(|v| v.as_str()),
            Some("1597026383085")
        );
        assert_eq!(
            serialized.get("before").and_then(|v| v.as_str()),
            Some("1597026440000")
        );
        assert_eq!(serialized.get("bar").and_then(|v| v.as_str()), Some("1H"));
        assert_eq!(serialized.get("limit").and_then(|v| v.as_str()), Some("50"));
    }
}
