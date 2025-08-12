use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::binance::usdm::RestResult;

const TICKER_24HR_ENDPOINT: &str = "/fapi/v1/ticker/24hr";

/// Request parameters for 24hr ticker price change statistics.
#[derive(Debug, Clone, Serialize, Default)]
pub struct Ticker24hrRequest {
    /// Trading pair symbol (e.g., "BTCUSDT"). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Cow<'static, str>>,
}

/// Represents a 24hr ticker price change statistics response.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ticker24hr {
    /// Trading pair symbol (e.g., "BTCUSDT").
    pub symbol: Cow<'static, str>,

    /// Price change in the last 24 hours.
    pub price_change: String,

    /// Price change percent in the last 24 hours.
    pub price_change_percent: String,

    /// Weighted average price in the last 24 hours.
    pub weighted_avg_price: String,

    /// Last price.
    pub last_price: String,

    /// Last quantity.
    pub last_qty: String,

    /// Open price 24 hours ago.
    pub open_price: String,

    /// Highest price in the last 24 hours.
    pub high_price: String,

    /// Lowest price in the last 24 hours.
    pub low_price: String,

    /// Total traded base asset volume in the last 24 hours.
    pub volume: String,

    /// Total traded quote asset volume in the last 24 hours.
    pub quote_volume: String,

    /// Open time in milliseconds since epoch.
    pub open_time: u64,

    /// Close time in milliseconds since epoch.
    pub close_time: u64,

    /// First trade ID.
    pub first_id: u64,

    /// Last trade ID.
    pub last_id: u64,

    /// Total number of trades.
    pub count: u64,
}

/// Response wrapper for 24hr ticker statistics that can return either a single ticker or multiple tickers.
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum Ticker24hrResult {
    /// Multiple ticker statistics when no symbol is specified.
    Multiple(Vec<Ticker24hr>),

    /// Single ticker statistics when a specific symbol is requested.
    Single(Ticker24hr),
}

impl RestClient {
    /// 24hr Ticker Price Change Statistics
    ///
    /// 24 hour rolling window price change statistics. Careful when accessing this with no symbol.
    ///
    /// [docs](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/24hr-Ticker-Price-Change-Statistics)
    ///
    /// Rate limit: 1 for a single symbol; 40 when the symbol parameter is omitted
    ///
    /// # Arguments
    /// * `params` - The ticker 24hr request parameters
    ///
    /// # Returns
    /// Single ticker statistics or multiple ticker statistics depending on whether a symbol is specified
    pub async fn get_ticker_24hr(&self, params: Ticker24hrRequest) -> RestResult<Ticker24hrResult> {
        self.send_get_request(TICKER_24HR_ENDPOINT, Some(params), 1)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ticker_24hr_request_no_symbol() {
        let request = Ticker24hrRequest { symbol: None };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "");
    }

    #[test]
    fn test_ticker_24hr_single_response_deserialization() {
        let json = r#"{
            "symbol": "BTCUSDT",
            "priceChange": "2068.30000000",
            "priceChangePercent": "4.776",
            "weightedAvgPrice": "44267.45906168",
            "lastPrice": "45384.10000000",
            "lastQty": "0.003",
            "openPrice": "43315.80000000",
            "highPrice": "45710.20000000",
            "lowPrice": "42876.80000000",
            "volume": "507341.249",
            "quoteVolume": "22461274298.75970900",
            "openTime": 1625097600000,
            "closeTime": 1625184000000,
            "firstId": 194080557,
            "lastId": 194584694,
            "count": 504138
        }"#;

