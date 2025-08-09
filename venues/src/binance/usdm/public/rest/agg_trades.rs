use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::binance::usdm::RestResult;

// Endpoint path constant
const AGG_TRADES_ENDPOINT: &str = "/fapi/v1/aggTrades";

/// Request parameters for aggregate trades list.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AggTradesRequest {
    /// Trading pair symbol (e.g., "BTCUSDT").
    pub symbol: Cow<'static, str>,

    /// ID to get aggregate trades from INCLUSIVE.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_id: Option<u64>,

    /// Timestamp in ms to get aggregate trades from INCLUSIVE.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,

    /// Timestamp in ms to get aggregate trades until INCLUSIVE.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,

    /// Number of trades to return. Default 500; max 1000.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u16>,
}

/// Represents a single aggregate trade.
#[derive(Debug, Clone, Deserialize)]
pub struct AggTrade {
    /// Aggregate tradeId.
    #[serde(rename = "a")]
    pub agg_trade_id: u64,

    /// Price as a string.
    #[serde(rename = "p")]
    pub price: String,

    /// Quantity as a string.
    #[serde(rename = "q")]
    pub qty: String,

    /// First tradeId.
    #[serde(rename = "f")]
    pub first_trade_id: u64,

    /// Last tradeId.
    #[serde(rename = "l")]
    pub last_trade_id: u64,

    /// Timestamp.
    #[serde(rename = "T")]
    pub time: u64,

    /// Was the buyer the maker?
    #[serde(rename = "m")]
    pub is_buyer_maker: bool,
}

impl RestClient {
    /// Compressed Aggregate Trades List (GET /fapi/v1/aggTrades)
    ///
    /// Retrieves compressed aggregate trades for a symbol.
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Compressed-Aggregate-Trades-List
    ///
    /// Rate limit: 20 weight per minute
    ///
    /// # Arguments
    /// * `params` - Request parameters for the aggregate trades endpoint.
    ///
    /// # Returns
    /// A list of [`AggTrade`] instances representing compressed aggregate trades.
    pub async fn get_agg_trades(&self, params: AggTradesRequest) -> RestResult<Vec<AggTrade>> {
        self.send_get_request(AGG_TRADES_ENDPOINT, Some(params), 20)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agg_trades_request_serialization() {
        let request = AggTradesRequest {
            symbol: "BTCUSDT".into(),
            from_id: Some(1234567),
            start_time: Some(1625097600000),
            end_time: Some(1625184000000),
            limit: Some(100),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTCUSDT"));
        assert!(serialized.contains("fromId=1234567"));
        assert!(serialized.contains("startTime=1625097600000"));
        assert!(serialized.contains("endTime=1625184000000"));
        assert!(serialized.contains("limit=100"));
    }

    #[test]
    fn test_agg_trades_request_minimal() {
        let request = AggTradesRequest {
            symbol: "ETHUSDT".into(),
            from_id: None,
            start_time: None,
            end_time: None,
            limit: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "symbol=ETHUSDT");
    }

    #[test]
    fn test_agg_trade_response_deserialization() {
        let json = r#"[
            {
                "a": 26129,
                "p": "0.01633102",
                "q": "4.70443515",
                "f": 27781,
                "l": 27781,
                "T": 1498793709153,
                "m": true
            },
            {
                "a": 26130,
                "p": "0.01633103",
                "q": "5.12345678",
                "f": 27782,
                "l": 27785,
                "T": 1498793709154,
                "m": false
            }
        ]"#;

        let trades: Vec<AggTrade> = serde_json::from_str(json).unwrap();
        assert_eq!(trades.len(), 2);

        let first_trade = &trades[0];
        assert_eq!(first_trade.agg_trade_id, 26129);
        assert_eq!(first_trade.price, "0.01633102");
        assert_eq!(first_trade.qty, "4.70443515");
        assert_eq!(first_trade.first_trade_id, 27781);
        assert_eq!(first_trade.last_trade_id, 27781);
        assert_eq!(first_trade.time, 1498793709153);
        assert!(first_trade.is_buyer_maker);

        let second_trade = &trades[1];
        assert_eq!(second_trade.agg_trade_id, 26130);
        assert_eq!(second_trade.price, "0.01633103");
        assert_eq!(second_trade.qty, "5.12345678");
        assert_eq!(second_trade.first_trade_id, 27782);
        assert_eq!(second_trade.last_trade_id, 27785);
        assert_eq!(second_trade.time, 1498793709154);
        assert!(!second_trade.is_buyer_maker);
    }

    #[test]
    fn test_agg_trade_large_values() {
        let json = r#"[
            {
                "a": 999999999,
                "p": "45000.50000000",
                "q": "1000.00000000",
                "f": 888888888,
                "l": 888888999,
                "T": 1625184000000,
                "m": false
            }
        ]"#;

        let trades: Vec<AggTrade> = serde_json::from_str(json).unwrap();
        assert_eq!(trades.len(), 1);
        assert_eq!(trades[0].agg_trade_id, 999999999);
        assert_eq!(trades[0].price, "45000.50000000");
        assert_eq!(trades[0].qty, "1000.00000000");
        assert_eq!(trades[0].first_trade_id, 888888888);
        assert_eq!(trades[0].last_trade_id, 888888999);
    }

    #[test]
    fn test_agg_trade_empty_response() {
        let json = r#"[]"#;
        let trades: Vec<AggTrade> = serde_json::from_str(json).unwrap();
        assert_eq!(trades.len(), 0);
    }

    #[test]
    fn test_agg_trades_cow_str() {
        // Test with static string
        let request = AggTradesRequest {
            symbol: "BTCUSDT".into(),
            from_id: None,
            start_time: None,
            end_time: None,
            limit: None,
        };
        assert_eq!(request.symbol, "BTCUSDT");

        // Test with owned string
        let owned_symbol = String::from("ETHUSDT");
        let request2 = AggTradesRequest {
            symbol: owned_symbol.into(),
            from_id: None,
            start_time: None,
            end_time: None,
            limit: None,
        };
        assert_eq!(request2.symbol, "ETHUSDT");
    }
}
