use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::gateio::RestResult;

const CURRENCY_PAIRS_ENDPOINT: &str = "/spot/currency_pairs";

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
    /// List all currency pairs
    ///
    /// This endpoint returns a list of all supported currency pairs with their
    /// trading fees, precision settings, and trading status.
    ///
    /// [docs](https://www.gate.io/docs/developers/apiv4/en/#list-all-currency-pairs-supported)
    pub async fn list_currency_pairs(&self) -> RestResult<Vec<CurrencyPair>> {
        self.get(CURRENCY_PAIRS_ENDPOINT).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_currency_pair_full_deserialization() {
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
    fn test_currency_pair_minimal_deserialization() {
        let json = r#"{
            "id": "ETH_USDT",
            "base": "ETH",
            "quote": "USDT",
            "fee": "0.002",
            "amount_precision": 3,
            "precision": 2,
            "trade_status": "tradable",
            "sell_start": 0,
            "buy_start": 0
        }"#;

        let pair: CurrencyPair = serde_json::from_str(json).unwrap();
        assert_eq!(pair.id, "ETH_USDT");
        assert_eq!(pair.base, "ETH");
        assert_eq!(pair.quote, "USDT");
        assert_eq!(pair.fee, "0.002");
        assert_eq!(pair.min_base_amount, None);
        assert_eq!(pair.min_quote_amount, None);
        assert_eq!(pair.max_base_amount, None);
        assert_eq!(pair.max_quote_amount, None);
        assert_eq!(pair.amount_precision, 3);
        assert_eq!(pair.precision, 2);
        assert_eq!(pair.trade_status, "tradable");
        assert_eq!(pair.sell_start, 0);
        assert_eq!(pair.buy_start, 0);
    }

    #[test]
    fn test_currency_pair_untradable() {
        let json = r#"{
            "id": "TEST_USDT",
            "base": "TEST",
            "quote": "USDT",
            "fee": "0.002",
            "amount_precision": 8,
            "precision": 8,
            "trade_status": "untradable",
            "sell_start": 1640995200,
            "buy_start": 1640995200
        }"#;

        let pair: CurrencyPair = serde_json::from_str(json).unwrap();
        assert_eq!(pair.id, "TEST_USDT");
        assert_eq!(pair.trade_status, "untradable");
        assert_eq!(pair.sell_start, 1640995200);
        assert_eq!(pair.buy_start, 1640995200);
    }

    #[test]
    fn test_currency_pair_different_fee_rates() {
        let fees = vec!["0", "0.001", "0.002", "0.005", "0.01", "0.1"];

        for fee in fees {
            let json = format!(
                r#"{{
                "id": "PAIR_USDT",
                "base": "PAIR",
                "quote": "USDT",
                "fee": "{}",
                "amount_precision": 8,
                "precision": 8,
                "trade_status": "tradable",
                "sell_start": 0,
                "buy_start": 0
            }}"#,
                fee
            );

            let pair: CurrencyPair = serde_json::from_str(&json).unwrap();
            assert_eq!(pair.fee, fee);
        }
    }

    #[test]
    fn test_currency_pair_different_precisions() {
        let precisions = vec![(0, 0), (2, 2), (4, 4), (6, 6), (8, 8), (10, 10)];

        for (amt_prec, price_prec) in precisions {
            let json = format!(
                r#"{{
                "id": "PAIR_USDT",
                "base": "PAIR",
                "quote": "USDT",
                "fee": "0.002",
                "amount_precision": {},
                "precision": {},
                "trade_status": "tradable",
                "sell_start": 0,
                "buy_start": 0
            }}"#,
                amt_prec, price_prec
            );

            let pair: CurrencyPair = serde_json::from_str(&json).unwrap();
            assert_eq!(pair.amount_precision, amt_prec);
            assert_eq!(pair.precision, price_prec);
        }
    }

    #[test]
    fn test_currency_pair_extreme_amounts() {
        let json = r#"{
            "id": "BTC_USDT",
            "base": "BTC",
            "quote": "USDT",
            "fee": "0.002",
            "min_base_amount": "0.00000001",
            "min_quote_amount": "0.01",
            "max_base_amount": "99999999.99999999",
            "max_quote_amount": "999999999999.99",
            "amount_precision": 8,
            "precision": 2,
            "trade_status": "tradable",
            "sell_start": 0,
            "buy_start": 0
        }"#;

        let pair: CurrencyPair = serde_json::from_str(json).unwrap();
        assert_eq!(pair.min_base_amount, Some("0.00000001".to_string()));
        assert_eq!(pair.min_quote_amount, Some("0.01".to_string()));
        assert_eq!(pair.max_base_amount, Some("99999999.99999999".to_string()));
        assert_eq!(pair.max_quote_amount, Some("999999999999.99".to_string()));
    }

    #[test]
    fn test_currency_pair_array_deserialization() {
        let json = r#"[
            {
                "id": "BTC_USDT",
                "base": "BTC",
                "quote": "USDT",
                "fee": "0.002",
                "min_base_amount": "0.0001",
                "amount_precision": 4,
                "precision": 2,
                "trade_status": "tradable",
                "sell_start": 0,
                "buy_start": 0
            },
            {
                "id": "ETH_USDT",
                "base": "ETH",
                "quote": "USDT",
                "fee": "0.002",
                "min_base_amount": "0.001",
                "amount_precision": 3,
                "precision": 2,
                "trade_status": "tradable",
                "sell_start": 0,
                "buy_start": 0
            },
            {
                "id": "BNB_USDT",
                "base": "BNB",
                "quote": "USDT",
                "fee": "0.001",
                "min_base_amount": "0.01",
                "amount_precision": 2,
                "precision": 2,
                "trade_status": "tradable",
                "sell_start": 0,
                "buy_start": 0
            }
        ]"#;

        let pairs: Vec<CurrencyPair> = serde_json::from_str(json).unwrap();
        assert_eq!(pairs.len(), 3);

        assert_eq!(pairs[0].id, "BTC_USDT");
        assert_eq!(pairs[0].base, "BTC");
        assert_eq!(pairs[0].quote, "USDT");

        assert_eq!(pairs[1].id, "ETH_USDT");
        assert_eq!(pairs[1].base, "ETH");
        assert_eq!(pairs[1].quote, "USDT");

        assert_eq!(pairs[2].id, "BNB_USDT");
        assert_eq!(pairs[2].base, "BNB");
        assert_eq!(pairs[2].quote, "USDT");
    }

    #[test]
    fn test_currency_pair_empty_array_deserialization() {
        let json = r#"[]"#;
        let pairs: Vec<CurrencyPair> = serde_json::from_str(json).unwrap();
        assert_eq!(pairs.len(), 0);
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
    fn test_currency_pair_serialization_round_trip() {
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
    fn test_currency_pair_different_quote_currencies() {
        let quotes = vec!["USDT", "BTC", "ETH", "BUSD", "USDC"];

        for quote in quotes {
            let json = format!(
                r#"{{
                "id": "TEST_{}",
                "base": "TEST",
                "quote": "{}",
                "fee": "0.002",
                "amount_precision": 8,
                "precision": 8,
                "trade_status": "tradable",
                "sell_start": 0,
                "buy_start": 0
            }}"#,
                quote, quote
            );

            let pair: CurrencyPair = serde_json::from_str(&json).unwrap();
            assert_eq!(pair.quote, quote);
            assert_eq!(pair.id, format!("TEST_{}", quote));
        }
    }

    #[test]
    fn test_currency_pair_realistic_scenarios() {
        // Major pair with high liquidity
        let major_json = r#"{
            "id": "BTC_USDT",
            "base": "BTC",
            "quote": "USDT",
            "fee": "0.001",
            "min_base_amount": "0.0001",
            "min_quote_amount": "1",
            "max_base_amount": "1000",
            "max_quote_amount": "50000000",
            "amount_precision": 4,
            "precision": 2,
            "trade_status": "tradable",
            "sell_start": 0,
            "buy_start": 0
        }"#;

        let major: CurrencyPair = serde_json::from_str(major_json).unwrap();
        assert_eq!(major.id, "BTC_USDT");
        assert_eq!(major.trade_status, "tradable");
        assert_eq!(major.fee, "0.001"); // Lower fee for major pair

        // New listing with restrictions
        let new_listing_json = r#"{
            "id": "NEWTOKEN_USDT",
            "base": "NEWTOKEN",
            "quote": "USDT",
            "fee": "0.005",
            "min_base_amount": "10",
            "min_quote_amount": "10",
            "max_base_amount": "10000",
            "max_quote_amount": "100000",
            "amount_precision": 2,
            "precision": 4,
            "trade_status": "buyonly",
            "sell_start": 1700000000,
            "buy_start": 1640995200
        }"#;

        let new_listing: CurrencyPair = serde_json::from_str(new_listing_json).unwrap();
        assert_eq!(new_listing.id, "NEWTOKEN_USDT");
        assert_eq!(new_listing.trade_status, "buyonly");
        assert_eq!(new_listing.fee, "0.005"); // Higher fee for new listing
        assert!(new_listing.sell_start > new_listing.buy_start); // Can buy before sell
    }

    #[test]
    fn test_currency_pair_negative_timestamps() {
        let json = r#"{
            "id": "TEST_USDT",
            "base": "TEST",
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
}
