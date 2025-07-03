//! Modify Plan Order endpoint for Bitget Spot API
//!
//! This endpoint allows modifying existing trigger/stop orders (plan orders) for spot trading.
//!
//! Reference: https://www.bitget.com/api-doc/spot/plan/Modify-Plan-Order
//! Endpoint: POST /api/v2/spot/plan/modify-plan-order
//! Rate limit: 10 requests/second/UID

use serde::{Deserialize, Serialize};

use super::super::RestClient;
use crate::bitget::{OrderSide, OrderType, RestResult};
use super::place_order::Force;
use super::place_plan_order::{PlanType, TriggerType};

/// Request parameters for modifying a plan order
#[derive(Debug, Clone, Serialize)]
pub struct ModifyPlanOrderRequest {
    /// Trading pair name, e.g. BTCUSDT
    pub symbol: String,

    /// Plan order ID to modify (either orderId or clientOrderId is required)
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,

    /// Client order ID to modify (either orderId or clientOrderId is required)
    #[serde(rename = "clientOrderId", skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,

    /// New order direction: buy or sell (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<OrderSide>,

    /// New order type: limit or market (optional)
    #[serde(rename = "orderType", skip_serializing_if = "Option::is_none")]
    pub order_type: Option<OrderType>,

    /// New execution strategy (optional, invalid when orderType is market)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<Force>,

    /// New plan order type (optional)
    #[serde(rename = "planType", skip_serializing_if = "Option::is_none")]
    pub plan_type: Option<PlanType>,

    /// New trigger price type (optional)
    #[serde(rename = "triggerType", skip_serializing_if = "Option::is_none")]
    pub trigger_type: Option<TriggerType>,

    /// New trigger price (optional)
    #[serde(rename = "triggerPrice", skip_serializing_if = "Option::is_none")]
    pub trigger_price: Option<String>,

    /// New limit price (optional, required for limit orders)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,

    /// New order amount (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,

    /// New callback rate for track plan orders (optional, percentage)
    /// Only valid when planType is track_plan
    #[serde(rename = "callbackRatio", skip_serializing_if = "Option::is_none")]
    pub callback_ratio: Option<String>,

    /// Request timestamp (Unix milliseconds)
    #[serde(rename = "requestTime", skip_serializing_if = "Option::is_none")]
    pub request_time: Option<i64>,

    /// Valid time window (Unix milliseconds)
    /// If set, request is valid only when server time is within receiveWindow
    #[serde(rename = "receiveWindow", skip_serializing_if = "Option::is_none")]
    pub receive_window: Option<i64>,
}

impl ModifyPlanOrderRequest {
    /// Create a request to modify a plan order by order ID
    pub fn by_order_id(symbol: impl Into<String>, order_id: impl Into<String>) -> Self {
        Self {
            symbol: symbol.into(),
            order_id: Some(order_id.into()),
            client_order_id: None,
            side: None,
            order_type: None,
            force: None,
            plan_type: None,
            trigger_type: None,
            trigger_price: None,
            price: None,
            size: None,
            callback_ratio: None,
            request_time: None,
            receive_window: None,
        }
    }

    /// Create a request to modify a plan order by client order ID
    pub fn by_client_order_id(
        symbol: impl Into<String>,
        client_order_id: impl Into<String>,
    ) -> Self {
        Self {
            symbol: symbol.into(),
            order_id: None,
            client_order_id: Some(client_order_id.into()),
            side: None,
            order_type: None,
            force: None,
            plan_type: None,
            trigger_type: None,
            trigger_price: None,
            price: None,
            size: None,
            callback_ratio: None,
            request_time: None,
            receive_window: None,
        }
    }

    /// Set the order side
    pub fn side(mut self, side: OrderSide) -> Self {
        self.side = Some(side);
        self
    }

    /// Set the order type
    pub fn order_type(mut self, order_type: OrderType) -> Self {
        self.order_type = Some(order_type);
        self
    }

    /// Set the execution force/strategy
    pub fn force(mut self, force: Force) -> Self {
        self.force = Some(force);
        self
    }

    /// Set the plan type
    pub fn plan_type(mut self, plan_type: PlanType) -> Self {
        self.plan_type = Some(plan_type);
        self
    }

    /// Set the trigger type
    pub fn trigger_type(mut self, trigger_type: TriggerType) -> Self {
        self.trigger_type = Some(trigger_type);
        self
    }

    /// Set the trigger price
    pub fn trigger_price(mut self, trigger_price: impl Into<String>) -> Self {
        self.trigger_price = Some(trigger_price.into());
        self
    }

    /// Set the limit price
    pub fn price(mut self, price: impl Into<String>) -> Self {
        self.price = Some(price.into());
        self
    }

    /// Set the order size
    pub fn size(mut self, size: impl Into<String>) -> Self {
        self.size = Some(size.into());
        self
    }

