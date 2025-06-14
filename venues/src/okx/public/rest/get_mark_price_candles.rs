use super::client::RestClient;
use crate::okx::{EndpointType, RestResult};
use serde::{Deserialize, Serialize};

/// Request parameters for getting mark price candlesticks
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMarkPriceCandlesRequest {
    /// Instrument ID, e.g. "BTC-USD-SWAP"
    #[serde(rename = "instId")]
    pub inst_id: String,
    /// Pagination of data to return records earlier than the requested ts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Pagination of data to return records newer than the requested ts. 
    /// The latest data will be returned when using before individually
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Bar size, the default is "1m"
    /// e.g. [1m/3m/5m/15m/30m/1H/2H/4H]
    /// Hong Kong time opening price k-line: [6H/12H/1D/1W/1M/3M]
    /// UTC time opening price k-line: [6Hutc/12Hutc/1Dutc/1Wutc/1Mutc/3Mutc]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bar: Option<String>,
    /// Number of results per request. The maximum is "100"; The default is "100"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

/// Mark price candlestick data
/// The data returned will be arranged in an array like this: [ts,o,h,l,c,confirm]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkPriceCandle(
    /// Opening time of the candlestick, Unix timestamp format in milliseconds, e.g. "1597026383085"
    pub String,
    /// Open price
    pub String,
    /// Highest price
    pub String,
    /// Lowest price
    pub String,
    /// Close price
    pub String,
    /// The state of candlesticks. "0" represents that it is uncompleted, "1" represents that it is completed
    pub String,
);

/// Response for getting mark price candlesticks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMarkPriceCandlesResponse {
    /// Response code ("0" for success)
    pub code: String,
    /// Response message
    pub msg: String,
    /// Mark price candlestick data. Each entry is [ts,o,h,l,c,confirm]
    pub data: Vec<MarkPriceCandle>,
}

