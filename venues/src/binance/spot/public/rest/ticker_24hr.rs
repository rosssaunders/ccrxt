use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::binance::spot::RestResult;

const TICKER_24HR_ENDPOINT: &str = "/api/v3/ticker/24hr";

/// Request parameters for 24hr ticker statistics
#[derive(Debug, Clone, Serialize, Default)]
pub struct Ticker24hrRequest {
    /// Single symbol
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,

    /// Multiple symbols (JSON array format)
    #[serde(rename = "symbols", skip_serializing_if = "Option::is_none")]
    pub symbols: Option<String>,

    /// Type of ticker response (FULL or MINI)
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub ticker_type: Option<String>,
}

/// 24hr ticker statistics (FULL type)
#[derive(Debug, Clone, Deserialize)]
pub struct Ticker24hr {
    /// Symbol
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Price change
    #[serde(rename = "priceChange")]
    pub price_change: Decimal,

    /// Price change percent
    #[serde(rename = "priceChangePercent")]
    pub price_change_percent: Decimal,

    /// Weighted average price
    #[serde(rename = "weightedAvgPrice")]
    pub weighted_avg_price: Decimal,

    /// Previous close price
    #[serde(rename = "prevClosePrice")]
    pub prev_close_price: Decimal,

    /// Last price
    #[serde(rename = "lastPrice")]
    pub last_price: Decimal,

    /// Last quantity
    #[serde(rename = "lastQty")]
    pub last_qty: Decimal,

    /// Best bid price
    #[serde(rename = "bidPrice")]
    pub bid_price: Decimal,

    /// Best bid quantity
    #[serde(rename = "bidQty")]
    pub bid_qty: Decimal,

    /// Best ask price
    #[serde(rename = "askPrice")]
    pub ask_price: Decimal,

    /// Best ask quantity
    #[serde(rename = "askQty")]
    pub ask_qty: Decimal,

    /// Open price
    #[serde(rename = "openPrice")]
    pub open_price: Decimal,

    /// High price
    #[serde(rename = "highPrice")]
    pub high_price: Decimal,

    /// Low price
    #[serde(rename = "lowPrice")]
    pub low_price: Decimal,

    /// Total traded base asset volume
    #[serde(rename = "volume")]
    pub volume: Decimal,

    /// Total traded quote asset volume
    #[serde(rename = "quoteVolume")]
    pub quote_volume: Decimal,

    /// Statistics open time
    #[serde(rename = "openTime")]
    pub open_time: u64,

    /// Statistics close time
    #[serde(rename = "closeTime")]
    pub close_time: u64,

    /// First trade ID
    #[serde(rename = "firstId")]
    pub first_id: i64,

    /// Last trade ID
    #[serde(rename = "lastId")]
    pub last_id: i64,

    /// Total number of trades
    #[serde(rename = "count")]
    pub count: u64,
}

/// 24hr ticker statistics (MINI type)
#[derive(Debug, Clone, Deserialize)]
pub struct Ticker24hrMini {
    /// Symbol
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Close price
    #[serde(rename = "closePrice")]
    pub close_price: Decimal,

    /// Open price
    #[serde(rename = "openPrice")]
    pub open_price: Decimal,

    /// High price
    #[serde(rename = "highPrice")]
    pub high_price: Decimal,

    /// Low price
    #[serde(rename = "lowPrice")]
    pub low_price: Decimal,

    /// Total traded base asset volume
    #[serde(rename = "volume")]
    pub volume: Decimal,

    /// Total traded quote asset volume
    #[serde(rename = "quoteVolume")]
    pub quote_volume: Decimal,

    /// Statistics open time
    #[serde(rename = "openTime")]
    pub open_time: u64,

    /// Statistics close time
    #[serde(rename = "closeTime")]
    pub close_time: u64,

    /// First trade ID
    #[serde(rename = "firstId")]
    pub first_id: i64,

    /// Last trade ID
    #[serde(rename = "lastId")]
    pub last_id: i64,

    /// Total number of trades
    #[serde(rename = "count")]
    pub count: u64,
}

