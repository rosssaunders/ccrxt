use serde::{Deserialize, Serialize};

use crate::okx::{EndpointType, RestResult, public_client::RestClient};

const GET_SPREAD_HISTORY_CANDLES_ENDPOINT: &str = "/api/v5/market/sprd-history-candles";

/// Request parameters for getting spread historical candlesticks
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSpreadHistoryCandlesRequest {
    /// Spread ID
    #[serde(rename = "sprdId")]
    pub sprd_id: String,

    /// Pagination of data to return records earlier than the requested ts
    #[serde(rename = "after", skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,

    /// Pagination of data to return records newer than the requested ts
    /// The latest data will be returned when using before individually
    #[serde(rename = "before", skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,

    /// Bar size, the default is 1m
    /// e.g. [1m/3m/5m/15m/30m/1H/2H/4H]
    /// UTC+8 opening price k-line:[6H/12H/1D/2D/3D/1W/1M/3M]
    /// UTC+0 opening price k-line:[6Hutc/12Hutc/1Dutc/2Dutc/3Dutc/1Wutc/1Mutc/3Mutc]
    #[serde(rename = "bar", skip_serializing_if = "Option::is_none")]
    pub bar: Option<String>,

    /// Number of results per request. The maximum is 100. The default is 100.
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

/// Response data for getting spread historical candlesticks
/// The data is returned as an array: [ts,o,h,l,c,vol,confirm]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpreadHistoryCandleData {
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
    /// Get spread historical candlesticks
    /// Retrieve history candlestick charts from recent years
    /// [docs](https://www.okx.com/docs-v5/en/#spread-trading-rest-api-get-candlesticks-history)
    pub async fn get_spread_history_candles(
        &self,
        request: GetSpreadHistoryCandlesRequest,
    ) -> RestResult<SpreadHistoryCandleData> {
        self.send_get_request(
            GET_SPREAD_HISTORY_CANDLES_ENDPOINT,
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
    fn test_get_spread_history_candles_request_full() {
        let request = GetSpreadHistoryCandlesRequest {
            sprd_id: "BTC-USDT_BTC-USDT-SWAP".to_string(),
            after: Some("1597026383085".to_string()),
            before: Some("1597112783085".to_string()),
            bar: Some("1D".to_string()),
            limit: Some("50".to_string()),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: GetSpreadHistoryCandlesRequest =
            serde_json::from_str(&serialized).unwrap();
        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_get_spread_history_candles_request_minimal() {
        let request = GetSpreadHistoryCandlesRequest {
            sprd_id: "BTC-USDT_BTC-USDT-SWAP".to_string(),
            after: None,
            before: None,
            bar: None,
            limit: None,
        };

        let serialized = serde_json::to_string(&request).unwrap();
        assert!(serialized.contains("sprdId"));
        assert!(!serialized.contains("after"));
        assert!(!serialized.contains("before"));
        assert!(!serialized.contains("bar"));
        assert!(!serialized.contains("limit"));
    }

    #[test]
    fn test_spread_history_candle_data_deserialization() {
        let json_response = r#"{
            "ts": "1597026383085",
            "o": "45000",
            "h": "46000",
            "l": "44500",
            "c": "45500",
            "vol": "2468.13",
            "confirm": "1"
        }"#;

        let candle: SpreadHistoryCandleData = serde_json::from_str(json_response).unwrap();
        assert_eq!(candle.ts, "1597026383085");
        assert_eq!(candle.o, "45000");
        assert_eq!(candle.h, "46000");
        assert_eq!(candle.l, "44500");
        assert_eq!(candle.c, "45500");
        assert_eq!(candle.vol, "2468.13");
        assert_eq!(candle.confirm, "1");
    }

    #[test]
    fn test_spread_history_candle_data_serialization() {
        let candle = SpreadHistoryCandleData {
            ts: "1597026383085".to_string(),
            o: "45000".to_string(),
            h: "46000".to_string(),
            l: "44500".to_string(),
            c: "45500".to_string(),
            vol: "2468.13".to_string(),
            confirm: "1".to_string(),
        };

        let serialized = serde_json::to_string(&candle).unwrap();
        let deserialized: SpreadHistoryCandleData = serde_json::from_str(&serialized).unwrap();
        assert_eq!(candle, deserialized);
    }

    #[test]
    fn test_history_bar_intervals() {
        let intervals = vec![
            "1m", "3m", "5m", "15m", "30m", "1H", "2H", "4H", "6H", "12H", "1D", "2D", "3D", "1W",
            "1M", "3M", "6Hutc", "12Hutc", "1Dutc", "2Dutc", "3Dutc", "1Wutc", "1Mutc", "3Mutc",
        ];

        for interval in intervals {
            let request = GetSpreadHistoryCandlesRequest {
                sprd_id: "BTC-USDT_BTC-USDT-SWAP".to_string(),
                after: None,
                before: None,
                bar: Some(interval.to_string()),
                limit: None,
            };

            let serialized = serde_json::to_string(&request).unwrap();
            assert!(serialized.contains(&format!("\"bar\":\"{}\"", interval)));
        }
    }

    #[test]
    fn test_history_candle_confirmation_states() {
        let states = vec!["0", "1"];

        for state in states {
            let candle = SpreadHistoryCandleData {
                ts: "1597026383085".to_string(),
                o: "45000".to_string(),
                h: "46000".to_string(),
                l: "44500".to_string(),
                c: "45500".to_string(),
                vol: "2468.13".to_string(),
                confirm: state.to_string(),
            };

            let serialized = serde_json::to_string(&candle).unwrap();
            assert!(serialized.contains(&format!("\"confirm\":\"{}\"", state)));
        }
    }

    #[test]
    fn test_history_candle_array_format_documentation() {
        // The API returns data as [ts,o,h,l,c,vol,confirm]
        // This test documents the expected order of fields
        let candle_array = vec![
            "1597026383085".to_string(), // ts
            "45000".to_string(),         // o (open)
            "46000".to_string(),         // h (high)
            "44500".to_string(),         // l (low)
            "45500".to_string(),         // c (close)
            "2468.13".to_string(),       // vol (volume)
            "1".to_string(),             // confirm
        ];

        assert_eq!(candle_array.len(), 7);
        assert_eq!(candle_array[0], "1597026383085"); // timestamp
        assert_eq!(candle_array[1], "45000"); // open
        assert_eq!(candle_array[2], "46000"); // high
        assert_eq!(candle_array[3], "44500"); // low
        assert_eq!(candle_array[4], "45500"); // close
        assert_eq!(candle_array[5], "2468.13"); // volume
        assert_eq!(candle_array[6], "1"); // confirm
    }

    #[test]
    fn test_history_limit_values() {
        let limits = vec!["1", "10", "25", "50", "100"];

        for limit in limits {
            let request = GetSpreadHistoryCandlesRequest {
                sprd_id: "BTC-USDT_BTC-USDT-SWAP".to_string(),
                after: None,
                before: None,
                bar: None,
                limit: Some(limit.to_string()),
            };

            let serialized = serde_json::to_string(&request).unwrap();
            assert!(serialized.contains(&format!("\"limit\":\"{}\"", limit)));
        }
    }

    #[test]
    fn test_history_pagination_parameters() {
        let request = GetSpreadHistoryCandlesRequest {
            sprd_id: "BTC-USDT_BTC-USDT-SWAP".to_string(),
            after: Some("1597026383085".to_string()),
            before: Some("1597112783085".to_string()),
            bar: Some("1D".to_string()),
            limit: Some("25".to_string()),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        assert!(serialized.contains("\"after\":\"1597026383085\""));
        assert!(serialized.contains("\"before\":\"1597112783085\""));
    }

    #[test]
    fn test_history_vs_regular_candles() {
        // History candles have the same structure as regular candles
        // but are retrieved from a different endpoint for historical data
        let history_candle = SpreadHistoryCandleData {
            ts: "1597026383085".to_string(),
            o: "45000".to_string(),
            h: "46000".to_string(),
            l: "44500".to_string(),
            c: "45500".to_string(),
            vol: "2468.13".to_string(),
            confirm: "1".to_string(),
        };

        // All fields should be present and properly typed as strings
        assert!(!history_candle.ts.is_empty());
        assert!(!history_candle.o.is_empty());
        assert!(!history_candle.h.is_empty());
        assert!(!history_candle.l.is_empty());
        assert!(!history_candle.c.is_empty());
        assert!(!history_candle.vol.is_empty());
        assert!(!history_candle.confirm.is_empty());
    }

    #[test]
    fn test_spread_id_formats_history() {
        let spread_ids = vec![
            "BTC-USDT_BTC-USDT-SWAP",
            "ETH-USD_ETH-USD-SWAP",
            "SOL-USDT_SOL-USDT-SWAP",
        ];

        for sprd_id in spread_ids {
            let request = GetSpreadHistoryCandlesRequest {
                sprd_id: sprd_id.to_string(),
                after: None,
                before: None,
                bar: None,
                limit: None,
            };

            let serialized = serde_json::to_string(&request).unwrap();
            assert!(serialized.contains(&format!("\"sprdId\":\"{}\"", sprd_id)));
        }
    }
}
