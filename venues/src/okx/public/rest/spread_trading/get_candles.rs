use serde::{Deserialize, Serialize};

use crate::okx::{EndpointType, RestResult, public_client::RestClient};

const GET_SPREAD_CANDLES_ENDPOINT: &str = "/api/v5/market/sprd-candles";

/// Request parameters for getting spread candlesticks
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSpreadCandlesRequest {
    /// Spread ID
    #[serde(rename = "sprdId")]
    pub sprd_id: String,

    /// Bar size, the default is 1m
    /// e.g. [1m/3m/5m/15m/30m/1H/2H/4H]
    /// UTC+8 opening price k-line:[6H/12H/1D/2D/3D/1W/1M/3M]
    /// UTC+0 opening price k-line:[6Hutc/12Hutc/1Dutc/2Dutc/3Dutc/1Wutc/1Mutc/3Mutc]
    #[serde(rename = "bar", skip_serializing_if = "Option::is_none")]
    pub bar: Option<String>,

    /// Pagination of data to return records earlier than the requested ts
    #[serde(rename = "after", skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,

    /// Pagination of data to return records newer than the requested ts
    /// The latest data will be returned when using before individually
    #[serde(rename = "before", skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,

    /// Number of results per request. The maximum is 300. The default is 100.
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

/// Response data for getting spread candlesticks
/// The data is returned as an array: [ts,o,h,l,c,vol,confirm]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpreadCandleData {
    /// Opening time of the candlestick, Unix timestamp format in milliseconds
    #[serde(rename = "ts")]
    pub ts: String,

    /// Open price
    #[serde(rename = "o")]
    pub o: String,

    /// Highest price
    #[serde(rename = "h")]
    pub h: String,

    /// Lowest price
    #[serde(rename = "l")]
    pub l: String,

    /// Close price
    #[serde(rename = "c")]
    pub c: String,

    /// Trading volume
    #[serde(rename = "vol")]
    pub vol: String,

    /// The state of candlesticks
    /// 0 represents that it is uncompleted
    /// 1 represents that it is completed
    #[serde(rename = "confirm")]
    pub confirm: String,
}