impl RestClient {
    /// Get 24hr ticker price change statistics
    ///
    /// Returns 24 hour rolling window price change statistics.
    /// Careful when accessing this with no symbol.
    ///
    /// [docs](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/public-api-endpoints#24hr-ticker-price-change-statistics)
    ///
    /// Method: GET /api/v3/ticker/24hr
    /// Weight: Variable (2-80 based on symbols count)
    /// Security: None
    pub async fn get_24hr_ticker(
        &self,
        params: Option<Ticker24hrRequest>,
    ) -> RestResult<serde_json::Value> {
        let weight = if let Some(ref p) = params {
            if p.symbol.is_some() {
                2 // Single symbol
            } else if p.symbols.is_some() {
                40 // Multiple symbols
            } else {
                80 // All symbols
            }
        } else {
            80 // All symbols
        };

        self.send_get_request(TICKER_24HR_ENDPOINT, params, weight)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ticker_24hr_request_serialization_empty() {
        let request = Ticker24hrRequest::default();
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "");
    }

    #[test]
    fn test_ticker_24hr_request_serialization_single_symbol() {
        let request = Ticker24hrRequest {
            symbol: Some("BTCUSDT".to_string()),
            symbols: None,
            ticker_type: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "symbol=BTCUSDT");
    }

    #[test]
    fn test_ticker_24hr_request_serialization_multiple_symbols() {
        let request = Ticker24hrRequest {
            symbol: None,
            symbols: Some("[\"BTCUSDT\",\"ETHUSDT\"]".to_string()),
            ticker_type: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbols=%5B%22BTCUSDT%22%2C%22ETHUSDT%22%5D"));
    }

    #[test]
    fn test_ticker_24hr_request_serialization_with_type() {
        let request = Ticker24hrRequest {
            symbol: Some("BTCUSDT".to_string()),
            symbols: None,
            ticker_type: Some("MINI".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTCUSDT"));
        assert!(serialized.contains("type=MINI"));
    }

    #[test]
    fn test_ticker_24hr_full_deserialization() {
        let json = r#"{
            "symbol": "BTCUSDT",
            "priceChange": "-94.99999800",
            "priceChangePercent": "-95.960",
            "weightedAvgPrice": "0.29628482",
            "prevClosePrice": "0.10002000",
            "lastPrice": "4.00000200",
            "lastQty": "200.00000000",
            "bidPrice": "4.00000000",
            "bidQty": "100.00000000",
            "askPrice": "4.00000200",
            "askQty": "100.00000000",
            "openPrice": "99.00000000",
            "highPrice": "100.00000000",
            "lowPrice": "0.10000000",
            "volume": "8913.30000000",
            "quoteVolume": "15.30000000",
            "openTime": 1499783499040,
            "closeTime": 1499869899040,
            "firstId": 28385,
            "lastId": 28460,
            "count": 76
        }"#;

        let ticker: Ticker24hr = serde_json::from_str(json).unwrap();
        assert_eq!(ticker.symbol, "BTCUSDT");
        assert_eq!(ticker.price_change.to_string(), "-94.99999800");
        assert_eq!(ticker.price_change_percent.to_string(), "-95.960");
        assert_eq!(ticker.weighted_avg_price.to_string(), "0.29628482");
        assert_eq!(ticker.prev_close_price.to_string(), "0.10002000");
        assert_eq!(ticker.last_price.to_string(), "4.00000200");
        assert_eq!(ticker.last_qty.to_string(), "200.00000000");
        assert_eq!(ticker.bid_price.to_string(), "4.00000000");
        assert_eq!(ticker.bid_qty.to_string(), "100.00000000");
        assert_eq!(ticker.ask_price.to_string(), "4.00000200");
        assert_eq!(ticker.ask_qty.to_string(), "100.00000000");
        assert_eq!(ticker.open_price.to_string(), "99.00000000");
        assert_eq!(ticker.high_price.to_string(), "100.00000000");
        assert_eq!(ticker.low_price.to_string(), "0.10000000");
        assert_eq!(ticker.volume.to_string(), "8913.30000000");
        assert_eq!(ticker.quote_volume.to_string(), "15.30000000");
        assert_eq!(ticker.open_time, 1499783499040);
        assert_eq!(ticker.close_time, 1499869899040);
        assert_eq!(ticker.first_id, 28385);
        assert_eq!(ticker.last_id, 28460);
        assert_eq!(ticker.count, 76);
    }

    #[test]
    fn test_ticker_24hr_mini_deserialization() {
        let json = r#"{
            "symbol": "ETHUSDT",
            "closePrice": "3070.50",
            "openPrice": "3100.00",
            "highPrice": "3150.00",
            "lowPrice": "3050.00",
            "volume": "1234.56789",
            "quoteVolume": "3800000.00",
            "openTime": 1625184000000,
            "closeTime": 1625270400000,
            "firstId": 50000,
            "lastId": 55000,
            "count": 5001
        }"#;

        let ticker: Ticker24hrMini = serde_json::from_str(json).unwrap();
        assert_eq!(ticker.symbol, "ETHUSDT");
        assert_eq!(ticker.close_price.to_string(), "3070.50");
        assert_eq!(ticker.open_price.to_string(), "3100.00");
        assert_eq!(ticker.high_price.to_string(), "3150.00");
        assert_eq!(ticker.low_price.to_string(), "3050.00");
        assert_eq!(ticker.volume.to_string(), "1234.56789");
        assert_eq!(ticker.quote_volume.to_string(), "3800000.00");
        assert_eq!(ticker.open_time, 1625184000000);
        assert_eq!(ticker.close_time, 1625270400000);
        assert_eq!(ticker.first_id, 50000);
        assert_eq!(ticker.last_id, 55000);
        assert_eq!(ticker.count, 5001);
    }

    #[test]
    fn test_ticker_24hr_array_deserialization() {
        let json = r#"[
            {
                "symbol": "BTCUSDT",
                "priceChange": "100.00000000",
                "priceChangePercent": "2.222",
                "weightedAvgPrice": "4500.50",
                "prevClosePrice": "4500.00",
                "lastPrice": "4600.00",
                "lastQty": "1.00000000",
                "bidPrice": "4599.00",
                "bidQty": "2.50000000",
                "askPrice": "4601.00",
                "askQty": "1.80000000",
                "openPrice": "4500.00",
                "highPrice": "4650.00",
                "lowPrice": "4450.00",
                "volume": "100.50000000",
                "quoteVolume": "452250.00000000",
                "openTime": 1625184000000,
                "closeTime": 1625270400000,
                "firstId": 100000,
                "lastId": 101000,
                "count": 1001
            },
            {
                "symbol": "ETHUSDT",
                "priceChange": "-50.00000000",
                "priceChangePercent": "-1.600",
                "weightedAvgPrice": "3075.25",
                "prevClosePrice": "3125.00",
                "lastPrice": "3075.00",
                "lastQty": "5.00000000",
                "bidPrice": "3074.00",
                "bidQty": "10.00000000",
                "askPrice": "3076.00",
                "askQty": "8.50000000",
                "openPrice": "3125.00",
                "highPrice": "3140.00",
                "lowPrice": "3050.00",
                "volume": "500.00000000",
                "quoteVolume": "1537625.00000000",
                "openTime": 1625184000000,
                "closeTime": 1625270400000,
                "firstId": 200000,
                "lastId": 205000,
                "count": 5001
            }
        ]"#;

        let tickers: Vec<Ticker24hr> = serde_json::from_str(json).unwrap();
        assert_eq!(tickers.len(), 2);

        assert_eq!(tickers[0].symbol, "BTCUSDT");
        assert_eq!(tickers[0].price_change.to_string(), "100.00000000");
        assert_eq!(tickers[0].price_change_percent.to_string(), "2.222");

        assert_eq!(tickers[1].symbol, "ETHUSDT");
        assert_eq!(tickers[1].price_change.to_string(), "-50.00000000");
        assert_eq!(tickers[1].price_change_percent.to_string(), "-1.600");
    }

