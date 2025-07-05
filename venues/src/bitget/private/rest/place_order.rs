//! Place Order endpoint for Bitget Spot API
//!
//! This endpoint allows placing spot trading orders.
//!
//! Reference: https://www.bitget.com/api-doc/spot/trade/Place-Order
//! Endpoint: POST /api/v2/spot/trade/place-order
//! Rate limit: 10 requests/second/UID

use serde::{Deserialize, Serialize};

use super::super::RestClient;
use crate::bitget::{OrderSide, OrderType, RestResult};

/// Order execution strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Force {
    /// Good Till Cancel - normal limit order
    #[serde(rename = "gtc")]
    #[allow(clippy::upper_case_acronyms)]
    GTC,
    /// Post only - only add liquidity to the book
    #[serde(rename = "post_only")]
    PostOnly,
    /// Fill or Kill - fill completely or cancel
    #[serde(rename = "fok")]
    #[allow(clippy::upper_case_acronyms)]
    FOK,
    /// Immediate or Cancel - fill as much as possible, cancel remainder
    #[serde(rename = "ioc")]
    #[allow(clippy::upper_case_acronyms)]
    IOC,
}

/// Self-trade prevention mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum STPMode {
    /// No self-trade prevention
    #[serde(rename = "none")]
    None,
    /// Cancel taker order when self-trade occurs
    #[serde(rename = "cancel_taker")]
    CancelTaker,
    /// Cancel maker order when self-trade occurs
    #[serde(rename = "cancel_maker")]
    CancelMaker,
    /// Cancel both orders when self-trade occurs
    #[serde(rename = "cancel_both")]
    CancelBoth,
}

/// Request parameters for placing a spot order
#[derive(Debug, Clone, Serialize)]
pub struct PlaceOrderRequest {
    /// Trading pair name, e.g. BTCUSDT
    pub symbol: String,

    /// Order direction: buy or sell
    pub side: OrderSide,

    /// Order type: limit or market
    #[serde(rename = "orderType")]
    pub order_type: OrderType,

    /// Execution strategy (invalid when orderType is market)
    pub force: Force,

    /// Limit price (required for limit orders)
    /// The decimal places of price can be obtained from Get Symbol Info
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,

    /// Order amount
    /// For Limit and Market-Sell orders: represents the number of base coins
    /// For Market-Buy orders: represents the number of quote coins
    pub size: String,

    /// Custom order ID (optional)
    #[serde(rename = "clientOid", skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,

    /// Self-trade prevention mode
    #[serde(rename = "stpMode", skip_serializing_if = "Option::is_none")]
    pub stp_mode: Option<STPMode>,

    /// Request timestamp (Unix milliseconds)
    #[serde(rename = "requestTime", skip_serializing_if = "Option::is_none")]
    pub request_time: Option<i64>,

    /// Valid time window (Unix milliseconds)
    /// If set, request is valid only when server time is within receiveWindow
    #[serde(rename = "receiveWindow", skip_serializing_if = "Option::is_none")]
    pub receive_window: Option<i64>,
}

/// Response from placing an order
#[derive(Debug, Clone, Deserialize)]
pub struct PlaceOrderResponse {
    /// Order ID assigned by the system
    #[serde(rename = "orderId")]
    pub order_id: String,

    /// Custom order ID (if provided in request)
    #[serde(rename = "clientOid")]
    pub client_order_id: Option<String>,
}

