//! BitMart submit order REST API endpoint
//!
//! This module implements the BitMart submit order API endpoint for placing new orders.

use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::bitmart::rate_limit::EndpointType;
use crate::bitmart::{OrderSide, OrderType, RestResult};

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_submit_order_request_limit() {
        let request = SubmitOrderRequest {
            symbol: "BTC_USDT".to_string(),
            side: OrderSide::Buy,
            order_type: OrderType::Limit,
            client_order_id: Some("test_order_123".to_string()),
            size: Some("0.001".to_string()),
            price: Some("50000.00".to_string()),
            notional: None,
        };

        assert_eq!(request.symbol, "BTC_USDT");
        assert_eq!(request.side, OrderSide::Buy);
        assert_eq!(request.order_type, OrderType::Limit);
        assert_eq!(request.client_order_id, Some("test_order_123".to_string()));
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
        assert!(request.notional.is_none());
    }

    #[test]
    fn test_submit_order_request_ioc() {
        let request = SubmitOrderRequest {
            symbol: "ETH_USDT".to_string(),
            side: OrderSide::Buy,
            order_type: OrderType::Ioc,
            client_order_id: None,
            size: Some("1.0".to_string()),
            price: Some("2500.00".to_string()),
            notional: None,
        };

        assert_eq!(request.order_type, OrderType::Ioc);
        assert_eq!(request.side, OrderSide::Buy);
        assert_eq!(request.size, Some("1.0".to_string()));
        assert_eq!(request.price, Some("2500.00".to_string()));
        assert!(request.notional.is_none());
    }

    #[test]
    fn test_submit_order_response_structure() {
        let json = r#"{"order_id": "12345"}"#;
        let response: SubmitOrderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.order_id, "12345");
    }

    #[test]
    fn test_submit_order_response_serialization() {
        let response = SubmitOrderResponse {
            order_id: "67890".to_string(),
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"order_id\":\"67890\""));
    }
}
