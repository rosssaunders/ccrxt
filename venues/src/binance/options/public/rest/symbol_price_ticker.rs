use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::binance::options::RestResult;

const INDEX_ENDPOINT: &str = "/eapi/v1/index";

/// Request parameters for symbol price ticker
#[derive(Debug, Clone, Serialize)]
pub struct SymbolPriceTickerRequest {
    /// Spot pair (Option contract underlying asset, e.g BTCUSDT)
    #[serde(rename = "underlying")]
    pub underlying: String,
}

/// Symbol price ticker response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolPriceTickerResponse {
    /// Time
    #[serde(rename = "time")]
    pub time: u64,

    /// Current spot index price
    #[serde(rename = "indexPrice")]
    pub index_price: Decimal,
}

impl RestClient {
    /// Get symbol price ticker
    ///
    /// Returns spot index price for option underlying.
    ///
    /// [docs]: (https://developers.binance.com/docs/derivatives/option/market-data/Symbol-Price-Ticker)
    /// Method: GET /eapi/v1/index
    /// Weight: 1
    /// Security: None
    pub async fn get_symbol_price_ticker(
        &self,
        params: SymbolPriceTickerRequest,
    ) -> RestResult<SymbolPriceTickerResponse> {
        self.send_public_request(INDEX_ENDPOINT, reqwest::Method::GET, Some(params), 1)
            .await
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    use super::*;

    #[test]
    fn test_symbol_price_ticker_request_serialization() {
        let request = SymbolPriceTickerRequest {
            underlying: "BTCUSDT".to_string(),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("underlying=BTCUSDT"));
    }

    #[test]
    fn test_symbol_price_ticker_request_serialization_different_underlyings() {
        let underlyings = vec![
            "BTCUSDT", "ETHUSDT", "BNBUSDT", "ADAUSDT", "SOLUSDT", "DOGEUSDT", "XRPUSDT",
            "DOTUSDT", "LINKUSDT", "UNIUSDT",
        ];

        for underlying in underlyings {
            let request = SymbolPriceTickerRequest {
                underlying: underlying.to_string(),
            };

            let serialized = serde_urlencoded::to_string(&request).unwrap();
            assert!(serialized.contains(&format!("underlying={}", underlying)));
        }
    }

    #[test]
    fn test_symbol_price_ticker_request_serialization_with_special_characters() {
        let request = SymbolPriceTickerRequest {
            underlying: "BTC-USDT".to_string(),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("underlying=BTC-USDT"));
    }

    #[test]
    fn test_symbol_price_ticker_request_serialization_uppercase() {
        let request = SymbolPriceTickerRequest {
            underlying: "BTCUSDT".to_string(),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("underlying=BTCUSDT"));
        assert!(!serialized.contains("underlying=btcusdt"));
    }

    #[test]
    fn test_symbol_price_ticker_request_serialization_lowercase() {
        let request = SymbolPriceTickerRequest {
            underlying: "btcusdt".to_string(),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("underlying=btcusdt"));
        assert!(!serialized.contains("underlying=BTCUSDT"));
    }

    #[test]
    fn test_symbol_price_ticker_request_serialization_mixed_case() {
        let request = SymbolPriceTickerRequest {
            underlying: "BtcUsdt".to_string(),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("underlying=BtcUsdt"));
    }

    #[test]
    fn test_symbol_price_ticker_request_serialization_long_underlying() {
        let request = SymbolPriceTickerRequest {
            underlying: "VERYLONGUNDERLYINGASSETNAME".to_string(),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("underlying=VERYLONGUNDERLYINGASSETNAME"));
    }

    #[test]
    fn test_symbol_price_ticker_request_clone() {
        let request = SymbolPriceTickerRequest {
            underlying: "BTCUSDT".to_string(),
        };

        let cloned = request.clone();
        assert_eq!(request.underlying, cloned.underlying);
    }

    #[test]
    fn test_symbol_price_ticker_request_debug() {
        let request = SymbolPriceTickerRequest {
            underlying: "BTCUSDT".to_string(),
        };

        let debug_output = format!("{:?}", request);
        assert!(debug_output.contains("SymbolPriceTickerRequest"));
        assert!(debug_output.contains("BTCUSDT"));
    }

    #[test]
    fn test_symbol_price_ticker_response_deserialization() {
        let json = r#"{
            "time": 1625097600000,
            "indexPrice": "35000.12345678"
        }"#;

        let response: SymbolPriceTickerResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.time, 1625097600000);
        assert_eq!(response.index_price, dec!(35000.12345678));
    }

    #[test]
    fn test_symbol_price_ticker_response_deserialization_high_precision() {
        let json = r#"{
            "time": 1625097600000,
            "indexPrice": "35000.12345678901234"
        }"#;

        let response: SymbolPriceTickerResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.time, 1625097600000);
        assert_eq!(response.index_price.to_string(), "35000.12345678901234");
    }

    #[test]
    fn test_symbol_price_ticker_response_deserialization_low_precision() {
        let json = r#"{
            "time": 1625097600000,
            "indexPrice": "35000"
        }"#;

        let response: SymbolPriceTickerResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.time, 1625097600000);
        assert_eq!(response.index_price, dec!(35000));
    }

    #[test]
    fn test_symbol_price_ticker_response_deserialization_decimal_precision() {
        let json = r#"{
            "time": 1625097600000,
            "indexPrice": "35000.50"
        }"#;

        let response: SymbolPriceTickerResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.time, 1625097600000);
        assert_eq!(response.index_price, dec!(35000.50));
    }

    #[test]
    fn test_symbol_price_ticker_response_deserialization_zero_price() {
        let json = r#"{
            "time": 1625097600000,
            "indexPrice": "0.00000000"
        }"#;

        let response: SymbolPriceTickerResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.time, 1625097600000);
        assert_eq!(response.index_price, dec!(0.00000000));
    }

    #[test]
    fn test_symbol_price_ticker_response_deserialization_small_price() {
        let json = r#"{
            "time": 1625097600000,
            "indexPrice": "0.00000001"
        }"#;

        let response: SymbolPriceTickerResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.time, 1625097600000);
        assert_eq!(response.index_price, dec!(0.00000001));
    }

    #[test]
    fn test_symbol_price_ticker_response_deserialization_large_price() {
        let json = r#"{
            "time": 1625097600000,
            "indexPrice": "999999999.99999999"
        }"#;

        let response: SymbolPriceTickerResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.time, 1625097600000);
        assert_eq!(response.index_price, dec!(999999999.99999999));
    }

    #[test]
    fn test_symbol_price_ticker_response_deserialization_current_timestamp() {
        let json = r#"{
            "time": 1700000000000,
            "indexPrice": "42000.12345678"
        }"#;

        let response: SymbolPriceTickerResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.time, 1700000000000);
        assert_eq!(response.index_price, dec!(42000.12345678));
    }

    #[test]
    fn test_symbol_price_ticker_response_deserialization_old_timestamp() {
        let json = r#"{
            "time": 1500000000000,
            "indexPrice": "3000.00000000"
        }"#;

        let response: SymbolPriceTickerResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.time, 1500000000000);
        assert_eq!(response.index_price, dec!(3000.00000000));
    }

    #[test]
    fn test_symbol_price_ticker_response_deserialization_btc_price() {
        let json = r#"{
            "time": 1625097600000,
            "indexPrice": "35000.12345678"
        }"#;

        let response: SymbolPriceTickerResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.time, 1625097600000);
        assert_eq!(response.index_price, dec!(35000.12345678));
    }

    #[test]
    fn test_symbol_price_ticker_response_deserialization_eth_price() {
        let json = r#"{
            "time": 1625097600000,
            "indexPrice": "2500.87654321"
        }"#;

        let response: SymbolPriceTickerResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.time, 1625097600000);
        assert_eq!(response.index_price, dec!(2500.87654321));
    }

    #[test]
    fn test_symbol_price_ticker_response_deserialization_bnb_price() {
        let json = r#"{
            "time": 1625097600000,
            "indexPrice": "300.45678901"
        }"#;

        let response: SymbolPriceTickerResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.time, 1625097600000);
        assert_eq!(response.index_price, dec!(300.45678901));
    }

    #[test]
    fn test_symbol_price_ticker_response_deserialization_various_prices() {
        let prices = vec![
            "1.00000000",
            "10.12345678",
            "100.98765432",
            "1000.11111111",
            "10000.22222222",
            "100000.33333333",
        ];

        for price in prices {
            let json = format!(
                r#"{{
                    "time": 1625097600000,
                    "indexPrice": "{}"
                }}"#,
                price
            );

            let response: SymbolPriceTickerResponse = serde_json::from_str(&json).unwrap();
            assert_eq!(response.time, 1625097600000);
            assert_eq!(response.index_price.to_string(), price);
        }
    }

    #[test]
    fn test_symbol_price_ticker_response_deserialization_various_timestamps() {
        let timestamps = vec![
            1625097600000u64,
            1625097600001u64,
            1625097600999u64,
            1625184000000u64,
            1700000000000u64,
            1800000000000u64,
        ];

        for timestamp in timestamps {
            let json = format!(
                r#"{{
                    "time": {},
                    "indexPrice": "35000.12345678"
                }}"#,
                timestamp
            );

            let response: SymbolPriceTickerResponse = serde_json::from_str(&json).unwrap();
            assert_eq!(response.time, timestamp);
            assert_eq!(response.index_price, dec!(35000.12345678));
        }
    }

    #[test]
    fn test_symbol_price_ticker_response_clone() {
        let json = r#"{
            "time": 1625097600000,
            "indexPrice": "35000.12345678"
        }"#;

        let response: SymbolPriceTickerResponse = serde_json::from_str(json).unwrap();
        let cloned = response.clone();

        assert_eq!(response.time, cloned.time);
        assert_eq!(response.index_price, cloned.index_price);
    }

    #[test]
    fn test_symbol_price_ticker_response_debug() {
        let json = r#"{
            "time": 1625097600000,
            "indexPrice": "35000.12345678"
        }"#;

        let response: SymbolPriceTickerResponse = serde_json::from_str(json).unwrap();
        let debug_output = format!("{:?}", response);

        assert!(debug_output.contains("SymbolPriceTickerResponse"));
        assert!(debug_output.contains("1625097600000"));
        assert!(debug_output.contains("35000.12345678"));
    }

    #[test]
    fn test_symbol_price_ticker_response_time_validation() {
        let json = r#"{
            "time": 1625097600000,
            "indexPrice": "35000.12345678"
        }"#;

        let response: SymbolPriceTickerResponse = serde_json::from_str(json).unwrap();

        // Verify timestamp is in reasonable range (after year 2020)
        assert!(response.time > 1577836800000); // 2020-01-01 00:00:00 UTC
        // Verify timestamp is not too far in the future (before year 2100)
        assert!(response.time < 4102444800000); // 2100-01-01 00:00:00 UTC
    }

    #[test]
    fn test_symbol_price_ticker_response_price_validation() {
        let json = r#"{
            "time": 1625097600000,
            "indexPrice": "35000.12345678"
        }"#;

        let response: SymbolPriceTickerResponse = serde_json::from_str(json).unwrap();

        // Verify price is positive
        assert!(response.index_price > dec!(0));
        // Verify price is in reasonable range for crypto
        assert!(response.index_price < dec!(10000000)); // Less than 10 million
    }

    #[test]
    fn test_symbol_price_ticker_response_precision_preservation() {
        let json = r#"{
            "time": 1625097600000,
            "indexPrice": "35000.12345678"
        }"#;

        let response: SymbolPriceTickerResponse = serde_json::from_str(json).unwrap();

        // Verify precision is preserved
        assert_eq!(response.index_price.to_string(), "35000.12345678");

        // Verify decimal operations work correctly
        let doubled = response.index_price * dec!(2);
        assert_eq!(doubled.to_string(), "70000.24691356");
    }

    #[test]
    fn test_symbol_price_ticker_response_mathematical_operations() {
        let json = r#"{
            "time": 1625097600000,
            "indexPrice": "35000.12345678"
        }"#;

        let response: SymbolPriceTickerResponse = serde_json::from_str(json).unwrap();

        // Test addition
        let added = response.index_price + dec!(1000);
        assert_eq!(added.to_string(), "36000.12345678");

        // Test subtraction
        let subtracted = response.index_price - dec!(1000);
        assert_eq!(subtracted.to_string(), "34000.12345678");

        // Test multiplication
        let multiplied = response.index_price * dec!(2);
        assert_eq!(multiplied.to_string(), "70000.24691356");

        // Test division
        let divided = response.index_price / dec!(2);
        assert_eq!(divided.to_string(), "17500.06172839");
    }

    #[test]
    fn test_symbol_price_ticker_response_comparison_operations() {
        let json1 = r#"{
            "time": 1625097600000,
            "indexPrice": "35000.12345678"
        }"#;

        let json2 = r#"{
            "time": 1625097600000,
            "indexPrice": "36000.12345678"
        }"#;

        let response1: SymbolPriceTickerResponse = serde_json::from_str(json1).unwrap();
        let response2: SymbolPriceTickerResponse = serde_json::from_str(json2).unwrap();

        // Test comparison operations
        assert!(response1.index_price < response2.index_price);
        assert!(response2.index_price > response1.index_price);
        assert_ne!(response1.index_price, response2.index_price);
    }

    #[test]
    fn test_symbol_price_ticker_response_edge_cases() {
        // Test with minimal precision
        let json_minimal = r#"{
            "time": 1625097600000,
            "indexPrice": "1"
        }"#;

        let response_minimal: SymbolPriceTickerResponse =
            serde_json::from_str(json_minimal).unwrap();
        assert_eq!(response_minimal.index_price, dec!(1));

        // Test with maximum precision
        let json_max = r#"{
            "time": 1625097600000,
            "indexPrice": "1.123456789012345678"
        }"#;

        let response_max: SymbolPriceTickerResponse = serde_json::from_str(json_max).unwrap();
        assert_eq!(response_max.index_price.to_string(), "1.123456789012345678");
    }

    #[test]
    fn test_symbol_price_ticker_response_serialization_roundtrip() {
        let original_json = r#"{
            "time": 1625097600000,
            "indexPrice": "35000.12345678"
        }"#;

        let response: SymbolPriceTickerResponse = serde_json::from_str(original_json).unwrap();
        let serialized = serde_json::to_string(&response).unwrap();
        let deserialized: SymbolPriceTickerResponse = serde_json::from_str(&serialized).unwrap();

        assert_eq!(response.time, deserialized.time);
        assert_eq!(response.index_price, deserialized.index_price);
    }
}
