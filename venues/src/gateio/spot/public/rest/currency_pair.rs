use serde::{Deserialize, Serialize};

use super::RestClient;

/// Currency pair information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrencyPair {
    /// Currency pair ID
    pub id: String,

    /// Base currency
    pub base: String,

    /// Quote currency
    pub quote: String,

    /// Trading fee rate
    pub fee: String,

    /// Minimum base currency amount per order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_base_amount: Option<String>,

    /// Minimum quote currency amount per order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_quote_amount: Option<String>,

    /// Maximum base currency amount per order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_base_amount: Option<String>,

    /// Maximum quote currency amount per order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_quote_amount: Option<String>,

    /// Amount precision
    pub amount_precision: i32,

    /// Price precision
    pub precision: i32,

    /// Trading status (0: disabled, 1: enabled)
    pub trade_status: String,

    /// Sell start timestamp
    pub sell_start: i64,

    /// Buy start timestamp
    pub buy_start: i64,
}

impl RestClient {
    /// Get specific currency pair details
    ///
    /// This endpoint returns detailed information about a specific currency pair
    /// including trading fees, precision, and trading status.
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#get-details-of-a-specifc-currency-pair>
    pub async fn get_currency_pair(
        &self,
        currency_pair: &str,
    ) -> crate::gateio::spot::Result<CurrencyPair> {
        let endpoint = format!("/spot/currency_pairs/{}", currency_pair);
        self.get(&endpoint).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_currency_pair_btc_usdt_deserialization() {
        let json = r#"{
            "id": "BTC_USDT",
            "base": "BTC",
            "quote": "USDT",
            "fee": "0.002",
            "min_base_amount": "0.0001",
            "min_quote_amount": "1",
            "max_base_amount": "1000",
            "max_quote_amount": "10000000",
            "amount_precision": 4,
            "precision": 2,
            "trade_status": "tradable",
            "sell_start": 0,
            "buy_start": 0
        }"#;

        let pair: CurrencyPair = serde_json::from_str(json).unwrap();
        assert_eq!(pair.id, "BTC_USDT");
        assert_eq!(pair.base, "BTC");
        assert_eq!(pair.quote, "USDT");
        assert_eq!(pair.fee, "0.002");
        assert_eq!(pair.min_base_amount, Some("0.0001".to_string()));
        assert_eq!(pair.min_quote_amount, Some("1".to_string()));
        assert_eq!(pair.max_base_amount, Some("1000".to_string()));
        assert_eq!(pair.max_quote_amount, Some("10000000".to_string()));
        assert_eq!(pair.amount_precision, 4);
        assert_eq!(pair.precision, 2);
        assert_eq!(pair.trade_status, "tradable");
        assert_eq!(pair.sell_start, 0);
        assert_eq!(pair.buy_start, 0);
    }

    #[test]
    fn test_currency_pair_eth_btc_deserialization() {
        let json = r#"{
            "id": "ETH_BTC",
            "base": "ETH",
            "quote": "BTC",
            "fee": "0.001",
            "min_base_amount": "0.01",
            "min_quote_amount": "0.0001",
            "max_base_amount": "10000",
            "max_quote_amount": "100",
            "amount_precision": 3,
            "precision": 6,
            "trade_status": "tradable",
            "sell_start": 1500000000,
            "buy_start": 1500000000
        }"#;

