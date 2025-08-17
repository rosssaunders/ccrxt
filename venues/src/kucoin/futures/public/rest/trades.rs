use serde::{Deserialize, Serialize};

use crate::kucoin::spot::{OrderSide, ResponseHeaders, RestResponse, Result};

// API endpoints
const TRADE_HISTORY_ENDPOINT: &str = "/api/v1/trade/history";

/// Get trade history request
#[derive(Debug, Clone, Serialize)]
pub struct GetTradeHistoryRequest {
    pub symbol: String,
}

/// Trade history item
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeHistoryItem {
    /// Sequence number
    pub sequence: i64,

    /// Deprecated param
    pub contract_id: i64,

    /// Transaction ID
    pub trade_id: String,

    /// Maker order ID
    pub maker_order_id: String,

    /// Taker order ID
    pub taker_order_id: String,

    /// Filled timestamp (nanosecond)
    pub ts: i64,

    /// Filled amount
    pub size: i64,

    /// Filled price
    pub price: String,

    /// Trade side (taker order side)
    pub side: OrderSide,
}

/// Response for getting trade history
pub type GetTradeHistoryResponse = Vec<TradeHistoryItem>;

impl super::RestClient {
    /// Get trade history for a specific symbol (last 100 records)
    ///
    /// [docs](https://www.kucoin.com/docs-new/rest/futures-trading/market-data/get-trade-history)
    pub async fn get_trade_history(
        &self,
        request: GetTradeHistoryRequest,
    ) -> Result<(RestResponse<GetTradeHistoryResponse>, ResponseHeaders)> {
        self.send_request(TRADE_HISTORY_ENDPOINT, Some(&request))
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_trade_history_request() {
        let request = GetTradeHistoryRequest {
            symbol: "XBTUSDTM".to_string(),
        };
        assert_eq!(request.symbol, "XBTUSDTM");
    }

    #[test]
    fn test_trade_history_item_deserialization() {
        let json = r#"{
            "sequence": 1234567890,
            "contractId": 1,
            "tradeId": "5e8c8c2f1a3b4a001c5d8e31",
            "makerOrderId": "5e8c8c2f1a3b4a001c5d8e32",
            "takerOrderId": "5e8c8c2f1a3b4a001c5d8e33",
            "ts": 1634567890123456789,
            "size": 100,
            "price": "50000.0",
            "side": "buy"
        }"#;

        let trade: TradeHistoryItem = serde_json::from_str(json).unwrap();
        assert_eq!(trade.sequence, 1234567890);
        assert_eq!(trade.trade_id, "5e8c8c2f1a3b4a001c5d8e31");
        assert_eq!(trade.size, 100);
        assert_eq!(trade.price, "50000.0");
        assert_eq!(trade.side, OrderSide::Buy);
    }

    #[test]
    fn test_trade_history_response_deserialization() {
        let json = r#"[
            {
                "sequence": 1234567890,
                "contractId": 1,
                "tradeId": "5e8c8c2f1a3b4a001c5d8e31",
                "makerOrderId": "5e8c8c2f1a3b4a001c5d8e32",
                "takerOrderId": "5e8c8c2f1a3b4a001c5d8e33",
                "ts": 1634567890123456789,
                "size": 100,
                "price": "50000.0",
                "side": "buy"
            },
            {
                "sequence": 1234567891,
                "contractId": 1,
                "tradeId": "5e8c8c2f1a3b4a001c5d8e34",
                "makerOrderId": "5e8c8c2f1a3b4a001c5d8e35",
                "takerOrderId": "5e8c8c2f1a3b4a001c5d8e36",
                "ts": 1634567890223456789,
                "size": 50,
                "price": "50001.0",
                "side": "sell"
            }
        ]"#;

        let trades: GetTradeHistoryResponse = serde_json::from_str(json).unwrap();
        assert_eq!(trades.len(), 2);
        assert_eq!(trades[0].trade_id, "5e8c8c2f1a3b4a001c5d8e31");
        assert_eq!(trades[1].side, OrderSide::Sell);
    }
}
