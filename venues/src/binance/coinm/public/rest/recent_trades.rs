use serde::{Deserialize, Serialize};

use crate::binance::coinm::{RestResult, public::rest::RestClient};

/// Endpoint path for recent trades list.
const RECENT_TRADES_ENDPOINT: &str = "/dapi/v1/trades";

/// Request parameters for the recent trades endpoint.
///
/// Retrieves recent market trades for a given symbol. Only trades filled in the order book are returned.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RecentTradesRequest {
    /// Trading symbol to query (e.g., "BTCUSD_PERP").
    /// Must be a valid symbol listed on Binance Coin-M Futures.
    ///
    /// Required.
    pub symbol: String,

    /// Number of trades to return. Default is 500; maximum is 1000.
    ///
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

/// Represents a single trade returned by the recent trades endpoint.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Trade {
    /// Unique trade identifier.
    pub id: u64,

    /// Price at which the trade was executed.
    pub price: String,

    /// Quantity traded.
    pub qty: String,

    /// Base asset quantity for the trade.
    pub base_qty: String,

    /// Trade timestamp in milliseconds since epoch.
    pub time: u64,

    /// True if the buyer is the market maker; false otherwise.
    pub is_buyer_maker: bool,
}

impl RestClient {
    /// Recent Trades List
    ///
    /// Retrieves recent market trades for a given symbol. Only trades filled in the order book are returned; insurance fund trades and ADL trades are excluded.
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/coin-margined-futures/market-data/rest-api/Recent-Trades-List
    ///
    /// Rate limit: 5
    ///
    /// # Arguments
    /// * `params` - Parameters for the recent trades request
    ///
    /// # Returns
    /// A vector of `Trade` structs representing recent trades for the symbol.
    pub async fn get_recent_trades(&self, params: RecentTradesRequest) -> RestResult<Vec<Trade>> {
        self.send_get_request(RECENT_TRADES_ENDPOINT, Some(params), 5)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recent_trades_request_serialization() {
        let request = RecentTradesRequest {
            symbol: "BTCUSD_PERP".to_string(),
            limit: Some(100),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTCUSD_PERP"));
        assert!(serialized.contains("limit=100"));
    }

    #[test]
    fn test_recent_trades_request_serialization_minimal() {
        let request = RecentTradesRequest {
            symbol: "ETHUSD_PERP".to_string(),
            limit: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "symbol=ETHUSD_PERP");
    }

    #[test]
    fn test_trade_response_deserialization() {
        let json = r#"[
            {
                "id": 28457,
                "price": "4.00000100",
                "qty": "12.00000000",
                "baseQty": "48.00000400",
                "time": 1499865549590,
                "isBuyerMaker": true
            },
            {
                "id": 28458,
                "price": "4.00000200",
                "qty": "8.00000000",
                "baseQty": "32.00001600",
                "time": 1499865549591,
                "isBuyerMaker": false
            }
        ]"#;

        let trades: Vec<Trade> = serde_json::from_str(json).unwrap();
        assert_eq!(trades.len(), 2);

        let first_trade = &trades[0];
        assert_eq!(first_trade.id, 28457);
        assert_eq!(first_trade.price, "4.00000100");
        assert_eq!(first_trade.qty, "12.00000000");
        assert_eq!(first_trade.base_qty, "48.00000400");
        assert_eq!(first_trade.time, 1499865549590);
        assert!(first_trade.is_buyer_maker);

        let second_trade = &trades[1];
        assert_eq!(second_trade.id, 28458);
        assert_eq!(second_trade.price, "4.00000200");
        assert_eq!(second_trade.qty, "8.00000000");
        assert_eq!(second_trade.base_qty, "32.00001600");
        assert_eq!(second_trade.time, 1499865549591);
        assert!(!second_trade.is_buyer_maker);
    }

    #[test]
    fn test_trade_large_values() {
        let json = r#"[
            {
                "id": 999999999,
                "price": "45000.50000000",
                "qty": "1000.00000000",
                "baseQty": "45000500.00000000",
                "time": 1625184000000,
                "isBuyerMaker": false
            }
        ]"#;

        let trades: Vec<Trade> = serde_json::from_str(json).unwrap();
        assert_eq!(trades.len(), 1);
        assert_eq!(trades[0].id, 999999999);
        assert_eq!(trades[0].price, "45000.50000000");
        assert_eq!(trades[0].qty, "1000.00000000");
        assert_eq!(trades[0].base_qty, "45000500.00000000");
    }

    #[test]
    fn test_trade_empty_response() {
        let json = r#"[]"#;
        let trades: Vec<Trade> = serde_json::from_str(json).unwrap();
        assert_eq!(trades.len(), 0);
    }

    #[test]
    fn test_trade_max_limit() {
        let request = RecentTradesRequest {
            symbol: "BTCUSD_PERP".to_string(),
            limit: Some(1000), // Max limit
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("limit=1000"));
    }
}
