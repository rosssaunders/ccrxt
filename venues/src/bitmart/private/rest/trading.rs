//! BitMart spot and margin trading REST API endpoints
//!
//! This module implements the BitMart trading API endpoints for submitting orders,
//! canceling orders, and querying order and trade information.

use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::bitmart::{OrderMode, OrderSide, OrderStatus, OrderType, RestResult, TradeRole};
use crate::bitmart::rate_limit::EndpointType;

/// Request parameters for submitting a new order
#[derive(Debug, Serialize)]
pub struct SubmitOrderRequest {
    /// Trading pair (e.g. BTC_USDT)
    pub symbol: String,
    /// Order side (buy/sell)
    pub side: OrderSide,
    /// Order type (limit/market/limit_maker/ioc)
    #[serde(rename = "type")]
    pub order_type: OrderType,
    /// Client-defined OrderId (optional, max 32 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    /// Order size (required for limit/limit_maker/ioc and market sell orders)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    /// Price (required for limit/limit_maker/ioc orders)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    /// Notional amount (required for market buy orders)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notional: Option<String>,
}

/// Response for submitting a new order
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitOrderResponse {
    /// Order ID
    pub order_id: String,
}

/// Request parameters for canceling an order
#[derive(Debug, Serialize)]
pub struct CancelOrderRequest {
    /// Trading pair (e.g. BTC_USDT)
    pub symbol: String,
    /// Order ID (required if client_order_id not provided)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// Client-defined Order ID (required if order_id not provided)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
}

/// Response for canceling an order
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelOrderResponse {
    /// Whether the cancellation was successful
    pub result: bool,
}

/// Request parameters for querying order details
#[derive(Debug, Serialize)]
pub struct QueryOrderRequest {
    /// Order ID (required if client_order_id not provided)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// Client-defined Order ID (required if order_id not provided) 
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    /// Query time window (optional, max 60000ms, default 5000ms)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<i64>,
}

/// Order details information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderDetails {
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
    /// Order status
    pub state: OrderStatus,
    /// Order price
    pub price: String,
    /// Average filled price
    #[serde(rename = "priceAvg")]
    pub price_avg: String,
    /// Order size
    pub size: String,
    /// Filled size
    #[serde(rename = "filledSize")]
    pub filled_size: String,
    /// Notional amount
    pub notional: String,
    /// Filled notional amount
    #[serde(rename = "filledNotional")]
    pub filled_notional: String,
    /// Order creation time in milliseconds
    #[serde(rename = "createTime")]
    pub create_time: i64,
    /// Last update time in milliseconds
    #[serde(rename = "updateTime")]
    pub update_time: i64,
}