    /// Set the callback ratio for track plan orders
    pub fn callback_ratio(mut self, callback_ratio: impl Into<String>) -> Self {
        self.callback_ratio = Some(callback_ratio.into());
        self
    }

    /// Set the request timestamp
    pub fn request_time(mut self, request_time: i64) -> Self {
        self.request_time = Some(request_time);
        self
    }

    /// Set the receive window
    pub fn receive_window(mut self, receive_window: i64) -> Self {
        self.receive_window = Some(receive_window);
        self
    }
}

/// Response from modifying a plan order
#[derive(Debug, Clone, Deserialize)]
pub struct ModifyPlanOrderResponse {
    /// Plan order ID
    #[serde(rename = "orderId")]
    pub order_id: String,

    /// Client order ID (if provided)
    #[serde(rename = "clientOid")]
    pub client_order_id: Option<String>,
}

impl RestClient {
    /// Modify a spot plan order (trigger/stop order)
    ///
    /// Modifies an existing plan order for spot trading with the specified parameters.
    /// Only the provided fields will be updated.
    ///
    /// # Arguments
    /// * `request` - The plan order modification request parameters
    ///
    /// # Rate Limit
    /// 10 requests per second per UID
    ///
    /// # Returns
    /// A result containing the plan order modification response or an error
    pub async fn modify_plan_order(
        &self,
        request: ModifyPlanOrderRequest,
    ) -> RestResult<ModifyPlanOrderResponse> {
        let body = serde_json::to_string(&request).map_err(|e| {
            crate::bitget::Errors::Error(format!("Failed to serialize request: {e}"))
        })?;

        self.send_signed_request(
            "/api/v2/spot/plan/modify-plan-order",
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
    fn test_modify_plan_order_request_by_order_id() {
        let request = ModifyPlanOrderRequest::by_order_id("BTCUSDT", "plan_1001");

        assert_eq!(request.symbol, "BTCUSDT");
        assert_eq!(request.order_id, Some("plan_1001".to_string()));
        assert!(request.client_order_id.is_none());
        assert!(request.side.is_none());
        assert!(request.order_type.is_none());
        assert!(request.trigger_price.is_none());
    }

    #[test]
    fn test_modify_plan_order_request_by_client_order_id() {
        let request = ModifyPlanOrderRequest::by_client_order_id("ETHUSDT", "my-plan-123");

        assert_eq!(request.symbol, "ETHUSDT");
        assert!(request.order_id.is_none());
        assert_eq!(request.client_order_id, Some("my-plan-123".to_string()));
    }

    #[test]
    fn test_modify_plan_order_request_builder() {
        let request = ModifyPlanOrderRequest::by_order_id("BTCUSDT", "plan_1001")
            .trigger_price("51000")
            .price("52000")
            .size("0.002")
            .force(Force::PostOnly);

        assert_eq!(request.trigger_price, Some("51000".to_string()));
        assert_eq!(request.price, Some("52000".to_string()));
        assert_eq!(request.size, Some("0.002".to_string()));
        assert_eq!(request.force, Some(Force::PostOnly));
    }

    #[test]
    fn test_modify_plan_order_request_track_plan() {
        let request = ModifyPlanOrderRequest::by_order_id("BTCUSDT", "plan_1001")
            .plan_type(PlanType::TrackPlan)
            .callback_ratio("0.2")
            .trigger_type(TriggerType::MarkPrice);

        assert_eq!(request.plan_type, Some(PlanType::TrackPlan));
        assert_eq!(request.callback_ratio, Some("0.2".to_string()));
        assert_eq!(request.trigger_type, Some(TriggerType::MarkPrice));
    }

    #[test]
    fn test_modify_plan_order_request_serialization() {
        let request = ModifyPlanOrderRequest::by_order_id("BTCUSDT", "plan_1001")
            .trigger_price("51000")
            .price("52000");

        let json = serde_json::to_string(&request).unwrap();

        assert!(json.contains("\"symbol\":\"BTCUSDT\""));
        assert!(json.contains("\"orderId\":\"plan_1001\""));
        assert!(json.contains("\"triggerPrice\":\"51000\""));
        assert!(json.contains("\"price\":\"52000\""));
        // Should not contain fields that weren't set
        assert!(!json.contains("clientOrderId"));
        assert!(!json.contains("side"));
        assert!(!json.contains("orderType"));
    }

    #[test]
    fn test_modify_plan_order_response_deserialization() {
        let json = r#"{
            "orderId": "plan_1001",
            "clientOid": "my-plan-123"
        }"#;

        let response: ModifyPlanOrderResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.order_id, "plan_1001");
        assert_eq!(response.client_order_id, Some("my-plan-123".to_string()));
    }

    #[test]
    fn test_modify_plan_order_response_deserialization_no_client_id() {
        let json = r#"{
            "orderId": "plan_1001",
            "clientOid": null
        }"#;

        let response: ModifyPlanOrderResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.order_id, "plan_1001");
        assert!(response.client_order_id.is_none());
    }
}
