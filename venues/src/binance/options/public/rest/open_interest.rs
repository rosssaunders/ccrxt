use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::binance::options::RestResult;

const OPEN_INTEREST_ENDPOINT: &str = "/eapi/v1/openInterest";

/// Request parameters for open interest
#[derive(Debug, Clone, Serialize)]
pub struct OpenInterestRequest {
    /// Underlying asset, e.g ETH/BTC
    #[serde(rename = "underlyingAsset")]
    pub underlying_asset: String,

    /// Expiration date, e.g 221225
    #[serde(rename = "expiration")]
    pub expiration: String,
}

/// Open interest information
#[derive(Debug, Clone, Deserialize)]
pub struct OpenInterestResponse {
    /// Symbol
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Sum of open interest
    #[serde(rename = "sumOpenInterest")]
    pub sum_open_interest: Decimal,

    /// Sum of open interest in USD
    #[serde(rename = "sumOpenInterestUsd")]
    pub sum_open_interest_usd: Decimal,

    /// Timestamp
    #[serde(rename = "timestamp")]
    pub timestamp: String,
}

impl RestClient {
    /// Get open interest
    ///
    /// Returns open interest for specific underlying asset on specific expiration date.
    ///
    /// [docs]: (https://developers.binance.com/docs/derivatives/option/market-data/Open-Interest)
    /// Method: GET /eapi/v1/openInterest
    /// Weight: 0
    /// Security: None
    pub async fn get_open_interest(
        &self,
        params: OpenInterestRequest,
    ) -> RestResult<Vec<OpenInterestResponse>> {
        self.send_public_request(
            OPEN_INTEREST_ENDPOINT,
            reqwest::Method::GET,
            Some(params),
            0,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    use super::*;

    #[test]
    fn test_open_interest_request_serialization_btc() {
        let request = OpenInterestRequest {
            underlying_asset: "BTC".to_string(),
            expiration: "240329".to_string(),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("underlyingAsset=BTC"));
        assert!(serialized.contains("expiration=240329"));
    }

    #[test]
    fn test_open_interest_request_serialization_eth() {
        let request = OpenInterestRequest {
            underlying_asset: "ETH".to_string(),
            expiration: "240426".to_string(),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("underlyingAsset=ETH"));
        assert!(serialized.contains("expiration=240426"));
    }

    #[test]
    fn test_open_interest_request_serialization_different_underlying_assets() {
        let assets = vec!["BTC", "ETH", "BNB", "SOL", "DOGE"];
        let expiration = "240329";

        for asset in assets {
            let request = OpenInterestRequest {
                underlying_asset: asset.to_string(),
                expiration: expiration.to_string(),
            };

            let serialized = serde_urlencoded::to_string(&request).unwrap();
            assert!(serialized.contains(&format!("underlyingAsset={}", asset)));
            assert!(serialized.contains(&format!("expiration={}", expiration)));
        }
    }

    #[test]
    fn test_open_interest_request_serialization_different_expirations() {
        let asset = "BTC";
        let expirations = vec!["240329", "240426", "240531", "240628", "240726"];

        for expiration in expirations {
            let request = OpenInterestRequest {
                underlying_asset: asset.to_string(),
                expiration: expiration.to_string(),
            };

            let serialized = serde_urlencoded::to_string(&request).unwrap();
            assert!(serialized.contains(&format!("underlyingAsset={}", asset)));
            assert!(serialized.contains(&format!("expiration={}", expiration)));
        }
    }

    #[test]
    fn test_open_interest_request_serialization_quarterly_expiration() {
        let request = OpenInterestRequest {
            underlying_asset: "BTC".to_string(),
            expiration: "240329".to_string(), // Quarterly expiration
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "underlyingAsset=BTC&expiration=240329");
    }

    #[test]
    fn test_open_interest_request_serialization_monthly_expiration() {
        let request = OpenInterestRequest {
            underlying_asset: "ETH".to_string(),
            expiration: "240426".to_string(), // Monthly expiration
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "underlyingAsset=ETH&expiration=240426");
    }

    #[test]
    fn test_open_interest_request_serialization_field_order() {
        let request = OpenInterestRequest {
            underlying_asset: "BTC".to_string(),
            expiration: "240329".to_string(),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        // serde_urlencoded should maintain field order as defined in struct
        assert_eq!(serialized, "underlyingAsset=BTC&expiration=240329");
    }

    #[test]
    fn test_open_interest_response_deserialization_btc() {
        let json = r#"{
            "symbol": "BTC-240329-70000-C",
            "sumOpenInterest": "123.45",
            "sumOpenInterestUsd": "8642100.00",
            "timestamp": "1625097600000"
        }"#;

        let response: OpenInterestResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.symbol, "BTC-240329-70000-C");
        assert_eq!(response.sum_open_interest, dec!(123.45));
        assert_eq!(response.sum_open_interest_usd, dec!(8642100.00));
        assert_eq!(response.timestamp, "1625097600000");
    }

    #[test]
    fn test_open_interest_response_deserialization_eth() {
        let json = r#"{
            "symbol": "ETH-240329-3000-P",
            "sumOpenInterest": "456.78",
            "sumOpenInterestUsd": "1370340.00",
            "timestamp": "1625097600000"
        }"#;

        let response: OpenInterestResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.symbol, "ETH-240329-3000-P");
        assert_eq!(response.sum_open_interest, dec!(456.78));
        assert_eq!(response.sum_open_interest_usd, dec!(1370340.00));
        assert_eq!(response.timestamp, "1625097600000");
    }

    #[test]
    fn test_open_interest_response_deserialization_high_precision() {
        let json = r#"{
            "symbol": "BTC-240329-65000-C",
            "sumOpenInterest": "123.45678901",
            "sumOpenInterestUsd": "8642100.12345678",
            "timestamp": "1625097600000"
        }"#;

        let response: OpenInterestResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.symbol, "BTC-240329-65000-C");
        assert_eq!(response.sum_open_interest.to_string(), "123.45678901");
        assert_eq!(
            response.sum_open_interest_usd.to_string(),
            "8642100.12345678"
        );
        assert_eq!(response.timestamp, "1625097600000");
    }

    #[test]
    fn test_open_interest_response_deserialization_zero_values() {
        let json = r#"{
            "symbol": "BTC-240329-70000-C",
            "sumOpenInterest": "0.00000000",
            "sumOpenInterestUsd": "0.00000000",
            "timestamp": "1625097600000"
        }"#;

        let response: OpenInterestResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.symbol, "BTC-240329-70000-C");
        assert_eq!(response.sum_open_interest, dec!(0.00000000));
        assert_eq!(response.sum_open_interest_usd, dec!(0.00000000));
        assert_eq!(response.timestamp, "1625097600000");
    }

    #[test]
    fn test_open_interest_response_deserialization_large_values() {
        let json = r#"{
            "symbol": "BTC-240329-50000-C",
            "sumOpenInterest": "999999.99999999",
            "sumOpenInterestUsd": "49999999999.99999999",
            "timestamp": "1625097600000"
        }"#;

        let response: OpenInterestResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.symbol, "BTC-240329-50000-C");
        assert_eq!(response.sum_open_interest.to_string(), "999999.99999999");
        assert_eq!(
            response.sum_open_interest_usd.to_string(),
            "49999999999.99999999"
        );
        assert_eq!(response.timestamp, "1625097600000");
    }

    #[test]
    fn test_open_interest_response_deserialization_call_option() {
        let json = r#"{
            "symbol": "BTC-240329-70000-C",
            "sumOpenInterest": "123.45",
            "sumOpenInterestUsd": "8642100.00",
            "timestamp": "1625097600000"
        }"#;

        let response: OpenInterestResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.symbol, "BTC-240329-70000-C");
        assert!(response.symbol.ends_with("-C")); // Call option
        assert_eq!(response.sum_open_interest, dec!(123.45));
        assert_eq!(response.sum_open_interest_usd, dec!(8642100.00));
    }

    #[test]
    fn test_open_interest_response_deserialization_put_option() {
        let json = r#"{
            "symbol": "BTC-240329-70000-P",
            "sumOpenInterest": "456.78",
            "sumOpenInterestUsd": "32003460.00",
            "timestamp": "1625097600000"
        }"#;

        let response: OpenInterestResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.symbol, "BTC-240329-70000-P");
        assert!(response.symbol.ends_with("-P")); // Put option
        assert_eq!(response.sum_open_interest, dec!(456.78));
        assert_eq!(response.sum_open_interest_usd, dec!(32003460.00));
    }

    #[test]
    fn test_open_interest_response_deserialization_different_strikes() {
        let strikes = vec!["50000", "60000", "70000", "80000"];
        let base_timestamp = "1625097600000";

        for strike in strikes {
            let json = format!(
                r#"{{
                    "symbol": "BTC-240329-{}-C",
                    "sumOpenInterest": "100.00",
                    "sumOpenInterestUsd": "7000000.00",
                    "timestamp": "{}"
                }}"#,
                strike, base_timestamp
            );

            let response: OpenInterestResponse = serde_json::from_str(&json).unwrap();
            assert_eq!(response.symbol, format!("BTC-240329-{}-C", strike));
            assert_eq!(response.sum_open_interest, dec!(100.00));
            assert_eq!(response.sum_open_interest_usd, dec!(7000000.00));
            assert_eq!(response.timestamp, base_timestamp);
        }
    }

    #[test]
    fn test_open_interest_response_deserialization_different_expirations() {
        let expirations = vec!["240329", "240426", "240531", "240628"];
        let base_timestamp = "1625097600000";

        for expiration in expirations {
            let json = format!(
                r#"{{
                    "symbol": "BTC-{}-70000-C",
                    "sumOpenInterest": "100.00",
                    "sumOpenInterestUsd": "7000000.00",
                    "timestamp": "{}"
                }}"#,
                expiration, base_timestamp
            );

            let response: OpenInterestResponse = serde_json::from_str(&json).unwrap();
            assert_eq!(response.symbol, format!("BTC-{}-70000-C", expiration));
            assert_eq!(response.sum_open_interest, dec!(100.00));
            assert_eq!(response.sum_open_interest_usd, dec!(7000000.00));
            assert_eq!(response.timestamp, base_timestamp);
        }
    }

    #[test]
    fn test_open_interest_response_deserialization_different_underlying_assets() {
        let assets = vec!["BTC", "ETH", "BNB", "SOL"];
        let base_timestamp = "1625097600000";

        for asset in assets {
            let json = format!(
                r#"{{
                    "symbol": "{}-240329-70000-C",
                    "sumOpenInterest": "100.00",
                    "sumOpenInterestUsd": "7000000.00",
                    "timestamp": "{}"
                }}"#,
                asset, base_timestamp
            );

            let response: OpenInterestResponse = serde_json::from_str(&json).unwrap();
            assert_eq!(response.symbol, format!("{}-240329-70000-C", asset));
            assert_eq!(response.sum_open_interest, dec!(100.00));
            assert_eq!(response.sum_open_interest_usd, dec!(7000000.00));
            assert_eq!(response.timestamp, base_timestamp);
        }
    }

    #[test]
    fn test_open_interest_response_array_deserialization() {
        let json = r#"[
            {
                "symbol": "BTC-240329-70000-C",
                "sumOpenInterest": "123.45",
                "sumOpenInterestUsd": "8642100.00",
                "timestamp": "1625097600000"
            },
            {
                "symbol": "BTC-240329-70000-P",
                "sumOpenInterest": "456.78",
                "sumOpenInterestUsd": "32003460.00",
                "timestamp": "1625097600000"
            }
        ]"#;

        let responses: Vec<OpenInterestResponse> = serde_json::from_str(json).unwrap();
        assert_eq!(responses.len(), 2);

        // First response (Call option)
        assert_eq!(responses[0].symbol, "BTC-240329-70000-C");
        assert_eq!(responses[0].sum_open_interest, dec!(123.45));
        assert_eq!(responses[0].sum_open_interest_usd, dec!(8642100.00));

        // Second response (Put option)
        assert_eq!(responses[1].symbol, "BTC-240329-70000-P");
        assert_eq!(responses[1].sum_open_interest, dec!(456.78));
        assert_eq!(responses[1].sum_open_interest_usd, dec!(32003460.00));
    }

    #[test]
    fn test_open_interest_response_empty_array_deserialization() {
        let json = r#"[]"#;
        let responses: Vec<OpenInterestResponse> = serde_json::from_str(json).unwrap();
        assert_eq!(responses.len(), 0);
    }

    #[test]
    fn test_open_interest_response_deserialization_timestamp_formats() {
        let timestamps = vec![
            "1625097600000", // Standard timestamp
            "1625097600001", // Millisecond precision
            "1640995200000", // Different timestamp
        ];

        for timestamp in timestamps {
            let json = format!(
                r#"{{
                    "symbol": "BTC-240329-70000-C",
                    "sumOpenInterest": "100.00",
                    "sumOpenInterestUsd": "7000000.00",
                    "timestamp": "{}"
                }}"#,
                timestamp
            );

            let response: OpenInterestResponse = serde_json::from_str(&json).unwrap();
            assert_eq!(response.timestamp, timestamp);
        }
    }

    #[test]
    fn test_open_interest_response_deserialization_mixed_underlying_assets() {
        let json = r#"[
            {
                "symbol": "BTC-240329-70000-C",
                "sumOpenInterest": "100.00",
                "sumOpenInterestUsd": "7000000.00",
                "timestamp": "1625097600000"
            },
            {
                "symbol": "ETH-240329-3000-P",
                "sumOpenInterest": "200.00",
                "sumOpenInterestUsd": "600000.00",
                "timestamp": "1625097600000"
            },
            {
                "symbol": "BNB-240329-400-C",
                "sumOpenInterest": "300.00",
                "sumOpenInterestUsd": "120000.00",
                "timestamp": "1625097600000"
            }
        ]"#;

        let responses: Vec<OpenInterestResponse> = serde_json::from_str(json).unwrap();
        assert_eq!(responses.len(), 3);

        // BTC response
        assert_eq!(responses[0].symbol, "BTC-240329-70000-C");
        assert_eq!(responses[0].sum_open_interest, dec!(100.00));
        assert_eq!(responses[0].sum_open_interest_usd, dec!(7000000.00));

        // ETH response
        assert_eq!(responses[1].symbol, "ETH-240329-3000-P");
        assert_eq!(responses[1].sum_open_interest, dec!(200.00));
        assert_eq!(responses[1].sum_open_interest_usd, dec!(600000.00));

        // BNB response
        assert_eq!(responses[2].symbol, "BNB-240329-400-C");
        assert_eq!(responses[2].sum_open_interest, dec!(300.00));
        assert_eq!(responses[2].sum_open_interest_usd, dec!(120000.00));
    }

    #[test]
    fn test_open_interest_response_deserialization_very_small_values() {
        let json = r#"{
            "symbol": "BTC-240329-70000-C",
            "sumOpenInterest": "0.00000001",
            "sumOpenInterestUsd": "0.0007",
            "timestamp": "1625097600000"
        }"#;

        let response: OpenInterestResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.symbol, "BTC-240329-70000-C");
        assert_eq!(response.sum_open_interest.to_string(), "0.00000001");
        assert_eq!(response.sum_open_interest_usd.to_string(), "0.0007");
        assert_eq!(response.timestamp, "1625097600000");
    }

    #[test]
    fn test_open_interest_response_deserialization_fractional_contracts() {
        let json = r#"{
            "symbol": "BTC-240329-70000-C",
            "sumOpenInterest": "123.456789",
            "sumOpenInterestUsd": "8642100.123456",
            "timestamp": "1625097600000"
        }"#;

        let response: OpenInterestResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.symbol, "BTC-240329-70000-C");
        assert_eq!(response.sum_open_interest.to_string(), "123.456789");
        assert_eq!(response.sum_open_interest_usd.to_string(), "8642100.123456");
        assert_eq!(response.timestamp, "1625097600000");
    }

    #[test]
    fn test_open_interest_response_deserialization_consistency() {
        let json = r#"{
            "symbol": "BTC-240329-70000-C",
            "sumOpenInterest": "100.00",
            "sumOpenInterestUsd": "7000000.00",
            "timestamp": "1625097600000"
        }"#;

        let response: OpenInterestResponse = serde_json::from_str(json).unwrap();

        // Verify that the USD value is consistent with the open interest
        // For a $70,000 strike price, 100 contracts should be worth $7,000,000
        let expected_usd = response.sum_open_interest * dec!(70000.00);
        assert_eq!(response.sum_open_interest_usd, expected_usd);
    }

    #[test]
    fn test_open_interest_request_serialization_url_encoding() {
        let request = OpenInterestRequest {
            underlying_asset: "BTC".to_string(),
            expiration: "240329".to_string(),
        };

        let query_string = serde_urlencoded::to_string(&request).unwrap();
        assert!(!query_string.contains(" ")); // No spaces in URL encoding
        assert!(!query_string.contains("+")); // No plus signs in this case
        assert_eq!(query_string, "underlyingAsset=BTC&expiration=240329");
    }
}