impl RestClient {
    /// Get mark price candlesticks
    ///
    /// Retrieve the candlestick charts of mark price. This endpoint can retrieve the
    /// latest 1,440 data entries. Charts are returned in groups based on the requested bar.
    ///
    /// See: https://www.okx.com/docs-v5/en/#rest-api-market-data-get-mark-price-candlesticks
    ///
    /// Rate limit: 20 requests per 2 seconds
    ///
    /// # Arguments
    /// * `request` - The mark price candlesticks request parameters
    ///
    /// # Returns
    /// Response containing the list of mark price candlesticks
    pub async fn get_mark_price_candles(
        &self,
        request: &GetMarkPriceCandlesRequest,
    ) -> RestResult<GetMarkPriceCandlesResponse> {
        self.send_request(
            "api/v5/market/mark-price-candles",
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
    fn test_get_mark_price_candles_request_structure() {
        let request = GetMarkPriceCandlesRequest {
            inst_id: "BTC-USD-SWAP".to_string(),
            after: None,
            before: None,
            bar: Some("1H".to_string()),
            limit: Some("50".to_string()),
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(
            serialized.get("instId").and_then(|v| v.as_str()),
            Some("BTC-USD-SWAP")
        );
        assert_eq!(
            serialized.get("bar").and_then(|v| v.as_str()),
            Some("1H")
        );
        assert_eq!(
            serialized.get("limit").and_then(|v| v.as_str()),
            Some("50")
        );
        assert!(serialized.get("after").is_none());
        assert!(serialized.get("before").is_none());
    }

    #[test]
    fn test_get_mark_price_candles_request_with_pagination() {
        let request = GetMarkPriceCandlesRequest {
            inst_id: "ETH-USD-SWAP".to_string(),
            after: Some("1597026383085".to_string()),
            before: Some("1597026483085".to_string()),
            bar: Some("15m".to_string()),
            limit: Some("100".to_string()),
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(
            serialized.get("instId").and_then(|v| v.as_str()),
            Some("ETH-USD-SWAP")
        );
        assert_eq!(
            serialized.get("after").and_then(|v| v.as_str()),
            Some("1597026383085")
        );
        assert_eq!(
            serialized.get("before").and_then(|v| v.as_str()),
            Some("1597026483085")
        );
        assert_eq!(
            serialized.get("bar").and_then(|v| v.as_str()),
            Some("15m")
        );
        assert_eq!(
            serialized.get("limit").and_then(|v| v.as_str()),
            Some("100")
        );
    }

    #[test]
    fn test_mark_price_candle_structure() {
        let candle_json = json!([
            "1597026383085",
            "50000.0",
            "50100.5",
            "49900.0",
            "50050.25",
            "1"
        ]);

        let candle: MarkPriceCandle = serde_json::from_value(candle_json).unwrap();
        assert_eq!(candle.0, "1597026383085"); // ts
        assert_eq!(candle.1, "50000.0");       // o
        assert_eq!(candle.2, "50100.5");       // h
        assert_eq!(candle.3, "49900.0");       // l
        assert_eq!(candle.4, "50050.25");      // c
        assert_eq!(candle.5, "1");             // confirm
    }

    #[test]
    fn test_get_mark_price_candles_response_structure() {
        let response_json = json!({
            "code": "0",
            "msg": "",
            "data": [
                [
                    "1597026383085",
                    "50000.0",
                    "50100.5",
                    "49900.0",
                    "50050.25",
                    "1"
                ],
                [
                    "1597026383086",
                    "50050.25",
                    "50200.0",
                    "50000.0",
                    "50150.0",
                    "0"
                ]
            ]
        });

        let response: GetMarkPriceCandlesResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.msg, "");
        assert_eq!(response.data.len(), 2);
        
        let first_candle = &response.data[0];
        assert_eq!(first_candle.0, "1597026383085");
        assert_eq!(first_candle.1, "50000.0");
        assert_eq!(first_candle.2, "50100.5");
        assert_eq!(first_candle.3, "49900.0");
        assert_eq!(first_candle.4, "50050.25");
        assert_eq!(first_candle.5, "1");

        let second_candle = &response.data[1];
        assert_eq!(second_candle.0, "1597026383086");
        assert_eq!(second_candle.5, "0"); // uncompleted
    }

    #[test]
    fn test_mark_price_candles_serialization_roundtrip() {
        let original = GetMarkPriceCandlesRequest {
            inst_id: "SOL-USD-SWAP".to_string(),
            after: Some("1597026383085".to_string()),
            before: None,
            bar: Some("4H".to_string()),
            limit: Some("25".to_string()),
        };

        let serialized = serde_json::to_value(&original).unwrap();
        let deserialized: GetMarkPriceCandlesRequest = serde_json::from_value(serialized).unwrap();

        assert_eq!(original.inst_id, deserialized.inst_id);
        assert_eq!(original.after, deserialized.after);
        assert_eq!(original.before, deserialized.before);
        assert_eq!(original.bar, deserialized.bar);
        assert_eq!(original.limit, deserialized.limit);
    }

    #[test]
    fn test_mark_price_candles_minimal_request() {
        let request = GetMarkPriceCandlesRequest {
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
        // Optional fields should not be present when None
        assert!(serialized.get("after").is_none());
        assert!(serialized.get("before").is_none());
        assert!(serialized.get("bar").is_none());
        assert!(serialized.get("limit").is_none());
    }

    #[test]
    fn test_mark_price_candle_serialization_roundtrip() {
        let original = MarkPriceCandle(
            "1597026383085".to_string(),
            "50000.0".to_string(),
            "50100.5".to_string(),
            "49900.0".to_string(),
            "50050.25".to_string(),
            "1".to_string(),
        );

        let serialized = serde_json::to_value(&original).unwrap();
        let deserialized: MarkPriceCandle = serde_json::from_value(serialized).unwrap();

        assert_eq!(original.0, deserialized.0);
        assert_eq!(original.1, deserialized.1);
        assert_eq!(original.2, deserialized.2);
        assert_eq!(original.3, deserialized.3);
        assert_eq!(original.4, deserialized.4);
        assert_eq!(original.5, deserialized.5);
    }
}