        let pair: CurrencyPair = serde_json::from_str(json).unwrap();
        assert_eq!(pair.id, "ETH_BTC");
        assert_eq!(pair.base, "ETH");
        assert_eq!(pair.quote, "BTC");
        assert_eq!(pair.fee, "0.001");
        assert_eq!(pair.amount_precision, 3);
        assert_eq!(pair.precision, 6);
        assert_eq!(pair.trade_status, "tradable");
        assert_eq!(pair.sell_start, 1500000000);
        assert_eq!(pair.buy_start, 1500000000);
    }

    #[test]
    fn test_currency_pair_minimal_fields() {
        let json = r#"{
            "id": "TEST_USDT",
            "base": "TEST",
            "quote": "USDT",
            "fee": "0.002",
            "amount_precision": 8,
            "precision": 8,
            "trade_status": "untradable",
            "sell_start": -1,
            "buy_start": -1
        }"#;

        let pair: CurrencyPair = serde_json::from_str(json).unwrap();
        assert_eq!(pair.id, "TEST_USDT");
        assert_eq!(pair.base, "TEST");
        assert_eq!(pair.quote, "USDT");
        assert_eq!(pair.fee, "0.002");
        assert_eq!(pair.min_base_amount, None);
        assert_eq!(pair.min_quote_amount, None);
        assert_eq!(pair.max_base_amount, None);
        assert_eq!(pair.max_quote_amount, None);
        assert_eq!(pair.amount_precision, 8);
        assert_eq!(pair.precision, 8);
        assert_eq!(pair.trade_status, "untradable");
        assert_eq!(pair.sell_start, -1);
        assert_eq!(pair.buy_start, -1);
    }

    #[test]
    fn test_currency_pair_buy_only_status() {
        let json = r#"{
            "id": "NEWCOIN_USDT",
            "base": "NEWCOIN",
            "quote": "USDT",
            "fee": "0.005",
            "min_base_amount": "10",
            "min_quote_amount": "10",
            "amount_precision": 2,
            "precision": 4,
            "trade_status": "buyonly",
            "sell_start": 1700000000,
            "buy_start": 1640995200
        }"#;

        let pair: CurrencyPair = serde_json::from_str(json).unwrap();
        assert_eq!(pair.id, "NEWCOIN_USDT");
        assert_eq!(pair.trade_status, "buyonly");
        assert_eq!(pair.fee, "0.005"); // Higher fee for new listings
        assert!(pair.sell_start > pair.buy_start); // Can buy before sell
    }

    #[test]
    fn test_currency_pair_sell_only_status() {
        let json = r#"{
            "id": "DELISTED_USDT",
            "base": "DELISTED",
            "quote": "USDT",
            "fee": "0.01",
            "amount_precision": 8,
            "precision": 8,
            "trade_status": "sellonly",
            "sell_start": 0,
            "buy_start": 0
        }"#;

        let pair: CurrencyPair = serde_json::from_str(json).unwrap();
        assert_eq!(pair.id, "DELISTED_USDT");
        assert_eq!(pair.trade_status, "sellonly");
        assert_eq!(pair.fee, "0.01"); // Higher fee for delisted coins
    }

    #[test]
    fn test_currency_pair_extreme_precision() {
        let json = r#"{
            "id": "MICRO_USDT",
            "base": "MICRO",
            "quote": "USDT",
            "fee": "0.002",
            "min_base_amount": "0.00000001",
            "min_quote_amount": "0.00000001",
            "max_base_amount": "999999999.99999999",
            "max_quote_amount": "999999999.99999999",
            "amount_precision": 8,
            "precision": 8,
            "trade_status": "tradable",
            "sell_start": 0,
            "buy_start": 0
        }"#;

        let pair: CurrencyPair = serde_json::from_str(json).unwrap();
        assert_eq!(pair.min_base_amount, Some("0.00000001".to_string()));
        assert_eq!(pair.min_quote_amount, Some("0.00000001".to_string()));
        assert_eq!(pair.max_base_amount, Some("999999999.99999999".to_string()));
        assert_eq!(
            pair.max_quote_amount,
            Some("999999999.99999999".to_string())
        );
        assert_eq!(pair.amount_precision, 8);
        assert_eq!(pair.precision, 8);
    }

    #[test]
    fn test_currency_pair_zero_fee() {
        let json = r#"{
            "id": "PROMO_USDT",
            "base": "PROMO",
            "quote": "USDT",
            "fee": "0",
            "amount_precision": 2,
            "precision": 2,
            "trade_status": "tradable",
            "sell_start": 0,
            "buy_start": 0
        }"#;

        let pair: CurrencyPair = serde_json::from_str(json).unwrap();
        assert_eq!(pair.fee, "0");
    }

    #[test]
    fn test_currency_pair_stablecoin_pair() {
        let json = r#"{
            "id": "USDC_USDT",
            "base": "USDC",
            "quote": "USDT",
            "fee": "0.0005",
            "min_base_amount": "1",
            "min_quote_amount": "1",
            "max_base_amount": "10000000",
            "max_quote_amount": "10000000",
            "amount_precision": 6,
            "precision": 6,
            "trade_status": "tradable",
            "sell_start": 0,
            "buy_start": 0
        }"#;

        let pair: CurrencyPair = serde_json::from_str(json).unwrap();
        assert_eq!(pair.id, "USDC_USDT");
        assert_eq!(pair.fee, "0.0005"); // Lower fee for stablecoin pairs
        assert_eq!(pair.amount_precision, 6);
        assert_eq!(pair.precision, 6); // Higher precision for stablecoins
    }

    #[test]
    fn test_currency_pair_with_underscores_in_symbol() {
        let json = r#"{
            "id": "SUPER_TOKEN_USDT",
            "base": "SUPER_TOKEN",
            "quote": "USDT",
            "fee": "0.002",
            "amount_precision": 4,
            "precision": 4,
            "trade_status": "tradable",
            "sell_start": 0,
            "buy_start": 0
        }"#;

        let pair: CurrencyPair = serde_json::from_str(json).unwrap();
        assert_eq!(pair.id, "SUPER_TOKEN_USDT");
        assert_eq!(pair.base, "SUPER_TOKEN");
        assert_eq!(pair.quote, "USDT");
    }

    #[test]
    fn test_currency_pair_negative_timestamps() {
        let json = r#"{
            "id": "OLD_USDT",
            "base": "OLD",
            "quote": "USDT",
            "fee": "0.002",
            "amount_precision": 8,
            "precision": 8,
            "trade_status": "tradable",
            "sell_start": -1,
            "buy_start": -1
        }"#;

        let pair: CurrencyPair = serde_json::from_str(json).unwrap();
        assert_eq!(pair.sell_start, -1);
        assert_eq!(pair.buy_start, -1);
    }

    #[test]
    fn test_currency_pair_max_timestamps() {
        let json = format!(
            r#"{{
            "id": "FUTURE_USDT",
            "base": "FUTURE",
            "quote": "USDT",
            "fee": "0.002",
            "amount_precision": 8,
            "precision": 8,
            "trade_status": "tradable",
            "sell_start": {},
            "buy_start": {}
        }}"#,
            i64::MAX,
            i64::MAX
        );

        let pair: CurrencyPair = serde_json::from_str(&json).unwrap();
        assert_eq!(pair.sell_start, i64::MAX);
        assert_eq!(pair.buy_start, i64::MAX);
    }

    #[test]
    fn test_currency_pair_serialization() {
        let pair = CurrencyPair {
            id: "BTC_USDT".to_string(),
            base: "BTC".to_string(),
            quote: "USDT".to_string(),
            fee: "0.002".to_string(),
            min_base_amount: Some("0.0001".to_string()),
            min_quote_amount: Some("1".to_string()),
            max_base_amount: Some("1000".to_string()),
            max_quote_amount: Some("10000000".to_string()),
            amount_precision: 4,
            precision: 2,
            trade_status: "tradable".to_string(),
            sell_start: 0,
            buy_start: 0,
        };

        let json = serde_json::to_value(&pair).unwrap();
        assert_eq!(json["id"], "BTC_USDT");
        assert_eq!(json["base"], "BTC");
        assert_eq!(json["quote"], "USDT");
        assert_eq!(json["fee"], "0.002");
        assert_eq!(json["min_base_amount"], "0.0001");
        assert_eq!(json["min_quote_amount"], "1");
        assert_eq!(json["max_base_amount"], "1000");
        assert_eq!(json["max_quote_amount"], "10000000");
        assert_eq!(json["amount_precision"], 4);
        assert_eq!(json["precision"], 2);
        assert_eq!(json["trade_status"], "tradable");
        assert_eq!(json["sell_start"], 0);
        assert_eq!(json["buy_start"], 0);
    }

    #[test]
    fn test_currency_pair_serialization_with_none_fields() {
        let pair = CurrencyPair {
            id: "TEST_USDT".to_string(),
            base: "TEST".to_string(),
            quote: "USDT".to_string(),
            fee: "0.002".to_string(),
            min_base_amount: None,
            min_quote_amount: None,
            max_base_amount: None,
            max_quote_amount: None,
            amount_precision: 8,
            precision: 8,
            trade_status: "tradable".to_string(),
            sell_start: 0,
            buy_start: 0,
        };

        let json = serde_json::to_string(&pair).unwrap();
        assert!(!json.contains("min_base_amount"));
        assert!(!json.contains("min_quote_amount"));
        assert!(!json.contains("max_base_amount"));
        assert!(!json.contains("max_quote_amount"));
    }

    #[test]
    fn test_currency_pair_round_trip() {
        let original = CurrencyPair {
            id: "ETH_BTC".to_string(),
            base: "ETH".to_string(),
            quote: "BTC".to_string(),
            fee: "0.001".to_string(),
            min_base_amount: Some("0.01".to_string()),
            min_quote_amount: Some("0.0001".to_string()),
            max_base_amount: Some("10000".to_string()),
            max_quote_amount: Some("100".to_string()),
            amount_precision: 3,
            precision: 6,
            trade_status: "tradable".to_string(),
            sell_start: 1640995200,
            buy_start: 1640995200,
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: CurrencyPair = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.id, original.id);
        assert_eq!(deserialized.base, original.base);
        assert_eq!(deserialized.quote, original.quote);
        assert_eq!(deserialized.fee, original.fee);
        assert_eq!(deserialized.min_base_amount, original.min_base_amount);
        assert_eq!(deserialized.min_quote_amount, original.min_quote_amount);
        assert_eq!(deserialized.max_base_amount, original.max_base_amount);
        assert_eq!(deserialized.max_quote_amount, original.max_quote_amount);
        assert_eq!(deserialized.amount_precision, original.amount_precision);
        assert_eq!(deserialized.precision, original.precision);
        assert_eq!(deserialized.trade_status, original.trade_status);
        assert_eq!(deserialized.sell_start, original.sell_start);
        assert_eq!(deserialized.buy_start, original.buy_start);
    }

    #[test]
    fn test_currency_pair_clone() {
        let original = CurrencyPair {
            id: "BTC_USDT".to_string(),
            base: "BTC".to_string(),
            quote: "USDT".to_string(),
            fee: "0.002".to_string(),
            min_base_amount: Some("0.0001".to_string()),
            min_quote_amount: Some("1".to_string()),
            max_base_amount: Some("1000".to_string()),
            max_quote_amount: Some("10000000".to_string()),
            amount_precision: 4,
            precision: 2,
            trade_status: "tradable".to_string(),
            sell_start: 0,
            buy_start: 0,
        };

        let cloned = original.clone();
        assert_eq!(cloned.id, original.id);
        assert_eq!(cloned.base, original.base);
        assert_eq!(cloned.quote, original.quote);
        assert_eq!(cloned.fee, original.fee);
        assert_eq!(cloned.min_base_amount, original.min_base_amount);
        assert_eq!(cloned.min_quote_amount, original.min_quote_amount);
        assert_eq!(cloned.max_base_amount, original.max_base_amount);
        assert_eq!(cloned.max_quote_amount, original.max_quote_amount);
        assert_eq!(cloned.amount_precision, original.amount_precision);
        assert_eq!(cloned.precision, original.precision);
        assert_eq!(cloned.trade_status, original.trade_status);
        assert_eq!(cloned.sell_start, original.sell_start);
        assert_eq!(cloned.buy_start, original.buy_start);
    }

    #[test]
    fn test_currency_pair_debug() {
        let pair = CurrencyPair {
            id: "BTC_USDT".to_string(),
            base: "BTC".to_string(),
            quote: "USDT".to_string(),
            fee: "0.002".to_string(),
            min_base_amount: Some("0.0001".to_string()),
            min_quote_amount: Some("1".to_string()),
            max_base_amount: Some("1000".to_string()),
            max_quote_amount: Some("10000000".to_string()),
            amount_precision: 4,
            precision: 2,
            trade_status: "tradable".to_string(),
            sell_start: 0,
            buy_start: 0,
        };

        let debug_str = format!("{:?}", pair);
        assert!(debug_str.contains("CurrencyPair"));
        assert!(debug_str.contains("BTC_USDT"));
        assert!(debug_str.contains("0.002"));
    }
}
