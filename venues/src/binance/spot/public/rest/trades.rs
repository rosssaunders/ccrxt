use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::binance::spot::PublicRestClient as RestClient;
use crate::binance::spot::RestResult;

const TRADES_ENDPOINT: &str = "/api/v3/trades";

/// Request parameters for recent trades
#[derive(Debug, Clone, Serialize)]
pub struct TradesRequest {
    /// Trading pair symbol
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Number of trades to return. Default 500, Max 1000
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

/// Trade information
#[derive(Debug, Clone, Deserialize)]
pub struct Trade {
    /// Trade ID
    #[serde(rename = "id")]
    pub id: u64,

    /// Trade price
    #[serde(rename = "price")]
    pub price: Decimal,

    /// Trade quantity
    #[serde(rename = "qty")]
    pub qty: Decimal,

    /// Quote quantity
    #[serde(rename = "quoteQty")]
    pub quote_qty: Decimal,

    /// Trade time
    #[serde(rename = "time")]
    pub time: u64,

    /// Was the buyer the maker?
    #[serde(rename = "isBuyerMaker")]
    pub is_buyer_maker: bool,

    /// Was this trade the best price match?
    #[serde(rename = "isBestMatch")]
    pub is_best_match: bool,
}

impl RestClient {
    /// Get recent trades
    ///
    /// Get recent trades for a given symbol.
    ///
    /// [docs](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/public-api-endpoints#recent-trades-list)
    ///
    /// Method: GET /api/v3/trades
    /// Weight: 25
    /// Security: None
    pub async fn get_recent_trades(&self, params: TradesRequest) -> RestResult<Vec<Trade>> {
        self.send_get_request(TRADES_ENDPOINT, Some(params), 25)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trades_request_serialization() {
        let request = TradesRequest {
            symbol: "BTCUSDT".to_string(),
            limit: Some(500),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTCUSDT"));
        assert!(serialized.contains("limit=500"));
    }

    #[test]
    fn test_trades_request_minimal() {
        let request = TradesRequest {
            symbol: "ETHUSDT".to_string(),
            limit: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "symbol=ETHUSDT");
    }

    #[test]
    fn test_trades_request_max_limit() {
        let request = TradesRequest {
            symbol: "BTCUSDT".to_string(),
            limit: Some(1000),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTCUSDT"));
        assert!(serialized.contains("limit=1000"));
    }

    #[test]
    fn test_trade_deserialization() {
        let json = r#"{
            "id": 123456,
            "price": "45380.10000000",
            "qty": "0.50000000",
            "quoteQty": "22690.05000000",
            "time": 1625184000000,
            "isBuyerMaker": true,
            "isBestMatch": true
        }"#;

        let trade: Trade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.id, 123456);
        assert_eq!(trade.price.to_string(), "45380.10000000");
        assert_eq!(trade.qty.to_string(), "0.50000000");
        assert_eq!(trade.quote_qty.to_string(), "22690.05000000");
        assert_eq!(trade.time, 1625184000000);
        assert!(trade.is_buyer_maker);
        assert!(trade.is_best_match);
    }

    #[test]
    fn test_trades_array_deserialization() {
        let json = r#"[
            {
                "id": 100000,
                "price": "45000.00000000",
                "qty": "0.10000000",
                "quoteQty": "4500.00000000",
                "time": 1625184000000,
                "isBuyerMaker": true,
                "isBestMatch": true
            },
            {
                "id": 100001,
                "price": "45001.00000000",
                "qty": "0.20000000",
                "quoteQty": "9000.20000000",
                "time": 1625184001000,
                "isBuyerMaker": false,
                "isBestMatch": true
            },
            {
                "id": 100002,
                "price": "44999.50000000",
                "qty": "0.15000000",
                "quoteQty": "6749.92500000",
                "time": 1625184002000,
                "isBuyerMaker": true,
                "isBestMatch": false
            }
        ]"#;

        let trades: Vec<Trade> = serde_json::from_str(json).unwrap();
        assert_eq!(trades.len(), 3);

        assert_eq!(trades[0].id, 100000);
        assert_eq!(trades[0].price.to_string(), "45000.00000000");
        assert!(trades[0].is_buyer_maker);

        assert_eq!(trades[1].id, 100001);
        assert_eq!(trades[1].price.to_string(), "45001.00000000");
        assert!(!trades[1].is_buyer_maker);

        assert_eq!(trades[2].id, 100002);
        assert!(!trades[2].is_best_match);
    }

