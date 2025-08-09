use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::binance::spot::RestResult;

const AGG_TRADES_ENDPOINT: &str = "/api/v3/aggTrades";

/// Request parameters for aggregate trades
#[derive(Debug, Clone, Serialize, Default)]
pub struct AggTradesRequest {
    /// Trading pair symbol
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// ID to get aggregate trades from INCLUSIVE
    #[serde(rename = "fromId", skip_serializing_if = "Option::is_none")]
    pub from_id: Option<u64>,

    /// Timestamp in ms to get aggregate trades from INCLUSIVE
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,

    /// Timestamp in ms to get aggregate trades until INCLUSIVE
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,

    /// Number of trades to return. Default 500, Max 1000
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

/// Aggregate trade information
#[derive(Debug, Clone, Deserialize)]
pub struct AggTrade {
    /// Aggregate trade ID
    #[serde(rename = "a")]
    pub agg_trade_id: u64,

    /// Trade price
    #[serde(rename = "p")]
    pub price: Decimal,

    /// Trade quantity
    #[serde(rename = "q")]
    pub quantity: Decimal,

    /// First trade ID
    #[serde(rename = "f")]
    pub first_trade_id: u64,

    /// Last trade ID
    #[serde(rename = "l")]
    pub last_trade_id: u64,

    /// Trade time
    #[serde(rename = "T")]
    pub timestamp: u64,

    /// Was the buyer the maker?
    #[serde(rename = "m")]
    pub is_buyer_maker: bool,

    /// Was this trade the best price match?
    #[serde(rename = "M")]
    pub is_best_match: bool,
}

impl RestClient {
    /// Get compressed/aggregate trades
    ///
    /// Trades that fill at the time, from the same order, with the same price will have the quantity aggregated.
    ///
    /// [docs]: (https://developers.binance.com/docs/binance-spot-api-docs/rest-api/public-api-endpoints#compressed-aggregate-trades-list)
    /// Method: GET /api/v3/aggTrades
    /// Weight: 2
    /// Security: None
    pub async fn get_agg_trades(&self, params: AggTradesRequest) -> RestResult<Vec<AggTrade>> {
        self.send_get_request(AGG_TRADES_ENDPOINT, Some(params), 2)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agg_trades_request_serialization() {
        let request = AggTradesRequest {
            symbol: "BTCUSDT".to_string(),
            from_id: Some(1000),
            start_time: Some(1625184000000),
            end_time: Some(1625270400000),
            limit: Some(500),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTCUSDT"));
        assert!(serialized.contains("fromId=1000"));
        assert!(serialized.contains("startTime=1625184000000"));
        assert!(serialized.contains("endTime=1625270400000"));
        assert!(serialized.contains("limit=500"));
    }

    #[test]
    fn test_agg_trades_request_minimal() {
        let request = AggTradesRequest {
            symbol: "ETHUSDT".to_string(),
            from_id: None,
            start_time: None,
            end_time: None,
            limit: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "symbol=ETHUSDT");
    }

    #[test]
    fn test_agg_trade_deserialization() {
        let json = r#"{
            "a": 26129,
            "p": "0.01633102",
            "q": "4.70443515",
            "f": 27781,
            "l": 27781,
            "T": 1498793709153,
            "m": true,
            "M": true
        }"#;

        let trade: AggTrade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.agg_trade_id, 26129);
        assert_eq!(trade.price.to_string(), "0.01633102");
        assert_eq!(trade.quantity.to_string(), "4.70443515");
        assert_eq!(trade.first_trade_id, 27781);
        assert_eq!(trade.last_trade_id, 27781);
        assert_eq!(trade.timestamp, 1498793709153);
        assert!(trade.is_buyer_maker);
        assert!(trade.is_best_match);
    }

    #[test]
    fn test_agg_trades_array_deserialization() {
        let json = r#"[
            {
                "a": 26129,
                "p": "0.01633102",
                "q": "4.70443515",
                "f": 27781,
                "l": 27781,
                "T": 1498793709153,
                "m": true,
                "M": true
            },
            {
                "a": 26130,
                "p": "0.01633103",
                "q": "2.35221757",
                "f": 27782,
                "l": 27782,
                "T": 1498793709163,
                "m": false,
                "M": true
            }
        ]"#;

        let trades: Vec<AggTrade> = serde_json::from_str(json).unwrap();
        assert_eq!(trades.len(), 2);
        assert_eq!(trades[0].agg_trade_id, 26129);
        assert_eq!(trades[1].agg_trade_id, 26130);
        assert!(trades[0].is_buyer_maker);
        assert!(!trades[1].is_buyer_maker);
    }

    #[test]
    fn test_agg_trades_empty_response() {
        let json = r#"[]"#;
        let trades: Vec<AggTrade> = serde_json::from_str(json).unwrap();
        assert_eq!(trades.len(), 0);
    }

    #[test]
    fn test_agg_trades_request_with_large_limit() {
        let request = AggTradesRequest {
            symbol: "BTCUSDT".to_string(),
            from_id: None,
            start_time: None,
            end_time: None,
            limit: Some(1000),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("limit=1000"));
    }

    #[test]
    fn test_agg_trade_high_precision_values() {
        let json = r#"{
            "a": 999999,
            "p": "12345.12345678",
            "q": "0.00000001",
            "f": 1000000,
            "l": 1000001,
            "T": 1625184000000,
            "m": false,
            "M": true
        }"#;

        let trade: AggTrade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.price.to_string(), "12345.12345678");
        assert_eq!(trade.quantity.to_string(), "0.00000001");
        assert_eq!(trade.first_trade_id, 1000000);
        assert_eq!(trade.last_trade_id, 1000001);
    }

    #[test]
    fn test_agg_trade_aggregated_trades() {
        let json = r#"{
            "a": 50000,
            "p": "45380.10",
            "q": "0.50000000",
            "f": 100000,
            "l": 100005,
            "T": 1625184000000,
            "m": true,
            "M": true
        }"#;

        let trade: AggTrade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.first_trade_id, 100000);
        assert_eq!(trade.last_trade_id, 100005);
        // This represents 6 individual trades aggregated into one
        assert_eq!(trade.last_trade_id - trade.first_trade_id + 1, 6);
    }

    #[test]
    fn test_agg_trade_single_trade() {
        let json = r#"{
            "a": 60000,
            "p": "3070.50",
            "q": "1.00000000",
            "f": 200000,
            "l": 200000,
            "T": 1625184000000,
            "m": false,
            "M": true
        }"#;

        let trade: AggTrade = serde_json::from_str(json).unwrap();
        // When first_trade_id equals last_trade_id, it's a single trade
        assert_eq!(trade.first_trade_id, trade.last_trade_id);
        assert_eq!(trade.first_trade_id, 200000);
    }
}
