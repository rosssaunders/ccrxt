//! Plan Sub Order endpoint for Bitget Spot API
//!
//! This endpoint allows querying details of a specific trigger/stop order (plan order).
//!
//! Reference: https://www.bitget.com/api-doc/spot/plan/Plan-Sub-Order
//! Endpoint: GET /api/v2/spot/plan/plan-sub-order
//! Rate limit: 20 requests/second/UID

use serde::{Deserialize, Serialize};

use super::super::RestClient;
use crate::bitget::{OrderSide, OrderType, RestResult};
use super::place_plan_order::{PlanType, TriggerType};
use super::current_plan_order::PlanOrderStatus;

/// Request parameters for querying plan sub order details
#[derive(Debug, Clone, Serialize)]
pub struct PlanSubOrderRequest {
    /// Trading pair name, e.g. BTCUSDT
    pub symbol: String,

    /// Plan order ID (either orderId or clientOrderId is required)
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,

    /// Client order ID (either orderId or clientOrderId is required)
    #[serde(rename = "clientOrderId", skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
}

impl PlanSubOrderRequest {
    /// Create a request to query plan order details by order ID
    pub fn by_order_id(symbol: impl Into<String>, order_id: impl Into<String>) -> Self {
        Self {
            symbol: symbol.into(),
            order_id: Some(order_id.into()),
            client_order_id: None,
        }
    }

    /// Create a request to query plan order details by client order ID
    pub fn by_client_order_id(
        symbol: impl Into<String>,
        client_order_id: impl Into<String>,
    ) -> Self {
        Self {
            symbol: symbol.into(),
            order_id: None,
            client_order_id: Some(client_order_id.into()),
        }
    }
}

/// Detailed plan order information
#[derive(Debug, Clone, Deserialize)]
pub struct PlanSubOrderInfo {
    /// Plan order ID
    #[serde(rename = "orderId")]
    pub order_id: String,

    /// Client order ID (if provided)
    #[serde(rename = "clientOid")]
    pub client_order_id: Option<String>,

    /// Trading pair name
    pub symbol: String,

    /// Order direction
    pub side: OrderSide,

    /// Order type
    #[serde(rename = "orderType")]
    pub order_type: OrderType,

    /// Plan order type
    #[serde(rename = "planType")]
    pub plan_type: PlanType,

    /// Trigger price type
    #[serde(rename = "triggerType")]
    pub trigger_type: TriggerType,

    /// Trigger price
    #[serde(rename = "triggerPrice")]
    pub trigger_price: String,

    /// Limit price (if applicable)
    pub price: Option<String>,

    /// Order size
    pub size: String,

    /// Plan order status
    pub status: PlanOrderStatus,

    /// Callback ratio for track plans (if applicable)
    #[serde(rename = "callbackRatio")]
    pub callback_ratio: Option<String>,

    /// Order creation time (Unix milliseconds)
    #[serde(rename = "cTime")]
    pub create_time: i64,

    /// Order update time (Unix milliseconds)
    #[serde(rename = "uTime")]
    pub update_time: i64,

    /// Executed order ID (if triggered)
    #[serde(rename = "executeOrderId")]
    pub execute_order_id: Option<String>,

    /// Trigger time (Unix milliseconds, if triggered)
    #[serde(rename = "triggerTime")]
    pub trigger_time: Option<i64>,

    /// Failure reason (if plan order failed)
    #[serde(rename = "failReason")]
    pub fail_reason: Option<String>,

    /// Execution strategy
    pub force: Option<String>,

    /// Self-trade prevention mode
    #[serde(rename = "stpMode")]
    pub stp_mode: Option<String>,
}

/// Response from querying plan sub order details
#[derive(Debug, Clone, Deserialize)]
pub struct PlanSubOrderResponse {
    /// Plan order details
    #[serde(rename = "orderInfo")]
    pub order_info: PlanSubOrderInfo,
}