impl RestClient {
    /// Get spread candlesticks
    /// Retrieve the candlestick charts. This endpoint can retrieve the latest 1,440 data entries.
    /// [docs](https://www.okx.com/docs-v5/en/#spread-trading-rest-api-get-candlesticks)
    pub async fn get_spread_candles(
        &self,
        request: GetSpreadCandlesRequest,
    ) -> RestResult<SpreadCandleData> {
        self.send_get_request(
            GET_SPREAD_CANDLES_ENDPOINT,
            Some(&request),
            EndpointType::PublicMarketData,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use serde_json;

    use super::*;

    #[test]
    fn test_get_spread_candles_request_full() {
        let request = GetSpreadCandlesRequest {
            sprd_id: "BTC-USDT_BTC-USDT-SWAP".to_string(),
            bar: Some("1H".to_string()),
            after: Some("1597026383085".to_string()),
            before: Some("1597112783085".to_string()),
            limit: Some("100".to_string()),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: GetSpreadCandlesRequest = serde_json::from_str(&serialized).unwrap();
        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_get_spread_candles_request_minimal() {
        let request = GetSpreadCandlesRequest {
            sprd_id: "BTC-USDT_BTC-USDT-SWAP".to_string(),
            bar: None,
            after: None,
            before: None,
            limit: None,
        };

        let serialized = serde_json::to_string(&request).unwrap();
        assert!(serialized.contains("sprdId"));
        assert!(!serialized.contains("bar"));
        assert!(!serialized.contains("after"));
        assert!(!serialized.contains("before"));
        assert!(!serialized.contains("limit"));
    }

    #[test]
    fn test_spread_candle_data_deserialization() {
        let json_response = r#"{
            "ts": "1597026383085",
            "o": "50000",
            "h": "51000",
            "l": "49500",
            "c": "50500",
            "vol": "1234.56",
            "confirm": "1"
        }"#;

        let candle: SpreadCandleData = serde_json::from_str(json_response).unwrap();
        assert_eq!(candle.ts, "1597026383085");
        assert_eq!(candle.o, "50000");
        assert_eq!(candle.h, "51000");
        assert_eq!(candle.l, "49500");
        assert_eq!(candle.c, "50500");
        assert_eq!(candle.vol, "1234.56");
        assert_eq!(candle.confirm, "1");
    }

    #[test]
    fn test_spread_candle_data_serialization() {
        let candle = SpreadCandleData {
            ts: "1597026383085".to_string(),
            o: "50000".to_string(),
            h: "51000".to_string(),
            l: "49500".to_string(),
            c: "50500".to_string(),
            vol: "1234.56".to_string(),
            confirm: "1".to_string(),
        };

        let serialized = serde_json::to_string(&candle).unwrap();
        let deserialized: SpreadCandleData = serde_json::from_str(&serialized).unwrap();
        assert_eq!(candle, deserialized);
    }

    #[test]
    fn test_bar_intervals() {
        let intervals = vec![
            "1m", "3m", "5m", "15m", "30m", "1H", "2H", "4H", "6H", "12H", "1D", "2D", "3D", "1W",
            "1M", "3M", "6Hutc", "12Hutc", "1Dutc", "2Dutc", "3Dutc", "1Wutc", "1Mutc", "3Mutc",
        ];

        for interval in intervals {
            let request = GetSpreadCandlesRequest {
                sprd_id: "BTC-USDT_BTC-USDT-SWAP".to_string(),
                bar: Some(interval.to_string()),
                after: None,
                before: None,
                limit: None,
            };

            let serialized = serde_json::to_string(&request).unwrap();
            assert!(serialized.contains(&format!("\"bar\":\"{}\"", interval)));
        }
    }

    #[test]
    fn test_candle_confirmation_states() {
        let states = vec!["0", "1"];

        for state in states {
            let candle = SpreadCandleData {
                ts: "1597026383085".to_string(),
                o: "50000".to_string(),
                h: "51000".to_string(),
                l: "49500".to_string(),
                c: "50500".to_string(),
                vol: "1234.56".to_string(),
                confirm: state.to_string(),
            };

            let serialized = serde_json::to_string(&candle).unwrap();
            assert!(serialized.contains(&format!("\"confirm\":\"{}\"", state)));
        }
    }

    #[test]
    fn test_candle_array_format_documentation() {
        // The API returns data as [ts,o,h,l,c,vol,confirm]
        // This test documents the expected order of fields
        let candle_array = [
            "1597026383085".to_string(), // ts
            "50000".to_string(),         // o (open)
            "51000".to_string(),         // h (high)
            "49500".to_string(),         // l (low)
            "50500".to_string(),         // c (close)
            "1234.56".to_string(),       // vol (volume)
            "1".to_string(),             // confirm
        ];

        assert_eq!(candle_array.len(), 7);
        assert_eq!(candle_array[0], "1597026383085"); // timestamp
        assert_eq!(candle_array[1], "50000"); // open
        assert_eq!(candle_array[2], "51000"); // high
        assert_eq!(candle_array[3], "49500"); // low
        assert_eq!(candle_array[4], "50500"); // close
        assert_eq!(candle_array[5], "1234.56"); // volume
        assert_eq!(candle_array[6], "1"); // confirm
    }

    #[test]
    fn test_limit_values() {
        let limits = ["1", "10", "50", "100", "200", "300"];

        for limit in limits {
            let request = GetSpreadCandlesRequest {
                sprd_id: "BTC-USDT_BTC-USDT-SWAP".to_string(),
                bar: None,
                after: None,
                before: None,
                limit: Some(limit.to_string()),
            };

            let serialized = serde_json::to_string(&request).unwrap();
            assert!(serialized.contains(&format!("\"limit\":\"{}\"", limit)));
        }
    }

    #[test]
    fn test_pagination_parameters() {
        let request = GetSpreadCandlesRequest {
            sprd_id: "BTC-USDT_BTC-USDT-SWAP".to_string(),
            bar: Some("1H".to_string()),
            after: Some("1597026383085".to_string()),
            before: Some("1597112783085".to_string()),
            limit: Some("50".to_string()),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        assert!(serialized.contains("\"after\":\"1597026383085\""));
        assert!(serialized.contains("\"before\":\"1597112783085\""));
    }
}
