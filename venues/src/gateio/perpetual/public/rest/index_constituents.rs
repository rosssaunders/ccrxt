use serde::{Deserialize, Serialize};

use super::RestClient;

/// Request parameters for futures index constituents
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FuturesIndexConstituentsRequest {
    /// Settlement currency
    pub settle: String,
    /// Index name
    pub index: String,
}

/// Index constituent information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexConstituent {
    /// Exchange name
    pub exchange: String,

    /// Trading pair
    pub symbol: Option<String>,

    /// Weight percentage
    pub weight: Option<String>,

    /// Price
    pub price: Option<String>,

    /// Last update time
    pub update_time: Option<i64>,
}

/// Index constituents response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexConstituentsResponse {
    /// Index name
    pub index: String,

    /// List of constituents
    pub constituents: Vec<IndexConstituent>,
}

impl RestClient {
    /// Get index constituents
    ///
    /// Retrieves the constituent exchanges and their weights for a specific index.
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#get-index-constituents>
    pub async fn get_futures_index_constituents(
        &self,
        params: FuturesIndexConstituentsRequest,
    ) -> crate::gateio::perpetual::Result<IndexConstituentsResponse> {
        let endpoint = format!(
            "/futures/{}/index_constituents/{}",
            params.settle, params.index
        );
        self.get(&endpoint).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_futures_index_constituents_request() {
        let request = FuturesIndexConstituentsRequest {
            settle: "USDT".to_string(),
            index: "BTCUSD".to_string(),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["settle"], "USDT");
        assert_eq!(json["index"], "BTCUSD");

        let obj = json.as_object().unwrap();
        assert_eq!(obj.len(), 2);
    }

    #[test]
    fn test_different_settlement_currencies() {
        let currencies = vec!["USDT", "BTC", "USD"];

        for settle in currencies {
            let request = FuturesIndexConstituentsRequest {
                settle: settle.to_string(),
                index: "BTCUSD".to_string(),
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["settle"], settle);
        }
    }

    #[test]
    fn test_different_index_names() {
        let indices = vec![
            "BTCUSD", "ETHUSD", "ADAUSD", "SOLUSD", "MATICUSD", "DOTUSD", "AVAXUSD", "LINKUSD",
        ];

        for index in indices {
            let request = FuturesIndexConstituentsRequest {
                settle: "USDT".to_string(),
                index: index.to_string(),
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["index"], index);
        }
    }

    #[test]
    fn test_index_constituent_deserialization_full() {
        let json = r#"{
            "exchange": "Binance",
            "symbol": "BTCUSDT",
            "weight": "0.35",
            "price": "43251.25",
            "update_time": 1640995200
        }"#;

        let constituent: IndexConstituent = serde_json::from_str(json).unwrap();
        assert_eq!(constituent.exchange, "Binance");
        assert_eq!(constituent.symbol.unwrap(), "BTCUSDT");
        assert_eq!(constituent.weight.unwrap(), "0.35");
        assert_eq!(constituent.price.unwrap(), "43251.25");
        assert_eq!(constituent.update_time.unwrap(), 1640995200);
    }

    #[test]
    fn test_index_constituent_deserialization_minimal() {
        let json = r#"{
            "exchange": "Kraken"
        }"#;

