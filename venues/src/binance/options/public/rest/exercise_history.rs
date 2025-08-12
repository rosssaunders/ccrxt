use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::binance::options::RestResult;

const EXERCISE_HISTORY_ENDPOINT: &str = "/eapi/v1/exerciseHistory";

/// Request parameters for exercise history
#[derive(Debug, Clone, Serialize, Default)]
pub struct ExerciseHistoryRequest {
    /// Underlying index like BTCUSDT
    #[serde(rename = "underlying", skip_serializing_if = "Option::is_none")]
    pub underlying: Option<String>,

    /// Start time
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,

    /// End time
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,

    /// Number of records (Default: 100, Max: 100)
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

/// Exercise result type
#[derive(Debug, Clone, Deserialize)]
pub enum StrikeResult {
    /// Exercised
    #[serde(rename = "REALISTIC_VALUE_STRICKEN")]
    RealisticValueStricken,

    /// Expired OTM
    #[serde(rename = "EXTRINSIC_VALUE_EXPIRED")]
    ExtrinsicValueExpired,
}

/// Historical exercise record
#[derive(Debug, Clone, Deserialize)]
pub struct ExerciseHistoryRecord {
    /// Symbol
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Strike price
    #[serde(rename = "strikePrice")]
    pub strike_price: Decimal,

    /// Real strike price
    #[serde(rename = "realStrikePrice")]
    pub real_strike_price: Decimal,

    /// Exercise time
    #[serde(rename = "expiryDate")]
    pub expiry_date: u64,

    /// Strike result
    #[serde(rename = "strikeResult")]
    pub strike_result: StrikeResult,
}

impl RestClient {
    /// Get historical exercise records
    ///
    /// Returns historical exercise records.
    /// - REALISTIC_VALUE_STRICKEN -> Exercised
    /// - EXTRINSIC_VALUE_EXPIRED -> Expired OTM
    ///
    /// [docs](https://developers.binance.com/docs/derivatives/option/market-data/Historical-Exercise-Records)
    /// Method: GET /eapi/v1/exerciseHistory
    /// Weight: 3
    /// Security: None
    pub async fn get_exercise_history(
        &self,
        params: ExerciseHistoryRequest,
    ) -> RestResult<Vec<ExerciseHistoryRecord>> {
        if params.underlying.is_none()
            && params.start_time.is_none()
            && params.end_time.is_none()
            && params.limit.is_none()
        {
            self.send_get_request(EXERCISE_HISTORY_ENDPOINT, None::<()>, 3)
                .await
        } else {
            self.send_get_request(EXERCISE_HISTORY_ENDPOINT, Some(params), 3)
                .await
        }
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    use super::*;

    #[test]
    fn test_exercise_history_request_serialization_empty() {
        let request = ExerciseHistoryRequest::default();
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "");
    }

    #[test]
    fn test_exercise_history_request_serialization_with_underlying() {
        let request = ExerciseHistoryRequest {
            underlying: Some("BTCUSDT".to_string()),
            ..Default::default()
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("underlying=BTCUSDT"));
    }

    #[test]
    fn test_exercise_history_request_serialization_with_start_time() {
        let request = ExerciseHistoryRequest {
            start_time: Some(1640995200000),
            ..Default::default()
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("startTime=1640995200000"));
    }

    #[test]
    fn test_exercise_history_request_serialization_with_end_time() {
        let request = ExerciseHistoryRequest {
            end_time: Some(1641081600000),
            ..Default::default()
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("endTime=1641081600000"));
    }

    #[test]
    fn test_exercise_history_request_serialization_with_limit() {
        let request = ExerciseHistoryRequest {
            limit: Some(50),
            ..Default::default()
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("limit=50"));
    }

    #[test]
    fn test_exercise_history_request_serialization_with_all_fields() {
        let request = ExerciseHistoryRequest {
            underlying: Some("ETHUSDT".to_string()),
            start_time: Some(1640995200000),
            end_time: Some(1641081600000),
            limit: Some(25),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("underlying=ETHUSDT"));
        assert!(serialized.contains("startTime=1640995200000"));
        assert!(serialized.contains("endTime=1641081600000"));
        assert!(serialized.contains("limit=25"));
    }

