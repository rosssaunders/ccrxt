use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::binance::usdm::RestResult;

/// Endpoint path for recent trades list.
const RECENT_TRADES_ENDPOINT: &str = "/fapi/v1/trades";

/// Request parameters for the recent trades list endpoint.
///
/// Used to query recent trades for a given symbol on Binance USDM.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RecentTradesRequest {
    /// Trading pair symbol (e.g., "BTCUSDT"). Required.
    /// Must be a valid symbol supported by Binance USDM.
    pub symbol: Cow<'static, str>,

    /// Number of trades to return. Optional.
    /// Default is 500; maximum is 1000.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u16>,
}

/// Represents a single recent trade returned by the API.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecentTrade {
    /// Trade ID.
    pub id: u64,

    /// Price as a string.
    pub price: String,

    /// Quantity as a string.
    pub qty: String,

    /// Quote quantity as a string.
    pub quote_qty: String,

    /// Trade time (milliseconds since epoch).
    pub time: u64,

    /// True if buyer is the maker.
    pub is_buyer_maker: bool,
}

/// Response wrapper for recent trades list.
///
/// The API returns a direct array of trades, so this struct is a transparent wrapper.
#[derive(Debug, Clone, Deserialize)]
#[serde(transparent)]
pub struct RecentTradesResult {
    pub trades: Vec<RecentTrade>,
}

impl RestClient {
    /// Recent Trades List
    ///
    /// Get recent market trades. Market trades means trades filled in the order book.
    /// Only market trades will be returned, which means the insurance fund trades and ADL trades won't be returned.
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Recent-Trades-List
    ///
    /// Rate limit: Weight 5
    ///
    /// # Arguments
    /// * `params` - The request parameters for recent trades
    ///
    /// # Returns
    /// List of recent trades for the specified symbol
    pub async fn recent_trades(
        &self,
        params: RecentTradesRequest,
    ) -> RestResult<RecentTradesResult> {
        self.send_public_request(
            RECENT_TRADES_ENDPOINT,
            reqwest::Method::GET,
            Some(params),
            5,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recent_trades_request_serialization() {
        let request = RecentTradesRequest {
            symbol: "BTCUSDT".into(),
            limit: Some(100),
        };
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTCUSDT"));
        assert!(serialized.contains("limit=100"));
    }

    #[test]
    fn test_recent_trades_request_minimal() {
        let request = RecentTradesRequest {
            symbol: "ETHUSDT".into(),
            limit: None,
        };
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "symbol=ETHUSDT");
    }

    #[test]
    fn test_recent_trade_response_deserialization() {
        let json = r#"[
            {
                "id": 1234567,
                "price": "45384.10",
                "qty": "0.003",
                "quoteQty": "136.1523",
                "time": 1625184000000,
                "isBuyerMaker": true
            },
            {
                "id": 1234568,
                "price": "45385.20",
                "qty": "0.005",
                "quoteQty": "226.926",
                "time": 1625184001000,
                "isBuyerMaker": false
            }
        ]"#;
        let trades: Vec<RecentTrade> = serde_json::from_str(json).unwrap();
        assert_eq!(trades.len(), 2);
        let first_trade = &trades[0];
        assert_eq!(first_trade.id, 1234567);
        assert_eq!(first_trade.price, "45384.10");
        assert_eq!(first_trade.qty, "0.003");
        assert_eq!(first_trade.quote_qty, "136.1523");
        assert_eq!(first_trade.time, 1625184000000);
        assert!(first_trade.is_buyer_maker);
        let second_trade = &trades[1];
        assert_eq!(second_trade.id, 1234568);
        assert_eq!(second_trade.price, "45385.20");
        assert_eq!(second_trade.qty, "0.005");
        assert_eq!(second_trade.quote_qty, "226.926");
        assert_eq!(second_trade.time, 1625184001000);
        assert!(!second_trade.is_buyer_maker);
    }

    #[test]
    fn test_recent_trade_large_volume() {
        let json = r#"[
            {
                "id": 9999999,
                "price": "45000.00",
                "qty": "100.000",
                "quoteQty": "4500000.00",
                "time": 1625184000000,
                "isBuyerMaker": false
            }
        ]"#;
        let trades: Vec<RecentTrade> = serde_json::from_str(json).unwrap();
        assert_eq!(trades.len(), 1);
        assert_eq!(trades[0].qty, "100.000");
        assert_eq!(trades[0].quote_qty, "4500000.00");
    }

    #[test]
    fn test_recent_trade_small_values() {
        let json = r#"[
            {
                "id": 1,
                "price": "0.00000001",
                "qty": "0.001",
                "quoteQty": "0.00000000001",
                "time": 1625184000000,
                "isBuyerMaker": true
            }
        ]"#;
        let trades: Vec<RecentTrade> = serde_json::from_str(json).unwrap();
        assert_eq!(trades.len(), 1);
        assert_eq!(trades[0].price, "0.00000001");
        assert_eq!(trades[0].qty, "0.001");
    }

    #[test]
    fn test_recent_trades_max_limit() {
        let request = RecentTradesRequest {
            symbol: "BTCUSDT".into(),
            limit: Some(1000), // Max limit
        };
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("limit=1000"));
    }

    #[test]
    fn test_recent_trades_empty_response() {
        let json = r#"[]"#;
        let trades: Vec<RecentTrade> = serde_json::from_str(json).unwrap();
        assert_eq!(trades.len(), 0);
    }

    #[test]
    fn test_recent_trades_result_deserialization() {
        let json = r#"[
            {
                "id": 28457,
                "price": "4.00000100",
                "qty": "12.00000000",
                "quoteQty": "48.00",
                "time": 1499865549590,
                "isBuyerMaker": true
            }
        ]"#;
        let result: RecentTradesResult = serde_json::from_str(json).unwrap();
        assert_eq!(result.trades.len(), 1);
        assert_eq!(result.trades[0].id, 28457);
        assert_eq!(result.trades[0].price, "4.00000100");
        assert_eq!(result.trades[0].qty, "12.00000000");
        assert_eq!(result.trades[0].quote_qty, "48.00");
        assert_eq!(result.trades[0].time, 1499865549590);
        assert!(result.trades[0].is_buyer_maker);
    }

    #[test]
    fn test_recent_trades_request_default() {
        let request = RecentTradesRequest::default();
        assert_eq!(request.symbol, "");
        assert_eq!(request.limit, None);
    }

    #[test]
    fn test_recent_trades_request_with_cow_static() {
        let request = RecentTradesRequest {
            symbol: Cow::Borrowed("BTCUSDT"),
            limit: Some(50),
        };
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTCUSDT"));
        assert!(serialized.contains("limit=50"));
    }

    #[test]
    fn test_recent_trade_field_names() {
        // Test that field names match the API exactly
        let json = r#"{
            "id": 28457,
            "price": "4.00000100",
            "qty": "12.00000000",
            "quoteQty": "48.00",
            "time": 1499865549590,
            "isBuyerMaker": true
        }"#;
        let trade: RecentTrade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.id, 28457);
        assert_eq!(trade.quote_qty, "48.00");
        assert!(trade.is_buyer_maker);
    }
}
