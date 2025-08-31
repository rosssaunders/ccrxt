//! BitMart submit margin order REST API endpoint
//!
//! This module implements the BitMart margin order API endpoint for placing margin orders.

use serde::{Deserialize, Serialize};

use crate::bitmart::{
    OrderSide, OrderType, RestResult, rate_limit::EndpointType, spot::private_client::RestClient,
};

const SUBMIT_MARGIN_ORDER_ENDPOINT: &str = "/spot/v1/margin/submit_order";

/// Request parameters for submitting a new margin order
#[derive(Debug, Serialize)]
pub struct SubmitMarginOrderRequest {
    /// Trading pair (e.g. BTC_USDT)
    pub symbol: String,
    /// Order side (buy/sell)
    pub side: OrderSide,
    /// Order type (limit/market/limit_maker/ioc)
    #[serde(rename = "type")]
    pub order_type: OrderType,
    /// Client-defined OrderId (optional, max 32 characters)
    #[serde(rename = "clientOrderId", skip_serializing_if = "Option::is_none")]
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

/// Response for submitting a new margin order
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitMarginOrderResponse {
    /// Order ID
    #[serde(rename = "orderId")]
    pub order_id: String,
}

impl RestClient {
    /// New Margin Order (v1)
    ///
    /// Places a new margin order on the BitMart exchange.
    ///
    /// [docs](https://developer-pro.bitmart.com/en/spot/#new-margin-orderv1-signed)
    ///
    /// Rate limit: varies by endpoint type
    ///
    /// # Arguments
    /// * `request` - The margin order request parameters
    ///
    /// # Returns
    /// Margin order submission response with order ID
    pub async fn submit_margin_order(
        &self,
        request: SubmitMarginOrderRequest,
    ) -> RestResult<SubmitMarginOrderResponse> {
        self.send_post_signed_request(
            SUBMIT_MARGIN_ORDER_ENDPOINT,
            request,
            EndpointType::SpotTrading,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_submit_margin_order_request_limit() {
        let request = SubmitMarginOrderRequest {
            symbol: "BTC_USDT".to_string(),
            side: OrderSide::Buy,
            order_type: OrderType::Limit,
            client_order_id: Some("margin_order_123".to_string()),
            size: Some("0.001".to_string()),
            price: Some("50000.00".to_string()),
            notional: None,
        };

        assert_eq!(request.symbol, "BTC_USDT");
        assert_eq!(request.side, OrderSide::Buy);
        assert_eq!(request.order_type, OrderType::Limit);
        assert_eq!(
            request.client_order_id,
            Some("margin_order_123".to_string())
        );
        assert_eq!(request.size, Some("0.001".to_string()));
        assert_eq!(request.price, Some("50000.00".to_string()));
        assert!(request.notional.is_none());
    }

    #[test]
    fn test_submit_margin_order_request_market_buy() {
        let request = SubmitMarginOrderRequest {
            symbol: "ETH_USDT".to_string(),
            side: OrderSide::Buy,
            order_type: OrderType::Market,
            client_order_id: None,
            size: None,
            price: None,
            notional: Some("100.00".to_string()),
        };

        assert_eq!(request.symbol, "ETH_USDT");
        assert_eq!(request.side, OrderSide::Buy);
        assert_eq!(request.order_type, OrderType::Market);
        assert!(request.client_order_id.is_none());
        assert!(request.size.is_none());
        assert!(request.price.is_none());
        assert_eq!(request.notional, Some("100.00".to_string()));
    }

    #[test]
    fn test_submit_margin_order_request_market_sell() {
        let request = SubmitMarginOrderRequest {
            symbol: "BTC_USDT".to_string(),
            side: OrderSide::Sell,
            order_type: OrderType::Market,
            client_order_id: None,
            size: Some("0.01".to_string()),
            price: None,
            notional: None,
        };

        assert_eq!(request.symbol, "BTC_USDT");
        assert_eq!(request.side, OrderSide::Sell);
        assert_eq!(request.order_type, OrderType::Market);
        assert!(request.client_order_id.is_none());
        assert_eq!(request.size, Some("0.01".to_string()));
        assert!(request.price.is_none());
        assert!(request.notional.is_none());
    }
}