    #[test]
    fn test_exercise_history_request_serialization_different_underlyings() {
        let underlyings = vec!["BTCUSDT", "ETHUSDT", "BNBUSDT", "ADAUSDT", "DOTUSDT"];

        for underlying in underlyings {
            let request = ExerciseHistoryRequest {
                underlying: Some(underlying.to_string()),
                ..Default::default()
            };

            let serialized = serde_urlencoded::to_string(&request).unwrap();
            assert!(serialized.contains(&format!("underlying={}", underlying)));
        }
    }

    #[test]
    fn test_exercise_history_request_serialization_different_limits() {
        let limits = vec![1, 10, 25, 50, 100];

        for limit in limits {
            let request = ExerciseHistoryRequest {
                limit: Some(limit),
                ..Default::default()
            };

            let serialized = serde_urlencoded::to_string(&request).unwrap();
            assert!(serialized.contains(&format!("limit={}", limit)));
        }
    }

    #[test]
    fn test_exercise_history_request_serialization_time_ranges() {
        let test_cases = vec![
            (1640995200000, 1641081600000), // 1 day range
            (1640995200000, 1641168000000), // 2 day range
            (1640995200000, 1641600000000), // 1 week range
            (1640995200000, 1643587200000), // 1 month range
        ];

        for (start, end) in test_cases {
            let request = ExerciseHistoryRequest {
                start_time: Some(start),
                end_time: Some(end),
                ..Default::default()
            };

            let serialized = serde_urlencoded::to_string(&request).unwrap();
            assert!(serialized.contains(&format!("startTime={}", start)));
            assert!(serialized.contains(&format!("endTime={}", end)));
        }
    }

    #[test]
    fn test_exercise_history_request_serialization_skip_none_fields() {
        let request = ExerciseHistoryRequest {
            underlying: Some("BTCUSDT".to_string()),
            start_time: None,
            end_time: Some(1641081600000),
            limit: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("underlying=BTCUSDT"));
        assert!(!serialized.contains("startTime"));
        assert!(serialized.contains("endTime=1641081600000"));
        assert!(!serialized.contains("limit"));
    }

    #[test]
    fn test_exercise_history_request_clone() {
        let request = ExerciseHistoryRequest {
            underlying: Some("BTCUSDT".to_string()),
            start_time: Some(1640995200000),
            end_time: Some(1641081600000),
            limit: Some(50),
        };

        let cloned = request.clone();
        assert_eq!(request.underlying, cloned.underlying);
        assert_eq!(request.start_time, cloned.start_time);
        assert_eq!(request.end_time, cloned.end_time);
        assert_eq!(request.limit, cloned.limit);
    }

    #[test]
    fn test_exercise_history_request_debug() {
        let request = ExerciseHistoryRequest {
            underlying: Some("BTCUSDT".to_string()),
            start_time: Some(1640995200000),
            end_time: Some(1641081600000),
            limit: Some(50),
        };

        let debug_output = format!("{:?}", request);
        assert!(debug_output.contains("ExerciseHistoryRequest"));
        assert!(debug_output.contains("BTCUSDT"));
        assert!(debug_output.contains("1640995200000"));
        assert!(debug_output.contains("1641081600000"));
        assert!(debug_output.contains("50"));
    }

    #[test]
    fn test_strike_result_deserialization_realistic_value_stricken() {
        let json = r#""REALISTIC_VALUE_STRICKEN""#;
        let result: StrikeResult = serde_json::from_str(json).unwrap();

        match result {
            StrikeResult::RealisticValueStricken => {}
            _ => panic!("Expected RealisticValueStricken"),
        }
    }

    #[test]
    fn test_strike_result_deserialization_extrinsic_value_expired() {
        let json = r#""EXTRINSIC_VALUE_EXPIRED""#;
        let result: StrikeResult = serde_json::from_str(json).unwrap();

        match result {
            StrikeResult::ExtrinsicValueExpired => {}
            _ => panic!("Expected ExtrinsicValueExpired"),
        }
    }

    #[test]
    fn test_strike_result_clone() {
        let result = StrikeResult::RealisticValueStricken;
        let cloned = result.clone();

        match (result, cloned) {
            (StrikeResult::RealisticValueStricken, StrikeResult::RealisticValueStricken) => {}
            _ => panic!("Clone should maintain the same variant"),
        }
    }

    #[test]
    fn test_strike_result_debug() {
        let result = StrikeResult::RealisticValueStricken;
        let debug_output = format!("{:?}", result);
        assert!(debug_output.contains("RealisticValueStricken"));

        let result = StrikeResult::ExtrinsicValueExpired;
        let debug_output = format!("{:?}", result);
        assert!(debug_output.contains("ExtrinsicValueExpired"));
    }