    #[test]
    fn test_trades_empty_response() {
        let json = r#"[]"#;
        let trades: Vec<Trade> = serde_json::from_str(json).unwrap();
        assert_eq!(trades.len(), 0);
    }

    #[test]
    fn test_trade_high_precision_values() {
        let json = r#"{
            "id": 999999999,
            "price": "0.00001234",
            "qty": "1000000.00000000",
            "quoteQty": "12.34000000",
            "time": 1625184000000,
            "isBuyerMaker": false,
            "isBestMatch": true
        }"#;

        let trade: Trade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.id, 999999999);
        assert_eq!(trade.price.to_string(), "0.00001234");
        assert_eq!(trade.qty.to_string(), "1000000.00000000");
        assert_eq!(trade.quote_qty.to_string(), "12.34000000");
    }

    #[test]
    fn test_trade_large_price_btc() {
        let json = r#"{
            "id": 12345678,
            "price": "65432.10000000",
            "qty": "0.00100000",
            "quoteQty": "65.43210000",
            "time": 1625184000000,
            "isBuyerMaker": true,
            "isBestMatch": true
        }"#;

        let trade: Trade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.price.to_string(), "65432.10000000");
        assert_eq!(trade.qty.to_string(), "0.00100000");
        assert_eq!(trade.quote_qty.to_string(), "65.43210000");
    }

    #[test]
    fn test_trades_sequential_timestamps() {
        let json = r#"[
            {
                "id": 200000,
                "price": "3070.50000000",
                "qty": "1.00000000",
                "quoteQty": "3070.50000000",
                "time": 1625184000000,
                "isBuyerMaker": true,
                "isBestMatch": true
            },
            {
                "id": 200001,
                "price": "3070.51000000",
                "qty": "2.00000000",
                "quoteQty": "6141.02000000",
                "time": 1625184000100,
                "isBuyerMaker": false,
                "isBestMatch": true
            },
            {
                "id": 200002,
                "price": "3070.52000000",
                "qty": "1.50000000",
                "quoteQty": "4605.78000000",
                "time": 1625184000200,
                "isBuyerMaker": true,
                "isBestMatch": true
            }
        ]"#;

        let trades: Vec<Trade> = serde_json::from_str(json).unwrap();
        assert_eq!(trades.len(), 3);

        // Verify sequential IDs
        assert_eq!(trades[0].id, 200000);
        assert_eq!(trades[1].id, 200001);
        assert_eq!(trades[2].id, 200002);

        // Verify sequential timestamps
        assert_eq!(trades[0].time, 1625184000000);
        assert_eq!(trades[1].time, 1625184000100);
        assert_eq!(trades[2].time, 1625184000200);
    }

    #[test]
    fn test_trade_small_altcoin_values() {
        let json = r#"{
            "id": 555555,
            "price": "0.00000012",
            "qty": "10000000.00000000",
            "quoteQty": "1.20000000",
            "time": 1625184000000,
            "isBuyerMaker": false,
            "isBestMatch": false
        }"#;

        let trade: Trade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.price.to_string(), "0.00000012");
        assert_eq!(trade.qty.to_string(), "10000000.00000000");
        assert_eq!(trade.quote_qty.to_string(), "1.20000000");
        assert!(!trade.is_buyer_maker);
        assert!(!trade.is_best_match);
    }

    #[test]
    fn test_trades_different_symbols() {
        let request1 = TradesRequest {
            symbol: "BTCUSDT".to_string(),
            limit: Some(100),
        };
        let request2 = TradesRequest {
            symbol: "ETHBTC".to_string(),
            limit: Some(200),
        };
        let request3 = TradesRequest {
            symbol: "ADABNB".to_string(),
            limit: Some(50),
        };

        let serialized1 = serde_urlencoded::to_string(&request1).unwrap();
        let serialized2 = serde_urlencoded::to_string(&request2).unwrap();
        let serialized3 = serde_urlencoded::to_string(&request3).unwrap();

        assert!(serialized1.contains("symbol=BTCUSDT"));
        assert!(serialized1.contains("limit=100"));

        assert!(serialized2.contains("symbol=ETHBTC"));
        assert!(serialized2.contains("limit=200"));

        assert!(serialized3.contains("symbol=ADABNB"));
        assert!(serialized3.contains("limit=50"));
    }
}
