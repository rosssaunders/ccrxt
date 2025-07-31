//! BitMart query order trades REST API endpoint
//!
//! This module implements the BitMart query order trades API endpoint for retrieving trades for a specific order.

use serde::Serialize;

use super::{client::RestClient, query_trades::TradeInfo};
use crate::bitmart::{RestResult, rate_limit::EndpointType};

const QUERY_ORDER_TRADES_ENDPOINT: &str = "/spot/v4/query/order-trades";

/// Request parameters for querying order trades
#[derive(Debug, Serialize)]
pub struct QueryOrderTradesRequest {
    /// Order ID (required)
    pub order_id: String,
    /// Query time window (optional, max 60000ms, default 5000ms)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<i64>,
}

/// Response for querying order trades
pub type QueryOrderTradesResponse = Vec<TradeInfo>;

impl RestClient {
    /// Order Trade List (v4)
    ///
    /// Retrieves the trade list for a specific order.
    ///
    /// [docs]: https://developer-pro.bitmart.com/en/spot/#order-trade-listv4-signed
    ///
    /// Rate limit: varies by endpoint type
    ///
    /// # Arguments
    /// * `request` - The query order trades request parameters
    ///
    /// # Returns
    /// List of trade information for the specified order
    pub async fn query_order_trades(
        &self,
        request: QueryOrderTradesRequest,
    ) -> RestResult<QueryOrderTradesResponse> {
        self.send_post_request(
            QUERY_ORDER_TRADES_ENDPOINT,
            Some(&request),
            EndpointType::SpotTrading,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_order_trades_request_minimal() {
        let request = QueryOrderTradesRequest {
            order_id: "12345".to_string(),
            recv_window: None,
        };

        assert_eq!(request.order_id, "12345");
        assert!(request.recv_window.is_none());
    }

    #[test]
    fn test_query_order_trades_request_with_recv_window() {
        let request = QueryOrderTradesRequest {
            order_id: "67890".to_string(),
            recv_window: Some(10000),
        };

        assert_eq!(request.order_id, "67890");
        assert_eq!(request.recv_window, Some(10000));
    }

    #[test]
    fn test_query_order_trades_response_empty() {
        let json = r#"[]"#;
        let response: QueryOrderTradesResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.len(), 0);
    }

    #[test]
    fn test_query_order_trades_response_with_data() {
        let json = r#"[{
            "tradeId": "trade789",
            "orderId": "order789",
            "clientOrderId": "client789",
            "symbol": "BTC_USDT",
            "side": "buy",
            "orderMode": "spot",
            "type": "limit",
            "price": "45000.00",
            "size": "0.002",
            "notional": "90.00",
            "fee": "0.18",
            "feeCoinName": "USDT",
            "tradeRole": "maker",
            "createTime": 1609459200000,
            "updateTime": 1609459200000
        }]"#;

        let response: QueryOrderTradesResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.len(), 1);
        assert_eq!(response[0].trade_id, "trade789");
        assert_eq!(response[0].order_id, "order789");
        assert_eq!(response[0].price, "45000.00");
        assert_eq!(response[0].size, "0.002");
    }
}