    #[test]
    fn test_exercise_history_record_deserialization_realistic_value_stricken() {
        let json = r#"{
            "symbol": "BTC-240329-70000-C",
            "strikePrice": "70000.00000000",
            "realStrikePrice": "69500.25000000",
            "expiryDate": 1711641600000,
            "strikeResult": "REALISTIC_VALUE_STRICKEN"
        }"#;

        let record: ExerciseHistoryRecord = serde_json::from_str(json).unwrap();
        assert_eq!(record.symbol, "BTC-240329-70000-C");
        assert_eq!(record.strike_price, dec!(70000.00000000));
        assert_eq!(record.real_strike_price, dec!(69500.25000000));
        assert_eq!(record.expiry_date, 1711641600000);

        match record.strike_result {
            StrikeResult::RealisticValueStricken => {}
            _ => panic!("Expected RealisticValueStricken"),
        }
    }

    #[test]
    fn test_exercise_history_record_deserialization_extrinsic_value_expired() {
        let json = r#"{
            "symbol": "ETH-240329-4000-P",
            "strikePrice": "4000.00000000",
            "realStrikePrice": "4200.50000000",
            "expiryDate": 1711641600000,
            "strikeResult": "EXTRINSIC_VALUE_EXPIRED"
        }"#;

        let record: ExerciseHistoryRecord = serde_json::from_str(json).unwrap();
        assert_eq!(record.symbol, "ETH-240329-4000-P");
        assert_eq!(record.strike_price, dec!(4000.00000000));
        assert_eq!(record.real_strike_price, dec!(4200.50000000));
        assert_eq!(record.expiry_date, 1711641600000);

