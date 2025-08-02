//! BitMart query trades REST API endpoint
//!
//! This module implements the BitMart query trades API endpoint for retrieving account trade history.

use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::bitmart::{
    OrderMode, OrderSide, OrderType, RestResult, TradeRole, rate_limit::EndpointType,
};

const QUERY_TRADES_ENDPOINT: &str = "/spot/v4/query/trades";

/// Request parameters for querying account trades
#[derive(Debug, Serialize)]
pub struct QueryTradesRequest {
    /// Trading pair (optional, e.g. BTC_USDT)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// Order mode (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_mode: Option<OrderMode>,
    /// Start time in milliseconds (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    /// End time in milliseconds (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    /// Number of queries (optional, max 200, default 200)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// Query time window (optional, max 60000ms, default 5000ms)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<i64>,
}

/// Trade information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeInfo {
    /// Trade ID
    #[serde(rename = "tradeId")]
    pub trade_id: String,
    /// Order ID
    #[serde(rename = "orderId")]
    pub order_id: String,
    /// Client-defined Order ID
    #[serde(rename = "clientOrderId")]
    pub client_order_id: String,
    /// Trading pair (e.g. BTC_USDT)
    pub symbol: String,
    /// Order side
    pub side: OrderSide,
    /// Order mode
    #[serde(rename = "orderMode")]
    pub order_mode: OrderMode,
    /// Order type
    #[serde(rename = "type")]
    pub order_type: OrderType,
    /// Transaction price
    pub price: String,
    /// Transaction quantity
    pub size: String,
    /// Transaction amount
    pub notional: String,
    /// Fee amount
    pub fee: String,
    /// Fee coin name
    #[serde(rename = "feeCoinName")]
    pub fee_coin_name: String,
    /// Trade role (taker/maker)
    #[serde(rename = "tradeRole")]
    pub trade_role: TradeRole,
    /// Order creation time in milliseconds
    #[serde(rename = "createTime")]
    pub create_time: i64,
    /// Last update time in milliseconds
    #[serde(rename = "updateTime")]
    pub update_time: i64,
}

/// Response for querying account trades
pub type QueryTradesResponse = Vec<TradeInfo>;

impl RestClient {
    /// Account Trade List (v4)
    ///
    /// Retrieves the account's trade history based on filtering criteria.
    ///
    /// [docs]: https://developer-pro.bitmart.com/en/spot/#account-trade-listv4-signed
    ///
    /// Rate limit: varies by endpoint type
    ///
    /// # Arguments
    /// * `request` - The query trades request parameters
    ///
    /// # Returns
    /// List of trade information
    pub async fn query_trades(
        &self,
        request: QueryTradesRequest,
    ) -> RestResult<QueryTradesResponse> {
        self.send_post_signed_request(QUERY_TRADES_ENDPOINT, request,
            EndpointType::SpotTrading,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_trades_request_all_params() {
        let request = QueryTradesRequest {
            symbol: Some("BTC_USDT".to_string()),
            order_mode: Some(OrderMode::Spot),
            start_time: Some(1609459200000),
            end_time: Some(1609545600000),
            limit: Some(100),
            recv_window: Some(5000),
        };

        assert_eq!(request.symbol, Some("BTC_USDT".to_string()));
        assert_eq!(request.order_mode, Some(OrderMode::Spot));
        assert_eq!(request.start_time, Some(1609459200000));
        assert_eq!(request.end_time, Some(1609545600000));
        assert_eq!(request.limit, Some(100));
        assert_eq!(request.recv_window, Some(5000));
    }

    #[test]
    fn test_query_trades_request_minimal() {
        let request = QueryTradesRequest {
            symbol: None,
            order_mode: None,
            start_time: None,
            end_time: None,
            limit: None,
            recv_window: None,
        };

        assert!(request.symbol.is_none());
        assert!(request.order_mode.is_none());
        assert!(request.start_time.is_none());
        assert!(request.end_time.is_none());
        assert!(request.limit.is_none());
        assert!(request.recv_window.is_none());
    }

    #[test]
    fn test_trade_info_structure() {
        let json = r#"{
            "tradeId": "trade123",
            "orderId": "order123",
            "clientOrderId": "client123",
            "symbol": "BTC_USDT",
            "side": "buy",
            "orderMode": "spot",
            "type": "limit",
            "price": "50000.00",
            "size": "0.001",
            "notional": "50.00",
            "fee": "0.1",
            "feeCoinName": "USDT",
            "tradeRole": "taker",
            "createTime": 1609459200000,
            "updateTime": 1609459200000
        }"#;

        let trade: TradeInfo = serde_json::from_str(json).unwrap();
        assert_eq!(trade.trade_id, "trade123");
        assert_eq!(trade.order_id, "order123");
        assert_eq!(trade.client_order_id, "client123");
        assert_eq!(trade.symbol, "BTC_USDT");
        assert_eq!(trade.side, OrderSide::Buy);
        assert_eq!(trade.order_mode, OrderMode::Spot);
        assert_eq!(trade.order_type, OrderType::Limit);
        assert_eq!(trade.price, "50000.00");
        assert_eq!(trade.size, "0.001");
        assert_eq!(trade.notional, "50.00");
        assert_eq!(trade.fee, "0.1");
        assert_eq!(trade.fee_coin_name, "USDT");
        assert_eq!(trade.trade_role, TradeRole::Taker);
        assert_eq!(trade.create_time, 1609459200000);
        assert_eq!(trade.update_time, 1609459200000);
    }

    #[test]
    fn test_query_trades_response_serialization() {
        let json = r#"[{
            "tradeId": "trade456",
            "orderId": "order456",
            "clientOrderId": "client456",
            "symbol": "ETH_USDT",
            "side": "sell",
            "orderMode": "spot",
            "type": "market",
            "price": "3000.00",
            "size": "0.5",
            "notional": "1500.00",
            "fee": "3.0",
            "feeCoinName": "USDT",
            "tradeRole": "maker",
            "createTime": 1609459200000,
            "updateTime": 1609459200000
        }]"#;

        let response: QueryTradesResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.len(), 1);
        assert_eq!(response[0].trade_id, "trade456");
        assert_eq!(response[0].symbol, "ETH_USDT");
        assert_eq!(response[0].side, OrderSide::Sell);
        assert_eq!(response[0].trade_role, TradeRole::Maker);
    }
}
