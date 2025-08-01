use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::binance::spot::RestResult;

const TICKER_ENDPOINT: &str = "/api/v3/ticker";

/// Request parameters for rolling window ticker statistics
#[derive(Debug, Clone, Serialize)]
pub struct TickerRequest {
    /// Single symbol
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,

    /// Multiple symbols (JSON array format)
    #[serde(rename = "symbols", skip_serializing_if = "Option::is_none")]
    pub symbols: Option<String>,

    /// Window size for statistics (default: 1d)
    #[serde(rename = "windowSize", skip_serializing_if = "Option::is_none")]
    pub window_size: Option<String>,

    /// Type of ticker response (FULL or MINI)
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub ticker_type: Option<String>,
}

/// Rolling window ticker statistics
#[derive(Debug, Clone, Deserialize)]
pub struct Ticker {
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

    /// Open price
    #[serde(rename = "openPrice")]
    pub open_price: Decimal,

    /// High price
    #[serde(rename = "highPrice")]
    pub high_price: Decimal,

    /// Low price
    #[serde(rename = "lowPrice")]
    pub low_price: Decimal,

    /// Last price
    #[serde(rename = "lastPrice")]
    pub last_price: Decimal,

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
    /// Get rolling window price change statistics
    ///
    /// Returns rolling window price change statistics.
    /// The window used to compute statistics is typically slightly wider than requested windowSize.
    ///
    /// [docs]: (https://developers.binance.com/docs/binance-spot-api-docs/rest-api/public-api-endpoints#rolling-window-price-change-statistics)
    /// Method: GET /api/v3/ticker
    /// Weight: 4 per symbol (max 200)
    /// Security: None
    pub async fn get_ticker(&self, params: TickerRequest) -> RestResult<serde_json::Value> {
        // Weight is 4 per symbol, max 200 for multiple symbols
        let weight = if params.symbols.is_some() { 200 } else { 4 };

        self.send_public_request(TICKER_ENDPOINT, reqwest::Method::GET, Some(params), weight)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ticker_request_serialization_empty() {
        let request = TickerRequest {
            symbol: None,
            symbols: None,
            window_size: None,
            ticker_type: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "");
    }

    #[test]
    fn test_ticker_request_serialization_single_symbol() {
        let request = TickerRequest {
            symbol: Some("BTCUSDT".to_string()),
            symbols: None,
            window_size: None,
            ticker_type: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "symbol=BTCUSDT");
    }

    #[test]
    fn test_ticker_request_serialization_multiple_symbols() {
        let request = TickerRequest {
            symbol: None,
            symbols: Some("[\"BTCUSDT\",\"ETHUSDT\"]".to_string()),
            window_size: None,
            ticker_type: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbols=%5B%22BTCUSDT%22%2C%22ETHUSDT%22%5D"));
    }

    #[test]
    fn test_ticker_request_serialization_with_window_size() {
        let request = TickerRequest {
            symbol: Some("BTCUSDT".to_string()),
            symbols: None,
            window_size: Some("1h".to_string()),
            ticker_type: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTCUSDT"));
        assert!(serialized.contains("windowSize=1h"));
    }

    #[test]
    fn test_ticker_request_serialization_full_params() {
        let request = TickerRequest {
            symbol: Some("ETHUSDT".to_string()),
            symbols: None,
            window_size: Some("4h".to_string()),
            ticker_type: Some("MINI".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=ETHUSDT"));
        assert!(serialized.contains("windowSize=4h"));
        assert!(serialized.contains("type=MINI"));
    }

    #[test]
    fn test_ticker_deserialization() {
        let json = r#"{
            "symbol": "BTCUSDT",
            "priceChange": "1500.25000000",
            "priceChangePercent": "3.456",
            "weightedAvgPrice": "44500.75",
            "openPrice": "43000.00",
            "highPrice": "45100.00",
            "lowPrice": "42800.00",
            "lastPrice": "44500.25",
            "volume": "1234.56789012",
            "quoteVolume": "54950000.00",
            "openTime": 1625184000000,
            "closeTime": 1625270400000,
            "firstId": 123456,
            "lastId": 789012,
            "count": 665557
        }"#;

        let ticker: Ticker = serde_json::from_str(json).unwrap();
        assert_eq!(ticker.symbol, "BTCUSDT");
        assert_eq!(ticker.price_change.to_string(), "1500.25000000");
        assert_eq!(ticker.price_change_percent.to_string(), "3.456");
        assert_eq!(ticker.weighted_avg_price.to_string(), "44500.75");
        assert_eq!(ticker.open_price.to_string(), "43000.00");
        assert_eq!(ticker.high_price.to_string(), "45100.00");
        assert_eq!(ticker.low_price.to_string(), "42800.00");
        assert_eq!(ticker.last_price.to_string(), "44500.25");
        assert_eq!(ticker.volume.to_string(), "1234.56789012");
        assert_eq!(ticker.quote_volume.to_string(), "54950000.00");
        assert_eq!(ticker.open_time, 1625184000000);
        assert_eq!(ticker.close_time, 1625270400000);
        assert_eq!(ticker.first_id, 123456);
        assert_eq!(ticker.last_id, 789012);
        assert_eq!(ticker.count, 665557);
    }

    #[test]
    fn test_ticker_array_deserialization() {
        let json = r#"[
            {
                "symbol": "BTCUSDT",
                "priceChange": "100.00000000",
                "priceChangePercent": "2.222",
                "weightedAvgPrice": "4500.50",
                "openPrice": "4500.00",
                "highPrice": "4650.00",
                "lowPrice": "4450.00",
                "lastPrice": "4600.00",
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
                "openPrice": "3125.00",
                "highPrice": "3140.00",
                "lowPrice": "3050.00",
                "lastPrice": "3075.00",
                "volume": "500.00000000",
                "quoteVolume": "1537625.00000000",
                "openTime": 1625184000000,
                "closeTime": 1625270400000,
                "firstId": 200000,
                "lastId": 205000,
                "count": 5001
            }
        ]"#;

        let tickers: Vec<Ticker> = serde_json::from_str(json).unwrap();
        assert_eq!(tickers.len(), 2);

        assert_eq!(tickers[0].symbol, "BTCUSDT");
        assert_eq!(tickers[0].price_change.to_string(), "100.00000000");
        assert_eq!(tickers[0].price_change_percent.to_string(), "2.222");

        assert_eq!(tickers[1].symbol, "ETHUSDT");
        assert_eq!(tickers[1].price_change.to_string(), "-50.00000000");
        assert_eq!(tickers[1].price_change_percent.to_string(), "-1.600");
    }

    #[test]
    fn test_ticker_negative_price_change() {
        let json = r#"{
            "symbol": "BTCUSDT",
            "priceChange": "-2500.50000000",
            "priceChangePercent": "-5.678",
            "weightedAvgPrice": "42000.25",
            "openPrice": "46000.00",
            "highPrice": "46200.00",
            "lowPrice": "41800.00",
            "lastPrice": "43499.50",
            "volume": "2000.00000000",
            "quoteVolume": "84000500.00",
            "openTime": 1625184000000,
            "closeTime": 1625270400000,
            "firstId": 500000,
            "lastId": 1000000,
            "count": 500001
        }"#;

        let ticker: Ticker = serde_json::from_str(json).unwrap();
        assert_eq!(ticker.price_change.to_string(), "-2500.50000000");
        assert_eq!(ticker.price_change_percent.to_string(), "-5.678");
        assert!(ticker.price_change < Decimal::ZERO);
        assert!(ticker.price_change_percent < Decimal::ZERO);
    }

    #[test]
    fn test_ticker_zero_volume() {
        let json = r#"{
            "symbol": "RAREUSDT",
            "priceChange": "0.00000000",
            "priceChangePercent": "0.000",
            "weightedAvgPrice": "0.00000000",
            "openPrice": "1.23456789",
            "highPrice": "1.23456789",
            "lowPrice": "1.23456789",
            "lastPrice": "1.23456789",
            "volume": "0.00000000",
            "quoteVolume": "0.00000000",
            "openTime": 1625184000000,
            "closeTime": 1625270400000,
            "firstId": -1,
            "lastId": -1,
            "count": 0
        }"#;

        let ticker: Ticker = serde_json::from_str(json).unwrap();
        assert_eq!(ticker.symbol, "RAREUSDT");
        assert_eq!(ticker.price_change.to_string(), "0.00000000");
        assert_eq!(ticker.price_change_percent.to_string(), "0.000");
        assert_eq!(ticker.weighted_avg_price.to_string(), "0.00000000");
        assert_eq!(ticker.volume.to_string(), "0.00000000");
        assert_eq!(ticker.quote_volume.to_string(), "0.00000000");
        assert_eq!(ticker.count, 0);
    }

    #[test]
    fn test_ticker_high_precision_values() {
        let json = r#"{
            "symbol": "DOGEUSDT",
            "priceChange": "0.00012345",
            "priceChangePercent": "1.234567",
            "weightedAvgPrice": "0.12345678",
            "openPrice": "0.12333333",
            "highPrice": "0.12400000",
            "lowPrice": "0.12300000",
            "lastPrice": "0.12345678",
            "volume": "1000000.00000000",
            "quoteVolume": "123456.78900000",
            "openTime": 1625184000000,
            "closeTime": 1625270400000,
            "firstId": 1000000,
            "lastId": 2000000,
            "count": 1000001
        }"#;

        let ticker: Ticker = serde_json::from_str(json).unwrap();
        assert_eq!(ticker.symbol, "DOGEUSDT");
        assert_eq!(ticker.price_change.to_string(), "0.00012345");
        assert_eq!(ticker.price_change_percent.to_string(), "1.234567");
        assert_eq!(ticker.weighted_avg_price.to_string(), "0.12345678");
        assert_eq!(ticker.volume.to_string(), "1000000.00000000");
    }

