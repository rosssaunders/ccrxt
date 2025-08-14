use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::binance::options::RestResult;

const TICKER_ENDPOINT: &str = "/eapi/v1/ticker";

/// Request parameters for 24hr ticker
#[derive(Debug, Clone, Serialize, Default)]
pub struct TickerRequest {
    /// Option trading pair (if omitted, returns all symbols)
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
}

/// 24hr ticker price change statistics
#[derive(Debug, Clone, Deserialize)]
pub struct TickerResponse {
    /// Option trading pair
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// 24-hour price change
    #[serde(rename = "priceChange")]
    pub price_change: Decimal,

    /// 24-hour percent price change
    #[serde(rename = "priceChangePercent")]
    pub price_change_percent: Decimal,

    /// Last trade price
    #[serde(rename = "lastPrice")]
    pub last_price: Decimal,

    /// Last trade amount
    #[serde(rename = "lastQty")]
    pub last_qty: Decimal,

    /// 24-hour open price
    #[serde(rename = "open")]
    pub open: Decimal,

    /// 24-hour high
    #[serde(rename = "high")]
    pub high: Decimal,

    /// 24-hour low
    #[serde(rename = "low")]
    pub low: Decimal,

    /// Trading volume (contracts)
    #[serde(rename = "volume")]
    pub volume: Decimal,

    /// Trade amount (in quote asset)
    #[serde(rename = "amount")]
    pub amount: Decimal,

    /// The best buy price
    #[serde(rename = "bidPrice")]
    pub bid_price: Decimal,

    /// The best sell price
    #[serde(rename = "askPrice")]
    pub ask_price: Decimal,

    /// Time the first trade occurred within the last 24 hours
    #[serde(rename = "openTime")]
    pub open_time: u64,

    /// Time the last trade occurred within the last 24 hours
    #[serde(rename = "closeTime")]
    pub close_time: u64,

    /// First trade ID
    #[serde(rename = "firstTradeId")]
    pub first_trade_id: u64,

    /// Number of trades
    #[serde(rename = "tradeCount")]
    pub trade_count: u64,

    /// Strike price
    #[serde(rename = "strikePrice")]
    pub strike_price: Decimal,

    /// Estimated settlement price one hour before exercise, index price at other times
    #[serde(rename = "exercisePrice")]
    pub exercise_price: Decimal,
}