        match record.strike_result {
            StrikeResult::ExtrinsicValueExpired => {}
            _ => panic!("Expected ExtrinsicValueExpired"),
        }
    }

    #[test]
    fn test_exercise_history_record_deserialization_high_precision() {
        let json = r#"{
            "symbol": "BTC-240329-65000-C",
            "strikePrice": "65000.12345678",
            "realStrikePrice": "64875.87654321",
            "expiryDate": 1711641600000,
            "strikeResult": "REALISTIC_VALUE_STRICKEN"
        }"#;

        let record: ExerciseHistoryRecord = serde_json::from_str(json).unwrap();
        assert_eq!(record.symbol, "BTC-240329-65000-C");
        assert_eq!(record.strike_price.to_string(), "65000.12345678");
        assert_eq!(record.real_strike_price.to_string(), "64875.87654321");
        assert_eq!(record.expiry_date, 1711641600000);
    }

    #[test]
    fn test_exercise_history_record_deserialization_zero_values() {
        let json = r#"{
            "symbol": "BTC-240329-0-C",
            "strikePrice": "0.00000000",
            "realStrikePrice": "0.00000000",
            "expiryDate": 0,
            "strikeResult": "EXTRINSIC_VALUE_EXPIRED"
        }"#;

        let record: ExerciseHistoryRecord = serde_json::from_str(json).unwrap();
        assert_eq!(record.symbol, "BTC-240329-0-C");
        assert_eq!(record.strike_price, dec!(0.00000000));
        assert_eq!(record.real_strike_price, dec!(0.00000000));
        assert_eq!(record.expiry_date, 0);
    }

    #[test]
    fn test_exercise_history_record_deserialization_large_values() {
        let json = r#"{
            "symbol": "BTC-240329-999999-C",
            "strikePrice": "999999.99999999",
            "realStrikePrice": "999999.99999999",
            "expiryDate": 9999999999999,
            "strikeResult": "REALISTIC_VALUE_STRICKEN"
        }"#;

        let record: ExerciseHistoryRecord = serde_json::from_str(json).unwrap();
        assert_eq!(record.symbol, "BTC-240329-999999-C");
        assert_eq!(record.strike_price, dec!(999999.99999999));
        assert_eq!(record.real_strike_price, dec!(999999.99999999));
        assert_eq!(record.expiry_date, 9999999999999);
    }

    #[test]
    fn test_exercise_history_record_deserialization_different_symbols() {
        let symbols = vec![
            "BTC-240329-70000-C",
            "BTC-240329-70000-P",
            "ETH-240329-4000-C",
            "ETH-240329-4000-P",
            "BNB-240329-500-C",
            "BNB-240329-500-P",
        ];

        for symbol in symbols {
            let json = format!(
                r#"{{
                "symbol": "{}",
                "strikePrice": "1000.00000000",
                "realStrikePrice": "1050.00000000",
                "expiryDate": 1711641600000,
                "strikeResult": "REALISTIC_VALUE_STRICKEN"
            }}"#,
                symbol
            );

            let record: ExerciseHistoryRecord = serde_json::from_str(&json).unwrap();
            assert_eq!(record.symbol, symbol);
            assert_eq!(record.strike_price, dec!(1000.00000000));
            assert_eq!(record.real_strike_price, dec!(1050.00000000));
            assert_eq!(record.expiry_date, 1711641600000);
        }
    }

    #[test]
    fn test_exercise_history_record_deserialization_different_expiry_dates() {
        let expiry_dates = vec![
            1711641600000, // March 29, 2024
            1719619200000, // June 29, 2024
            1727596800000, // September 29, 2024
            1735574400000, // December 29, 2024
        ];

        for expiry_date in expiry_dates {
            let json = format!(
                r#"{{
                "symbol": "BTC-240329-70000-C",
                "strikePrice": "70000.00000000",
                "realStrikePrice": "69500.00000000",
                "expiryDate": {},
                "strikeResult": "REALISTIC_VALUE_STRICKEN"
            }}"#,
                expiry_date
            );

            let record: ExerciseHistoryRecord = serde_json::from_str(&json).unwrap();
            assert_eq!(record.expiry_date, expiry_date);
        }
    }

    #[test]
    fn test_exercise_history_record_deserialization_different_strike_prices() {
        let strike_prices = vec![
            "1000.00000000",
            "10000.00000000",
            "50000.00000000",
            "100000.00000000",
            "0.00000001",
        ];

        for strike_price in strike_prices {
            let json = format!(
                r#"{{
                "symbol": "BTC-240329-70000-C",
                "strikePrice": "{}",
                "realStrikePrice": "{}",
                "expiryDate": 1711641600000,
                "strikeResult": "REALISTIC_VALUE_STRICKEN"
            }}"#,
                strike_price, strike_price
            );

            let record: ExerciseHistoryRecord = serde_json::from_str(&json).unwrap();
            assert_eq!(record.strike_price.to_string(), strike_price);
            assert_eq!(record.real_strike_price.to_string(), strike_price);
        }
    }

    #[test]
    fn test_exercise_history_record_deserialization_both_strike_results() {
        let test_cases = vec![
            ("REALISTIC_VALUE_STRICKEN", "BTC-240329-70000-C"),
            ("EXTRINSIC_VALUE_EXPIRED", "ETH-240329-4000-P"),
        ];

        for (strike_result, symbol) in test_cases {
            let json = format!(
                r#"{{
                "symbol": "{}",
                "strikePrice": "50000.00000000",
                "realStrikePrice": "49750.00000000",
                "expiryDate": 1711641600000,
                "strikeResult": "{}"
            }}"#,
                symbol, strike_result
            );

            let record: ExerciseHistoryRecord = serde_json::from_str(&json).unwrap();
            assert_eq!(record.symbol, symbol);

            match strike_result {
                "REALISTIC_VALUE_STRICKEN" => match record.strike_result {
                    StrikeResult::RealisticValueStricken => {}
                    _ => panic!("Expected RealisticValueStricken"),
                },
                "EXTRINSIC_VALUE_EXPIRED" => match record.strike_result {
                    StrikeResult::ExtrinsicValueExpired => {}
                    _ => panic!("Expected ExtrinsicValueExpired"),
                },
                _ => panic!("Unexpected strike result"),
            }
        }
    }

    #[test]
    fn test_exercise_history_record_clone() {
        let json = r#"{
            "symbol": "BTC-240329-70000-C",
            "strikePrice": "70000.00000000",
            "realStrikePrice": "69500.25000000",
            "expiryDate": 1711641600000,
            "strikeResult": "REALISTIC_VALUE_STRICKEN"
        }"#;

        let record: ExerciseHistoryRecord = serde_json::from_str(json).unwrap();
        let cloned = record.clone();

        assert_eq!(record.symbol, cloned.symbol);
        assert_eq!(record.strike_price, cloned.strike_price);
        assert_eq!(record.real_strike_price, cloned.real_strike_price);
        assert_eq!(record.expiry_date, cloned.expiry_date);

        match (record.strike_result, cloned.strike_result) {
            (StrikeResult::RealisticValueStricken, StrikeResult::RealisticValueStricken) => {}
            (StrikeResult::ExtrinsicValueExpired, StrikeResult::ExtrinsicValueExpired) => {}
            _ => panic!("Clone should maintain the same strike result variant"),
        }
    }

    #[test]
    fn test_exercise_history_record_debug() {
        let json = r#"{
            "symbol": "BTC-240329-70000-C",
            "strikePrice": "70000.00000000",
            "realStrikePrice": "69500.25000000",
            "expiryDate": 1711641600000,
            "strikeResult": "REALISTIC_VALUE_STRICKEN"
        }"#;

        let record: ExerciseHistoryRecord = serde_json::from_str(json).unwrap();
        let debug_output = format!("{:?}", record);

        assert!(debug_output.contains("ExerciseHistoryRecord"));
        assert!(debug_output.contains("BTC-240329-70000-C"));
        assert!(debug_output.contains("70000"));
        assert!(debug_output.contains("69500.25"));
        assert!(debug_output.contains("1711641600000"));
        assert!(debug_output.contains("RealisticValueStricken"));
    }

    #[test]
    fn test_exercise_history_record_deserialization_edge_case_symbols() {
        let edge_symbols = vec![
            "BTC-991231-999999-C", // Far expiry, high strike
            "ETH-000101-1-P",      // Low strike
            "BNB-240229-100000-C", // Leap year
            "ADA-240101-0-P",      // Zero strike
        ];

        for symbol in edge_symbols {
            let json = format!(
                r#"{{
                "symbol": "{}",
                "strikePrice": "1000.00000000",
                "realStrikePrice": "1050.00000000",
                "expiryDate": 1711641600000,
                "strikeResult": "REALISTIC_VALUE_STRICKEN"
            }}"#,
                symbol
            );

            let record: ExerciseHistoryRecord = serde_json::from_str(&json).unwrap();
            assert_eq!(record.symbol, symbol);
            assert_eq!(record.strike_price, dec!(1000.00000000));
            assert_eq!(record.real_strike_price, dec!(1050.00000000));
        }
    }

    #[test]
    fn test_exercise_history_record_deserialization_minimal_precision() {
        let json = r#"{
            "symbol": "BTC-240329-70000-C",
            "strikePrice": "0.00000001",
            "realStrikePrice": "0.00000001",
            "expiryDate": 1711641600000,
            "strikeResult": "EXTRINSIC_VALUE_EXPIRED"
        }"#;

        let record: ExerciseHistoryRecord = serde_json::from_str(json).unwrap();
        assert_eq!(record.strike_price, dec!(0.00000001));
        assert_eq!(record.real_strike_price, dec!(0.00000001));

        match record.strike_result {
            StrikeResult::ExtrinsicValueExpired => {}
            _ => panic!("Expected ExtrinsicValueExpired"),
        }
    }

    #[test]
    fn test_exercise_history_record_deserialization_vector() {
        let json = r#"[
            {
                "symbol": "BTC-240329-70000-C",
                "strikePrice": "70000.00000000",
                "realStrikePrice": "69500.25000000",
                "expiryDate": 1711641600000,
                "strikeResult": "REALISTIC_VALUE_STRICKEN"
            },
            {
                "symbol": "ETH-240329-4000-P",
                "strikePrice": "4000.00000000",
                "realStrikePrice": "4200.50000000",
                "expiryDate": 1711641600000,
                "strikeResult": "EXTRINSIC_VALUE_EXPIRED"
            }
        ]"#;

        let records: Vec<ExerciseHistoryRecord> = serde_json::from_str(json).unwrap();
        assert_eq!(records.len(), 2);

        assert_eq!(records[0].symbol, "BTC-240329-70000-C");
        assert_eq!(records[0].strike_price, dec!(70000.00000000));
        match records[0].strike_result {
            StrikeResult::RealisticValueStricken => {}
            _ => panic!("Expected RealisticValueStricken"),
        }

        assert_eq!(records[1].symbol, "ETH-240329-4000-P");
        assert_eq!(records[1].strike_price, dec!(4000.00000000));
        match records[1].strike_result {
            StrikeResult::ExtrinsicValueExpired => {}
            _ => panic!("Expected ExtrinsicValueExpired"),
        }
    }
}