    #[test]
    fn test_ticker_24hr_zero_volume() {
        let json = r#"{
            "symbol": "RAREUSDT",
            "priceChange": "0.00000000",
            "priceChangePercent": "0.000",
            "weightedAvgPrice": "0.00000000",
            "prevClosePrice": "1.23456789",
            "lastPrice": "1.23456789",
            "lastQty": "0.00000000",
            "bidPrice": "1.23456700",
            "bidQty": "50.00000000",
            "askPrice": "1.23456800",
            "askQty": "30.00000000",
            "openPrice": "1.23456789",
            "highPrice": "1.23456789",
            "lowPrice": "1.23456789",
            "volume": "0.00000000",
            "quoteVolume": "0.00000000",
            "openTime": 1625184000000,
            "closeTime": 1625270400000,
            "firstId": -1,
            "lastId": -1,
            "count": 0
        }"#;

        let ticker: Ticker24hr = serde_json::from_str(json).unwrap();
        assert_eq!(ticker.symbol, "RAREUSDT");
        assert_eq!(ticker.price_change.to_string(), "0.00000000");
        assert_eq!(ticker.price_change_percent.to_string(), "0.000");
        assert_eq!(ticker.weighted_avg_price.to_string(), "0.00000000");
        assert_eq!(ticker.volume.to_string(), "0.00000000");
        assert_eq!(ticker.quote_volume.to_string(), "0.00000000");
        assert_eq!(ticker.count, 0);
        // Note: Binance uses -1 for first_id/last_id when there are no trades
    }

    #[test]
    fn test_ticker_24hr_negative_price_change() {
        let json = r#"{
            "symbol": "BTCUSDT",
            "priceChange": "-1500.50000000",
            "priceChangePercent": "-3.456",
            "weightedAvgPrice": "42500.75",
            "prevClosePrice": "45000.00",
            "lastPrice": "43499.50",
            "lastQty": "0.50000000",
            "bidPrice": "43499.00",
            "bidQty": "1.25000000",
            "askPrice": "43500.00",
            "askQty": "2.75000000",
            "openPrice": "45000.00",
            "highPrice": "45100.00",
            "lowPrice": "43400.00",
            "volume": "1000.00000000",
            "quoteVolume": "43500000.00",
            "openTime": 1625184000000,
            "closeTime": 1625270400000,
            "firstId": 123456,
            "lastId": 789012,
            "count": 665557
        }"#;

        let ticker: Ticker24hr = serde_json::from_str(json).unwrap();
        assert_eq!(ticker.price_change.to_string(), "-1500.50000000");
        assert_eq!(ticker.price_change_percent.to_string(), "-3.456");
        assert!(ticker.price_change < Decimal::ZERO);
        assert!(ticker.price_change_percent < Decimal::ZERO);
    }

    #[test]
    fn test_ticker_24hr_high_precision_values() {
        let json = r#"{
            "symbol": "DOGEUSDT",
            "priceChange": "0.00012345",
            "priceChangePercent": "1.234567",
            "weightedAvgPrice": "0.12345678",
            "prevClosePrice": "0.12333333",
            "lastPrice": "0.12345678",
            "lastQty": "10000.12345678",
            "bidPrice": "0.12345670",
            "bidQty": "50000.00000000",
            "askPrice": "0.12345680",
            "askQty": "25000.00000000",
            "openPrice": "0.12333333",
            "highPrice": "0.12400000",
            "lowPrice": "0.12300000",
            "volume": "1000000.00000000",
            "quoteVolume": "123456.78900000",
            "openTime": 1625184000000,
            "closeTime": 1625270400000,
            "firstId": 1000000,
            "lastId": 2000000,
            "count": 1000001
        }"#;

        let ticker: Ticker24hr = serde_json::from_str(json).unwrap();
        assert_eq!(ticker.symbol, "DOGEUSDT");
        assert_eq!(ticker.price_change.to_string(), "0.00012345");
        assert_eq!(ticker.price_change_percent.to_string(), "1.234567");
        assert_eq!(ticker.last_qty.to_string(), "10000.12345678");
    }

    #[test]
    fn test_ticker_24hr_request_full_type() {
        let request = Ticker24hrRequest {
            symbol: Some("BTCUSDT".to_string()),
            symbols: None,
            ticker_type: Some("FULL".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTCUSDT"));
        assert!(serialized.contains("type=FULL"));
    }

    #[test]
    fn test_ticker_24hr_request_multiple_symbols_with_type() {
        let request = Ticker24hrRequest {
            symbol: None,
            symbols: Some("[\"BTCUSDT\",\"ETHUSDT\",\"ADAUSDT\"]".to_string()),
            ticker_type: Some("MINI".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbols="));
        assert!(serialized.contains("type=MINI"));
        assert!(!serialized.contains("symbol="));
    }
}