impl RestClient {
    /// Get 24hr ticker price change statistics
    ///
    /// Returns 24 hour rolling window price change statistics.
    ///
    /// [docs](https://developers.binance.com/docs/derivatives/option/market-data/24hr-Ticker-Price-Change-Statistics)
    ///
    /// Method: GET /eapi/v1/ticker
    /// Weight: 5
    /// Security: None
    pub async fn get_ticker(&self, params: TickerRequest) -> RestResult<Vec<TickerResponse>> {
        if params.symbol.is_none() {
            self.send_get_request(TICKER_ENDPOINT, None::<()>, 5).await
        } else {
            self.send_get_request(TICKER_ENDPOINT, Some(params), 5)
                .await
        }
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    use super::*;

    #[test]
    fn test_ticker_request_serialization_with_symbol() {
        let request = TickerRequest {
            symbol: Some("BTC-240329-70000-C".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTC-240329-70000-C"));
    }

    #[test]
    fn test_ticker_request_serialization_without_symbol() {
        let request = TickerRequest { symbol: None };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(!serialized.contains("symbol"));
        assert_eq!(serialized, "");
    }

    #[test]
    fn test_ticker_request_serialization_default() {
        let request = TickerRequest::default();

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(!serialized.contains("symbol"));
        assert_eq!(serialized, "");
    }

    #[test]
    fn test_ticker_request_serialization_various_symbols() {
        let symbols = vec![
            "BTC-240329-70000-C",
            "BTC-240329-70000-P",
            "ETH-240329-3000-C",
            "ETH-240329-3000-P",
            "BNB-240329-500-C",
        ];

        for symbol in symbols {
            let request = TickerRequest {
                symbol: Some(symbol.to_string()),
            };

            let serialized = serde_urlencoded::to_string(&request).unwrap();
            assert!(serialized.contains(&format!("symbol={}", symbol)));
        }
    }

    #[test]
    fn test_ticker_request_clone() {
        let request = TickerRequest {
            symbol: Some("BTC-240329-70000-C".to_string()),
        };

        let cloned = request.clone();
        assert_eq!(request.symbol, cloned.symbol);
    }

    #[test]
    fn test_ticker_request_debug() {
        let request = TickerRequest {
            symbol: Some("BTC-240329-70000-C".to_string()),
        };

        let debug_output = format!("{:?}", request);
        assert!(debug_output.contains("TickerRequest"));
        assert!(debug_output.contains("BTC-240329-70000-C"));
    }

    #[test]
    fn test_ticker_response_deserialization() {
        let json = r#"{
            "symbol": "BTC-240329-70000-C",
            "priceChange": "150.50",
            "priceChangePercent": "2.15",
            "lastPrice": "7150.50",
            "lastQty": "0.10",
            "open": "7000.00",
            "high": "7200.00",
            "low": "6950.00",
            "volume": "1000.50",
            "amount": "7150500.00",
            "bidPrice": "7140.00",
            "askPrice": "7160.00",
            "openTime": 1625097600000,
            "closeTime": 1625184000000,
            "firstTradeId": 12345,
            "tradeCount": 2500,
            "strikePrice": "70000.00",
            "exercisePrice": "65000.00"
        }"#;

        let ticker: TickerResponse = serde_json::from_str(json).unwrap();
        assert_eq!(ticker.symbol, "BTC-240329-70000-C");
        assert_eq!(ticker.price_change, dec!(150.50));
        assert_eq!(ticker.price_change_percent, dec!(2.15));
        assert_eq!(ticker.last_price, dec!(7150.50));
        assert_eq!(ticker.last_qty, dec!(0.10));
        assert_eq!(ticker.open, dec!(7000.00));
        assert_eq!(ticker.high, dec!(7200.00));
        assert_eq!(ticker.low, dec!(6950.00));
        assert_eq!(ticker.volume, dec!(1000.50));
        assert_eq!(ticker.amount, dec!(7150500.00));
        assert_eq!(ticker.bid_price, dec!(7140.00));
        assert_eq!(ticker.ask_price, dec!(7160.00));
        assert_eq!(ticker.open_time, 1625097600000);
        assert_eq!(ticker.close_time, 1625184000000);
        assert_eq!(ticker.first_trade_id, 12345);
        assert_eq!(ticker.trade_count, 2500);
        assert_eq!(ticker.strike_price, dec!(70000.00));
        assert_eq!(ticker.exercise_price, dec!(65000.00));
    }

    #[test]
    fn test_ticker_response_deserialization_high_precision() {
        let json = r#"{
            "symbol": "ETH-240329-3000-P",
            "priceChange": "15.12345678",
            "priceChangePercent": "0.50123456",
            "lastPrice": "3015.12345678",
            "lastQty": "0.12345678",
            "open": "3000.00000000",
            "high": "3020.87654321",
            "low": "2995.11111111",
            "volume": "999.12345678",
            "amount": "3011234.56789012",
            "bidPrice": "3010.00000000",
            "askPrice": "3020.00000000",
            "openTime": 1625097600000,
            "closeTime": 1625184000000,
            "firstTradeId": 98765,
            "tradeCount": 1500,
            "strikePrice": "3000.00000000",
            "exercisePrice": "2950.12345678"
        }"#;

        let ticker: TickerResponse = serde_json::from_str(json).unwrap();
        assert_eq!(ticker.price_change.to_string(), "15.12345678");
        assert_eq!(ticker.price_change_percent.to_string(), "0.50123456");
        assert_eq!(ticker.last_price.to_string(), "3015.12345678");
        assert_eq!(ticker.last_qty.to_string(), "0.12345678");
        assert_eq!(ticker.open.to_string(), "3000.00000000");
        assert_eq!(ticker.high.to_string(), "3020.87654321");
        assert_eq!(ticker.low.to_string(), "2995.11111111");
        assert_eq!(ticker.volume.to_string(), "999.12345678");
        assert_eq!(ticker.amount.to_string(), "3011234.56789012");
        assert_eq!(ticker.bid_price.to_string(), "3010.00000000");
        assert_eq!(ticker.ask_price.to_string(), "3020.00000000");
        assert_eq!(ticker.strike_price.to_string(), "3000.00000000");
        assert_eq!(ticker.exercise_price.to_string(), "2950.12345678");
    }

    #[test]
    fn test_ticker_response_deserialization_zero_values() {
        let json = r#"{
            "symbol": "BTC-240329-70000-C",
            "priceChange": "0.00000000",
            "priceChangePercent": "0.00000000",
            "lastPrice": "0.00000000",
            "lastQty": "0.00000000",
            "open": "0.00000000",
            "high": "0.00000000",
            "low": "0.00000000",
            "volume": "0.00000000",
            "amount": "0.00000000",
            "bidPrice": "0.00000000",
            "askPrice": "0.00000000",
            "openTime": 1625097600000,
            "closeTime": 1625184000000,
            "firstTradeId": 0,
            "tradeCount": 0,
            "strikePrice": "0.00000000",
            "exercisePrice": "0.00000000"
        }"#;

        let ticker: TickerResponse = serde_json::from_str(json).unwrap();
        assert_eq!(ticker.price_change, dec!(0.00000000));
        assert_eq!(ticker.price_change_percent, dec!(0.00000000));
        assert_eq!(ticker.last_price, dec!(0.00000000));
        assert_eq!(ticker.last_qty, dec!(0.00000000));
        assert_eq!(ticker.open, dec!(0.00000000));
        assert_eq!(ticker.high, dec!(0.00000000));
        assert_eq!(ticker.low, dec!(0.00000000));
        assert_eq!(ticker.volume, dec!(0.00000000));
        assert_eq!(ticker.amount, dec!(0.00000000));
        assert_eq!(ticker.bid_price, dec!(0.00000000));
        assert_eq!(ticker.ask_price, dec!(0.00000000));
        assert_eq!(ticker.first_trade_id, 0);
        assert_eq!(ticker.trade_count, 0);
        assert_eq!(ticker.strike_price, dec!(0.00000000));
        assert_eq!(ticker.exercise_price, dec!(0.00000000));
    }

