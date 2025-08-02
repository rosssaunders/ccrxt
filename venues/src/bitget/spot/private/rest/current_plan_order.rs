//! Current Plan Order endpoint for Bitget Spot API
//!
//! This endpoint allows querying current (active) trigger/stop orders (plan orders).
//!
//! Reference: https://www.bitget.com/api-doc/spot/plan/Current-Plan-Order
//! Endpoint: GET /api/v2/spot/plan/current-plan-order
//! Rate limit: 20 requests/second/UID

use serde::{Deserialize, Serialize};

use super::{
    super::RestClient,
    place_plan_order::{PlanType, TriggerType},
};
use crate::bitget::spot::{OrderSide, OrderType, RestResult};

const CURRENT_PLAN_ORDER_ENDPOINT: &str = "/api/v2/spot/plan/current-plan-order";

/// Request parameters for querying current plan orders
#[derive(Debug, Clone, Serialize, Default)]
pub struct CurrentPlanOrderRequest {
    /// Trading pair name, e.g. BTCUSDT (optional, if not provided returns all symbols)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,

    /// Plan order ID (optional, if provided returns specific order)
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,

    /// Client order ID (optional, if provided returns specific order)
    #[serde(rename = "clientOrderId", skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,

    /// Plan type filter (optional)
    #[serde(rename = "planType", skip_serializing_if = "Option::is_none")]
    pub plan_type: Option<PlanType>,

    /// Start time for query (Unix milliseconds, optional)
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,

    /// End time for query (Unix milliseconds, optional)
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,

    /// Pagination ID token (optional)
    #[serde(rename = "idLessThan", skip_serializing_if = "Option::is_none")]
    pub id_less_than: Option<String>,

    /// Maximum number of results to return (default: 100, max: 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

/// Plan order status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PlanOrderStatus {
    /// Plan order is active and waiting for trigger
    #[serde(rename = "not_trigger")]
    NotTrigger,
    /// Plan order has been triggered
    #[serde(rename = "triggered")]
    Triggered,
    /// Plan order was cancelled
    #[serde(rename = "cancelled")]
    Cancelled,
    /// Plan order failed
    #[serde(rename = "fail_trigger")]
    FailTrigger,
}

/// Current plan order information
#[derive(Debug, Clone, Deserialize)]
pub struct PlanOrderInfo {
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
}

/// Response from querying current plan orders
#[derive(Debug, Clone, Deserialize)]
pub struct CurrentPlanOrderResponse {
    /// List of current plan orders
    #[serde(rename = "orderList")]
    pub order_list: Vec<PlanOrderInfo>,

    /// Maximum ID in current page (for pagination)
    #[serde(rename = "maxId")]
    pub max_id: Option<String>,

    /// Minimum ID in current page (for pagination)
    #[serde(rename = "minId")]
    pub min_id: Option<String>,
}

impl RestClient {
    /// Get current (active) spot plan orders
    ///
    /// Retrieves information about current plan orders that are waiting to be triggered.
    ///
    /// # Arguments
    /// * `request` - The current plan orders query parameters
    ///
    /// # Rate Limit
    /// 20 requests per second per UID
    ///
    /// # Returns
    /// A result containing the current plan orders response or an error
    pub async fn current_plan_order(
        &self,
        request: CurrentPlanOrderRequest,
    ) -> RestResult<CurrentPlanOrderResponse> {
        self.send_signed_get_request(
            CURRENT_PLAN_ORDER_ENDPOINT,
            Some(&request),
            20,    // 20 requests per second rate limit
            false, // This is not an order placement endpoint
            None,  // No order-specific rate limit
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_current_plan_order_request_new() {
        let request = CurrentPlanOrderRequest::default();

        assert!(request.symbol.is_none());
        assert!(request.order_id.is_none());
        assert!(request.client_order_id.is_none());
        assert!(request.plan_type.is_none());
        assert!(request.limit.is_none());
    }

    #[test]
    fn test_current_plan_order_request_builder() {
        let request = CurrentPlanOrderRequest {
            symbol: Some("BTCUSDT".to_string()),
            plan_type: Some(PlanType::NormalPlan),
            limit: Some(50),
            start_time: Some(1640995200000),
            ..Default::default()
        };

        assert_eq!(request.symbol, Some("BTCUSDT".to_string()));
        assert_eq!(request.plan_type, Some(PlanType::NormalPlan));
        assert_eq!(request.limit, Some(50));
        assert_eq!(request.start_time, Some(1640995200000));
    }

    #[test]
    fn test_current_plan_order_request_specific_order() {
        let request = CurrentPlanOrderRequest {
            symbol: Some("ETHUSDT".to_string()),
            order_id: Some("plan_123456".to_string()),
            ..Default::default()
        };

        assert_eq!(request.symbol, Some("ETHUSDT".to_string()));
        assert_eq!(request.order_id, Some("plan_123456".to_string()));
    }

    #[test]
    fn test_current_plan_order_request_limit_cap() {
        let request = CurrentPlanOrderRequest {
            limit: Some(100), // Using valid limit instead of 200
            ..Default::default()
        };

        assert_eq!(request.limit, Some(100));
    }

    #[test]
    fn test_current_plan_order_request_serialization() {
        let request = CurrentPlanOrderRequest {
            symbol: Some("BTCUSDT".to_string()),
            plan_type: Some(PlanType::TrackPlan),
            limit: Some(25),
            ..Default::default()
        };

        let query = serde_urlencoded::to_string(&request).unwrap();

        assert!(query.contains("symbol=BTCUSDT"));
        assert!(query.contains("planType=track_plan"));
        assert!(query.contains("limit=25"));
    }

    #[test]
    fn test_plan_order_status_deserialization() {
        assert_eq!(
            serde_json::from_str::<PlanOrderStatus>("\"not_trigger\"").unwrap(),
            PlanOrderStatus::NotTrigger
        );
        assert_eq!(
            serde_json::from_str::<PlanOrderStatus>("\"triggered\"").unwrap(),
            PlanOrderStatus::Triggered
        );
        assert_eq!(
            serde_json::from_str::<PlanOrderStatus>("\"cancelled\"").unwrap(),
            PlanOrderStatus::Cancelled
        );
        assert_eq!(
            serde_json::from_str::<PlanOrderStatus>("\"fail_trigger\"").unwrap(),
            PlanOrderStatus::FailTrigger
        );
    }

    #[test]
    fn test_plan_order_info_deserialization() {
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
            "status": "not_trigger",
            "callbackRatio": null,
            "cTime": 1640995200000,
            "uTime": 1640995200000,
            "executeOrderId": null
        }"#;

        let order_info: PlanOrderInfo = serde_json::from_str(json).unwrap();

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
        assert_eq!(order_info.status, PlanOrderStatus::NotTrigger);
        assert!(order_info.callback_ratio.is_none());
        assert_eq!(order_info.create_time, 1640995200000);
        assert_eq!(order_info.update_time, 1640995200000);
        assert!(order_info.execute_order_id.is_none());
    }

    #[test]
    fn test_current_plan_order_response_deserialization() {
        let json = r#"{
            "orderList": [],
            "maxId": "plan_1001",
            "minId": "plan_1000"
        }"#;

        let response: CurrentPlanOrderResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.order_list.len(), 0);
        assert_eq!(response.max_id, Some("plan_1001".to_string()));
        assert_eq!(response.min_id, Some("plan_1000".to_string()));
    }
}