impl RestClient {
    /// Place a spot trading order
    ///
    /// Places a new order for spot trading with the specified parameters.
    ///
    /// # Arguments
    /// * `request` - The order placement request parameters
    ///
    /// # Rate Limit
    /// 10 requests per second per UID
    /// 1 request per second per UID for copy trading traders
    ///
    /// # Returns
    /// A result containing the order placement response or an error
    pub async fn place_order(&self, request: PlaceOrderRequest) -> RestResult<PlaceOrderResponse> {
        let body = serde_json::to_string(&request).map_err(|e| {
            crate::bitget::Errors::Error(format!("Failed to serialize request: {e}"))
        })?;

        self.send_signed_request(
            "/api/v2/spot/trade/place-order",
            reqwest::Method::POST,
            None,        // No query parameters
            Some(&body), // JSON body
            10,          // 10 requests per second rate limit
            true,        // This is an order endpoint
            Some(10),    // Order-specific rate limit
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_place_order_request_limit() {
        let request = PlaceOrderRequest {
            symbol: "BTCUSDT".to_string(),
            side: OrderSide::Buy,
            price: Some("50000".to_string()),
            size: "0.001".to_string(),
            order_type: OrderType::Limit,
            force: Force::GTC,
            client_order_id: None,
            stp_mode: None,
            request_time: None,
            receive_window: None,
        };

        assert_eq!(request.symbol, "BTCUSDT");
        assert_eq!(request.side, OrderSide::Buy);
        assert_eq!(request.order_type, OrderType::Limit);
        assert_eq!(request.force, Force::GTC);
        assert_eq!(request.price, Some("50000".to_string()));
        assert_eq!(request.size, "0.001");
        assert!(request.client_order_id.is_none());
    }

    #[test]
    fn test_place_order_request_market() {
        let request = PlaceOrderRequest {
            symbol: "ETHUSDT".to_string(),
            side: OrderSide::Sell,
            price: None,
            size: "1.0".to_string(),
            order_type: OrderType::Market,
            force: Force::GTC,
            client_order_id: None,
            stp_mode: None,
            request_time: None,
            receive_window: None,
        };

        assert_eq!(request.symbol, "ETHUSDT");
        assert_eq!(request.side, OrderSide::Sell);
        assert_eq!(request.order_type, OrderType::Market);
        assert!(request.price.is_none());
        assert_eq!(request.size, "1.0");
    }

    #[test]
    fn test_place_order_request_builder() {
        let request = PlaceOrderRequest {
            symbol: "BTCUSDT".to_string(),
            side: OrderSide::Buy,
            price: Some("50000".to_string()),
            size: "0.001".to_string(),
            order_type: OrderType::Limit,
            force: Force::PostOnly,
            client_order_id: Some("my-order-123".to_string()),
            stp_mode: Some(STPMode::CancelTaker),
            request_time: None,
            receive_window: None,
        };

        assert_eq!(request.force, Force::PostOnly);
        assert_eq!(request.client_order_id, Some("my-order-123".to_string()));
        assert_eq!(request.stp_mode, Some(STPMode::CancelTaker));
    }

    #[test]
    fn test_place_order_request_serialization() {
        let request = PlaceOrderRequest {
            symbol: "BTCUSDT".to_string(),
            side: OrderSide::Buy,
            price: Some("50000".to_string()),
            size: "0.001".to_string(),
            order_type: OrderType::Limit,
            force: Force::GTC,
            client_order_id: Some("test-123".to_string()),
            stp_mode: None,
            request_time: None,
            receive_window: None,
        };

        let json = serde_json::to_string(&request).unwrap();

        assert!(json.contains("\"symbol\":\"BTCUSDT\""));
        assert!(json.contains("\"side\":\"buy\""));
        assert!(json.contains("\"orderType\":\"limit\""));
        assert!(json.contains("\"force\":\"gtc\""));
        assert!(json.contains("\"price\":\"50000\""));
        assert!(json.contains("\"size\":\"0.001\""));
        assert!(json.contains("\"clientOid\":\"test-123\""));
    }

    #[test]
    fn test_place_order_response_deserialization() {
        let json = r#"{
            "orderId": "1001",
            "clientOid": "my-order-123"
        }"#;

        let response: PlaceOrderResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.order_id, "1001");
        assert_eq!(response.client_order_id, Some("my-order-123".to_string()));
    }

    #[test]
    fn test_place_order_response_deserialization_no_client_id() {
        let json = r#"{
            "orderId": "1001",
            "clientOid": null
        }"#;

        let response: PlaceOrderResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.order_id, "1001");
        assert_eq!(response.client_order_id, None);
    }
}