        let constituent: IndexConstituent = serde_json::from_str(json).unwrap();
        assert_eq!(constituent.exchange, "Kraken");
        assert!(constituent.symbol.is_none());
        assert!(constituent.weight.is_none());
        assert!(constituent.price.is_none());
        assert!(constituent.update_time.is_none());
    }

    #[test]
    fn test_btc_index_constituents_scenario() {
        let btc_constituents = vec![
            ("Binance", "BTCUSDT", "0.35", "43251.25"),
            ("Coinbase", "BTC-USD", "0.25", "43250.80"),
            ("Kraken", "XBTUSD", "0.20", "43252.10"),
            ("Bitstamp", "BTCUSD", "0.15", "43251.50"),
            ("Gemini", "BTCUSD", "0.05", "43251.75"),
        ];

        let mut total_weight = 0.0;
        for (exchange, symbol, weight, price) in btc_constituents {
            let json = format!(
                r#"{{
                "exchange": "{}",
                "symbol": "{}",
                "weight": "{}",
                "price": "{}",
                "update_time": 1640995200
            }}"#,
                exchange, symbol, weight, price
            );

            let constituent: IndexConstituent = serde_json::from_str(&json).unwrap();
            assert_eq!(constituent.exchange, exchange);
            assert_eq!(constituent.symbol.unwrap(), symbol);

            // Extract weight and price values before assertions
            let weight_str = constituent.weight.unwrap();
            let price_str = constituent.price.unwrap();

            assert_eq!(&weight_str, weight);
            assert_eq!(&price_str, price);

            // Verify weight is reasonable
            let weight_val: f64 = weight_str.parse().unwrap();
            assert!(weight_val > 0.0 && weight_val <= 1.0);
            total_weight += weight_val;

            // Verify price is reasonable for BTC
            let price_val: f64 = price_str.parse().unwrap();
            assert!(price_val > 40000.0 && price_val < 50000.0);
        }

        // Total weights should sum to approximately 1.0
        assert!((total_weight - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_eth_index_constituents_scenario() {
        let eth_constituents = vec![
            ("Binance", "ETHUSDT", "0.30", "2650.75"),
            ("Coinbase", "ETH-USD", "0.25", "2651.25"),
            ("Kraken", "ETHUSD", "0.20", "2650.50"),
            ("Bitstamp", "ETHUSD", "0.15", "2651.00"),
            ("Gemini", "ETHUSD", "0.10", "2650.85"),
        ];

        for (exchange, symbol, weight, price) in eth_constituents {
            let json = format!(
                r#"{{
                "exchange": "{}",
                "symbol": "{}",
                "weight": "{}",
                "price": "{}",
                "update_time": 1640995300
            }}"#,
                exchange, symbol, weight, price
            );

            let constituent: IndexConstituent = serde_json::from_str(&json).unwrap();
            assert_eq!(constituent.exchange, exchange);
            assert_eq!(constituent.symbol.unwrap(), symbol);

            // Extract values before using them
            let weight_str = constituent.weight.unwrap();
            let price_str = constituent.price.unwrap();

            assert_eq!(&weight_str, weight);
            assert_eq!(&price_str, price);

            // Verify ETH price is reasonable
            let price_val: f64 = price_str.parse().unwrap();
            assert!(price_val > 2000.0 && price_val < 3000.0);
        }
    }

    #[test]
    fn test_major_exchanges() {
        let major_exchanges = vec![
            "Binance", "Coinbase", "Kraken", "Bitstamp", "Gemini", "OKX", "Huobi", "FTX",
            "Bitfinex", "KuCoin",
        ];

        for exchange in major_exchanges {
            let json = format!(
                r#"{{
                "exchange": "{}",
                "symbol": "BTCUSDT",
                "weight": "0.1",
                "price": "43250.0",
                "update_time": 1640995200
            }}"#,
                exchange
            );

            let constituent: IndexConstituent = serde_json::from_str(&json).unwrap();
            assert_eq!(constituent.exchange, exchange);
        }
    }

    #[test]
    fn test_trading_pair_variations() {
        let trading_pairs = vec![
            ("Binance", "BTCUSDT"),
            ("Coinbase", "BTC-USD"),
            ("Kraken", "XBTUSD"),
            ("Bitstamp", "BTCUSD"),
            ("Bitfinex", "BTCUSD"),
            ("OKX", "BTC-USDT"),
        ];

        for (exchange, symbol) in trading_pairs {
            let json = format!(
                r#"{{
                "exchange": "{}",
                "symbol": "{}",
                "weight": "0.2",
                "price": "43250.0",
                "update_time": 1640995200
            }}"#,
                exchange, symbol
            );

            let constituent: IndexConstituent = serde_json::from_str(&json).unwrap();
            assert_eq!(constituent.exchange, exchange);
            assert_eq!(constituent.symbol.unwrap(), symbol);
        }
    }

    #[test]
    fn test_weight_distribution_scenarios() {
        let weight_scenarios = vec![
            ("0.50", "Dominant exchange"),
            ("0.25", "Major exchange"),
            ("0.15", "Significant exchange"),
            ("0.05", "Minor exchange"),
            ("0.02", "Small exchange"),
            ("0.01", "Minimal weight"),
        ];

        for (weight, description) in weight_scenarios {
            let json = format!(
                r#"{{
                "exchange": "Binance",
                "symbol": "BTCUSDT",
                "weight": "{}",
                "price": "43250.0",
                "update_time": 1640995200
            }}"#,
                weight
            );

            let constituent: IndexConstituent = serde_json::from_str(&json).unwrap();
            let weight_str = constituent.weight.unwrap();
            assert_eq!(&weight_str, weight);

            let weight_val: f64 = weight_str.parse().unwrap();
            assert!(weight_val > 0.0 && weight_val <= 1.0);

            if description.contains("Dominant") {
                assert!(weight_val >= 0.4);
            } else if description.contains("Minimal") {
                assert!(weight_val <= 0.02);
            }
        }
    }

    #[test]
    fn test_price_precision_scenarios() {
        let price_scenarios = vec![
            ("43250", "Whole number"),
            ("43250.5", "One decimal"),
            ("43250.25", "Two decimals"),
            ("43250.123", "Three decimals"),
            ("43250.12345", "Five decimals"),
            ("43250.123456789", "High precision"),
        ];

        for (price, _description) in price_scenarios {
            let json = format!(
                r#"{{
                "exchange": "Binance",
                "symbol": "BTCUSDT",
                "weight": "0.3",
                "price": "{}",
                "update_time": 1640995200
            }}"#,
                price
            );

            let constituent: IndexConstituent = serde_json::from_str(&json).unwrap();
            let price_str = constituent.price.unwrap();
            assert_eq!(&price_str, price);

            // Should be parseable as a number
            let price_val: f64 = price_str.parse().unwrap();
            assert!(price_val > 0.0);
        }
    }

    #[test]
    fn test_timestamp_scenarios() {
        let timestamps = vec![
            (1640995200, "Recent timestamp"),
            (1577836800, "Year 2020"),
            (1735689600, "Future timestamp"),
            (1609459200, "Year 2021"),
        ];

        for (timestamp, _description) in timestamps {
            let json = format!(
                r#"{{
                "exchange": "Binance",
                "symbol": "BTCUSDT",
                "weight": "0.3",
                "price": "43250.0",
                "update_time": {}
            }}"#,
                timestamp
            );

            let constituent: IndexConstituent = serde_json::from_str(&json).unwrap();
            assert_eq!(constituent.update_time.unwrap(), timestamp);
        }
    }

    #[test]
    fn test_index_constituents_response() {
        let json = r#"{
            "index": "BTCUSD",
            "constituents": [
                {
                    "exchange": "Binance",
                    "symbol": "BTCUSDT",
                    "weight": "0.35",
                    "price": "43251.25",
                    "update_time": 1640995200
                },
                {
                    "exchange": "Coinbase",
                    "symbol": "BTC-USD",
                    "weight": "0.25",
                    "price": "43250.80",
                    "update_time": 1640995200
                },
                {
                    "exchange": "Kraken",
                    "symbol": "XBTUSD",
                    "weight": "0.20",
                    "price": "43252.10",
                    "update_time": 1640995200
                }
            ]
        }"#;

        let response: IndexConstituentsResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.index, "BTCUSD");
        assert_eq!(response.constituents.len(), 3);

        // Verify first constituent
        let first = &response.constituents[0];
        assert_eq!(first.exchange, "Binance");
        assert_eq!(first.symbol.as_ref().unwrap(), "BTCUSDT");
        assert_eq!(first.weight.as_ref().unwrap(), "0.35");
    }

    #[test]
    fn test_altcoin_index_constituents() {
        let altcoin_scenarios = vec![
            (
                "ADAUSD",
                vec![
                    ("Binance", "ADAUSDT", "0.4", "0.485"),
                    ("Coinbase", "ADA-USD", "0.3", "0.486"),
                    ("Kraken", "ADAUSD", "0.3", "0.484"),
                ],
            ),
            (
                "SOLUSD",
                vec![
                    ("Binance", "SOLUSDT", "0.5", "98.25"),
                    ("Coinbase", "SOL-USD", "0.3", "98.30"),
                    ("FTX", "SOL/USD", "0.2", "98.20"),
                ],
            ),
        ];

        for (_index_name, constituents) in altcoin_scenarios {
            let mut constituent_objects = Vec::new();

            for (exchange, symbol, weight, price) in constituents {
                let constituent_json = format!(
                    r#"{{
                    "exchange": "{}",
                    "symbol": "{}",
                    "weight": "{}",
                    "price": "{}",
                    "update_time": 1640995200
                }}"#,
                    exchange, symbol, weight, price
                );

                let constituent: IndexConstituent =
                    serde_json::from_str(&constituent_json).unwrap();
                assert_eq!(constituent.exchange, exchange);
                constituent_objects.push(constituent);
            }

            // Verify we have valid constituents
            assert!(!constituent_objects.is_empty());
        }
    }

    #[test]
    fn test_weight_validation_edge_cases() {
        let edge_weights = vec![
            ("-0.1", false, "Negative weight"),
            ("0.0", true, "Zero weight"),
            ("0.000001", true, "Minimal weight"),
            ("0.999999", true, "Maximum weight"),
            ("1.0", true, "Full weight"),
            ("1.1", false, "Over 100%"),
        ];

        for (weight, should_be_valid, _description) in edge_weights {
            let json = format!(
                r#"{{
                "exchange": "Binance",
                "symbol": "BTCUSDT",
                "weight": "{}",
                "price": "43250.0",
                "update_time": 1640995200
            }}"#,
                weight
            );

            let constituent: IndexConstituent = serde_json::from_str(&json).unwrap();
            let weight_val: f64 = constituent.weight.unwrap().parse().unwrap();

            if should_be_valid {
                assert!(weight_val >= 0.0 && weight_val <= 1.0);
            } else {
                // Weight is outside valid range
                assert!(weight_val < 0.0 || weight_val > 1.0);
            }
        }
    }

    #[test]
    fn test_missing_optional_fields() {
        let field_combinations = vec![
            (r#"{"exchange": "Binance"}"#, "Only exchange"),
            (
                r#"{"exchange": "Binance", "symbol": "BTCUSDT"}"#,
                "Exchange + symbol",
            ),
            (
                r#"{"exchange": "Binance", "weight": "0.3"}"#,
                "Exchange + weight",
            ),
            (
                r#"{"exchange": "Binance", "price": "43250.0"}"#,
                "Exchange + price",
            ),
        ];

        for (json, _description) in field_combinations {
            let constituent: IndexConstituent = serde_json::from_str(json).unwrap();
            assert!(!constituent.exchange.is_empty());
            // Other fields should be None if not provided
        }
    }

    #[test]
    fn test_index_name_variations() {
        let index_names = vec![
            ("BTCUSD", "Bitcoin USD"),
            ("ETHUSD", "Ethereum USD"),
            ("ADAUSD", "Cardano USD"),
            ("SOLUSD", "Solana USD"),
            ("MATICUSD", "Polygon USD"),
            ("DOTUSD", "Polkadot USD"),
            ("AVAXUSD", "Avalanche USD"),
            ("LINKUSD", "Chainlink USD"),
        ];

        for (index, _description) in index_names {
            let request = FuturesIndexConstituentsRequest {
                settle: "USDT".to_string(),
                index: index.to_string(),
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["index"], index);

            // Verify index follows pattern
            assert!(index.ends_with("USD"));
            assert!(index.len() >= 6); // Minimum 3 chars + USD
        }
    }

    #[test]
    fn test_cross_exchange_price_analysis() {
        let cross_exchange_data = vec![
            ("Binance", "43251.25", "0.35"),
            ("Coinbase", "43250.80", "0.25"), // Slightly lower
            ("Kraken", "43252.10", "0.20"),   // Slightly higher
            ("Bitstamp", "43251.50", "0.15"),
            ("Gemini", "43251.75", "0.05"),
        ];

        let mut prices = Vec::new();
        let mut weights = Vec::new();

        for (exchange, price, weight) in cross_exchange_data {
            let json = format!(
                r#"{{
                "exchange": "{}",
                "symbol": "BTCUSDT",
                "weight": "{}",
                "price": "{}",
                "update_time": 1640995200
            }}"#,
                exchange, weight, price
            );

            let constituent: IndexConstituent = serde_json::from_str(&json).unwrap();

            let price_val: f64 = constituent.price.unwrap().parse().unwrap();
            let weight_val: f64 = constituent.weight.unwrap().parse().unwrap();

            prices.push(price_val);
            weights.push(weight_val);
        }

        // Calculate weighted average price
        let mut weighted_sum = 0.0;
        let mut total_weight = 0.0;

        for (price, weight) in prices.iter().zip(weights.iter()) {
            weighted_sum += price * weight;
            total_weight += weight;
        }

        let weighted_avg = weighted_sum / total_weight;

        // Verify weighted average is reasonable
        assert!(weighted_avg > 43250.0 && weighted_avg < 43252.0);

        // Verify price spread is reasonable (< 0.1%)
        let min_price = prices.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let max_price = prices.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        let spread_percentage = (max_price - min_price) / min_price;
        assert!(spread_percentage < 0.001); // < 0.1%
    }

    #[test]
    fn test_real_time_update_simulation() {
        let time_series = vec![
            (1640995200, "43250.0", "Initial price"),
            (1640995260, "43251.5", "Price increase"),
            (1640995320, "43249.8", "Price decrease"),
            (1640995380, "43252.2", "Recovery"),
            (1640995440, "43251.0", "Stabilization"),
        ];

        let mut prev_time = 0;
        for (timestamp, price, _phase) in time_series {
            let json = format!(
                r#"{{
                "exchange": "Binance",
                "symbol": "BTCUSDT",
                "weight": "0.3",
                "price": "{}",
                "update_time": {}
            }}"#,
                price, timestamp
            );

            let constituent: IndexConstituent = serde_json::from_str(&json).unwrap();

            // Verify timestamp progression
            assert!(constituent.update_time.unwrap() > prev_time);
            prev_time = constituent.update_time.unwrap();

            // Verify price is reasonable
            let price_val: f64 = constituent.price.unwrap().parse().unwrap();
            assert!(price_val > 43200.0 && price_val < 43300.0);
        }
    }

    #[test]
    fn test_exchange_outage_scenario() {
        // Simulate when an exchange is not providing data
        let outage_scenarios = vec![
            (true, "Normal operation", "BTCUSDT", "0.3", "43250.0"),
            (false, "Exchange down", "", "", ""),
            (true, "Exchange recovered", "BTCUSDT", "0.3", "43251.5"),
        ];

        for (is_operational, _description, symbol, weight, price) in outage_scenarios {
            if is_operational {
                let json = format!(
                    r#"{{
                    "exchange": "Binance",
                    "symbol": "{}",
                    "weight": "{}",
                    "price": "{}",
                    "update_time": 1640995200
                }}"#,
                    symbol, weight, price
                );

                let constituent: IndexConstituent = serde_json::from_str(&json).unwrap();
                assert_eq!(constituent.exchange, "Binance");
                assert!(constituent.symbol.is_some());
                assert!(constituent.weight.is_some());
                assert!(constituent.price.is_some());
            } else {
                // Exchange down - minimal data
                let json = r#"{"exchange": "Binance"}"#;
                let constituent: IndexConstituent = serde_json::from_str(json).unwrap();
                assert_eq!(constituent.exchange, "Binance");
                assert!(constituent.symbol.is_none());
                assert!(constituent.weight.is_none());
                assert!(constituent.price.is_none());
            }
        }
    }

    #[test]
    fn test_clone_behavior() {
        let request = FuturesIndexConstituentsRequest {
            settle: "USDT".to_string(),
            index: "BTCUSD".to_string(),
        };

        let cloned = request.clone();
        assert_eq!(cloned.settle, request.settle);
        assert_eq!(cloned.index, request.index);
    }

    #[test]
    fn test_debug_output() {
        let constituent = IndexConstituent {
            exchange: "Binance".to_string(),
            symbol: Some("BTCUSDT".to_string()),
            weight: Some("0.35".to_string()),
            price: Some("43251.25".to_string()),
            update_time: Some(1640995200),
        };

        let debug_str = format!("{:?}", constituent);
        assert!(debug_str.contains("IndexConstituent"));
        assert!(debug_str.contains("Binance"));
        assert!(debug_str.contains("BTCUSDT"));
        assert!(debug_str.contains("0.35"));
    }

    #[test]
    fn test_serialization_round_trip() {
        let response = IndexConstituentsResponse {
            index: "BTCUSD".to_string(),
            constituents: vec![IndexConstituent {
                exchange: "Binance".to_string(),
                symbol: Some("BTCUSDT".to_string()),
                weight: Some("0.35".to_string()),
                price: Some("43251.25".to_string()),
                update_time: Some(1640995200),
            }],
        };

        let json = serde_json::to_string(&response).unwrap();
        let deserialized: IndexConstituentsResponse = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.index, response.index);
        assert_eq!(deserialized.constituents.len(), response.constituents.len());
        assert_eq!(
            deserialized.constituents[0].exchange,
            response.constituents[0].exchange
        );
    }
}
