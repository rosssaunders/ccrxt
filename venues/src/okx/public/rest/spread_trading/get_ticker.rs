use serde::{Deserialize, Serialize};

use crate::okx::{EndpointType, RestResult, public_client::RestClient};

const GET_SPREAD_TICKER_ENDPOINT: &str = "/api/v5/market/sprd-ticker";

/// Request parameters for getting spread ticker
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSpreadTickerRequest {
    /// Spread ID, e.g. BTC-USDT_BTC-USDT-SWAP
    #[serde(rename = "sprdId")]
    pub sprd_id: String,
}

/// Response data for getting spread ticker
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpreadTickerData {
    /// Spread ID
    #[serde(rename = "sprdId")]
    pub sprd_id: String,

    /// Last traded price
    #[serde(rename = "last")]
    pub last: String,

    /// Last traded size
    #[serde(rename = "lastSz")]
    pub last_sz: String,

    /// Best ask price
    #[serde(rename = "askPx")]
    pub ask_px: String,

    /// Best ask size
    #[serde(rename = "askSz")]
    pub ask_sz: String,

    /// Best bid price
    #[serde(rename = "bidPx")]
    pub bid_px: String,

    /// Best bid size
    #[serde(rename = "bidSz")]
    pub bid_sz: String,

    /// Open price in the past 24 hours
    #[serde(rename = "open24h")]
    pub open24h: String,

    /// Highest price in the past 24 hours
    #[serde(rename = "high24h")]
    pub high24h: String,

    /// Lowest price in the past 24 hours
    #[serde(rename = "low24h")]
    pub low24h: String,

    /// 24h trading volume
    /// The unit is USD for inverse spreads, and the corresponding baseCcy for linear and hybrid spreads
    #[serde(rename = "vol24h")]
    pub vol24h: String,

    /// Ticker data generation time, Unix timestamp format in milliseconds
    #[serde(rename = "ts")]
    pub ts: String,
}