    #[test]
    fn test_ticker_request_window_sizes() {
        let window_sizes = vec![
            "1m", "2m", "5m", "15m", "30m", "1h", "2h", "4h", "6h", "8h", "12h", "1d", "3d", "7d",
        ];

        for window_size in window_sizes {
            let request = TickerRequest {
                symbol: Some("BTCUSDT".to_string()),
                symbols: None,
                window_size: Some(window_size.to_string()),
                ticker_type: None,
            };

            let serialized = serde_urlencoded::to_string(&request).unwrap();
            assert!(serialized.contains(&format!("windowSize={}", window_size)));
        }
    }

    #[test]
    fn test_ticker_request_types() {
        let types = vec!["FULL", "MINI"];

        for ticker_type in types {
            let request = TickerRequest {
                symbol: Some("BTCUSDT".to_string()),
                symbols: None,
                window_size: None,
                ticker_type: Some(ticker_type.to_string()),
            };

            let serialized = serde_urlencoded::to_string(&request).unwrap();
            assert!(serialized.contains(&format!("type={}", ticker_type)));
        }
    }

    #[test]
    fn test_ticker_request_multiple_symbols_with_all_params() {
        let request = TickerRequest {
            symbol: None,
            symbols: Some("[\"BTCUSDT\",\"ETHUSDT\",\"ADAUSDT\"]".to_string()),
            window_size: Some("4h".to_string()),
            ticker_type: Some("MINI".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbols="));
        assert!(serialized.contains("windowSize=4h"));
        assert!(serialized.contains("type=MINI"));
        assert!(!serialized.contains("symbol="));
    }

    #[test]
    fn test_ticker_same_open_close_price() {
        let json = r#"{
            "symbol": "STABLEUSDT",
            "priceChange": "0.00000000",
            "priceChangePercent": "0.000",
            "weightedAvgPrice": "1.00000000",
            "openPrice": "1.00000000",
            "highPrice": "1.00010000",
            "lowPrice": "0.99990000",
            "lastPrice": "1.00000000",
            "volume": "1000.00000000",
            "quoteVolume": "1000.00000000",
            "openTime": 1625184000000,
            "closeTime": 1625270400000,
            "firstId": 10000,
            "lastId": 12000,
            "count": 2001
        }"#;

        let ticker: Ticker = serde_json::from_str(json).unwrap();
        assert_eq!(ticker.open_price, ticker.last_price);
        assert_eq!(ticker.price_change.to_string(), "0.00000000");
        assert_eq!(ticker.price_change_percent.to_string(), "0.000");
        assert_eq!(ticker.weighted_avg_price.to_string(), "1.00000000");
    }
}