        let ticker: Ticker24hr = serde_json::from_str(json).unwrap();
        assert_eq!(ticker.symbol, "BTCUSDT");
        assert_eq!(ticker.price_change, "2068.30000000");
        assert_eq!(ticker.price_change_percent, "4.776");
        assert_eq!(ticker.weighted_avg_price, "44267.45906168");
        assert_eq!(ticker.last_price, "45384.10000000");
        assert_eq!(ticker.last_qty, "0.003");
        assert_eq!(ticker.open_price, "43315.80000000");
        assert_eq!(ticker.high_price, "45710.20000000");
        assert_eq!(ticker.low_price, "42876.80000000");
        assert_eq!(ticker.volume, "507341.249");
        assert_eq!(ticker.quote_volume, "22461274298.75970900");
        assert_eq!(ticker.open_time, 1625097600000);
        assert_eq!(ticker.close_time, 1625184000000);
        assert_eq!(ticker.first_id, 194080557);
        assert_eq!(ticker.last_id, 194584694);
        assert_eq!(ticker.count, 504138);
    }

    #[test]
    fn test_ticker_24hr_result_single_deserialization() {
        let json = r#"{
            "symbol": "BTCUSDT",
            "priceChange": "2068.30000000",
            "priceChangePercent": "4.776",
            "weightedAvgPrice": "44267.45906168",
            "lastPrice": "45384.10000000",
            "lastQty": "0.003",
            "openPrice": "43315.80000000",
            "highPrice": "45710.20000000",
            "lowPrice": "42876.80000000",
            "volume": "507341.249",
            "quoteVolume": "22461274298.75970900",
            "openTime": 1625097600000,
            "closeTime": 1625184000000,
            "firstId": 194080557,
            "lastId": 194584694,
            "count": 504138
        }"#;

        let result: Ticker24hrResult = serde_json::from_str(json).unwrap();
        match result {
            Ticker24hrResult::Single(ticker) => {
                assert_eq!(ticker.symbol, "BTCUSDT");
                assert_eq!(ticker.last_price, "45384.10000000");
            }
            Ticker24hrResult::Multiple(_) => {
                assert!(false, "Expected Single variant");
            }
        }
    }

    #[test]
    fn test_ticker_24hr_result_multiple_deserialization() {
        let json = r#"[
            {
                "symbol": "BTCUSDT",
                "priceChange": "2068.30000000",
                "priceChangePercent": "4.776",
                "weightedAvgPrice": "44267.45906168",
                "lastPrice": "45384.10000000",
                "lastQty": "0.003",
                "openPrice": "43315.80000000",
                "highPrice": "45710.20000000",
                "lowPrice": "42876.80000000",
                "volume": "507341.249",
                "quoteVolume": "22461274298.75970900",
                "openTime": 1625097600000,
                "closeTime": 1625184000000,
                "firstId": 194080557,
                "lastId": 194584694,
                "count": 504138
            },
            {
                "symbol": "ETHUSDT",
                "priceChange": "-123.40000000",
                "priceChangePercent": "-3.862",
                "weightedAvgPrice": "3076.89532901",
                "lastPrice": "3072.84000000",
                "lastQty": "0.08",
                "openPrice": "3196.24000000",
                "highPrice": "3246.30000000",
                "lowPrice": "3010.50000000",
                "volume": "3423156.19",
                "quoteVolume": "10528688494.87179060",
                "openTime": 1625097600000,
                "closeTime": 1625184000000,
                "firstId": 671093594,
                "lastId": 672368363,
                "count": 1274770
            }
        ]"#;

        let result: Ticker24hrResult = serde_json::from_str(json).unwrap();
        match result {
            Ticker24hrResult::Multiple(tickers) => {
                assert_eq!(tickers.len(), 2);
                assert_eq!(tickers[0].symbol, "BTCUSDT");
                assert_eq!(tickers[0].price_change_percent, "4.776");
                assert_eq!(tickers[1].symbol, "ETHUSDT");
                assert_eq!(tickers[1].price_change_percent, "-3.862");
            }
            Ticker24hrResult::Single(_) => {
                assert!(false, "Expected Multiple variant");
            }
        }
    }

    #[test]
    fn test_ticker_24hr_negative_values() {
        let json = r#"{
            "symbol": "DOGEUSD",
            "priceChange": "-0.00123400",
            "priceChangePercent": "-10.567",
            "weightedAvgPrice": "0.01100000",
            "lastPrice": "0.01044000",
            "lastQty": "1000",
            "openPrice": "0.01167400",
            "highPrice": "0.01200000",
            "lowPrice": "0.01000000",
            "volume": "123456789.00",
            "quoteVolume": "1358024.67900000",
            "openTime": 1625097600000,
            "closeTime": 1625184000000,
            "firstId": 1000000,
            "lastId": 2000000,
            "count": 1000000
        }"#;

        let ticker: Ticker24hr = serde_json::from_str(json).unwrap();
        assert_eq!(ticker.price_change, "-0.00123400");
        assert_eq!(ticker.price_change_percent, "-10.567");
    }

    #[test]
    fn test_ticker_24hr_request_with_symbol() {
        let request = Ticker24hrRequest {
            symbol: Some("BTCUSDT".into()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "symbol=BTCUSDT");
    }

    #[test]
    fn test_ticker_24hr_request_default() {
        let request = Ticker24hrRequest::default();
        assert!(request.symbol.is_none());
    }

    #[test]
    fn test_ticker_24hr_result_enum_size() {
        // Ensure the enum variants have expected structure
        let single_json = r#"{"symbol":"TEST","priceChange":"0","priceChangePercent":"0","weightedAvgPrice":"0","lastPrice":"0","lastQty":"0","openPrice":"0","highPrice":"0","lowPrice":"0","volume":"0","quoteVolume":"0","openTime":0,"closeTime":0,"firstId":0,"lastId":0,"count":0}"#;
        let multiple_json = r#"[{"symbol":"TEST","priceChange":"0","priceChangePercent":"0","weightedAvgPrice":"0","lastPrice":"0","lastQty":"0","openPrice":"0","highPrice":"0","lowPrice":"0","volume":"0","quoteVolume":"0","openTime":0,"closeTime":0,"firstId":0,"lastId":0,"count":0}]"#;

        let single_result: Ticker24hrResult = serde_json::from_str(single_json).unwrap();
        let multiple_result: Ticker24hrResult = serde_json::from_str(multiple_json).unwrap();

        assert!(matches!(single_result, Ticker24hrResult::Single(_)));
        assert!(matches!(multiple_result, Ticker24hrResult::Multiple(_)));
    }
}
