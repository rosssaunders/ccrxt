use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::gateio::RestResult;

const OPTIONS_UNDERLYINGS_ENDPOINT: &str = "/options/underlyings";

/// Options underlying asset information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionsUnderlying {
    /// Underlying name
    pub name: String,

    /// Spot index price (quote currency)
    pub index_price: String,
}

impl RestClient {
    /// List all underlyings
    ///
    /// Retrieves all available underlying assets for options trading.
    ///
    /// [docs](https://www.gate.io/docs/developers/apiv4/en/#list-all-underlyings)
    pub async fn get_options_underlyings(&self) -> RestResult<Vec<OptionsUnderlying>> {
        self.get(OPTIONS_UNDERLYINGS_ENDPOINT).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_options_underlying_deserialization() {
        let json = r#"{
            "name": "BTC_USDT",
            "index_price": "42000.50"
        }"#;

        let underlying: OptionsUnderlying = serde_json::from_str(json).unwrap();
        assert_eq!(underlying.name, "BTC_USDT");
        assert_eq!(underlying.index_price, "42000.50");
    }

    #[test]
    fn test_options_underlying_eth_deserialization() {
        let json = r#"{
            "name": "ETH_USDT",
            "index_price": "3000.75"
        }"#;

        let underlying: OptionsUnderlying = serde_json::from_str(json).unwrap();
        assert_eq!(underlying.name, "ETH_USDT");
        assert_eq!(underlying.index_price, "3000.75");
    }

    #[test]
    fn test_options_underlying_high_precision_price() {
        let json = r#"{
            "name": "BTC_USDT",
            "index_price": "42123.123456789"
        }"#;

        let underlying: OptionsUnderlying = serde_json::from_str(json).unwrap();
        assert_eq!(underlying.name, "BTC_USDT");
        assert_eq!(underlying.index_price, "42123.123456789");
    }

    #[test]
    fn test_options_underlying_low_price_asset() {
        let json = r#"{
            "name": "ADA_USDT",
            "index_price": "0.45"
        }"#;

        let underlying: OptionsUnderlying = serde_json::from_str(json).unwrap();
        assert_eq!(underlying.name, "ADA_USDT");
        assert_eq!(underlying.index_price, "0.45");
    }

    #[test]
    fn test_options_underlying_very_low_price() {
        let json = r#"{
            "name": "DOGE_USDT",
            "index_price": "0.00123456"
        }"#;

        let underlying: OptionsUnderlying = serde_json::from_str(json).unwrap();
        assert_eq!(underlying.name, "DOGE_USDT");
        assert_eq!(underlying.index_price, "0.00123456");
    }

    #[test]
    fn test_options_underlying_zero_price() {
        let json = r#"{
            "name": "TEST_USDT",
            "index_price": "0"
        }"#;

        let underlying: OptionsUnderlying = serde_json::from_str(json).unwrap();
        assert_eq!(underlying.name, "TEST_USDT");
        assert_eq!(underlying.index_price, "0");
    }

    #[test]
    fn test_options_underlying_zero_decimal_price() {
        let json = r#"{
            "name": "TEST_USDT",
            "index_price": "0.0"
        }"#;

        let underlying: OptionsUnderlying = serde_json::from_str(json).unwrap();
        assert_eq!(underlying.name, "TEST_USDT");
        assert_eq!(underlying.index_price, "0.0");
    }

    #[test]
    fn test_options_underlying_large_price() {
        let json = r#"{
            "name": "EXPENSIVE_USDT",
            "index_price": "999999.999999"
        }"#;

        let underlying: OptionsUnderlying = serde_json::from_str(json).unwrap();
        assert_eq!(underlying.name, "EXPENSIVE_USDT");
        assert_eq!(underlying.index_price, "999999.999999");
    }

    #[test]
    fn test_options_underlying_different_name_formats() {
        let names = vec![
            "BTC_USDT",
            "ETH_USDT",
            "BNB_USDT",
            "SOL_USDT",
            "ADA_USDT",
            "MATIC_USDT",
            "AVAX_USDT",
        ];

        for name in names {
            let json = format!(
                r#"{{
                "name": "{}",
                "index_price": "100.0"
            }}"#,
                name
            );

            let underlying: OptionsUnderlying = serde_json::from_str(&json).unwrap();
            assert_eq!(underlying.name, name);
            assert_eq!(underlying.index_price, "100.0");
        }
    }

    #[test]
    fn test_options_underlying_array_deserialization() {
        let json = r#"[
            {
                "name": "BTC_USDT",
                "index_price": "42000.50"
            },
            {
                "name": "ETH_USDT",
                "index_price": "3000.75"
            },
            {
                "name": "BNB_USDT",
                "index_price": "400.25"
            }
        ]"#;

        let underlyings: Vec<OptionsUnderlying> = serde_json::from_str(json).unwrap();
        assert_eq!(underlyings.len(), 3);

        assert_eq!(underlyings[0].name, "BTC_USDT");
        assert_eq!(underlyings[0].index_price, "42000.50");

        assert_eq!(underlyings[1].name, "ETH_USDT");
        assert_eq!(underlyings[1].index_price, "3000.75");

        assert_eq!(underlyings[2].name, "BNB_USDT");
        assert_eq!(underlyings[2].index_price, "400.25");
    }

    #[test]
    fn test_options_underlying_empty_array_deserialization() {
        let json = r#"[]"#;
        let underlyings: Vec<OptionsUnderlying> = serde_json::from_str(json).unwrap();
        assert_eq!(underlyings.len(), 0);
    }

    #[test]
    fn test_options_underlying_single_item_array() {
        let json = r#"[
            {
                "name": "BTC_USDT",
                "index_price": "42000.50"
            }
        ]"#;

        let underlyings: Vec<OptionsUnderlying> = serde_json::from_str(json).unwrap();
        assert_eq!(underlyings.len(), 1);
        assert_eq!(underlyings[0].name, "BTC_USDT");
        assert_eq!(underlyings[0].index_price, "42000.50");
    }

    #[test]
    fn test_options_underlying_serialization() {
        let underlying = OptionsUnderlying {
            name: "BTC_USDT".to_string(),
            index_price: "42000.50".to_string(),
        };

        let json = serde_json::to_value(&underlying).unwrap();
        assert_eq!(json["name"], "BTC_USDT");
        assert_eq!(json["index_price"], "42000.50");
    }

    #[test]
    fn test_options_underlying_serialization_round_trip() {
        let original = OptionsUnderlying {
            name: "ETH_USDT".to_string(),
            index_price: "3000.123456789".to_string(),
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: OptionsUnderlying = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.name, original.name);
        assert_eq!(deserialized.index_price, original.index_price);
    }

    #[test]
    fn test_options_underlying_realistic_crypto_prices() {
        let crypto_prices = vec![
            ("BTC_USDT", "45000.00"),
            ("ETH_USDT", "2800.50"),
            ("BNB_USDT", "350.75"),
            ("SOL_USDT", "95.25"),
            ("ADA_USDT", "0.45"),
            ("MATIC_USDT", "0.85"),
            ("AVAX_USDT", "25.50"),
            ("DOT_USDT", "6.75"),
            ("LINK_USDT", "14.25"),
            ("UNI_USDT", "7.50"),
        ];

        for (name, price) in crypto_prices {
            let json = format!(
                r#"{{
                "name": "{}",
                "index_price": "{}"
            }}"#,
                name, price
            );

            let underlying: OptionsUnderlying = serde_json::from_str(&json).unwrap();
            assert_eq!(underlying.name, name);
            assert_eq!(underlying.index_price, price);
        }
    }

    #[test]
    fn test_options_underlying_special_price_formats() {
        let price_formats = vec![
            "42000",
            "42000.0",
            "42000.50",
            "42000.123",
            "42000.123456",
            "42000.123456789",
            "0.001",
            "0.0001",
            "0.00000001",
        ];

        for price in price_formats {
            let json = format!(
                r#"{{
                "name": "BTC_USDT",
                "index_price": "{}"
            }}"#,
                price
            );

            let underlying: OptionsUnderlying = serde_json::from_str(&json).unwrap();
            assert_eq!(underlying.name, "BTC_USDT");
            assert_eq!(underlying.index_price, price);
        }
    }

    #[test]
    fn test_options_underlying_name_variations() {
        let name_variations = vec![
            "BTC_USDT", "btc_usdt", "BTC_usdt", "btc_USDT", "BTC-USDT", "BTC/USDT", "BTCUSDT",
        ];

        for name in name_variations {
            let json = format!(
                r#"{{
                "name": "{}",
                "index_price": "42000.50"
            }}"#,
                name
            );

            let underlying: OptionsUnderlying = serde_json::from_str(&json).unwrap();
            assert_eq!(underlying.name, name);
            assert_eq!(underlying.index_price, "42000.50");
        }
    }

    #[test]
    fn test_options_underlying_long_asset_names() {
        let json = r#"{
            "name": "VERY_LONG_ASSET_NAME_USDT",
            "index_price": "1.0"
        }"#;

        let underlying: OptionsUnderlying = serde_json::from_str(json).unwrap();
        assert_eq!(underlying.name, "VERY_LONG_ASSET_NAME_USDT");
        assert_eq!(underlying.index_price, "1.0");
    }

    #[test]
    fn test_options_underlying_numeric_asset_names() {
        let json = r#"{
            "name": "1INCH_USDT",
            "index_price": "0.45"
        }"#;

        let underlying: OptionsUnderlying = serde_json::from_str(json).unwrap();
        assert_eq!(underlying.name, "1INCH_USDT");
        assert_eq!(underlying.index_price, "0.45");
    }

    #[test]
    fn test_options_underlying_comprehensive_market_data() {
        let market_data = vec![
            ("BTC_USDT", "43250.75"),
            ("ETH_USDT", "2856.25"),
            ("BNB_USDT", "312.50"),
            ("SOL_USDT", "98.75"),
            ("ADA_USDT", "0.485"),
            ("MATIC_USDT", "0.912"),
            ("AVAX_USDT", "26.85"),
            ("DOT_USDT", "6.925"),
            ("LINK_USDT", "14.675"),
            ("UNI_USDT", "7.825"),
        ];

        for (name, price) in market_data {
            let json = format!(
                r#"{{
                "name": "{}",
                "index_price": "{}"
            }}"#,
                name, price
            );

            let underlying: OptionsUnderlying = serde_json::from_str(&json).unwrap();
            assert_eq!(underlying.name, name);
            assert_eq!(underlying.index_price, price);

            // Verify that the price string can be parsed as a valid number
            let price_value: f64 = price.parse().unwrap();
            assert!(price_value >= 0.0);
        }
    }
}
