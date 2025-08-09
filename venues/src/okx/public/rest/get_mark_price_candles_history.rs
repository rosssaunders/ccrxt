use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::okx::{EndpointType, RestResult};

const MARKET_HISTORY_MARK_PRICE_CANDLES_ENDPOINT: &str = "api/v5/market/history-mark-price-candles";

/// Request parameters for getting mark price candlesticks history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMarkPriceCandlesHistoryRequest {
    /// Instrument ID (e.g., "BTC-USD-SWAP"). Required.
    #[serde(rename = "instId")]
    pub inst_id: String,

    /// Pagination of data to return records earlier than the requested ts. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,

    /// Pagination of data to return records newer than the requested ts. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,

    /// Bar size, default is "1m". Optional.
    /// e.g. [1m/3m/5m/15m/30m/1H/2H/4H]
    /// Hong Kong time opening price k-line: [6H/12H/1D/1W/1M]
    /// UTC time opening price k-line: [6Hutc/12Hutc/1Dutc/1Wutc/1Mutc]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bar: Option<String>,

    /// Number of results per request. Maximum is 100; default is 100. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

impl RestClient {
    /// Get mark price candlesticks history
    ///
    /// Retrieve the candlestick charts of mark price from recent years.
    ///
    /// [docs]: https://www.okx.com/docs-v5/en/#rest-api-public-rest-api-get-mark-price-candlesticks-history
    ///
    /// Rate limit: 20 requests per 2 seconds
    ///
    /// # Arguments
    /// * `request` - The mark price candles history request parameters
    ///
    /// # Returns
    /// Response containing historical mark price candlestick data - arrays of [ts,o,h,l,c,confirm]
    pub async fn get_mark_price_candles_history(
        &self,
        request: GetMarkPriceCandlesHistoryRequest,
    ) -> RestResult<Vec<[String; 6]>> {
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
    use crate::okx::response::OkxApiResponse;

    #[test]
    fn test_get_mark_price_candles_history_request_structure() {
        let request = GetMarkPriceCandlesHistoryRequest {
            inst_id: "BTC-USD-SWAP".to_string(),
            after: None,
            before: None,
            bar: Some("1D".to_string()),
            limit: Some("50".to_string()),
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(
            serialized.get("instId").and_then(|v| v.as_str()),
            Some("BTC-USD-SWAP")
        );
        assert_eq!(serialized.get("bar").and_then(|v| v.as_str()), Some("1D"));
        assert_eq!(serialized.get("limit").and_then(|v| v.as_str()), Some("50"));
        // Optional fields should not be present when None
        assert!(serialized.get("after").is_none());
        assert!(serialized.get("before").is_none());
    }

    #[test]
    fn test_get_mark_price_candles_history_response_structure() {
        let response_json = json!({
            "code": "0",
            "msg": "",
            "data": [
                ["1597026383085", "3811.2", "3811.2", "3811.2", "3811.2", "1"],
                ["1597026420000", "3811.2", "3813.5", "3811.2", "3813.5", "1"]
            ]
        });

        let response: OkxApiResponse<[String; 6]> = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.msg, "");
        assert_eq!(response.data.len(), 2);
        assert_eq!(response.data[0][0], "1597026383085"); // timestamp
        assert_eq!(response.data[0][1], "3811.2"); // open
        assert_eq!(response.data[0][2], "3811.2"); // high
        assert_eq!(response.data[0][3], "3811.2"); // low
        assert_eq!(response.data[0][4], "3811.2"); // close
        assert_eq!(response.data[0][5], "1"); // confirm
    }

    #[test]
    fn test_mark_price_candles_history_serialization_roundtrip() {
        let original = GetMarkPriceCandlesHistoryRequest {
            inst_id: "ETH-USD-SWAP".to_string(),
            after: Some("1597026383085".to_string()),
            before: Some("1597026420000".to_string()),
            bar: Some("1W".to_string()),
            limit: Some("25".to_string()),
        };

        let serialized = serde_json::to_value(&original).unwrap();
        let deserialized: GetMarkPriceCandlesHistoryRequest =
            serde_json::from_value(serialized).unwrap();

        assert_eq!(original.inst_id, deserialized.inst_id);
        assert_eq!(original.after, deserialized.after);
        assert_eq!(original.before, deserialized.before);
        assert_eq!(original.bar, deserialized.bar);
        assert_eq!(original.limit, deserialized.limit);
    }

    #[test]
    fn test_minimal_history_request() {
        let request = GetMarkPriceCandlesHistoryRequest {
            inst_id: "BTC-USD-SWAP".to_string(),
            after: None,
            before: None,
            bar: None,
            limit: None,
        };

        let serialized = serde_json::to_value(&request).unwrap();
        // Only instId should be present
        assert_eq!(serialized.as_object().unwrap().len(), 1);
        assert_eq!(
            serialized.get("instId").and_then(|v| v.as_str()),
            Some("BTC-USD-SWAP")
        );
    }

    #[test]
    fn test_different_bar_sizes() {
        let bar_sizes = vec![
            "1m", "3m", "5m", "15m", "30m", "1H", "2H", "4H", "6H", "12H", "1D", "1W", "1M",
        ];

        for bar_size in bar_sizes {
            let request = GetMarkPriceCandlesHistoryRequest {
                inst_id: "BTC-USD-SWAP".to_string(),
                after: None,
                before: None,
                bar: Some(bar_size.to_string()),
                limit: None,
            };

            let serialized = serde_json::to_value(&request).unwrap();
            assert_eq!(
                serialized.get("bar").and_then(|v| v.as_str()),
                Some(bar_size)
            );
        }
    }
}
