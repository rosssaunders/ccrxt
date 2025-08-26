use serde::{Deserialize, Serialize};

use super::{RestClient, RestResult};

/// Endpoint path for the aggregate trades API.
const AGGREGATE_TRADES_ENDPOINT: &str = "/dapi/v1/aggTrades";

/// Request parameters for the aggregate trades endpoint.
#[derive(Debug, Clone, Serialize, Default)]
pub struct AggregateTradesRequest {
    /// Trading symbol (e.g., "BTCUSD_PERP").
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// ID to get aggregate trades from INCLUSIVE.
    #[serde(rename = "fromId", skip_serializing_if = "Option::is_none")]
    pub from_id: Option<u64>,

    /// Timestamp in ms to get aggregate trades from INCLUSIVE.
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,

    /// Timestamp in ms to get aggregate trades until INCLUSIVE.
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,

    /// Default 500; max 1000.
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

/// Represents a single aggregate trade.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AggregateTrade {
    /// Aggregate trade ID.
    #[serde(rename = "a")]
    pub agg_trade_id: u64,

    /// Price.
    #[serde(rename = "p")]
    pub price: String,

    /// Quantity.
    #[serde(rename = "q")]
    pub quantity: String,

    /// First trade ID.
    #[serde(rename = "f")]
    pub first_trade_id: u64,

    /// Last trade ID.
    #[serde(rename = "l")]
    pub last_trade_id: u64,

    /// Timestamp.
    #[serde(rename = "T")]
    pub timestamp: u64,

    /// Was the buyer the maker?
    #[serde(rename = "m")]
    pub is_buyer_maker: bool,
}

impl RestClient {
    /// Get compressed, aggregate trades.
    ///
    /// Market trades that fill in 100ms with the same price and the same taking side
    /// will have the quantity aggregated.
    ///
    /// Notes:
    /// - Support querying futures trade histories that are not older than one year
    /// - If both startTime and endTime are sent, time between startTime and endTime
    ///   must be less than 1 hour.
    /// - If fromId, startTime, and endTime are not sent, the most recent aggregate
    ///   trades will be returned.
    /// - Only market trades will be aggregated and returned, which means the insurance
    ///   fund trades and ADL trades won't be aggregated.
    /// - Sending both startTime/endTime and fromId might cause response timeout,
    ///   please send either fromId or startTime/endTime
    ///
    /// [docs](https://developers.binance.com/docs/derivatives/coin-margined-futures/market-data/rest-api/Compressed-Aggregate-Trades-List)
    ///
    /// Weight: 20
    pub async fn get_aggregate_trades(
        &self,
        params: AggregateTradesRequest,
    ) -> RestResult<Vec<AggregateTrade>> {
        self.send_get_request(AGGREGATE_TRADES_ENDPOINT, Some(params), 20)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aggregate_trades_request_serialization() {
        let request = AggregateTradesRequest {
            symbol: "BTCUSD_PERP".to_string(),
            from_id: None,
            start_time: None,
            end_time: None,
            limit: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "symbol=BTCUSD_PERP");
    }

    #[test]
    fn test_aggregate_trades_request_serialization_with_all_params() {
        let request = AggregateTradesRequest {
            symbol: "ETHUSD_PERP".to_string(),
            from_id: Some(12345),
            start_time: Some(1625097600000),
            end_time: Some(1625101200000),
            limit: Some(100),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=ETHUSD_PERP"));
        assert!(serialized.contains("fromId=12345"));
        assert!(serialized.contains("startTime=1625097600000"));
        assert!(serialized.contains("endTime=1625101200000"));
        assert!(serialized.contains("limit=100"));
    }

    #[test]
    fn test_aggregate_trade_deserialization() {
        let json = r#"{
            "a": 26129,
            "p": "50000.00",
            "q": "10.00000000",
            "f": 27781,
            "l": 27781,
            "T": 1498793709153,
            "m": true
        }"#;

        let trade: AggregateTrade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.agg_trade_id, 26129);
        assert_eq!(trade.price, "50000.00");
        assert_eq!(trade.quantity, "10.00000000");
        assert_eq!(trade.first_trade_id, 27781);
        assert_eq!(trade.last_trade_id, 27781);
        assert_eq!(trade.timestamp, 1498793709153);
        assert!(trade.is_buyer_maker);
    }

    #[test]
    fn test_aggregate_trades_list_deserialization() {
        let json = r#"[
            {
                "a": 26129,
                "p": "50000.00",
                "q": "10.00000000",
                "f": 27781,
                "l": 27781,
                "T": 1498793709153,
                "m": true
            },
            {
                "a": 26130,
                "p": "50001.00",
                "q": "5.00000000",
                "f": 27782,
                "l": 27783,
                "T": 1498793709200,
                "m": false
            }
        ]"#;

        let trades: Vec<AggregateTrade> = serde_json::from_str(json).unwrap();
        assert_eq!(trades.len(), 2);

        assert_eq!(trades[0].agg_trade_id, 26129);
        assert_eq!(trades[0].price, "50000.00");
        assert!(trades[0].is_buyer_maker);

        assert_eq!(trades[1].agg_trade_id, 26130);
        assert_eq!(trades[1].price, "50001.00");
        assert!(!trades[1].is_buyer_maker);
    }
}
