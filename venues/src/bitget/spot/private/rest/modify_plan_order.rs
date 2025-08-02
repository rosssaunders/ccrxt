//! Modify Plan Order endpoint for Bitget Spot API
//!
//! This endpoint allows modifying existing trigger/stop orders (plan orders) for spot trading.
//!
//! Reference: https://www.bitget.com/api-doc/spot/plan/Modify-Plan-Order
//! Endpoint: POST /api/v2/spot/plan/modify-plan-order
//! Rate limit: 10 requests/second/UID

use serde::{Deserialize, Serialize};

use super::{
    super::RestClient,
    place_order::Force,
    place_plan_order::{PlanType, TriggerType},
};
use crate::bitget::spot::{OrderSide, OrderType, RestResult};

const MODIFY_PLAN_ORDER_ENDPOINT: &str = "/api/v2/spot/plan/modify-plan-order";

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
        self.send_signed_post_request(
            MODIFY_PLAN_ORDER_ENDPOINT,
            &request,
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
        let request = ModifyPlanOrderRequest {
            symbol: "BTCUSDT".to_string(),
            order_id: Some("plan_1001".to_string()),
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
        };
        assert_eq!(request.symbol, "BTCUSDT");
        assert_eq!(request.order_id, Some("plan_1001".to_string()));
        assert!(request.client_order_id.is_none());
        assert!(request.side.is_none());
        assert!(request.order_type.is_none());
        assert!(request.trigger_price.is_none());
    }

    #[test]
    fn test_modify_plan_order_request_by_client_order_id() {
        let request = ModifyPlanOrderRequest {
            symbol: "ETHUSDT".to_string(),
            order_id: None,
            client_order_id: Some("my-plan-123".to_string()),
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
        };
        assert_eq!(request.symbol, "ETHUSDT");
        assert!(request.order_id.is_none());
        assert_eq!(request.client_order_id, Some("my-plan-123".to_string()));
    }

    #[test]
    fn test_modify_plan_order_request_builder() {
        let request = ModifyPlanOrderRequest {
            symbol: "BTCUSDT".to_string(),
            order_id: Some("plan_1001".to_string()),
            client_order_id: None,
            side: None,
            order_type: None,
            force: Some(Force::PostOnly),
            plan_type: None,
            trigger_type: None,
            trigger_price: Some("51000".to_string()),
            price: Some("52000".to_string()),
            size: Some("0.002".to_string()),
            callback_ratio: None,
            request_time: None,
            receive_window: None,
        };
        assert_eq!(request.trigger_price, Some("51000".to_string()));
        assert_eq!(request.price, Some("52000".to_string()));
        assert_eq!(request.size, Some("0.002".to_string()));
        assert_eq!(request.force, Some(Force::PostOnly));
    }

    #[test]
    fn test_modify_plan_order_request_track_plan() {
        let request = ModifyPlanOrderRequest {
            symbol: "BTCUSDT".to_string(),
            order_id: Some("plan_1001".to_string()),
            client_order_id: None,
            side: None,
            order_type: None,
            force: None,
            plan_type: Some(PlanType::TrackPlan),
            trigger_type: Some(TriggerType::MarkPrice),
            trigger_price: None,
            price: None,
            size: None,
            callback_ratio: Some("0.2".to_string()),
            request_time: None,
            receive_window: None,
        };
        assert_eq!(request.plan_type, Some(PlanType::TrackPlan));
        assert_eq!(request.callback_ratio, Some("0.2".to_string()));
        assert_eq!(request.trigger_type, Some(TriggerType::MarkPrice));
    }
    #[test]
    fn test_modify_plan_order_request_serialization() {
        let request = ModifyPlanOrderRequest {
            symbol: "BTCUSDT".to_string(),
            order_id: Some("plan_1001".to_string()),
            client_order_id: None,
            side: None,
            order_type: None,
            force: None,
            plan_type: None,
            trigger_type: None,
            trigger_price: Some("51000".to_string()),
            price: Some("52000".to_string()),
            size: None,
            callback_ratio: None,
            request_time: None,
            receive_window: None,
        };
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
