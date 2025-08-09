use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::binance::spot::RestResult;

const TICKER_TRADING_DAY_ENDPOINT: &str = "/api/v3/ticker/tradingDay";

/// Request parameters for trading day ticker statistics
#[derive(Debug, Clone, Serialize)]
pub struct TickerTradingDayRequest {
    /// Single symbol
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,

    /// Multiple symbols (JSON array format)
    #[serde(rename = "symbols", skip_serializing_if = "Option::is_none")]
    pub symbols: Option<String>,

    /// Time zone (default: 0 (UTC))
    #[serde(rename = "timeZone", skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,

    /// Type of ticker response (FULL or MINI)
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub ticker_type: Option<String>,
}

/// Trading day ticker statistics
#[derive(Debug, Clone, Deserialize)]
pub struct TickerTradingDay {
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
    /// Get trading day ticker
    ///
    /// Price change statistics for a trading day.
    ///
    /// [docs]: (https://developers.binance.com/docs/binance-spot-api-docs/rest-api/public-api-endpoints#trading-day-ticker)
    /// Method: GET /api/v3/ticker/tradingDay
    /// Weight: 4 per symbol (max 200)
    /// Security: None
    pub async fn get_trading_day_ticker(
        &self,
        params: TickerTradingDayRequest,
    ) -> RestResult<serde_json::Value> {
        // Weight is 4 per symbol, max 200 for multiple symbols
        let weight = if params.symbols.is_some() { 200 } else { 4 };

        self.send_get_request(TICKER_TRADING_DAY_ENDPOINT, Some(params), weight)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ticker_trading_day_request_serialization_single_symbol() {
        let request = TickerTradingDayRequest {
            symbol: Some("BTCUSDT".to_string()),
            symbols: None,
            time_zone: None,
            ticker_type: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "symbol=BTCUSDT");
    }

    #[test]
    fn test_ticker_trading_day_request_serialization_multiple_symbols() {
        let request = TickerTradingDayRequest {
            symbol: None,
            symbols: Some("[\"BTCUSDT\",\"ETHUSDT\"]".to_string()),
            time_zone: None,
            ticker_type: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbols=%5B%22BTCUSDT%22%2C%22ETHUSDT%22%5D"));
    }

    #[test]
    fn test_ticker_trading_day_request_serialization_with_timezone() {
        let request = TickerTradingDayRequest {
            symbol: Some("BTCUSDT".to_string()),
            symbols: None,
            time_zone: Some("+08:00".to_string()),
            ticker_type: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTCUSDT"));
        assert!(serialized.contains("timeZone=%2B08%3A00"));
    }

    #[test]
    fn test_ticker_trading_day_request_serialization_full_params() {
        let request = TickerTradingDayRequest {
            symbol: Some("ETHUSDT".to_string()),
            symbols: None,
            time_zone: Some("0".to_string()),
            ticker_type: Some("FULL".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=ETHUSDT"));
        assert!(serialized.contains("timeZone=0"));
        assert!(serialized.contains("type=FULL"));
    }

    #[test]
    fn test_ticker_trading_day_deserialization() {
        let json = r#"{
            "symbol": "BTCUSDT",
            "priceChange": "1500.00000000",
            "priceChangePercent": "3.456",
            "weightedAvgPrice": "44500.75000000",
            "openPrice": "43000.00000000",
            "highPrice": "45100.00000000",
            "lowPrice": "42800.00000000",
            "lastPrice": "44500.00000000",
            "volume": "1234.56789000",
            "quoteVolume": "54950000.00000000",
            "openTime": 1625184000000,
            "closeTime": 1625270400000,
            "firstId": 123456,
            "lastId": 789012,
            "count": 665557
        }"#;

        let ticker: TickerTradingDay = serde_json::from_str(json).unwrap();
        assert_eq!(ticker.symbol, "BTCUSDT");
        assert_eq!(ticker.price_change.to_string(), "1500.00000000");
        assert_eq!(ticker.price_change_percent.to_string(), "3.456");
        assert_eq!(ticker.weighted_avg_price.to_string(), "44500.75000000");
        assert_eq!(ticker.open_price.to_string(), "43000.00000000");
        assert_eq!(ticker.high_price.to_string(), "45100.00000000");
        assert_eq!(ticker.low_price.to_string(), "42800.00000000");
        assert_eq!(ticker.last_price.to_string(), "44500.00000000");
        assert_eq!(ticker.volume.to_string(), "1234.56789000");
        assert_eq!(ticker.quote_volume.to_string(), "54950000.00000000");
        assert_eq!(ticker.open_time, 1625184000000);
        assert_eq!(ticker.close_time, 1625270400000);
        assert_eq!(ticker.first_id, 123456);
        assert_eq!(ticker.last_id, 789012);
        assert_eq!(ticker.count, 665557);
    }

    #[test]
    fn test_ticker_trading_day_array_deserialization() {
        let json = r#"[
            {
                "symbol": "BTCUSDT",
                "priceChange": "100.00000000",
                "priceChangePercent": "2.222",
                "weightedAvgPrice": "4500.50000000",
                "openPrice": "4500.00000000",
                "highPrice": "4650.00000000",
                "lowPrice": "4450.00000000",
                "lastPrice": "4600.00000000",
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
                "weightedAvgPrice": "3075.25000000",
                "openPrice": "3125.00000000",
                "highPrice": "3140.00000000",
                "lowPrice": "3050.00000000",
                "lastPrice": "3075.00000000",
                "volume": "500.00000000",
                "quoteVolume": "1537625.00000000",
                "openTime": 1625184000000,
                "closeTime": 1625270400000,
                "firstId": 200000,
                "lastId": 205000,
                "count": 5001
            }
        ]"#;

        let tickers: Vec<TickerTradingDay> = serde_json::from_str(json).unwrap();
        assert_eq!(tickers.len(), 2);

        assert_eq!(tickers[0].symbol, "BTCUSDT");
        assert_eq!(tickers[0].price_change.to_string(), "100.00000000");
        assert_eq!(tickers[0].price_change_percent.to_string(), "2.222");

        assert_eq!(tickers[1].symbol, "ETHUSDT");
        assert_eq!(tickers[1].price_change.to_string(), "-50.00000000");
        assert_eq!(tickers[1].price_change_percent.to_string(), "-1.600");
    }

    #[test]
    fn test_ticker_trading_day_negative_price_change() {
        let json = r#"{
            "symbol": "BTCUSDT",
            "priceChange": "-2500.50000000",
            "priceChangePercent": "-5.678",
            "weightedAvgPrice": "42000.25000000",
            "openPrice": "46000.00000000",
            "highPrice": "46200.00000000",
            "lowPrice": "41800.00000000",
            "lastPrice": "43499.50000000",
            "volume": "2000.00000000",
            "quoteVolume": "84000500.00000000",
            "openTime": 1625184000000,
            "closeTime": 1625270400000,
            "firstId": 500000,
            "lastId": 1000000,
            "count": 500001
        }"#;

        let ticker: TickerTradingDay = serde_json::from_str(json).unwrap();
        assert_eq!(ticker.price_change.to_string(), "-2500.50000000");
        assert_eq!(ticker.price_change_percent.to_string(), "-5.678");
        assert!(ticker.price_change < Decimal::ZERO);
        assert!(ticker.price_change_percent < Decimal::ZERO);
    }

    #[test]
    fn test_ticker_trading_day_zero_volume() {
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

        let ticker: TickerTradingDay = serde_json::from_str(json).unwrap();
        assert_eq!(ticker.symbol, "RAREUSDT");
        assert_eq!(ticker.price_change.to_string(), "0.00000000");
        assert_eq!(ticker.price_change_percent.to_string(), "0.000");
        assert_eq!(ticker.weighted_avg_price.to_string(), "0.00000000");
        assert_eq!(ticker.volume.to_string(), "0.00000000");
        assert_eq!(ticker.quote_volume.to_string(), "0.00000000");
        assert_eq!(ticker.count, 0);
        assert_eq!(ticker.first_id, -1);
        assert_eq!(ticker.last_id, -1);
    }

    #[test]
    fn test_ticker_trading_day_high_precision_values() {
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

        let ticker: TickerTradingDay = serde_json::from_str(json).unwrap();
        assert_eq!(ticker.symbol, "DOGEUSDT");
        assert_eq!(ticker.price_change.to_string(), "0.00012345");
        assert_eq!(ticker.price_change_percent.to_string(), "1.234567");
        assert_eq!(ticker.weighted_avg_price.to_string(), "0.12345678");
        assert_eq!(ticker.volume.to_string(), "1000000.00000000");
    }

    #[test]
    fn test_ticker_trading_day_request_types() {
        let types = vec!["FULL", "MINI"];

        for ticker_type in types {
            let request = TickerTradingDayRequest {
                symbol: Some("BTCUSDT".to_string()),
                symbols: None,
                time_zone: None,
                ticker_type: Some(ticker_type.to_string()),
            };

            let serialized = serde_urlencoded::to_string(&request).unwrap();
            assert!(serialized.contains(&format!("type={}", ticker_type)));
        }
    }

    #[test]
    fn test_ticker_trading_day_request_timezones() {
        let timezones = vec!["0", "+01:00", "-05:00", "+08:00"];

        for tz in timezones {
            let request = TickerTradingDayRequest {
                symbol: Some("BTCUSDT".to_string()),
                symbols: None,
                time_zone: Some(tz.to_string()),
                ticker_type: None,
            };

            let serialized = serde_urlencoded::to_string(&request).unwrap();
            assert!(serialized.contains("timeZone="));
        }
    }

    #[test]
    fn test_ticker_trading_day_same_open_close_price() {
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

        let ticker: TickerTradingDay = serde_json::from_str(json).unwrap();
        assert_eq!(ticker.open_price, ticker.last_price);
        assert_eq!(ticker.price_change.to_string(), "0.00000000");
        assert_eq!(ticker.price_change_percent.to_string(), "0.000");
    }
}