/// Response for querying order details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryOrderResponse {
    /// Order details
    #[serde(flatten)]
    pub order: OrderDetails,
}

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
    /// Submit a new order (v2)
    ///
    /// Places a new order on the BitMart exchange.
    ///
    /// See: https://raw.githubusercontent.com/rosssaunders/coincise/refs/heads/main/docs/bitmart/spot/spot___margin_trading.md
    ///
    /// Rate limit: varies by endpoint type
    ///
    /// # Arguments
    /// * `request` - The order request parameters
    ///
    /// # Returns
    /// Order submission response with order ID
    pub async fn submit_order(&self, request: SubmitOrderRequest) -> RestResult<SubmitOrderResponse> {
        self.send_request(
            "/spot/v2/submit_order",
            reqwest::Method::POST,
            Some(&request),
            EndpointType::SpotTrading,
        )
        .await
    }

    /// Cancel an order (v3)
    ///
    /// Cancels a specified unfinished order.
    ///
    /// See: https://raw.githubusercontent.com/rosssaunders/coincise/refs/heads/main/docs/bitmart/spot/spot___margin_trading.md
    ///
    /// Rate limit: varies by endpoint type
    ///
    /// # Arguments
    /// * `request` - The cancel order request parameters
    ///
    /// # Returns
    /// Cancel order response with result status
    pub async fn cancel_order(&self, request: CancelOrderRequest) -> RestResult<CancelOrderResponse> {
        self.send_request(
            "/spot/v3/cancel_order",
            reqwest::Method::POST,
            Some(&request),
            EndpointType::SpotTrading,
        )
        .await
    }

    /// Query order details (v4)
    ///
    /// Retrieves details for a specific order.
    ///
    /// See: https://raw.githubusercontent.com/rosssaunders/coincise/refs/heads/main/docs/bitmart/spot/spot___margin_trading.md
    ///
    /// Rate limit: varies by endpoint type
    ///
    /// # Arguments
    /// * `request` - The query order request parameters
    ///
    /// # Returns
    /// Order details response
    pub async fn query_order(&self, request: QueryOrderRequest) -> RestResult<QueryOrderResponse> {
        self.send_request(
            "/spot/v4/query/order",
            reqwest::Method::POST,
            Some(&request),
            EndpointType::SpotTrading,
        )
        .await
    }

    /// Query order list (v4)
    ///
    /// Retrieves a list of orders based on specified criteria.
    ///
    /// See: https://raw.githubusercontent.com/rosssaunders/coincise/refs/heads/main/docs/bitmart/spot/spot___margin_trading.md
    ///
    /// Rate limit: varies by endpoint type
    ///
    /// # Arguments
    /// * `request` - The query orders request parameters
    ///
    /// # Returns
    /// Order list response
    pub async fn query_orders(&self, request: QueryOrdersRequest) -> RestResult<QueryOrdersResponse> {
        self.send_request(
            "/spot/v4/query/orders",
            reqwest::Method::POST,
            Some(&request),
            EndpointType::SpotTrading,
        )
        .await
    }

    /// Query account trade list (v4)
    ///
    /// Retrieves all transaction records of the account.
    ///
    /// See: https://raw.githubusercontent.com/rosssaunders/coincise/refs/heads/main/docs/bitmart/spot/spot___margin_trading.md
    ///
    /// Rate limit: varies by endpoint type
    ///
    /// # Arguments
    /// * `request` - The query trades request parameters
    ///
    /// # Returns
    /// Account trades response
    pub async fn query_trades(&self, request: QueryTradesRequest) -> RestResult<QueryTradesResponse> {
        self.send_request(
            "/spot/v4/query/trades",
            reqwest::Method::POST,
            Some(&request),
            EndpointType::SpotTrading,
        )
        .await
    }

    /// Query order trade list (v4)
    ///
    /// Retrieves all transaction records of a single order.
    ///
    /// See: https://raw.githubusercontent.com/rosssaunders/coincise/refs/heads/main/docs/bitmart/spot/spot___margin_trading.md
    ///
    /// Rate limit: varies by endpoint type
    ///
    /// # Arguments
    /// * `request` - The query order trades request parameters
    ///
    /// # Returns
    /// Order trades response
    pub async fn query_order_trades(&self, request: QueryOrderTradesRequest) -> RestResult<QueryOrderTradesResponse> {
        self.send_request(
            "/spot/v4/query/order-trades",
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
    fn test_submit_order_request_limit_order() {
        let request = SubmitOrderRequest {
            symbol: "BTC_USDT".to_string(),
            side: OrderSide::Buy,
            order_type: OrderType::Limit,
            client_order_id: Some("my_order_123".to_string()),
            size: Some("0.001".to_string()),
            price: Some("50000.00".to_string()),
            notional: None,
        };

        assert_eq!(request.symbol, "BTC_USDT");
        assert_eq!(request.side, OrderSide::Buy);
        assert_eq!(request.order_type, OrderType::Limit);
        assert_eq!(request.client_order_id, Some("my_order_123".to_string()));
        assert_eq!(request.size, Some("0.001".to_string()));
        assert_eq!(request.price, Some("50000.00".to_string()));
        assert!(request.notional.is_none());
    }

    #[test]
    fn test_submit_order_request_market_buy() {
        let request = SubmitOrderRequest {
            symbol: "BTC_USDT".to_string(),
            side: OrderSide::Buy,
            order_type: OrderType::Market,
            client_order_id: None,
            size: None,
            price: None,
            notional: Some("100.00".to_string()),
        };

        assert_eq!(request.order_type, OrderType::Market);
        assert_eq!(request.side, OrderSide::Buy);
        assert_eq!(request.notional, Some("100.00".to_string()));
        assert!(request.size.is_none());
        assert!(request.price.is_none());
    }

    #[test]
    fn test_submit_order_request_market_sell() {
        let request = SubmitOrderRequest {
            symbol: "BTC_USDT".to_string(),
            side: OrderSide::Sell,
            order_type: OrderType::Market,
            client_order_id: None,
            size: Some("0.001".to_string()),
            price: None,
            notional: None,
        };

        assert_eq!(request.order_type, OrderType::Market);
        assert_eq!(request.side, OrderSide::Sell);
        assert_eq!(request.size, Some("0.001".to_string()));
        assert!(request.notional.is_none());
        assert!(request.price.is_none());
    }

    #[test]
    fn test_submit_order_request_limit_maker() {
        let request = SubmitOrderRequest {
            symbol: "ETH_USDT".to_string(),
            side: OrderSide::Sell,
            order_type: OrderType::LimitMaker,
            client_order_id: Some("maker_order_456".to_string()),
            size: Some("0.5".to_string()),
            price: Some("3000.00".to_string()),
            notional: None,
        };

        assert_eq!(request.order_type, OrderType::LimitMaker);
        assert_eq!(request.side, OrderSide::Sell);
        assert_eq!(request.size, Some("0.5".to_string()));
        assert_eq!(request.price, Some("3000.00".to_string()));
    }

    #[test]
    fn test_submit_order_request_ioc() {
        let request = SubmitOrderRequest {
            symbol: "ETH_USDT".to_string(),
            side: OrderSide::Buy,
            order_type: OrderType::Ioc,
            client_order_id: None,
            size: Some("1.0".to_string()),
            price: Some("2950.00".to_string()),
            notional: None,
        };

        assert_eq!(request.order_type, OrderType::Ioc);
        assert_eq!(request.side, OrderSide::Buy);
        assert_eq!(request.size, Some("1.0".to_string()));
        assert_eq!(request.price, Some("2950.00".to_string()));
    }

    #[test]
    fn test_cancel_order_request_by_order_id() {
        let request = CancelOrderRequest {
            symbol: "BTC_USDT".to_string(),
            order_id: Some("123456789".to_string()),
            client_order_id: None,
        };

        assert_eq!(request.symbol, "BTC_USDT");
        assert_eq!(request.order_id, Some("123456789".to_string()));
        assert!(request.client_order_id.is_none());
    }

    #[test]
    fn test_cancel_order_request_by_client_order_id() {
        let request = CancelOrderRequest {
            symbol: "BTC_USDT".to_string(),
            order_id: None,
            client_order_id: Some("my_order_123".to_string()),
        };

        assert_eq!(request.symbol, "BTC_USDT");
        assert!(request.order_id.is_none());
        assert_eq!(request.client_order_id, Some("my_order_123".to_string()));
    }

    #[test]
    fn test_query_order_request_by_order_id() {
        let request = QueryOrderRequest {
            order_id: Some("123456789".to_string()),
            client_order_id: None,
            recv_window: Some(10000),
        };

        assert_eq!(request.order_id, Some("123456789".to_string()));
        assert!(request.client_order_id.is_none());
        assert_eq!(request.recv_window, Some(10000));
    }

    #[test]
    fn test_query_order_request_by_client_order_id() {
        let request = QueryOrderRequest {
            order_id: None,
            client_order_id: Some("my_order_123".to_string()),
            recv_window: None,
        };

        assert!(request.order_id.is_none());
        assert_eq!(request.client_order_id, Some("my_order_123".to_string()));
        assert!(request.recv_window.is_none());
    }

    #[test]
    fn test_query_orders_request_default() {
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
    fn test_query_orders_request_with_filters() {
        let request = QueryOrdersRequest {
            symbol: Some("BTC_USDT".to_string()),
            order_mode: Some(OrderMode::Spot),
            start_time: Some(1681701557927),
            end_time: Some(1681701557927 + 86400000), // +1 day
            limit: Some(50),
            recv_window: Some(10000),
        };

        assert_eq!(request.symbol, Some("BTC_USDT".to_string()));
        assert_eq!(request.order_mode, Some(OrderMode::Spot));
        assert_eq!(request.start_time, Some(1681701557927));
        assert_eq!(request.end_time, Some(1681701557927 + 86400000));
        assert_eq!(request.limit, Some(50));
        assert_eq!(request.recv_window, Some(10000));
    }

    #[test]
    fn test_query_trades_request_default() {
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
    fn test_query_trades_request_with_filters() {
        let request = QueryTradesRequest {
            symbol: Some("ETH_USDT".to_string()),
            order_mode: Some(OrderMode::IsoMargin),
            start_time: Some(1681701557927),
            end_time: Some(1681701557927 + 86400000), // +1 day
            limit: Some(100),
            recv_window: Some(15000),
        };

        assert_eq!(request.symbol, Some("ETH_USDT".to_string()));
        assert_eq!(request.order_mode, Some(OrderMode::IsoMargin));
        assert_eq!(request.start_time, Some(1681701557927));
        assert_eq!(request.end_time, Some(1681701557927 + 86400000));
        assert_eq!(request.limit, Some(100));
        assert_eq!(request.recv_window, Some(15000));
    }

    #[test]
    fn test_query_order_trades_request() {
        let request = QueryOrderTradesRequest {
            order_id: "118100034543076010".to_string(),
            recv_window: Some(5000),
        };

        assert_eq!(request.order_id, "118100034543076010");
        assert_eq!(request.recv_window, Some(5000));
    }

    #[test]
    fn test_order_details_structure() {
        let order = OrderDetails {
            order_id: "123456789".to_string(),
            client_order_id: "my_order_123".to_string(),
            symbol: "BTC_USDT".to_string(),
            side: OrderSide::Buy,
            order_mode: OrderMode::Spot,
            order_type: OrderType::Limit,
            state: OrderStatus::Filled,
            price: "50000.00".to_string(),
            price_avg: "50000.00".to_string(),
            size: "0.001".to_string(),
            filled_size: "0.001".to_string(),
            notional: "50.00".to_string(),
            filled_notional: "50.00".to_string(),
            create_time: 1681701557927,
            update_time: 1681701557927,
        };

        assert_eq!(order.order_id, "123456789");
        assert_eq!(order.client_order_id, "my_order_123");
        assert_eq!(order.symbol, "BTC_USDT");
        assert_eq!(order.side, OrderSide::Buy);
        assert_eq!(order.order_mode, OrderMode::Spot);
        assert_eq!(order.order_type, OrderType::Limit);
        assert_eq!(order.state, OrderStatus::Filled);
        assert_eq!(order.price, "50000.00");
        assert_eq!(order.price_avg, "50000.00");
        assert_eq!(order.size, "0.001");
        assert_eq!(order.filled_size, "0.001");
        assert_eq!(order.notional, "50.00");
        assert_eq!(order.filled_notional, "50.00");
        assert_eq!(order.create_time, 1681701557927);
        assert_eq!(order.update_time, 1681701557927);
    }

    #[test]
    fn test_trade_info_structure() {
        let trade = TradeInfo {
            trade_id: "125277182593091639".to_string(),
            order_id: "125213058731346053".to_string(),
            client_order_id: "125213058731346053".to_string(),
            symbol: "BTC_USDT".to_string(),
            side: OrderSide::Buy,
            order_mode: OrderMode::Spot,
            order_type: OrderType::Limit,
            price: "39999.00".to_string(),
            size: "0.10000".to_string(),
            notional: "3999.90000000".to_string(),
            fee: "9.99975000".to_string(),
            fee_coin_name: "USDT".to_string(),
            trade_role: TradeRole::Taker,
            create_time: 1681891896569,
            update_time: 1681891896569,
        };

        assert_eq!(trade.trade_id, "125277182593091639");
        assert_eq!(trade.order_id, "125213058731346053");
        assert_eq!(trade.client_order_id, "125213058731346053");
        assert_eq!(trade.symbol, "BTC_USDT");
        assert_eq!(trade.side, OrderSide::Buy);
        assert_eq!(trade.order_mode, OrderMode::Spot);
        assert_eq!(trade.order_type, OrderType::Limit);
        assert_eq!(trade.price, "39999.00");
        assert_eq!(trade.size, "0.10000");
        assert_eq!(trade.notional, "3999.90000000");
        assert_eq!(trade.fee, "9.99975000");
        assert_eq!(trade.fee_coin_name, "USDT");
        assert_eq!(trade.trade_role, TradeRole::Taker);
        assert_eq!(trade.create_time, 1681891896569);
        assert_eq!(trade.update_time, 1681891896569);
    }

    #[test]
    fn test_submit_order_response_structure() {
        let response = SubmitOrderResponse {
            order_id: "1223181".to_string(),
        };

        assert_eq!(response.order_id, "1223181");
    }

    #[test]
    fn test_cancel_order_response_structure() {
        let response = CancelOrderResponse {
            result: true,
        };

        assert!(response.result);
    }

    #[test]
    fn test_submit_order_response_json_parsing() {
        let json = r#"{"order_id":"1223181"}"#;
        let response: SubmitOrderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.order_id, "1223181");
    }

    #[test]
    fn test_cancel_order_response_json_parsing() {
        let json = r#"{"result":true}"#;
        let response: CancelOrderResponse = serde_json::from_str(json).unwrap();
        assert!(response.result);
    }

    #[test]
    fn test_query_trades_response_json_parsing() {
        let json = r#"[
            {
                "tradeId": "125277182593091639",
                "orderId": "125213058731346053",
                "clientOrderId": "125213058731346053",
                "symbol": "BTC_USDT",
                "side": "buy",
                "orderMode": "spot",
                "type": "limit",
                "price": "39999.00",
                "size": "0.10000",
                "notional": "3999.90000000",
                "fee": "9.99975000",
                "feeCoinName": "USDT",
                "tradeRole": "taker",
                "createTime": 1681891896569,
                "updateTime": 1681891896569
            }
        ]"#;

        let response: QueryTradesResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.len(), 1);
        assert_eq!(response[0].trade_id, "125277182593091639");
        assert_eq!(response[0].order_id, "125213058731346053");
        assert_eq!(response[0].symbol, "BTC_USDT");
        assert_eq!(response[0].side, OrderSide::Buy);
        assert_eq!(response[0].order_mode, OrderMode::Spot);
        assert_eq!(response[0].order_type, OrderType::Limit);
        assert_eq!(response[0].trade_role, TradeRole::Taker);
    }
}