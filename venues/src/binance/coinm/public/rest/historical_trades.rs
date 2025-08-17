use serde::{Deserialize, Serialize};

use crate::binance::coinm::{RestResult, public::rest::RestClient};

const HISTORICAL_TRADES_ENDPOINT: &str = "/dapi/v1/historicalTrades";

/// Request parameters for the historical trades endpoint.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct HistoricalTradesRequest {
    /// Trading symbol (e.g., "BTCUSD_PERP").
    pub symbol: String,

    /// Default 100; max 500.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    /// TradeId to fetch from. Default gets most recent trades.
    #[serde(rename = "fromId", skip_serializing_if = "Option::is_none")]
    pub from_id: Option<u64>,
}

/// Represents a single historical trade.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoricalTrade {
    /// Trade ID.
    pub id: u64,

    /// Price of the trade.
    pub price: String,

    /// Quantity of the trade.
    pub qty: String,

    /// Base asset quantity.
    pub base_qty: String,

    /// Trade timestamp in milliseconds.
    pub time: u64,

    /// Whether the buyer was the maker.
    pub is_buyer_maker: bool,
}

impl RestClient {
    /// Old Trades Lookup(MARKET_DATA)
    ///
    /// Get older market historical trades.
    ///
    /// Market trades means trades filled in the order book. Only market trades will be
    /// returned, which means the insurance fund trades and ADL trades won't be
    /// returned.
    ///
    /// [docs](https://developers.binance.com/docs/derivatives/coin-margined-futures/market-data/rest-api/Old-Trades-Lookup)
    ///
    /// Rate limit: Weight 20
    ///
    /// # Arguments
    /// * `params` - The historical trades request parameters
    ///
    /// # Returns
    /// A vector of historical trades
    pub async fn get_historical_trades(
        &self,
        params: HistoricalTradesRequest,
    ) -> RestResult<Vec<HistoricalTrade>> {
        self.send_get_request(HISTORICAL_TRADES_ENDPOINT, Some(params), 20)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_historical_trades_request_serialization() {
        let request = HistoricalTradesRequest {
            symbol: "BTCUSD_PERP".to_string(),
            limit: Some(100),
            from_id: Some(28457),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTCUSD_PERP"));
        assert!(serialized.contains("limit=100"));
        assert!(serialized.contains("fromId=28457"));
    }

    #[test]
    fn test_historical_trades_request_serialization_minimal() {
        let request = HistoricalTradesRequest {
            symbol: "ETHUSD_PERP".to_string(),
            limit: None,
            from_id: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "symbol=ETHUSD_PERP");
    }

    #[test]
    fn test_historical_trade_response_deserialization() {
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

        let trades: Vec<HistoricalTrade> = serde_json::from_str(json).unwrap();
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
    fn test_historical_trade_large_values() {
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

        let trades: Vec<HistoricalTrade> = serde_json::from_str(json).unwrap();
        assert_eq!(trades.len(), 1);
        assert_eq!(trades[0].id, 999999999);
        assert_eq!(trades[0].price, "45000.50000000");
        assert_eq!(trades[0].qty, "1000.00000000");
        assert_eq!(trades[0].base_qty, "45000500.00000000");
    }

    #[test]
    fn test_historical_trade_empty_response() {
        let json = r#"[]"#;
        let trades: Vec<HistoricalTrade> = serde_json::from_str(json).unwrap();
        assert_eq!(trades.len(), 0);
    }
}