impl RestClient {
    /// Get spread ticker
    /// Retrieve the latest price snapshot, best bid/ask price and quantity
    /// [docs](https://www.okx.com/docs-v5/en/#spread-trading-rest-api-get-ticker-public)
    pub async fn get_spread_ticker(
        &self,
        request: GetSpreadTickerRequest,
    ) -> RestResult<SpreadTickerData> {
        self.send_get_request(
            GET_SPREAD_TICKER_ENDPOINT,
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
    fn test_get_spread_ticker_request_serialization() {
        let request = GetSpreadTickerRequest {
            sprd_id: "BTC-USDT_BTC-USDT-SWAP".to_string(),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: GetSpreadTickerRequest = serde_json::from_str(&serialized).unwrap();
        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_spread_ticker_data_deserialization() {
        let json_response = r#"{
            "sprdId": "BTC-USDT_BTC-USDT-SWAP",
            "last": "50.5",
            "lastSz": "0.8",
            "askPx": "51.0",
            "askSz": "2.5",
            "bidPx": "50.0",
            "bidSz": "1.8",
            "open24h": "49.5",
            "high24h": "52.0",
            "low24h": "48.0",
            "vol24h": "1234.56",
            "ts": "1597026383085"
        }"#;

        let ticker: SpreadTickerData = serde_json::from_str(json_response).unwrap();
        assert_eq!(ticker.sprd_id, "BTC-USDT_BTC-USDT-SWAP");
        assert_eq!(ticker.last, "50.5");
        assert_eq!(ticker.last_sz, "0.8");
        assert_eq!(ticker.ask_px, "51.0");
        assert_eq!(ticker.ask_sz, "2.5");
        assert_eq!(ticker.bid_px, "50.0");
        assert_eq!(ticker.bid_sz, "1.8");
        assert_eq!(ticker.open24h, "49.5");
        assert_eq!(ticker.high24h, "52.0");
        assert_eq!(ticker.low24h, "48.0");
        assert_eq!(ticker.vol24h, "1234.56");
        assert_eq!(ticker.ts, "1597026383085");
    }

    #[test]
    fn test_spread_ticker_data_serialization() {
        let ticker = SpreadTickerData {
            sprd_id: "BTC-USDT_BTC-USDT-SWAP".to_string(),
            last: "50.5".to_string(),
            last_sz: "0.8".to_string(),
            ask_px: "51.0".to_string(),
            ask_sz: "2.5".to_string(),
            bid_px: "50.0".to_string(),
            bid_sz: "1.8".to_string(),
            open24h: "49.5".to_string(),
            high24h: "52.0".to_string(),
            low24h: "48.0".to_string(),
            vol24h: "1234.56".to_string(),
            ts: "1597026383085".to_string(),
        };

        let serialized = serde_json::to_string(&ticker).unwrap();
        let deserialized: SpreadTickerData = serde_json::from_str(&serialized).unwrap();
        assert_eq!(ticker, deserialized);
    }

    #[test]
    fn test_spread_ticker_with_zero_values() {
        let json_response = r#"{
            "sprdId": "BTC-USDT_BTC-USDT-SWAP",
            "last": "0",
            "lastSz": "0",
            "askPx": "0",
            "askSz": "0",
            "bidPx": "0",
            "bidSz": "0",
            "open24h": "0",
            "high24h": "0",
            "low24h": "0",
            "vol24h": "0",
            "ts": "1597026383085"
        }"#;

        let ticker: SpreadTickerData = serde_json::from_str(json_response).unwrap();
        assert_eq!(ticker.last, "0");
        assert_eq!(ticker.ask_px, "0");
        assert_eq!(ticker.bid_px, "0");
        assert_eq!(ticker.vol24h, "0");
    }

    #[test]
    fn test_spread_ticker_price_relationships() {
        // In a normal market, bid should be less than ask
        let json_response = r#"{
            "sprdId": "BTC-USDT_BTC-USDT-SWAP",
            "last": "50.5",
            "lastSz": "0.8",
            "askPx": "51.0",
            "askSz": "2.5",
            "bidPx": "50.0",
            "bidSz": "1.8",
            "open24h": "49.5",
            "high24h": "52.0",
            "low24h": "48.0",
            "vol24h": "1234.56",
            "ts": "1597026383085"
        }"#;

        let ticker: SpreadTickerData = serde_json::from_str(json_response).unwrap();

        // These are string comparisons, but the test documents the expected data format
        assert_eq!(ticker.bid_px, "50.0");
        assert_eq!(ticker.ask_px, "51.0");
        assert_eq!(ticker.low24h, "48.0");
        assert_eq!(ticker.high24h, "52.0");
    }

    #[test]
    fn test_spread_id_formats() {
        let spread_ids = vec![
            "BTC-USDT_BTC-USDT-SWAP",
            "ETH-USD_ETH-USD-SWAP",
            "SOL-USDT_SOL-USDT-SWAP",
        ];

        for sprd_id in spread_ids {
            let request = GetSpreadTickerRequest {
                sprd_id: sprd_id.to_string(),
            };

            let serialized = serde_json::to_string(&request).unwrap();
            assert!(serialized.contains(&format!("\"sprdId\":\"{}\"", sprd_id)));
        }
    }

    #[test]
    fn test_timestamp_format() {
        let ticker = SpreadTickerData {
            sprd_id: "BTC-USDT_BTC-USDT-SWAP".to_string(),
            last: "50.5".to_string(),
            last_sz: "0.8".to_string(),
            ask_px: "51.0".to_string(),
            ask_sz: "2.5".to_string(),
            bid_px: "50.0".to_string(),
            bid_sz: "1.8".to_string(),
            open24h: "49.5".to_string(),
            high24h: "52.0".to_string(),
            low24h: "48.0".to_string(),
            vol24h: "1234.56".to_string(),
            ts: "1597026383085".to_string(),
        };

        // The timestamp should be a Unix timestamp in milliseconds
        assert_eq!(ticker.ts.len(), 13); // Typical length of millisecond timestamp
        assert!(ticker.ts.chars().all(|c| c.is_ascii_digit()));
    }
}
