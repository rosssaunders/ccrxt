use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::binance::spot::RestResult;

const HISTORICAL_TRADES_ENDPOINT: &str = "/api/v3/historicalTrades";

/// Request parameters for historical trades
#[derive(Debug, Clone, Serialize)]
pub struct HistoricalTradesRequest {
    /// Trading pair symbol
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Number of trades to return. Default 500, Max 1000
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    /// TradeId to fetch from. Default gets most recent trades
    #[serde(rename = "fromId", skip_serializing_if = "Option::is_none")]
    pub from_id: Option<u64>,
}

/// Historical trade information
#[derive(Debug, Clone, Deserialize)]
pub struct HistoricalTrade {
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
    /// Get historical trades
    ///
    /// Get older market trades.
    ///
    /// [docs](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/public-api-endpoints#old-trade-lookup)
    /// Method: GET /api/v3/historicalTrades
    /// Weight: 25
    /// Security: None
    pub async fn get_historical_trades(
        &self,
        params: HistoricalTradesRequest,
    ) -> RestResult<Vec<HistoricalTrade>> {
        self.send_get_request(HISTORICAL_TRADES_ENDPOINT, Some(params), 25)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_historical_trades_request_serialization() {
        let request = HistoricalTradesRequest {
            symbol: "BTCUSDT".to_string(),
            limit: Some(500),
            from_id: Some(1000000),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTCUSDT"));
        assert!(serialized.contains("limit=500"));
        assert!(serialized.contains("fromId=1000000"));
    }

    #[test]
    fn test_historical_trades_request_minimal() {
        let request = HistoricalTradesRequest {
            symbol: "ETHUSDT".to_string(),
            limit: None,
            from_id: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "symbol=ETHUSDT");
    }

    #[test]
    fn test_historical_trades_request_with_limit_only() {
        let request = HistoricalTradesRequest {
            symbol: "ADAUSDT".to_string(),
            limit: Some(1000),
            from_id: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=ADAUSDT"));
        assert!(serialized.contains("limit=1000"));
        assert!(!serialized.contains("fromId"));
    }

    #[test]
    fn test_historical_trade_deserialization() {
        let json = r#"{
            "id": 28457,
            "price": "4.00000100",
            "qty": "12.00000000",
            "quoteQty": "48.00001200",
            "time": 1499865549590,
            "isBuyerMaker": true,
            "isBestMatch": true
        }"#;

        let trade: HistoricalTrade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.id, 28457);
        assert_eq!(trade.price.to_string(), "4.00000100");
        assert_eq!(trade.qty.to_string(), "12.00000000");
        assert_eq!(trade.quote_qty.to_string(), "48.00001200");
        assert_eq!(trade.time, 1499865549590);
        assert!(trade.is_buyer_maker);
        assert!(trade.is_best_match);
    }

    #[test]
    fn test_historical_trades_array_deserialization() {
        let json = r#"[
            {
                "id": 28457,
                "price": "4.00000100",
                "qty": "12.00000000",
                "quoteQty": "48.00001200",
                "time": 1499865549590,
                "isBuyerMaker": true,
                "isBestMatch": true
            },
            {
                "id": 28458,
                "price": "4.00000200",
                "qty": "5.50000000",
                "quoteQty": "22.00001100",
                "time": 1499865549600,
                "isBuyerMaker": false,
                "isBestMatch": true
            }
        ]"#;

        let trades: Vec<HistoricalTrade> = serde_json::from_str(json).unwrap();
        assert_eq!(trades.len(), 2);

        assert_eq!(trades[0].id, 28457);
        assert_eq!(trades[0].price.to_string(), "4.00000100");
        assert!(trades[0].is_buyer_maker);

        assert_eq!(trades[1].id, 28458);
        assert_eq!(trades[1].price.to_string(), "4.00000200");
        assert!(!trades[1].is_buyer_maker);
    }

    #[test]
    fn test_historical_trades_empty_response() {
        let json = r#"[]"#;
        let trades: Vec<HistoricalTrade> = serde_json::from_str(json).unwrap();
        assert_eq!(trades.len(), 0);
    }

    #[test]
    fn test_historical_trade_high_precision_values() {
        let json = r#"{
            "id": 999999999,
            "price": "45380.12345678",
            "qty": "0.00001234",
            "quoteQty": "0.56002195",
            "time": 1625184000000,
            "isBuyerMaker": false,
            "isBestMatch": true
        }"#;

        let trade: HistoricalTrade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.id, 999999999);
        assert_eq!(trade.price.to_string(), "45380.12345678");
        assert_eq!(trade.qty.to_string(), "0.00001234");
        assert_eq!(trade.quote_qty.to_string(), "0.56002195");
    }

    #[test]
    fn test_historical_trade_large_quantity() {
        let json = r#"{
            "id": 12345678,
            "price": "0.00001234",
            "qty": "1000000.00000000",
            "quoteQty": "12.34000000",
            "time": 1625184000000,
            "isBuyerMaker": true,
            "isBestMatch": false
        }"#;

        let trade: HistoricalTrade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.qty.to_string(), "1000000.00000000");
        assert_eq!(trade.quote_qty.to_string(), "12.34000000");
        assert!(trade.is_buyer_maker);
        assert!(!trade.is_best_match);
    }

    #[test]
    fn test_historical_trades_request_max_limit() {
        let request = HistoricalTradesRequest {
            symbol: "BTCUSDT".to_string(),
            limit: Some(1000),
            from_id: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("limit=1000"));
    }

    #[test]
    fn test_historical_trades_sequential_ids() {
        let json = r#"[
            {
                "id": 100000,
                "price": "45000.00",
                "qty": "0.10000000",
                "quoteQty": "4500.00000000",
                "time": 1625184000000,
                "isBuyerMaker": true,
                "isBestMatch": true
            },
            {
                "id": 100001,
                "price": "45001.00",
                "qty": "0.20000000",
                "quoteQty": "9000.20000000",
                "time": 1625184001000,
                "isBuyerMaker": false,
                "isBestMatch": true
            },
            {
                "id": 100002,
                "price": "45002.00",
                "qty": "0.30000000",
                "quoteQty": "13500.60000000",
                "time": 1625184002000,
                "isBuyerMaker": true,
                "isBestMatch": true
            }
        ]"#;

        let trades: Vec<HistoricalTrade> = serde_json::from_str(json).unwrap();
        assert_eq!(trades.len(), 3);

        // Check that IDs are sequential
        assert_eq!(trades[0].id, 100000);
        assert_eq!(trades[1].id, 100001);
        assert_eq!(trades[2].id, 100002);

        // Check that times are also sequential
        assert_eq!(trades[0].time, 1625184000000);
        assert_eq!(trades[1].time, 1625184001000);
        assert_eq!(trades[2].time, 1625184002000);
    }
}