impl RestClient {
    /// Get detailed information about a specific spot plan order
    ///
    /// Retrieves detailed information about a specific plan order including
    /// execution details, trigger information, and status.
    ///
    /// # Arguments
    /// * `request` - The plan sub order query parameters
    ///
    /// # Rate Limit
    /// 20 requests per second per UID
    ///
    /// # Returns
    /// A result containing the plan sub order response or an error
    pub async fn plan_sub_order(
        &self,
        request: PlanSubOrderRequest,
    ) -> RestResult<PlanSubOrderResponse> {
        let query_params = serde_urlencoded::to_string(&request).map_err(|e| {
            crate::bitget::Errors::Error(format!("Failed to serialize query parameters: {e}"))
        })?;

        self.send_signed_request(
            "/api/v2/spot/plan/plan-sub-order",
            reqwest::Method::GET,
            Some(&query_params), // Query parameters
            None,                // No body
            20,                  // 20 requests per second rate limit
            false,               // This is not an order placement endpoint
            None,                // No order-specific rate limit
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plan_sub_order_request_by_order_id() {
        let request = PlanSubOrderRequest::by_order_id("BTCUSDT", "plan_1234567890");

        assert_eq!(request.symbol, "BTCUSDT");
        assert_eq!(request.order_id, Some("plan_1234567890".to_string()));
        assert!(request.client_order_id.is_none());
    }

    #[test]
    fn test_plan_sub_order_request_by_client_order_id() {
        let request = PlanSubOrderRequest::by_client_order_id("ETHUSDT", "my-plan-order-123");

        assert_eq!(request.symbol, "ETHUSDT");
        assert!(request.order_id.is_none());
        assert_eq!(request.client_order_id, Some("my-plan-order-123".to_string()));
    }

    #[test]
    fn test_plan_sub_order_request_serialization() {
        let request = PlanSubOrderRequest::by_order_id("BTCUSDT", "plan_1234567890");
        let query = serde_urlencoded::to_string(&request).unwrap();

        assert!(query.contains("symbol=BTCUSDT"));
        assert!(query.contains("orderId=plan_1234567890"));
        assert!(!query.contains("clientOrderId"));
    }

    #[test]
    fn test_plan_sub_order_request_serialization_client_id() {
        let request = PlanSubOrderRequest::by_client_order_id("ETHUSDT", "my-plan-order-123");
        let query = serde_urlencoded::to_string(&request).unwrap();

        assert!(query.contains("symbol=ETHUSDT"));
        assert!(query.contains("clientOrderId=my-plan-order-123"));
        assert!(!query.contains("orderId"));
    }

    #[test]
    fn test_plan_sub_order_info_deserialization() {
        let json = r#"{
            "orderId": "plan_1001",
            "clientOid": "my-plan-123",
            "symbol": "BTCUSDT",
            "side": "buy",
            "orderType": "limit",
            "planType": "normal_plan",
            "triggerType": "fill_price",
            "triggerPrice": "49000",
            "price": "50000",
            "size": "0.001",
            "status": "triggered",
            "callbackRatio": null,
            "cTime": 1640995200000,
            "uTime": 1640995300000,
            "executeOrderId": "exec_2001",
            "triggerTime": 1640995250000,
            "failReason": null,
            "force": "gtc",
            "stpMode": "none"
        }"#;

        let order_info: PlanSubOrderInfo = serde_json::from_str(json).unwrap();

        assert_eq!(order_info.order_id, "plan_1001");
        assert_eq!(order_info.client_order_id, Some("my-plan-123".to_string()));
        assert_eq!(order_info.symbol, "BTCUSDT");
        assert_eq!(order_info.side, OrderSide::Buy);
        assert_eq!(order_info.order_type, OrderType::Limit);
        assert_eq!(order_info.plan_type, PlanType::NormalPlan);
        assert_eq!(order_info.trigger_type, TriggerType::FillPrice);
        assert_eq!(order_info.trigger_price, "49000");
        assert_eq!(order_info.price, Some("50000".to_string()));
        assert_eq!(order_info.size, "0.001");
        assert_eq!(order_info.status, PlanOrderStatus::Triggered);
        assert!(order_info.callback_ratio.is_none());
        assert_eq!(order_info.create_time, 1640995200000);
        assert_eq!(order_info.update_time, 1640995300000);
        assert_eq!(order_info.execute_order_id, Some("exec_2001".to_string()));
        assert_eq!(order_info.trigger_time, Some(1640995250000));
        assert!(order_info.fail_reason.is_none());
        assert_eq!(order_info.force, Some("gtc".to_string()));
        assert_eq!(order_info.stp_mode, Some("none".to_string()));
    }

    #[test]
    fn test_plan_sub_order_info_deserialization_failed() {
        let json = r#"{
            "orderId": "plan_1002",
            "clientOid": null,
            "symbol": "ETHUSDT",
            "side": "sell",
            "orderType": "market",
            "planType": "track_plan",
            "triggerType": "mark_price",
            "triggerPrice": "2900",
            "price": null,
            "size": "1.0",
            "status": "fail_trigger",
            "callbackRatio": "0.1",
            "cTime": 1640995200000,
            "uTime": 1640995300000,
            "executeOrderId": null,
            "triggerTime": null,
            "failReason": "Insufficient balance",
            "force": null,
            "stpMode": null
        }"#;

        let order_info: PlanSubOrderInfo = serde_json::from_str(json).unwrap();

        assert_eq!(order_info.order_id, "plan_1002");
        assert!(order_info.client_order_id.is_none());
        assert_eq!(order_info.symbol, "ETHUSDT");
        assert_eq!(order_info.side, OrderSide::Sell);
        assert_eq!(order_info.order_type, OrderType::Market);
        assert_eq!(order_info.plan_type, PlanType::TrackPlan);
        assert_eq!(order_info.trigger_type, TriggerType::MarkPrice);
        assert_eq!(order_info.trigger_price, "2900");
        assert!(order_info.price.is_none());
        assert_eq!(order_info.size, "1.0");
        assert_eq!(order_info.status, PlanOrderStatus::FailTrigger);
        assert_eq!(order_info.callback_ratio, Some("0.1".to_string()));
        assert_eq!(order_info.create_time, 1640995200000);
        assert_eq!(order_info.update_time, 1640995300000);
        assert!(order_info.execute_order_id.is_none());
        assert!(order_info.trigger_time.is_none());
        assert_eq!(order_info.fail_reason, Some("Insufficient balance".to_string()));
        assert!(order_info.force.is_none());
        assert!(order_info.stp_mode.is_none());
    }

    #[test]
    fn test_plan_sub_order_response_deserialization() {
        let json = r#"{
            "orderInfo": {
                "orderId": "plan_1001",
                "clientOid": "my-plan-123",
                "symbol": "BTCUSDT",
                "side": "buy",
                "orderType": "limit",
                "planType": "normal_plan",
                "triggerType": "fill_price",
                "triggerPrice": "49000",
                "price": "50000",
                "size": "0.001",
                "status": "not_trigger",
                "callbackRatio": null,
                "cTime": 1640995200000,
                "uTime": 1640995200000,
                "executeOrderId": null,
                "triggerTime": null,
                "failReason": null,
                "force": "gtc",
                "stpMode": "none"
            }
        }"#;

        let response: PlanSubOrderResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.order_info.order_id, "plan_1001");
        assert_eq!(response.order_info.status, PlanOrderStatus::NotTrigger);
    }
}
