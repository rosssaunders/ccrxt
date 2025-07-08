use serde::{Deserialize, Serialize};

use crate::kucoin::{OrderSide, OrderType, ResponseHeaders, RestResponse, Result, TimeInForce};

use super::RestClient;

const PLACE_ORDER_ENDPOINT: &str = "/api/v1/orders";

/// Request for placing a new order
#[derive(Debug, Clone, Serialize)]
pub struct PlaceOrderRequest {
    /// Client order ID (optional, max 40 characters)
    #[serde(rename = "clientOid")]
    pub client_order_id: Option<String>,

    /// Order side (buy/sell)
    pub side: OrderSide,

    /// Trading symbol (e.g., "BTC-USDT")
    pub symbol: String,

    /// Order type (limit/market)
    #[serde(rename = "type")]
    pub order_type: OrderType,

    /// Order remark (optional, max 20 characters)
    pub remark: Option<String>,

    /// Self-trade prevention (optional)
    pub stp: Option<String>,

    /// Trade type (optional, TRADE for spot trading)
    #[serde(rename = "tradeType")]
    pub trade_type: Option<String>,

    /// Price (required for limit orders)
    pub price: Option<String>,

    /// Order size (required)
    pub size: Option<String>,

    /// Funds (for market buy orders instead of size)
    pub funds: Option<String>,

    /// Time in force (optional)
    #[serde(rename = "timeInForce")]
    pub time_in_force: Option<TimeInForce>,

    /// Cancel after time (optional, for GTT orders)
    #[serde(rename = "cancelAfter")]
    pub cancel_after: Option<i64>,

    /// Post only flag (optional)
    #[serde(rename = "postOnly")]
    pub post_only: Option<bool>,

    /// Hidden order flag (optional)
    pub hidden: Option<bool>,

    /// Iceberg order flag (optional)
    pub iceberg: Option<bool>,

    /// Visible size for iceberg orders (optional)
    #[serde(rename = "visibleSize")]
    pub visible_size: Option<String>,

    /// Tags (optional)
    pub tags: Option<String>,
}

/// Order placement response
#[derive(Debug, Clone, Deserialize)]
pub struct PlaceOrderResponse {
    /// Order ID assigned by KuCoin
    #[serde(rename = "orderId")]
    pub order_id: String,
}

impl Default for PlaceOrderRequest {
    fn default() -> Self {
        Self {
            client_order_id: None,
            side: OrderSide::Buy,
            symbol: String::new(),
            order_type: OrderType::Limit,
            remark: None,
            stp: None,
            trade_type: Some("TRADE".to_string()),
            price: None,
            size: None,
            funds: None,
            time_in_force: None,
            cancel_after: None,
            post_only: None,
            hidden: None,
            iceberg: None,
            visible_size: None,
            tags: None,
        }
    }
}

impl RestClient {
    /// Place a new order
    ///
    /// Reference: https://docs.kucoin.com/#place-hf-order
    pub async fn place_order(
        &self,
        request: PlaceOrderRequest,
    ) -> Result<(PlaceOrderResponse, ResponseHeaders)> {
        let body = serde_json::to_string(&request).map_err(|e| {
            crate::kucoin::ApiError::JsonParsing(format!("Failed to serialize request: {}", e))
        })?;

        let (response, headers): (RestResponse<PlaceOrderResponse>, ResponseHeaders) =
            self.post(PLACE_ORDER_ENDPOINT, &body).await?;

        Ok((response.data, headers))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_place_order_request_default() {
        let request = PlaceOrderRequest::default();
        assert_eq!(request.side, OrderSide::Buy);
        assert_eq!(request.order_type, OrderType::Limit);
        assert_eq!(request.trade_type, Some("TRADE".to_string()));
    }

    #[test]
    fn test_place_order_request_creation() {
        let request = PlaceOrderRequest {
            symbol: "BTC-USDT".to_string(),
            side: OrderSide::Sell,
            order_type: OrderType::Market,
            size: Some("0.01".to_string()),
            ..Default::default()
        };
        assert_eq!(request.symbol, "BTC-USDT");
        assert_eq!(request.side, OrderSide::Sell);
        assert_eq!(request.size, Some("0.01".to_string()));
    }
}
