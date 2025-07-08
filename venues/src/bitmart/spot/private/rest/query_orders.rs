//! BitMart query orders REST API endpoint
//!
//! This module implements the BitMart query orders API endpoint for retrieving order lists.

use serde::Serialize;

use super::client::RestClient;
use super::query_order::OrderDetails;
use crate::bitmart::rate_limit::EndpointType;
use crate::bitmart::{OrderMode, RestResult};

const QUERY_ORDERS_ENDPOINT: &str = "/spot/v4/query/orders";

/// Request parameters for querying order list
#[derive(Debug, Serialize)]
pub struct QueryOrdersRequest {
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

/// Response for querying order list
pub type QueryOrdersResponse = Vec<OrderDetails>;

impl RestClient {
    /// Query order list (v4)
    ///
    /// Retrieves a list of orders based on filtering criteria.
    ///
    /// See: https://raw.githubusercontent.com/rosssaunders/coincise/refs/heads/main/docs/bitmart/spot/spot___margin_trading.md
    ///
    /// Rate limit: varies by endpoint type
    ///
    /// # Arguments
    /// * `request` - The query orders request parameters
    ///
    /// # Returns
    /// List of order details
    pub async fn query_orders(
        &self,
        request: QueryOrdersRequest,
    ) -> RestResult<QueryOrdersResponse> {
        self.send_request(
            QUERY_ORDERS_ENDPOINT,
            reqwest::Method::POST,
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
    fn test_query_orders_request_all_params() {
        let request = QueryOrdersRequest {
            symbol: Some("BTC_USDT".to_string()),
            order_mode: Some(OrderMode::Spot),
            start_time: Some(1609459200000),
            end_time: Some(1609545600000),
            limit: Some(50),
            recv_window: Some(5000),
        };

        assert_eq!(request.symbol, Some("BTC_USDT".to_string()));
        assert_eq!(request.order_mode, Some(OrderMode::Spot));
        assert_eq!(request.start_time, Some(1609459200000));
        assert_eq!(request.end_time, Some(1609545600000));
        assert_eq!(request.limit, Some(50));
        assert_eq!(request.recv_window, Some(5000));
    }

    #[test]
    fn test_query_orders_request_minimal() {
        let request = QueryOrdersRequest {
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
    fn test_query_orders_request_symbol_only() {
        let request = QueryOrdersRequest {
            symbol: Some("ETH_USDT".to_string()),
            order_mode: None,
            start_time: None,
            end_time: None,
            limit: None,
            recv_window: None,
        };

        assert_eq!(request.symbol, Some("ETH_USDT".to_string()));
        assert!(request.order_mode.is_none());
    }
}
