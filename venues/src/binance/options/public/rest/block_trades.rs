use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::binance::options::{RestResult, public_client::RestClient};

const BLOCK_TRADES_ENDPOINT: &str = "/eapi/v1/blockTrades";

/// Request parameters for recent block trades
#[derive(Debug, Clone, Serialize, Default)]
pub struct BlockTradesRequest {
    /// Option trading pair, e.g. BTC-200730-9000-C
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,

    /// Number of records (Default: 100, Max: 500)
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

/// Recent block trade information
#[derive(Debug, Clone, Deserialize)]
pub struct BlockTrade {
    /// ID
    #[serde(rename = "id")]
    pub id: u64,

    /// Trade ID
    #[serde(rename = "tradeId")]
    pub trade_id: u64,

    /// Symbol
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Price
    #[serde(rename = "price")]
    pub price: Decimal,

    /// Quantity
    #[serde(rename = "qty")]
    pub qty: Decimal,

    /// Quote quantity
    #[serde(rename = "quoteQty")]
    pub quote_qty: Decimal,

    /// Side (-1 Sell, 1 Buy)
    #[serde(rename = "side")]
    pub side: i32,

    /// Time
    #[serde(rename = "time")]
    pub time: u64,
}

impl RestClient {
    /// Get recent block trades list
    ///
    /// Returns recent block trades.
    ///
    /// [docs](https://developers.binance.com/docs/derivatives/option/market-data/Recent-Block-Trade-List)
    ///
    /// Method: GET /eapi/v1/blockTrades
    /// Weight: 5
    /// Security: None
    pub async fn get_block_trades(
        &self,
        params: BlockTradesRequest,
    ) -> RestResult<Vec<BlockTrade>> {
        if params.symbol.is_none() && params.limit.is_none() {
            self.send_get_request(BLOCK_TRADES_ENDPOINT, None::<()>, 5)
                .await
        } else {
            self.send_get_request(BLOCK_TRADES_ENDPOINT, Some(params), 5)
                .await
        }
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal::prelude::FromPrimitive;

    use super::*;

    #[test]
    fn test_block_trades_request_serialization() {
        let request = BlockTradesRequest {
            symbol: None,
            limit: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "");
    }

    #[test]
    fn test_block_trades_request_serialization_with_symbol() {
        let request = BlockTradesRequest {
            symbol: Some("BTC-240329-70000-C".to_string()),
            limit: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "symbol=BTC-240329-70000-C");
    }

    #[test]
    fn test_block_trades_request_serialization_with_all_params() {
        let request = BlockTradesRequest {
            symbol: Some("ETH-240329-3000-P".to_string()),
            limit: Some(100),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=ETH-240329-3000-P"));
        assert!(serialized.contains("limit=100"));
    }

    #[test]
    fn test_block_trade_deserialization() {
        let json = r#"{
            "id": 123456,
            "tradeId": 789012,
            "symbol": "BTC-240329-70000-C",
            "price": "1500.00",
            "qty": "10.00",
            "quoteQty": "15000.00",
            "side": 1,
            "time": 1625097600000
        }"#;

        let trade: BlockTrade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.id, 123456);
        assert_eq!(trade.trade_id, 789012);
        assert_eq!(trade.symbol, "BTC-240329-70000-C");
        assert_eq!(trade.price, Decimal::from_f64(1500.00).unwrap());
        assert_eq!(trade.qty, Decimal::from_f64(10.00).unwrap());
        assert_eq!(trade.quote_qty, Decimal::from_f64(15000.00).unwrap());
        assert_eq!(trade.side, 1); // Buy
        assert_eq!(trade.time, 1625097600000);
    }

    #[test]
    fn test_block_trades_list_deserialization() {
        let json = r#"[
            {
                "id": 123456,
                "tradeId": 789012,
                "symbol": "BTC-240329-70000-C",
                "price": "1500.00",
                "qty": "10.00",
                "quoteQty": "15000.00",
                "side": 1,
                "time": 1625097600000
            },
            {
                "id": 123457,
                "tradeId": 789013,
                "symbol": "BTC-240329-70000-C",
                "price": "1520.00",
                "qty": "5.00",
                "quoteQty": "7600.00",
                "side": -1,
                "time": 1625097700000
            }
        ]"#;

        let trades: Vec<BlockTrade> = serde_json::from_str(json).unwrap();
        assert_eq!(trades.len(), 2);

        assert_eq!(trades[0].id, 123456);
        assert_eq!(trades[0].side, 1); // Buy
        assert_eq!(trades[0].price, Decimal::from_f64(1500.00).unwrap());

        assert_eq!(trades[1].id, 123457);
        assert_eq!(trades[1].side, -1); // Sell
        assert_eq!(trades[1].price, Decimal::from_f64(1520.00).unwrap());
    }
}