    #[test]
    fn test_ticker_response_deserialization_negative_values() {
        let json = r#"{
            "symbol": "BTC-240329-70000-P",
            "priceChange": "-250.75",
            "priceChangePercent": "-3.58",
            "lastPrice": "6749.25",
            "lastQty": "0.05",
            "open": "7000.00",
            "high": "7050.00",
            "low": "6700.00",
            "volume": "500.25",
            "amount": "3375000.00",
            "bidPrice": "6740.00",
            "askPrice": "6760.00",
            "openTime": 1625097600000,
            "closeTime": 1625184000000,
            "firstTradeId": 54321,
            "tradeCount": 1250,
            "strikePrice": "70000.00",
            "exercisePrice": "65000.00"
        }"#;

        let ticker: TickerResponse = serde_json::from_str(json).unwrap();
        assert_eq!(ticker.price_change, dec!(-250.75));
        assert_eq!(ticker.price_change_percent, dec!(-3.58));
        assert_eq!(ticker.last_price, dec!(6749.25));
        assert_eq!(ticker.open, dec!(7000.00));
        assert_eq!(ticker.high, dec!(7050.00));
        assert_eq!(ticker.low, dec!(6700.00));

        // Verify the negative change is consistent
        assert!(ticker.price_change.is_sign_negative());
        assert!(ticker.price_change_percent.is_sign_negative());
    }

    #[test]
    fn test_ticker_response_deserialization_large_values() {
        let json = r#"{
            "symbol": "BTC-240329-100000-C",
            "priceChange": "9999.99999999",
            "priceChangePercent": "15.50000000",
            "lastPrice": "99999.99999999",
            "lastQty": "999.99999999",
            "open": "90000.00000000",
            "high": "100000.00000000",
            "low": "89000.00000000",
            "volume": "99999.99999999",
            "amount": "9999999999.99999999",
            "bidPrice": "99900.00000000",
            "askPrice": "100100.00000000",
            "openTime": 1625097600000,
            "closeTime": 1625184000000,
            "firstTradeId": 999999,
            "tradeCount": 999999,
            "strikePrice": "100000.00000000",
            "exercisePrice": "95000.00000000"
        }"#;

        let ticker: TickerResponse = serde_json::from_str(json).unwrap();
        assert_eq!(ticker.price_change.to_string(), "9999.99999999");
        assert_eq!(ticker.price_change_percent.to_string(), "15.50000000");
        assert_eq!(ticker.last_price.to_string(), "99999.99999999");
        assert_eq!(ticker.last_qty.to_string(), "999.99999999");
        assert_eq!(ticker.open.to_string(), "90000.00000000");
        assert_eq!(ticker.high.to_string(), "100000.00000000");
        assert_eq!(ticker.low.to_string(), "89000.00000000");
        assert_eq!(ticker.volume.to_string(), "99999.99999999");
        assert_eq!(ticker.amount.to_string(), "9999999999.99999999");
        assert_eq!(ticker.bid_price.to_string(), "99900.00000000");
        assert_eq!(ticker.ask_price.to_string(), "100100.00000000");
        assert_eq!(ticker.first_trade_id, 999999);
        assert_eq!(ticker.trade_count, 999999);
        assert_eq!(ticker.strike_price.to_string(), "100000.00000000");
        assert_eq!(ticker.exercise_price.to_string(), "95000.00000000");
    }

    #[test]
    fn test_ticker_response_deserialization_different_symbols() {
        let symbols = vec![
            "BTC-240329-70000-C",
            "BTC-240329-70000-P",
            "ETH-240329-3000-C",
            "ETH-240329-3000-P",
            "BNB-240329-500-C",
            "BNB-240329-500-P",
        ];

        for symbol in symbols {
            let json = format!(
                r#"{{
                    "symbol": "{}",
                    "priceChange": "100.00",
                    "priceChangePercent": "1.50",
                    "lastPrice": "6750.00",
                    "lastQty": "0.10",
                    "open": "6650.00",
                    "high": "6800.00",
                    "low": "6600.00",
                    "volume": "1000.00",
                    "amount": "6750000.00",
                    "bidPrice": "6740.00",
                    "askPrice": "6760.00",
                    "openTime": 1625097600000,
                    "closeTime": 1625184000000,
                    "firstTradeId": 12345,
                    "tradeCount": 2500,
                    "strikePrice": "70000.00",
                    "exercisePrice": "65000.00"
                }}"#,
                symbol
            );

            let ticker: TickerResponse = serde_json::from_str(&json).unwrap();
            assert_eq!(ticker.symbol, symbol);
        }
    }

    #[test]
    fn test_ticker_response_array_deserialization() {
        let json = r#"[
            {
                "symbol": "BTC-240329-70000-C",
                "priceChange": "150.50",
                "priceChangePercent": "2.15",
                "lastPrice": "7150.50",
                "lastQty": "0.10",
                "open": "7000.00",
                "high": "7200.00",
                "low": "6950.00",
                "volume": "1000.50",
                "amount": "7150500.00",
                "bidPrice": "7140.00",
                "askPrice": "7160.00",
                "openTime": 1625097600000,
                "closeTime": 1625184000000,
                "firstTradeId": 12345,
                "tradeCount": 2500,
                "strikePrice": "70000.00",
                "exercisePrice": "65000.00"
            },
            {
                "symbol": "ETH-240329-3000-P",
                "priceChange": "-25.75",
                "priceChangePercent": "-0.85",
                "lastPrice": "2974.25",
                "lastQty": "0.25",
                "open": "3000.00",
                "high": "3010.00",
                "low": "2950.00",
                "volume": "750.25",
                "amount": "2231187.50",
                "bidPrice": "2970.00",
                "askPrice": "2980.00",
                "openTime": 1625097600000,
                "closeTime": 1625184000000,
                "firstTradeId": 67890,
                "tradeCount": 1800,
                "strikePrice": "3000.00",
                "exercisePrice": "2950.00"
            }
        ]"#;

        let tickers: Vec<TickerResponse> = serde_json::from_str(json).unwrap();
        assert_eq!(tickers.len(), 2);

        // First ticker (BTC call)
        assert_eq!(tickers[0].symbol, "BTC-240329-70000-C");
        assert_eq!(tickers[0].price_change, dec!(150.50));
        assert_eq!(tickers[0].price_change_percent, dec!(2.15));
        assert_eq!(tickers[0].last_price, dec!(7150.50));
        assert_eq!(tickers[0].strike_price, dec!(70000.00));

        // Second ticker (ETH put)
        assert_eq!(tickers[1].symbol, "ETH-240329-3000-P");
        assert_eq!(tickers[1].price_change, dec!(-25.75));
        assert_eq!(tickers[1].price_change_percent, dec!(-0.85));
        assert_eq!(tickers[1].last_price, dec!(2974.25));
        assert_eq!(tickers[1].strike_price, dec!(3000.00));
    }

    #[test]
    fn test_ticker_response_empty_array_deserialization() {
        let json = r#"[]"#;
        let tickers: Vec<TickerResponse> = serde_json::from_str(json).unwrap();
        assert_eq!(tickers.len(), 0);
    }

    #[test]
    fn test_ticker_response_clone() {
        let json = r#"{
            "symbol": "BTC-240329-70000-C",
            "priceChange": "150.50",
            "priceChangePercent": "2.15",
            "lastPrice": "7150.50",
            "lastQty": "0.10",
            "open": "7000.00",
            "high": "7200.00",
            "low": "6950.00",
            "volume": "1000.50",
            "amount": "7150500.00",
            "bidPrice": "7140.00",
            "askPrice": "7160.00",
            "openTime": 1625097600000,
            "closeTime": 1625184000000,
            "firstTradeId": 12345,
            "tradeCount": 2500,
            "strikePrice": "70000.00",
            "exercisePrice": "65000.00"
        }"#;

        let ticker: TickerResponse = serde_json::from_str(json).unwrap();
        let cloned = ticker.clone();

        assert_eq!(ticker.symbol, cloned.symbol);
        assert_eq!(ticker.price_change, cloned.price_change);
        assert_eq!(ticker.last_price, cloned.last_price);
        assert_eq!(ticker.open_time, cloned.open_time);
        assert_eq!(ticker.strike_price, cloned.strike_price);
    }

    #[test]
    fn test_ticker_response_debug() {
        let json = r#"{
            "symbol": "BTC-240329-70000-C",
            "priceChange": "150.50",
            "priceChangePercent": "2.15",
            "lastPrice": "7150.50",
            "lastQty": "0.10",
            "open": "7000.00",
            "high": "7200.00",
            "low": "6950.00",
            "volume": "1000.50",
            "amount": "7150500.00",
            "bidPrice": "7140.00",
            "askPrice": "7160.00",
            "openTime": 1625097600000,
            "closeTime": 1625184000000,
            "firstTradeId": 12345,
            "tradeCount": 2500,
            "strikePrice": "70000.00",
            "exercisePrice": "65000.00"
        }"#;

        let ticker: TickerResponse = serde_json::from_str(json).unwrap();
        let debug_output = format!("{:?}", ticker);

        assert!(debug_output.contains("TickerResponse"));
        assert!(debug_output.contains("BTC-240329-70000-C"));
        assert!(debug_output.contains("7150.50"));
    }

    #[test]
    fn test_ticker_response_consistency_checks() {
        let json = r#"{
            "symbol": "BTC-240329-70000-C",
            "priceChange": "150.50",
            "priceChangePercent": "2.15",
            "lastPrice": "7150.50",
            "lastQty": "0.10",
            "open": "7000.00",
            "high": "7200.00",
            "low": "6950.00",
            "volume": "1000.50",
            "amount": "7150500.00",
            "bidPrice": "7140.00",
            "askPrice": "7160.00",
            "openTime": 1625097600000,
            "closeTime": 1625184000000,
            "firstTradeId": 12345,
            "tradeCount": 2500,
            "strikePrice": "70000.00",
            "exercisePrice": "65000.00"
        }"#;

        let ticker: TickerResponse = serde_json::from_str(json).unwrap();

        // Verify price consistency: low <= open, lastPrice <= high
        assert!(ticker.low <= ticker.open);
        assert!(ticker.low <= ticker.last_price);
        assert!(ticker.open <= ticker.high);
        assert!(ticker.last_price <= ticker.high);

        // Verify bid-ask spread: bid <= ask
        assert!(ticker.bid_price <= ticker.ask_price);

        // Verify time consistency: openTime <= closeTime
        assert!(ticker.open_time <= ticker.close_time);

        // Verify price change calculation consistency
        let expected_change = ticker.last_price - ticker.open;
        assert_eq!(ticker.price_change, expected_change);
    }

    #[test]
    fn test_ticker_response_time_range_validation() {
        let json = r#"{
            "symbol": "BTC-240329-70000-C",
            "priceChange": "150.50",
            "priceChangePercent": "2.15",
            "lastPrice": "7150.50",
            "lastQty": "0.10",
            "open": "7000.00",
            "high": "7200.00",
            "low": "6950.00",
            "volume": "1000.50",
            "amount": "7150500.00",
            "bidPrice": "7140.00",
            "askPrice": "7160.00",
            "openTime": 1625097600000,
            "closeTime": 1625184000000,
            "firstTradeId": 12345,
            "tradeCount": 2500,
            "strikePrice": "70000.00",
            "exercisePrice": "65000.00"
        }"#;

        let ticker: TickerResponse = serde_json::from_str(json).unwrap();

        // Verify 24-hour time range (86400000 ms = 24 hours)
        let time_diff = ticker.close_time - ticker.open_time;
        assert_eq!(time_diff, 86400000);
    }

    #[test]
    fn test_ticker_response_call_put_scenarios() {
        // Test CALL option
        let call_json = r#"{
            "symbol": "BTC-240329-70000-C",
            "priceChange": "150.50",
            "priceChangePercent": "2.15",
            "lastPrice": "7150.50",
            "lastQty": "0.10",
            "open": "7000.00",
            "high": "7200.00",
            "low": "6950.00",
            "volume": "1000.50",
            "amount": "7150500.00",
            "bidPrice": "7140.00",
            "askPrice": "7160.00",
            "openTime": 1625097600000,
            "closeTime": 1625184000000,
            "firstTradeId": 12345,
            "tradeCount": 2500,
            "strikePrice": "70000.00",
            "exercisePrice": "65000.00"
        }"#;

        let call_ticker: TickerResponse = serde_json::from_str(call_json).unwrap();
        assert!(call_ticker.symbol.contains("-C"));
        assert_eq!(call_ticker.strike_price, dec!(70000.00));

        // Test PUT option
        let put_json = r#"{
            "symbol": "BTC-240329-70000-P",
            "priceChange": "-250.75",
            "priceChangePercent": "-3.58",
            "lastPrice": "6749.25",
            "lastQty": "0.05",
            "open": "7000.00",
            "high": "7050.00",
            "low": "6700.00",
            "volume": "500.25",
            "amount": "3375000.00",
            "bidPrice": "6740.00",
            "askPrice": "6760.00",
            "openTime": 1625097600000,
            "closeTime": 1625184000000,
            "firstTradeId": 54321,
            "tradeCount": 1250,
            "strikePrice": "70000.00",
            "exercisePrice": "65000.00"
        }"#;

        let put_ticker: TickerResponse = serde_json::from_str(put_json).unwrap();
        assert!(put_ticker.symbol.contains("-P"));
        assert_eq!(put_ticker.strike_price, dec!(70000.00));

        // Both options should have same strike price but different behavior
        assert_eq!(call_ticker.strike_price, put_ticker.strike_price);
        assert!(call_ticker.price_change.is_sign_positive());
        assert!(put_ticker.price_change.is_sign_negative());
    }

    #[test]
    fn test_ticker_response_volume_amount_relationship() {
        let json = r#"{
            "symbol": "BTC-240329-70000-C",
            "priceChange": "150.50",
            "priceChangePercent": "2.15",
            "lastPrice": "7150.50",
            "lastQty": "0.10",
            "open": "7000.00",
            "high": "7200.00",
            "low": "6950.00",
            "volume": "1000.00",
            "amount": "7150500.00",
            "bidPrice": "7140.00",
            "askPrice": "7160.00",
            "openTime": 1625097600000,
            "closeTime": 1625184000000,
            "firstTradeId": 12345,
            "tradeCount": 2500,
            "strikePrice": "70000.00",
            "exercisePrice": "65000.00"
        }"#;

        let ticker: TickerResponse = serde_json::from_str(json).unwrap();

        // Volume should be positive for active trading
        assert!(ticker.volume > dec!(0));
        assert!(ticker.amount > dec!(0));

        // Amount should be roughly volume * average price
        // (allowing for some variation due to price changes during the period)
        let avg_price = (ticker.open + ticker.high + ticker.low + ticker.last_price) / dec!(4);
        let expected_amount = ticker.volume * avg_price;

        // Check that amount is in reasonable range (within 50% of expected)
        let ratio = ticker.amount / expected_amount;
        assert!(ratio > dec!(0.5) && ratio < dec!(1.5));
    }
}
